pub mod coordinate;
use coordinate::Coordinate;

pub fn calculate_delta(first: &Coordinate, second: &Coordinate) -> Coordinate {
    return Coordinate {
        latitude: (first.latitude - second.latitude),
        longitude: (first.longitude - second.longitude),
    };
}

fn calculate_distance(first: Coordinate, second: Coordinate) -> f64 {
    let earth_radius_kilometer = 6371.0_f64;
    let delta = calculate_delta(&first, &second);

    let central_angle_inner = (delta.latitude.to_radians() / 2.0).sin().powi(2)
        + first.rad().latitude.cos()
            * second.rad().latitude.cos()
            * (delta.rad().longitude / 2.0).sin().powi(2);

    let central_angle = 2.0 * central_angle_inner.sqrt().asin();

    return earth_radius_kilometer * central_angle;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_delta() {
        let first = super::Coordinate {
            latitude: 30_f64,
            longitude: 30_f64,
        };

        let second = super::Coordinate {
            latitude: 20_f64,
            longitude: 20_f64,
        };

        let delta = super::calculate_delta(&first, &second);

        assert_eq!(delta.latitude, 10_f64);
        assert_eq!(delta.longitude, 10_f64);
    }

    #[test]
    fn test_distance() {
        let paris = super::Coordinate {
            latitude: 48.85341_f64,
            longitude: -2.34880_f64,
        };
        let london = super::Coordinate {
            latitude: 51.50853_f64,
            longitude: -0.12574_f64,
        };

        let distance = super::calculate_distance(paris, london);
        let distance_string = format!("{:.2}", distance);

        assert_eq!(distance_string, "334.96");
    }
}
