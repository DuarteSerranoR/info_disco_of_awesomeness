use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc,Mutex};
use std::collections::HashMap;
use uuid::Uuid;
use std::{thread, time::Duration};
use ctrlc;
//use std::sync::mpsc::channel

// Internal crates
use logger;
use crate::logger::setup_logger;
use database_connector::models::*;
use database_connector::functions::*;

#[macro_use]
extern crate lazy_static;

//static NTHREADS: i32 = 3;

lazy_static! {
    static ref TARGETS: Arc<Mutex<HashMap<Uuid, Target>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref RUNNING: Arc<AtomicBool> = Arc::new(AtomicBool::new(true)).clone();
}

fn main() {
    setup_logger().expect("");
    // Configure SIGTERM and SIGHUP handling
    // For CRL + C listening
    set_handler();
    log::info!("Program starting ...");
    
    // Get all initial information from db
    log::info!("Loading targets ...");
    get_targets();
    //let targets = getTargets();

    
    let mut threads = Vec::new();

    // Set a producer that handles the number of active targets being crawled and creates a queue
    //let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    // Set a producer to get the targets data, crawl and send data to a consumer
    let producer = thread::spawn(move || target_producer());
    let consumer = thread::spawn(move || data_consumer());
    let crawler = thread::spawn(move || crawler_service());
    threads.push(producer);
    threads.push(consumer);
    threads.push(crawler);

    // Wait for the threads to complete any remaining work
    for thread in threads {
        thread.join().expect("oops! the {thread.name} thread panicked");
    }
    //crawler_service();
    //data_consumer();

    log::info!("Closing Crawler...");
    log::info!("Process finished");
}

fn set_handler() {
    let r = RUNNING.clone();
    ctrlc::set_handler(move || {
        log::info!("Received signal to stop application");
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
}

fn get_targets() {

    // Hardcoded targets to later implement
    let targets_vec = get_active_targets();
    for target in targets_vec {
        TARGETS.lock().unwrap().insert(target.guid, target );
    }

    log::info!("Database loaded to memory");
    log::info!("Starting crawler...");

    for (key, value) in TARGETS.lock().unwrap().iter() {
        log::info!("{} / {}", key, value.name);
    }
    
}

fn target_producer() {
    log::info!("producer Service started");
    
    // let d = Duration::from_millis(10);
    loop {
        let running = RUNNING.clone();
        if !running.load(Ordering::Acquire) {
            break;
        }

        log::info!("work1");
        thread::sleep(Duration::from_secs(2));
        log::info!("work2");
    }
}


fn crawler_service() {
    log::info!("Crawler Service started");
    
    // let d = Duration::from_millis(10);
    loop {
        let running = RUNNING.clone();
        if !running.load(Ordering::Acquire) {
            break;
        }

        log::info!("work1");
        thread::sleep(Duration::from_secs(2));
        log::info!("work2");
        //rx.recv_timeout(d);
    }
}


fn data_consumer() {
    log::info!("DataConsumer Service started");
    
    // let d = Duration::from_millis(10);
    loop {
        let running = RUNNING.clone();
        if !running.load(Ordering::Acquire) {
            break;
        }

        log::info!("work1");
        thread::sleep(Duration::from_secs(2));
        log::info!("work2");

        //rx.recv_timeout(d);
    }
}
