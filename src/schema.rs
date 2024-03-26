// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        #[max_length = 25]
        title -> Varchar,
        #[max_length = 50]
        slug -> Varchar,
        body -> Text,
    }
}
