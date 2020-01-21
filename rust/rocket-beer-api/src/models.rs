use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Brewery {
    id: Option<String>,
    name: Option<String>,
    address: Option<String>,
    city: Option<String>,
    state: Option<String>,
    code: Option<String>,
}

#[derive(Queryable, Serialize)]
pub struct Beer {
    id: Option<String>,
    brewery_id: Option<String>,
    name: Option<String>,
}
