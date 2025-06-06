use geo::Point;
use rand::rngs::SmallRng;
use rand::Rng;

pub struct GeographicArea {
    pub minimum_latitude: f64,
    pub maximum_latitude: f64,

    pub minimum_longitude: f64,
    pub maximum_longitude: f64,
}

impl GeographicArea {
    pub fn new(
        minimum_latitude: f64,
        maximum_latitude: f64,
        minimum_longitude: f64,
        maximum_longitude: f64,
    ) -> Self {
        GeographicArea {
            minimum_latitude,
            maximum_latitude,
            minimum_longitude,
            maximum_longitude,
        }
    }

    pub fn random_coordinate(&self, rng: &mut SmallRng) -> Point {
        let random_latitude_unit_range = rng.gen::<f64>();
        let random_longitude_unit_range = rng.gen::<f64>();

        let latitude = self.minimum_latitude
            + (self.maximum_latitude - self.minimum_latitude) * random_latitude_unit_range;
        let longitude = self.minimum_longitude
            + (self.maximum_longitude - self.minimum_longitude) * random_longitude_unit_range;

        Point::new(longitude, latitude)
    }
}
