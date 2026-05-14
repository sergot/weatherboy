use crate::weather::{WeatherLocation, WeatherSnapshot};

pub trait WeatherProvider {
    fn fetch_snapshot(
        &self,
        location: WeatherLocation,
    ) -> Result<WeatherSnapshot, WeatherProviderError>;
}

enum WeatherProviderError {
    RequestFailed(String),
    InvalidResponse(String),
    LocationNotFound,
}
