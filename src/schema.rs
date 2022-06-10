table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        level -> Nullable<Int4>,
        email -> Nullable<Varchar>,
        last_submitted -> Nullable<Varchar>,
    }
}
