use thiserror::Error;

/// Result type for SendWithUs operations
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when using the SendWithUs API client
///
/// This enum represents all possible errors that can occur when interacting
/// with the SendWithUs email service. Each variant provides specific information
/// about what went wrong to help with debugging and error handling.
#[derive(Error, Debug)]
pub enum Error {
  /// The API key provided is missing, invalid, or unauthorized
  ///
  /// This error occurs when the SendWithUs service rejects your API credentials.
  /// Check that you're using a valid API key and that it has the necessary permissions.
  #[error("Invalid or missing API key")]
  InvalidCredentials,

  /// A required template ID was not provided
  ///
  /// Email templates must have a valid ID to be used with the SendWithUs API.
  /// Check that you're providing a template ID for your email.
  #[error("Email template ID is required")]
  MissingTemplateId,

  /// A required recipient email address was not provided
  ///
  /// Every email must have at least one recipient address.
  /// Ensure you're providing valid recipient information in your request.
  #[error("Recipient email address is required")]
  MissingRecipientAddress,

  /// The provided API endpoint is invalid or cannot be accessed
  ///
  /// This typically indicates a configuration issue with custom API endpoints.
  #[error("Invalid API endpoint: {0}")]
  InvalidEndpoint(String),

  /// Failed to establish a connection to the SendWithUs API
  ///
  /// This may indicate network connectivity issues or that the SendWithUs
  /// service is temporarily unavailable.
  #[error("Connection to SendWithUs API failed")]
  ConnectionFailed,

  /// The SendWithUs API rejected the request due to invalid parameters
  ///
  /// The error message provides additional details about what was invalid.
  #[error("SendWithUs API rejected request: {0}")]
  InvalidRequest(String),

  /// The SendWithUs API returned an HTTP error
  ///
  /// This contains the HTTP status code and error message from the API.
  #[error("SendWithUs API error: {status} - {message}")]
  ApiError { status: u16, message: String },

  /// Error communicating with the SendWithUs API
  ///
  /// This is a lower-level error from the HTTP client, which may indicate
  /// network, timeout, or other communication issues.
  #[error("API communication error: {0}")]
  RequestFailed(#[from] reqwest::Error),

  /// Failed to serialize request or deserialize response data
  ///
  /// This typically indicates incompatible data structures or unexpected
  /// response formats from the API.
  #[error("Data serialization error: {0}")]
  SerializationFailed(#[from] serde_json::Error),

  /// Error accessing a file, typically when working with attachments
  ///
  /// Check that file paths are correct and that your application has
  /// permission to read the specified files.
  #[error("File access error: {0}")]
  FileAccessFailed(#[from] std::io::Error),

  /// The configured base URL for the SendWithUs API is invalid
  ///
  /// This typically indicates a configuration issue in your application.
  /// Ensure you're using a valid URL format if you've customized the API URL.
  #[error("Invalid SendWithUs API URL")]
  InvalidApiUrl,

  /// An unexpected error occurred that doesn't match any of the known categories
  ///
  /// The error message provides additional context about what went wrong.
  #[error("Unexpected error: {0}")]
  Unexpected(String),
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_error_display() {
    let error = Error::InvalidCredentials;
    assert_eq!(error.to_string(), "Invalid or missing API key");

    let error = Error::MissingTemplateId;
    assert_eq!(error.to_string(), "Email template ID is required");

    let error = Error::MissingRecipientAddress;
    assert_eq!(error.to_string(), "Recipient email address is required");

    let error = Error::InvalidEndpoint("custom/endpoint".to_string());
    assert_eq!(error.to_string(), "Invalid API endpoint: custom/endpoint");

    let error = Error::InvalidRequest("Invalid parameter".to_string());
    assert_eq!(
      error.to_string(),
      "SendWithUs API rejected request: Invalid parameter"
    );

    let error = Error::ApiError {
      status: 500,
      message: "Server error".to_string(),
    };
    assert_eq!(
      error.to_string(),
      "SendWithUs API error: 500 - Server error"
    );

    let error = Error::Unexpected("Something unexpected".to_string());
    assert_eq!(error.to_string(), "Unexpected error: Something unexpected");

    let error = Error::InvalidApiUrl;
    assert_eq!(error.to_string(), "Invalid SendWithUs API URL");

    let error = Error::ConnectionFailed;
    assert_eq!(error.to_string(), "Connection to SendWithUs API failed");

    let error = Error::FileAccessFailed(std::io::Error::new(
      std::io::ErrorKind::NotFound,
      "File not found",
    ));
    assert!(error.to_string().contains("File access error"));

    let io_error = std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "Connection refused");
    let error = Error::FileAccessFailed(io_error);
    assert!(error.to_string().contains("File access error"));
  }
}
