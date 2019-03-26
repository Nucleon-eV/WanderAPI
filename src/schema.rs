table! {
    hiking_trails (id) {
        id -> Int4,
        name -> Varchar,
        location -> Varchar,
    }
}

table! {
    pois (id) {
        id -> Int4,
        hiking_trail -> Int4,
        name -> Varchar,
        description -> Text,
        location -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    hiking_trails,
    pois,
);