use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Location {
    lat: i32,
    lon: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    loc: Location,
    rad: u32,
    id: String,
}
