use serde::{Serialize, Deserialize};
use osmpbfreader::Node;

const DECIMICRO: f64 = 1. / 10_000_000.;

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    lat: i32,
    lon: i32,
}

impl From<&Node> for Location {
    fn from(value: &Node) -> Self {
        Location {
            lat: value.decimicro_lat,
            lon: value.decimicro_lon,
        }
    }
}

impl Location {
    pub fn dist(&self, other: &Self) -> f64 {
        let diff_lat = (other.lat - self.lat) as f64 * DECIMICRO;
        let diff_lon = (other.lon - self.lon) as f64 * DECIMICRO;

        f64::sqrt(diff_lat.powi(2) + diff_lon.powi(2))
    }
}
