table! {
    organisations (name) {
        name -> Varchar,
        email -> Varchar,
        description -> Varchar,
    }
}

table! {
    subscribers (email) {
        email -> Varchar,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    organisations,
    subscribers,
);
