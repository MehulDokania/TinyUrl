// @generated automatically by Diesel CLI.

diesel::table! {
    url_map (id) {
        id -> Int4,
        original_url -> Text,
        tiny_url -> Text,
        fetch_count -> Int4,
    }
}
