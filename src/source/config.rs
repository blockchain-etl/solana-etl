//! Defines a RequestConfig Enum which represents the data necessary to be passed on for requests

/// An enum that should only contain one item, contains what is required to be
/// sent to the configuration
pub enum RequestConfig {
    /// Request Builder for the Solana Configuration
    #[cfg(feature = "SOLANA")]
    ReqBldr(reqwest::RequestBuilder),
}

impl RequestConfig {
    /// Attempts to clone
    pub fn try_clone(&self) -> Option<Self> {
        match self {
            #[cfg(feature = "SOLANA")]
            RequestConfig::ReqBldr(req_builder) => {
                req_builder.try_clone().map(RequestConfig::ReqBldr)
            }
        }
    }

    /// Returns the the value on the inside, consuming the enum
    pub fn to_requestbuilder(&self) -> reqwest::RequestBuilder {
        match self {
            #[cfg(feature = "SOLANA")]
            RequestConfig::ReqBldr(reqbldr) => reqbldr.try_clone().unwrap(),
        }
    }
}
