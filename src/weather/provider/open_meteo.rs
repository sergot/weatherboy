use std::time::Duration;

use serde::Deserialize;

use crate::weather::{WeatherLocation, WeatherSnapshot, provider::{NetworkError, ResponseDataError, WeatherProvider, WeatherProviderError}};

const BASE_URL: &str = "https://api.open-meteo.com/v1/forecast";
// TODO: make it configurable
const WEATHER_FIELDS: &str = "temperature_2m,is_day,wind_speed_10m,wind_direction_10m,precipitation,weather_code";

pub struct OpenMeteoProvider {
    client: reqwest::Client,
    base_url: &'static str,
}

impl OpenMeteoProvider {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_else(|_| {
                reqwest::Client::new()
            });
        Self {
            client,
            base_url: BASE_URL,
        }
    }
}

impl WeatherProvider for OpenMeteoProvider {
    async fn fetch_snapshot(
        &self,
        location: WeatherLocation,
    ) -> Result<WeatherSnapshot, WeatherProviderError> {
        let url = format!("{}?latitude={}&longitude={}&current={}", self.base_url, location.latitude, location.longitude, WEATHER_FIELDS);
        let response = self.client.get(url)
            .send()
            .await
            .map_err(map_network_error)?
            .error_for_status()
            .map_err(map_network_error)?;

        let payload: OpenMeteoResponse = response
            .json()
            .await
            .map_err(|err| ResponseDataError::Decode(err.to_string()))?;

        validate_payload(&payload)?;

        Ok(WeatherSnapshot{
            time: payload.current.time,
            temperature: payload.current.temperature_2m,
            is_day: payload.current.is_day != 0,
            wind_speed: payload.current.wind_speed_10m,
            wind_direction: payload.current.wind_direction_10m,
            precipitation: payload.current.precipitation,
            weather_code: payload.current.weather_code,
        })
    }
}

#[derive(Deserialize, Debug)]
struct OpenMeteoResponse {
    current: OpenMeteoCurrentWeather,
}

#[derive(Deserialize, Debug)]
struct OpenMeteoCurrentWeather {
    time: String,
    temperature_2m: f32,
    is_day: u8,
    wind_speed_10m: f32,
    wind_direction_10m: u16,
    precipitation: f32,
    weather_code: u8, 
}

fn map_network_error(err: reqwest::Error) -> WeatherProviderError {
    if err.is_timeout() {
        NetworkError::Timeout.into()
    } else if let Some(status) = err.status() {
        NetworkError::HttpStatus(status.as_u16()).into()
    } else {
        NetworkError::Connection.into()
    }
}

// XXX: do we need payload validation?
fn validate_payload(payload: &OpenMeteoResponse) -> Result<(), ResponseDataError> {
    if payload.current.wind_direction_10m > 360 {
        return Err(ResponseDataError::InvalidField("wind_direction_10m".into(), "expected value in 0..=360".into()))
    }
    Ok(())
}
