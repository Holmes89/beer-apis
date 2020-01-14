table! {
    beers (id) {
        id -> Nullable<Text>,
        brewery_id -> Nullable<Text>,
        name -> Nullable<Text>,
    }
}

table! {
    breweries (id) {
        id -> Nullable<Text>,
        name -> Nullable<Text>,
        address -> Nullable<Text>,
        city -> Nullable<Text>,
        state -> Nullable<Text>,
        zip -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    beers,
    breweries,
);
