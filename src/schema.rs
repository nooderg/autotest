table! {
    users (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        file_url -> Varchar,
        created_at -> Timestamp,
    }
}
