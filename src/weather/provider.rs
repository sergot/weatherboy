mod open_meteo;

use thiserror::Error;

use crate::weather::{WeatherLocation, WeatherSnapshot};

pub use open_meteo::OpenMeteoProvider;

pub trait WeatherProvider {
    // TODO: location handling, units configuration
    async fn fetch_snapshot(
        &self,
        location: WeatherLocation,
    ) -> Result<WeatherSnapshot, WeatherProviderError>;
}

// XXX: more errors?
#[derive(Debug, Error)]
pub enum WeatherProviderError {
    #[error(transparent)]
    Network(#[from] NetworkError),

    #[error(transparent)]
    ResponseData(#[from] ResponseDataError),
}

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("request timed out")]
    Timeout,

    #[error("connection failed")]
    Connection,

    #[error("returned HTTP {0}")]
    HttpStatus(u16),

}

#[derive(Debug, Error)]
pub enum ResponseDataError {
     #[error("response body could not be decoded: {0}")]
     Decode(String),

     #[error("response field {0} has invalid value: {1}")]
     InvalidField(String, String),
}
