use osmpbfreader::Node;

const DECIMICRO: f64 = 1. / 10_000_000.;

#[derive(Debug)]
pub struct Location {
    pub lat: f64,
    pub long: f64,
}

impl From<&Node> for Location {
    fn from(value: &Node) -> Self {
        Location {
            lat: value.decimicro_lat as f64 * DECIMICRO,
            long: value.decimicro_lon as f64 * DECIMICRO,
        }
    }
}

impl Location {
    pub fn dist(&self, other: &Self) -> f64 {
        let diff_lat = other.lat - self.lat;
        let diff_lon = other.long -self.long;

        f64::sqrt(diff_lat.powi(2) + diff_lon.powi(2))
    }
}
