
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Web Crawler Binary
/// 
/// The purpose of this program is to get the projects information from the web.
/// Using the database as configurationand outputing it's scraped data into the database for later usage.
/// 
/// Arguments:
///     arg[0] = debug -> if you pass the "debug" word into the first argument, the application will run
///                       in debug mode.
/// 
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////



/// External imports
use std::{sync, thread, env, time::Duration, collections::HashMap};
use std::sync::mpsc::channel;
use sync::{Arc, Mutex, atomic, mpsc};
use atomic::{AtomicBool, Ordering};
use uuid::Uuid;
use ctrlc;

#[macro_use]
extern crate lazy_static;


/// Internal lib crates
use logger;
use crate::logger::setup_logger;
use database_connector::models::*;
use database_connector::functions::*;

// Static program variables
lazy_static! {
    static ref TARGETS: Arc<Mutex<HashMap<Uuid, Target>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref RUNNING: Arc<AtomicBool> = Arc::new(AtomicBool::new(true)).clone();
    static ref RUNNING_CHANNEL: (Arc<Mutex<mpsc::Sender<()>>>, Arc<Mutex<mpsc::Receiver<()>>>) = setup_channel();
}

fn setup_channel() -> (Arc<Mutex<mpsc::Sender<()>>>, Arc<Mutex<mpsc::Receiver<()>>>) {
    let (send, recv) = channel();
    return (Arc::new(Mutex::new(send)), Arc::new(Mutex::new(recv)));
}

///////////////////////////////////////////////////////////////////////////////
/// Web Crawler
///////////////////////////////////////////////////////////////////////////////
fn main() {

    // Setup Configurations /////////////////////////////////////////////////

    // Setup the program logs
    setup_logger().expect("");

    // Configure SIGTERM and SIGHUP handling
    // For CRL + C listening
    set_handler();
    

    // For debug ataching purposes //////////////////////////////////////////
    let mut debug: bool = false;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let first_arg: &str = &args[1];
        let s: String = String::from("debug");

        debug = first_arg.eq(&s);
    }

    if debug {
        log::info!("Crawler launched in debug mode");

        let five_secs = Duration::from_secs(5);
        sleep(five_secs);
    }
    /////////////////////////////////////////////////////////////////////////


    // Startup
    log::info!("Program starting ...");

    
    // Get all initial information from db
    log::info!("Loading targets ...");
    get_targets();


    // Startup the service with the targets already cached in memory
    log::info!("Starting crawler...");
    let mut threads = Vec::new();

    // Set a producer to get the targets data, crawl and send data to a consumer
    let target_updater = thread::spawn(move || target_updater());
    threads.push(target_updater);

    let crawler_service = thread::spawn(move || crawler_service());
    threads.push(crawler_service);

    // Wait for the threads to complete any remaining work
    for thread in threads {
        thread.join().expect("oops! the {thread.name} thread panicked");
    }

    log::info!("Closing Crawler...");
    log::info!("Process finished");
}

///////////////////////////////////////////////////////////////
/// Configures SIGTERM and SIGHUP handling
/// For CRL + C listening
///////////////////////////////////////////////////////////////
fn set_handler() {

    let r = RUNNING.clone();
    ctrlc::set_handler(move || {
        log::info!("Received signal to stop application");
        // Update Atomic bool
        r.store(false, Ordering::SeqCst);
        // Interrupt/invoke_timeout sleeping threads
        let _ = RUNNING_CHANNEL.0.lock().unwrap().send(());
    }).expect("Error setting Ctrl-C handler");

}

///////////////////////////////////////////////////////////////
/// Interruptible sleep function
///////////////////////////////////////////////////////////////
fn sleep(duration: Duration) {
    
    // Check if the atomic bool RUNNING is already deactivated
    let running = RUNNING.clone();
    if running.load(Ordering::Acquire) {

        // If not, we wait
        if let Ok(_) = RUNNING_CHANNEL.1.lock().unwrap().recv_timeout(duration) {
            // Sleep was interrupted
            return;
        }
    }

}

////////////////////////////////////////////////////////////////////////
/// This function/method loads all initial target data 
/// into memory cache before starting to crawl.
////////////////////////////////////////////////////////////////////////
fn get_targets() {

    let targets_vec = get_active_targets();
    for target in targets_vec {
        TARGETS.lock().unwrap().insert(target.guid, target);
    }

    log::info!("Database loaded to memory");
    
}

/////////////////////////////////////////////////////////////////
/// This function/service will updates the 'TARGETS' vector
/// with data from the database_connector.
/////////////////////////////////////////////////////////////////
fn target_updater() {
    log::info!("Target updater service started");
    
    loop {
        //let five_secs = Duration::from_secs(120000);
        let five_secs: Duration = Duration::from_secs(5);
        sleep(five_secs);
        
        let running: Arc<AtomicBool> = RUNNING.clone();
        if !running.load(Ordering::Acquire) {
            break;
        }

        // Reload targets
        let targets_vec: Vec<Target> = get_active_targets();

        let mut lock = TARGETS.lock();
        let targets = lock.as_mut().unwrap();

        let mut loaded_targets_hash: HashMap<&Uuid, &Target> = HashMap::new();

        let t = targets.clone();
        for hash_target in t.iter() {
            loaded_targets_hash.insert(&hash_target.0, &hash_target.1);
        }

        for target in targets_vec {
            // Compare the Target object and pop it from an cloned targets_hash
            // to later pop the non-existing ones in the database.
            match targets.get(&target.guid) {
                Some(loaded_target) => {
                    loaded_targets_hash.remove(&target.guid);
                    
                    // Compare and alter
                    if loaded_target.eq(&target) {
                        continue;
                    } else {
                        //let mut mut_targets = lock.unwrap();
                        targets.entry(target.guid).and_modify(|t: &mut Target| { *t = target }); // TODO - To test
                        //drop(mut_targets);
                    }
                },
                None => {
                    // Add it
                    //let mut mut_targets = lock.unwrap();
                    targets.insert(target.guid, target); // TODO - To test
                    //drop(mut_targets);
                }
            }
        }
        
        for (uuid, _) in loaded_targets_hash.iter() {
            // Remove
            //let mut mut_targets = lock.unwrap();
            targets.remove(uuid); // TODO - To test
            //drop(mut_targets);
        }
        drop(lock);
    }

    log::info!("Target updater service exited");
}

fn crawler_service() {
    log::info!("Crawler Service started");

    loop {
        let running = RUNNING.clone();
        if !running.load(Ordering::Acquire) {
            break;
        }


    }

    log::info!("Crawler Service exited");
}
