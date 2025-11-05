use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line_2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_place: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
}

impl Address {
    pub fn builder() -> AddressBuilder {
        AddressBuilder::default()
    }
}

#[derive(Default)]
pub struct AddressBuilder {
    address_line: Option<String>,
    address_line_2: Option<String>,
    postal_code: Option<String>,
    postal_place: Option<String>,
    country: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

impl AddressBuilder {
    pub fn address_line(mut self, line: impl Into<String>) -> Self {
        self.address_line = Some(line.into());
        self
    }

    pub fn address_line_2(mut self, line: impl Into<String>) -> Self {
        self.address_line_2 = Some(line.into());
        self
    }

    pub fn postal_code(mut self, code: impl Into<String>) -> Self {
        self.postal_code = Some(code.into());
        self
    }

    pub fn postal_place(mut self, place: impl Into<String>) -> Self {
        self.postal_place = Some(place.into());
        self
    }

    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    pub fn coordinates(mut self, latitude: f64, longitude: f64) -> Self {
        self.latitude = Some(latitude);
        self.longitude = Some(longitude);
        self
    }

    pub fn build(self) -> Address {
        Address {
            address_line: self.address_line,
            address_line_2: self.address_line_2,
            postal_code: self.postal_code,
            postal_place: self.postal_place,
            country: self.country,
            latitude: self.latitude,
            longitude: self.longitude,
        }
    }
}
