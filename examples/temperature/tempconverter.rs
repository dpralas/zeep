//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.0.2
//!
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
pub struct Header {}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    root = "Fault",
    namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
    prefix = "soapenv"
)]
pub struct SoapFault {
    #[yaserde(rename = "faultcode", default)]
    pub fault_code: Option<String>,
    #[yaserde(rename = "faultstring", default)]
    pub fault_string: Option<String>,
}

pub mod ports {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    #[async_trait]
    pub trait TempConverterEndpoint {
        async fn celsius_to_fahrenheit(
            &self,
            celsius_to_fahrenheit: CelsiusToFahrenheit,
        ) -> Result<CelsiusToFahrenheitResponse, Option<SoapFault>>;
        async fn fahrenheit_to_celsius(
            &self,
            fahrenheit_to_celsius: FahrenheitToCelsius,
        ) -> Result<FahrenheitToCelsiusResponse, Option<SoapFault>>;
    }

    pub type CelsiusToFahrenheit = messages::CelsiusToFahrenheit;
    pub type CelsiusToFahrenheitResponse = messages::CelsiusToFahrenheitResponse;
    pub type FahrenheitToCelsius = messages::FahrenheitToCelsius;
    pub type FahrenheitToCelsiusResponse = messages::FahrenheitToCelsiusResponse;
}

pub mod bindings {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    impl TempConverterEndpointServiceSoapBinding {
        async fn send_soap_request<T: YaSerialize>(
            &self,
            request: &T,
            action: &str,
        ) -> (reqwest::StatusCode, String) {
            let body = to_string(request).expect("failed to generate xml");
            debug!("SOAP Request: {}", body);
            let mut req = self
                .client
                .post(&self.url)
                .body(body)
                .header("Content-Type", "text/xml")
                .header("Soapaction", action);
            if let Some(credentials) = &self.credentials {
                req = req.basic_auth(
                    credentials.0.to_string(),
                    Option::from(credentials.1.to_string()),
                );
            }
            let res = req.send().await.expect("can not send request");
            let status = res.status();
            debug!("SOAP Status: {}", status);
            let txt = res.text().await.unwrap_or_default();
            debug!("SOAP Response: {}", txt);
            (status, txt)
        }
    }
    pub struct TempConverterEndpointServiceSoapBinding {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }

    #[async_trait]
    impl ports::TempConverterEndpoint for TempConverterEndpointServiceSoapBinding {
        async fn celsius_to_fahrenheit(
            &self,
            celsius_to_fahrenheit: ports::CelsiusToFahrenheit,
        ) -> Result<ports::CelsiusToFahrenheitResponse, Option<SoapFault>> {
            let __request = CelsiusToFahrenheitSoapEnvelope::new(SoapCelsiusToFahrenheit {
                body: celsius_to_fahrenheit,
                xmlns: Option::from(
                    "http://learnwebservices.com/services/tempconverter".to_string(),
                ),
            });

            let (status, response) = self.send_soap_request(&__request, "").await;

            let r: CelsiusToFahrenheitResponseSoapEnvelope =
                from_str(&response).expect("can not unmarshal");
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn fahrenheit_to_celsius(
            &self,
            fahrenheit_to_celsius: ports::FahrenheitToCelsius,
        ) -> Result<ports::FahrenheitToCelsiusResponse, Option<SoapFault>> {
            let __request = FahrenheitToCelsiusSoapEnvelope::new(SoapFahrenheitToCelsius {
                body: fahrenheit_to_celsius,
                xmlns: Option::from(
                    "http://learnwebservices.com/services/tempconverter".to_string(),
                ),
            });

            let (status, response) = self.send_soap_request(&__request, "").await;

            let r: FahrenheitToCelsiusResponseSoapEnvelope =
                from_str(&response).expect("can not unmarshal");
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
    }

    impl Default for TempConverterEndpointServiceSoapBinding {
        fn default() -> Self {
            TempConverterEndpointServiceSoapBinding {
                client: reqwest::Client::new(),
                url: "http://learnwebservices.com/services/tempconverter".to_string(),
                credentials: Option::None,
            }
        }
    }
    impl TempConverterEndpointServiceSoapBinding {
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            TempConverterEndpointServiceSoapBinding {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapCelsiusToFahrenheit {
        #[yaserde(rename = "CelsiusToFahrenheitRequest", default)]
        pub body: ports::CelsiusToFahrenheit,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct CelsiusToFahrenheitSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapCelsiusToFahrenheit,
    }

    impl CelsiusToFahrenheitSoapEnvelope {
        pub fn new(body: SoapCelsiusToFahrenheit) -> Self {
            CelsiusToFahrenheitSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
                    "http://learnwebservices.com/services/tempconverter".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapCelsiusToFahrenheitResponse {
        #[yaserde(rename = "CelsiusToFahrenheitResponse", default)]
        pub body: ports::CelsiusToFahrenheitResponse,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct CelsiusToFahrenheitResponseSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapCelsiusToFahrenheitResponse,
    }

    impl CelsiusToFahrenheitResponseSoapEnvelope {
        pub fn new(body: SoapCelsiusToFahrenheitResponse) -> Self {
            CelsiusToFahrenheitResponseSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
                    "http://learnwebservices.com/services/tempconverter".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapFahrenheitToCelsius {
        #[yaserde(rename = "FahrenheitToCelsiusRequest", default)]
        pub body: ports::FahrenheitToCelsius,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct FahrenheitToCelsiusSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapFahrenheitToCelsius,
    }

    impl FahrenheitToCelsiusSoapEnvelope {
        pub fn new(body: SoapFahrenheitToCelsius) -> Self {
            FahrenheitToCelsiusSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
                    "http://learnwebservices.com/services/tempconverter".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapFahrenheitToCelsiusResponse {
        #[yaserde(rename = "FahrenheitToCelsiusResponse", default)]
        pub body: ports::FahrenheitToCelsiusResponse,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct FahrenheitToCelsiusResponseSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapFahrenheitToCelsiusResponse,
    }

    impl FahrenheitToCelsiusResponseSoapEnvelope {
        pub fn new(body: SoapFahrenheitToCelsiusResponse) -> Self {
            FahrenheitToCelsiusResponseSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
                    "http://learnwebservices.com/services/tempconverter".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }
}

pub mod types {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://learnwebservices.com/services/tempconverter",
        rename = "celsiusToFahrenheitRequest",
        default
    )]
    pub struct CelsiusToFahrenheitRequest {
        #[yaserde(prefix = "tns", rename = "TemperatureInCelsius", default)]
        pub temperature_in_celsius: f64,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://learnwebservices.com/services/tempconverter",
        rename = "celsiusToFahrenheitResponse",
        default
    )]
    pub struct CelsiusToFahrenheitResponse {
        #[yaserde(prefix = "tns", rename = "TemperatureInFahrenheit", default)]
        pub temperature_in_fahrenheit: f64,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://learnwebservices.com/services/tempconverter",
        rename = "fahrenheitToCelsiusRequest",
        default
    )]
    pub struct FahrenheitToCelsiusRequest {
        #[yaserde(prefix = "tns", rename = "TemperatureInFahrenheit", default)]
        pub temperature_in_fahrenheit: f64,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://learnwebservices.com/services/tempconverter",
        rename = "fahrenheitToCelsiusResponse",
        default
    )]
    pub struct FahrenheitToCelsiusResponse {
        #[yaserde(prefix = "tns", rename = "TemperatureInCelsius", default)]
        pub temperature_in_celsius: f64,
    }
}

pub mod messages {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "CelsiusToFahrenheit", default)]
    pub struct CelsiusToFahrenheit {
        #[yaserde(flatten)]
        pub celsius_to_fahrenheit_request: types::CelsiusToFahrenheitRequest,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "FahrenheitToCelsius", default)]
    pub struct FahrenheitToCelsius {
        #[yaserde(flatten)]
        pub fahrenheit_to_celsius_request: types::FahrenheitToCelsiusRequest,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "CelsiusToFahrenheitResponse", default)]
    pub struct CelsiusToFahrenheitResponse {
        #[yaserde(flatten)]
        pub celsius_to_fahrenheit_response: types::CelsiusToFahrenheitResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "FahrenheitToCelsiusResponse", default)]
    pub struct FahrenheitToCelsiusResponse {
        #[yaserde(flatten)]
        pub fahrenheit_to_celsius_response: types::FahrenheitToCelsiusResponse,
    }
}