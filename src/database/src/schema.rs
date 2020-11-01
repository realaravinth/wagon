table! {
    organisations (organisation_username) {
        organisation_username -> Varchar,
        organisation_name -> Nullable<Varchar>,
        email -> Varchar,
        password -> Varchar,
        description -> Nullable<Varchar>,
        email_verified -> Nullable<Bool>,
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
