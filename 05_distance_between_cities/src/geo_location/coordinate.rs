pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinate {
    pub fn rad(&self) -> Coordinate {
        return Coordinate {
            latitude: self.latitude.to_radians(),
            longitude: self.longitude.to_radians(),
        };
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_rad() {
        let coordinate = super::Coordinate {
            latitude: 90_f64,
            longitude: 45_f64,
        };

        let rad = coordinate.rad();
        let lat_rounded = (rad.latitude * 10000_f64).round() / 10000_f64;
        let lon_rounded = (rad.longitude * 10000_f64).round() / 10000_f64;
        assert_eq!(lat_rounded, 1.5708);
        assert_eq!(lon_rounded, 0.7854);
    }
}
