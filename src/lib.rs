use serde::{Deserialize, Serialize};
use serde_this_or_that::as_f64;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocationData {
    pub place_id: i64,
    pub licence: Option<String>,
    pub osm_type: Option<String>,
    pub osm_id: Option<i64>,
    #[serde(deserialize_with = "as_f64")]
    pub lat: f64,
    #[serde(deserialize_with = "as_f64")]
    pub lon: f64,
    pub display_name: String,
    pub address: Option<Address>,
    pub boundingbox: Vec<String>,
    pub class: Option<String>,
    pub r#type: Option<String>,
    pub importance: Option<f64>,
}
use std::fmt;
impl fmt::Display for LocationData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.display_name)
    }
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
    pub state: Option<String>,
    #[serde(rename = "ISO3166-2-lvl4")]
    pub iso3166_2_lvl4: Option<String>,
    pub postcode: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
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
            base_url: "https://geocode.maps.co".into(),
        }
    }

    pub async fn reverse(&self, lat: f64, lon: f64) -> Result<LocationData, reqwest::Error> {
        Ok(self
            .client
            .get(
                reqwest::Url::parse_with_params(
                    &format!("{}/reverse", self.base_url),
                    &[
                        ("lat", lat.to_string()),
                        ("lon", lon.to_string()),
                        ("api_key", self.api_key.clone()),
                    ],
                )
                .unwrap(),
            )
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn search(&self, query: &str) -> Result<Vec<LocationData>, reqwest::Error> {
        let text = &self
            .client
            .get(
                reqwest::Url::parse_with_params(
                    &format!("{}/search", self.base_url),
                    &[("q", query.to_string()), ("api_key", self.api_key.clone())],
                )
                .unwrap(),
            )
            .send()
            .await?
            .text()
            .await?;
        println!("text = {text}");
        Ok(serde_json::from_str(text).unwrap())
    }
}
