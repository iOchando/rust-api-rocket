use diesel::table;

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar
    }
}
