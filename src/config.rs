use url::Url;

/// Current crate version, automatically set from Cargo.toml
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Configuration for the SendWithUs API client.
///
/// This struct contains all the settings needed to connect to and interact
/// with the SendWithUs API, including authentication, API version, and
/// debugging options.
///
/// The configuration uses the builder pattern, allowing for fluent chaining
/// of configuration methods.
///
/// # Examples
///
/// ```
/// use send_with_us::Config;
///
/// let config = Config::new("your-api-key");
///
/// let config = Config::new("your-api-key")
///   .with_url("https://custom-instance.sendwithus.com")
///   .with_api_version("2")
///   .with_debug(true);
/// ```
#[derive(Debug, Clone)]
pub struct Config {
  /// Base URL for the SendWithUs API
  pub url: Url,

  /// API key used for authentication
  pub api_key: String,

  /// API version to use (default: "1")
  pub api_version: String,

  /// Debug mode flag for verbose logging
  pub debug: bool,

  /// Client identifier sent with API requests
  pub client_stub: String,
}

impl Config {
  /// Creates a new configuration with default values.
  ///
  /// This method initializes a configuration with sensible defaults:
  /// - URL: <https://api.sendwithus.com>
  /// - API version: "1"
  /// - Debug mode: false
  /// - Client stub: rust-{VERSION}
  ///
  /// # Arguments
  /// * `api_key` - Your SendWithUs API key
  ///
  /// # Returns
  /// A new Config instance with default settings and the provided API key
  ///
  /// # Examples
  ///
  /// ```
  /// use send_with_us::Config;
  ///
  /// let config = Config::new("api-key-123456");
  /// ```
  pub fn new(api_key: impl Into<String>) -> Self {
    let default_url = Url::parse("https://api.sendwithus.com").unwrap();

    Self {
      url: default_url,
      api_key: api_key.into(),
      api_version: "1".to_string(),
      debug: false,
      client_stub: format!("rust-{}", VERSION),
    }
  }

  /// Sets a custom API URL.
  ///
  /// Use this method if you need to connect to a custom SendWithUs instance
  /// or a different region. If the provided URL is invalid, it will fall back
  /// to the default URL.
  ///
  /// # Arguments
  /// * `url` - The custom API URL to use
  ///
  /// # Returns
  /// Self with the updated URL for method chaining
  ///
  /// # Examples
  ///
  /// ```
  /// use send_with_us::Config;
  ///
  /// let config = Config::new("api-key")
  ///   .with_url("https://api.eu.sendwithus.com");
  /// ```
  pub fn with_url(mut self, url: impl Into<String>) -> Self {
    self.url =
      Url::parse(&url.into()).unwrap_or_else(|_| Url::parse("https://api.sendwithus.com").unwrap());
    self
  }

  /// Sets the API version to use.
  ///
  /// SendWithUs supports multiple API versions. Use this method to specify
  /// which version your application should use.
  ///
  /// # Arguments
  /// * `version` - The API version as a string (e.g., "1", "2")
  ///
  /// # Returns
  /// Self with the updated API version for method chaining
  ///
  /// # Examples
  ///
  /// ```
  /// use send_with_us::Config;
  ///
  /// let config = Config::new("api-key")
  ///   .with_api_version("2");
  /// ```
  pub fn with_api_version(mut self, version: impl Into<String>) -> Self {
    self.api_version = version.into();
    self
  }

  /// Enables or disables debug mode.
  ///
  /// When debug mode is enabled, the client will output detailed information
  /// about API requests and responses, which can be helpful for troubleshooting.
  ///
  /// # Arguments
  /// * `debug` - Boolean flag to enable or disable debug mode
  ///
  /// # Returns
  /// Self with the updated debug setting for method chaining
  ///
  /// # Examples
  ///
  /// ```
  /// use send_with_us::Config;
  ///
  /// let config = Config::new("api-key")
  ///   .with_debug(true);
  /// ```
  pub fn with_debug(mut self, debug: bool) -> Self {
    self.debug = debug;
    self
  }

  /// Gets the protocol (http or https) from the configured URL.
  ///
  /// # Returns
  /// The URL scheme as a string slice
  pub fn protocol(&self) -> &str {
    self.url.scheme()
  }

  /// Gets the host from the configured URL.
  ///
  /// # Returns
  /// The host as a string slice, defaulting to "api.sendwithus.com" if not available
  pub fn host(&self) -> &str {
    self.url.host_str().unwrap_or("api.sendwithus.com")
  }

  /// Gets the port from the configured URL.
  ///
  /// If the URL doesn't specify a port, this method returns the default port
  /// for the protocol (443 for HTTPS, 80 for HTTP).
  ///
  /// # Returns
  /// The port number
  pub fn port(&self) -> u16 {
    self
      .url
      .port()
      .unwrap_or_else(|| if self.protocol() == "https" { 443 } else { 80 })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_default_config() {
    let config = Config::new("test-api-key");
    assert_eq!(config.protocol(), "https");
    assert_eq!(config.host(), "api.sendwithus.com");
    assert_eq!(config.port(), 443);
    assert_eq!(config.api_key, "test-api-key");
    assert_eq!(config.api_version, "1");
    assert!(!config.debug);
    assert_eq!(config.client_stub, format!("rust-{}", VERSION));
  }

  #[test]
  fn test_custom_url_config() {
    let config = Config::new("test-api-key").with_url("http://example.com");
    assert_eq!(config.protocol(), "http");
    assert_eq!(config.host(), "example.com");
    assert_eq!(config.port(), 80);
  }

  #[test]
  fn test_invalid_url_fallback() {
    let config = Config::new("test-api-key").with_url("invalid-url");
    assert_eq!(config.protocol(), "https");
    assert_eq!(config.host(), "api.sendwithus.com");
    assert_eq!(config.port(), 443);
  }

  #[test]
  fn test_with_api_version() {
    let config = Config::new("test-api-key").with_api_version("2");
    assert_eq!(config.api_version, "2");
  }

  #[test]
  fn test_with_debug_mode() {
    let config = Config::new("test-api-key").with_debug(true);
    assert!(config.debug);
  }

  #[test]
  fn test_custom_port() {
    let config = Config::new("test-api-key").with_url("https://example.com:8443");
    assert_eq!(config.protocol(), "https");
    assert_eq!(config.host(), "example.com");
    assert_eq!(config.port(), 8443);
  }
}
