extern crate json;

pub trait HttpClient {
    fn get(&self, uri: &str) -> String;
}

pub struct MapQuest<'a> {
    api_key: &'a str,
}

impl<'a> MapQuest<'a> {
    pub fn new(api_key: &str) -> MapQuest {
        return MapQuest { api_key };
    }
}

impl<'a> MapQuest<'a> {
    pub fn get_coordinates(&self, location: &str, client: &impl HttpClient) -> (f64, f64) {
        let response = self.get_location_info(location, client);
        let latlng = &response["results"][0]["locations"][0]["latLng"];
        return (
            latlng["lat"].as_f64().unwrap(),
            latlng["lng"].as_f64().unwrap(),
        );
    }

    fn get_location_info(&self, location: &str, client: &impl HttpClient) -> json::JsonValue {
        let base_uri = "http://www.mapquestapi.com/geocoding/v1/address";
        let uri = format!("{}?key={}&location={}", base_uri, self.api_key, location);
        let response = client.get(&uri);
        return json::parse(&response).unwrap();
    }
}

#[cfg(test)]
mod tests {
    extern crate json;
    struct ClientSpy;

    impl ClientSpy {
        const JSON_STRING: &'static str =
            "{\"results\":[{\"locations\":[{\"latLng\":{\"lat\":0.5,\"lng\":-0.5}}]}]}";

        fn new() -> ClientSpy {
            return ClientSpy {};
        }
    }

    impl super::HttpClient for ClientSpy {
        fn get(&self, _uri: &str) -> String {
            return String::from(ClientSpy::JSON_STRING);
        }
    }

    #[test]
    fn test_get_location_info() {
        let client = ClientSpy::new();
        let map_quest = super::MapQuest::new("key");
        let response = map_quest.get_location_info("location", &client);
        let expectation = json::parse(ClientSpy::JSON_STRING).unwrap();
        assert_eq!(response, expectation);
    }

    #[test]
    fn test_get_coordinates() {
        let client = ClientSpy::new();
        let map_quest = super::MapQuest::new("key");
        let response = map_quest.get_coordinates("location", &client);
        assert_eq!(response, (0.5, -0.5));
    }
}
