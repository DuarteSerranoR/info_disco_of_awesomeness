table! {
    targets (guid) {
        guid -> Uuid,
        name -> Varchar,
        url -> Varchar,
        active -> Bool,
        interval -> Int4,
        last_crawl -> Nullable<Timestamp>,
        creation_time -> Nullable<Timestamp>,
    }
}
