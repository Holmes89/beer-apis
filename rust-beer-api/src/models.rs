use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Brewery {
    id: String,
    name: String,
    address: String,
    city: String,
    state: String,
    code: String,
}

#[derive(Queryable, Serialize)]
pub struct Beer {
    id: Option<String>,
    brewery_id: Option<String>,
    name: Option<String>,
}
