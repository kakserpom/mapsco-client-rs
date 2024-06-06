use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocationData {
    pub place_id: i64,
    pub licence: String,
    pub osm_type: String,
    pub osm_id: i64,
    pub lat: String,
    pub lon: String,
    pub display_name: String,
    pub address: Address,
    pub boundingbox: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Address {
    pub house_number: Option<String>,
    pub road: Option<String>,
    pub suburb: Option<String>,
    pub city: Option<String>,
    pub town: Option<String>,
    pub village: Option<String>,
    pub county: Option<String>,
    pub state: String,
    #[serde(rename = "ISO3166-2-lvl4")]
    pub iso3166_2_lvl4: String,
    pub postcode: Option<String>,
    pub country: String,
    pub country_code: String,
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

    pub async fn reverse_geocode(
        &self,
        lat: f64,
        lon: f64,
    ) -> Result<LocationData, reqwest::Error> {
        let url = reqwest::Url::parse_with_params(
            &self.base_url,
            &[
                ("lat", lat.to_string()),
                ("lon", lon.to_string()),
                ("key", self.api_key.clone()),
            ],
        )
        .unwrap();
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
