// @generated automatically by Diesel CLI.

diesel::table! {
    todo (id) {
        id -> Int4,
        td_desc -> Varchar,
        td_status -> Bool,
    }
}
