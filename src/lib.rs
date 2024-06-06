extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationData {
    place_id: i64,
    licence: String,
    osm_type: String,
    osm_id: i64,
    lat: String,
    lon: String,
    display_name: String,
    address: Address,
    boundingbox: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    house_number: Option<String>,
    road: Option<String>,
    suburb: Option<String>,
    city: Option<String>,
    town: Option<String>,
    village: Option<String>,
    county: Option<String>,
    state: String,
    #[serde(rename = "ISO3166-2-lvl4")]
    iso3166_2_lvl4: String,
    postcode: Option<String>,
    country: String,
    country_code: String,
}

#[derive(Clone)]
#[cfg(feature = "reqwest")]
pub struct MapscoClient {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
}
#[cfg(feature = "reqwest")]
extern crate reqwest;
#[cfg(feature = "reqwest")]
impl MapscoClient {
    pub fn new_from_env() -> Self {
        Self::new(
            std::env::var("MAPSCO_API_KEY")
                .expect("MAPSCO_API_KEY must be set")
                .to_string(),
        )
    }
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
            base_url: "https://geocode.maps.co/reverse".to_string(),
        }
    }

    pub async fn reverse_geocode(&self, lat: f64, lon: f64) -> Result<LocationData, anyhow::Error> {
        let url = reqwest::Url::parse_with_params(
            &self.base_url,
            &[
                ("lat", lat.to_string()),
                ("lon", lon.to_string()),
                ("key", self.api_key.clone()),
            ],
        )?;
        let response = self.client.get(url).send().await?;
        let location_data: LocationData = response.json().await?;
        Ok(location_data)
    }

    pub async fn search(&self, query: &str) -> Result<Vec<LocationData>, reqwest::Error> {
        let url = reqwest::Url::parse_with_params(
            &format!("{}/search", self.base_url),
            &[("q", query.to_string()), ("key", self.api_key.clone())],
        )
        .unwrap();
        let response = self.client.get(url).send().await?;
        let search_results: Vec<LocationData> = response.json().await?;
        Ok(search_results)
    }
}
