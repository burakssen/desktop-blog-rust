// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Int4,
        user_name -> VarChar,
        password -> VarChar,
        mail -> VarChar,
    }
}
