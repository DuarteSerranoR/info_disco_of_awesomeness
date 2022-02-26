table! {
    articles (guid) {
        guid -> Varchar,
        targert_guid -> Uuid,
        title -> Varchar,
        summary -> Nullable<Varchar>,
        body -> Varchar,
        date -> Timestamp,
        author -> Nullable<Varchar>,
        link -> Varchar,
    }
}

table! {
    targets (guid) {
        guid -> Uuid,
        name -> Varchar,
        url -> Varchar,
        active -> Bool,
        interval -> Int4,
        last_crawl -> Nullable<Timestamp>,
        creation_time -> Nullable<Timestamp>,
        dns -> Varchar,
        comments -> Nullable<Varchar>,
        logs -> Nullable<Varchar>,
        fulltext_tag -> Nullable<Varchar>,
    }
}

joinable!(articles -> targets (targert_guid));

allow_tables_to_appear_in_same_query!(
    articles,
    targets,
);
