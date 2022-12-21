// @generated automatically by Diesel CLI.

diesel::table! {
    exams (id) {
        id -> Integer,
        name -> Varchar,
        created_at -> Datetime,
    }
}
