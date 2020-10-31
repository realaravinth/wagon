table! {
    lists (name) {
        name -> Varchar,
        description -> Varchar,
        organisation_name -> Nullable<Varchar>,
    }
}

table! {
    organisations (organisation_name) {
        organisation_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        description -> Varchar,
    }
}

table! {
    subscribers (email) {
        email -> Varchar,
        name -> Varchar,
    }
}

joinable!(lists -> organisations (organisation_name));

allow_tables_to_appear_in_same_query!(lists, organisations, subscribers,);
