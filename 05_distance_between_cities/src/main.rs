extern crate json;
use std::env;
use std::process::Command;
mod geo_location;
use geo_location::coordinate::Coordinate;

fn main() {
    match env::var("MAP_QUEST_API_KEY") {
        Ok(key) => start(&key),
        Err(error) => println!(
            "Get an API key from mapquestapi.com and set MAP_QUEST_API_KEY env var. ({})",
            error
        ),
    };
}

fn start(api_key: &str) {
    let paris = get_coordinate("Paris,France", api_key);
    let london = get_coordinate("London,England", api_key);
    println!("Rome: {}", paris);
    println!("London: {}", london);
    let distance = geo_location::calculate_distance(paris, london);
    println!("Distance: {}", distance);
}

fn get_coordinate(location: &str, api_key: &str) -> Coordinate {
    let base_uri = "http://www.mapquestapi.com/geocoding/v1/address";
    let uri = format!("{}?key={}&location={}", base_uri, api_key, location);
    let response = get(&uri);
    let parsed = json::parse(&response).unwrap();
    let latlng = &parsed["results"][0]["locations"][0]["latLng"];
    let latitude = latlng["lat"].as_f64().unwrap();
    let longitude = latlng["lng"].as_f64().unwrap();
    return Coordinate {
        latitude,
        longitude,
    };
}

fn get(uri: &str) -> String {
    let output = Command::new("curl")
        .arg(uri)
        .output()
        .expect("Failed to execute command");
    return String::from_utf8(output.stdout).expect("ERROR!!");
}
