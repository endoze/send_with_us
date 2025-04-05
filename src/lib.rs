//! # SendWithUs API client for Rust
//!
//! A fully-featured asynchronous client for the SendWithUs email API, allowing you to
//! send transactional emails, manage templates, and work with drip campaigns.
//!
//! ## Features
//!
//! - Complete API coverage for all SendWithUs endpoints
//! - Fully asynchronous with tokio runtime support
//! - Strong typing for API requests and responses
//! - Comprehensive error handling
//! - Support for attachments, file uploads, and more
//! - Debug mode for troubleshooting
//!
//! ## Getting Started
//!
//! ```no_run
//! use send_with_us::{Api, ApiClient, types::{EmailOptions, Recipient}};
//! use serde_json::json;
//! use std::collections::HashMap;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!   let api = Api::with_api_key("YOUR_API_KEY");
//!
//!   let recipient = Recipient::new("customer@example.com")
//!     .with_name("Customer Name");
//!
//!   let mut data = HashMap::new();
//!   data.insert("first_name".to_string(), json!("John"));
//!   data.insert("last_name".to_string(), json!("Doe"));
//!   data.insert("order_id".to_string(), json!("12345"));
//!
//!   let options = EmailOptions::new("template-id", recipient)
//!     .with_data(data);
//!
//!   let result = api.send_email(options).await?;
//!   println!("Email sent: {:?}", result);
//!
//!   Ok(())
//! }
//! ```
//!
//! ## Working with Templates
//!
//! ```no_run
//! # use send_with_us::{Api, ApiClient, types::{TemplateOptions}};
//! # use serde_json::Value;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let api = Api::with_api_key("YOUR_API_KEY");
//!
//! let template = TemplateOptions {
//!   name: "Welcome Email".to_string(),
//!   subject: "Welcome to Our Service".to_string(),
//!   html: "<html><body>Welcome, {{name}}!</body></html>".to_string(),
//!   text: "Welcome, {{name}}!".to_string(),
//!   preheader: Some("Welcome to our service".to_string()),
//!   amp_html: None,
//! };
//!
//! let result = api.create_template(template).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Error Handling
//!
//! ```no_run
//! # use send_with_us::{Api, ApiClient, Error, types::{EmailOptions, Recipient}};
//! # #[tokio::main]
//! # async fn main() {
//! let api = Api::with_api_key("YOUR_API_KEY");
//! let recipient = Recipient::new("test@example.com");
//! let options = EmailOptions::new("", recipient); // Empty template ID
//!
//! match api.send_email(options).await {
//!   Ok(response) => println!("Email sent: {:?}", response),
//!   Err(Error::MissingTemplateId) => eprintln!("Error: Template ID is required"),
//!   Err(Error::InvalidCredentials) => eprintln!("Error: Invalid API key"),
//!   Err(Error::ConnectionFailed) => eprintln!("Error: Could not connect to SendWithUs API"),
//!   Err(err) => eprintln!("Error: {}", err),
//! }
//! # }
//! ```

pub mod api;
pub mod attachment;
pub mod config;
pub mod error;
pub mod types;

pub use api::Api;
pub use api::ApiClient;
pub use attachment::Attachment;
pub use config::Config;
pub use error::{Error, Result};
