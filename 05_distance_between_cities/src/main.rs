extern crate json;
use std::env;
use std::process::Command;
mod geo_location;
use geo_location::coordinate::Coordinate;
mod map_quest;
use map_quest::MapQuest;

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
    let map_quest = MapQuest::new(api_key);
    let paris = map_quest.coordinates("Paris,France");
    let london = map_quest.coordinates("London,England");

    println!("Paris: {}", paris);
    println!("London: {}", london);
    let distance = geo_location::calculate_distance(paris, london);
    println!("Distance: {}", distance);
}

impl<'a> MapQuest<'a> {
    fn coordinates(&self, location: &str) -> Coordinate {
        let client = CurlHtmlClient::new();
        let coordinates_tuple = self.get_coordinates(location, &client);
        return Coordinate {
            latitude: coordinates_tuple.0,
            longitude: coordinates_tuple.1,
        };
    }
}

struct CurlHtmlClient {}

impl CurlHtmlClient {
    fn new() -> CurlHtmlClient {
        return CurlHtmlClient {};
    }
}

impl map_quest::HttpClient for CurlHtmlClient {
    fn get(&self, uri: &str) -> String {
        let output = Command::new("curl")
            .arg(uri)
            .output()
            .expect("Failed to execute command");
        return String::from_utf8(output.stdout).expect("ERROR!!");
    }
}
