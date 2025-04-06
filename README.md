# SendWithUs Rust Client

![Build Status](https://github.com/endoze/send_with_us/actions/workflows/ci.yml/badge.svg?branch=master)
[![Coverage Status](https://coveralls.io/repos/github/endoze/send_with_us/badge.svg?branch=master)](https://coveralls.io/github/endoze/send_with_us?branch=master)
[![Crate](https://img.shields.io/crates/v/send_with_us.svg)](https://crates.io/crates/send_with_us)
[![Docs](https://docs.rs/send_with_us/badge.svg)](https://docs.rs/send_with_us)

An async Rust client for the [SendWithUs](https://www.sendwithus.com) API.

## Features

* Full async support with Tokio
* Type-safe API
* Comprehensive error handling
* Support for all SendWithUs API endpoints
* Simple and ergonomic interface
* File attachment support
* Optional logging support via tracing

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
send_with_us = "0.1.0"
# With optional logging feature enabled
send_with_us = { version = "0.1.0", features = ["logging"] }
```

## Usage

### Basic Example

```rust , no_run
use send_with_us::{Api, Config, ApiClient};
use send_with_us::types::{EmailOptions, Recipient};
use std::collections::HashMap;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let api = Api::with_api_key("YOUR_API_KEY");
  
  let recipient = Recipient::new("user@example.com").with_name("Example User");
  
  let mut email_data = HashMap::new();
  email_data.insert("first_name".to_string(), json!("John"));
  email_data.insert("amount".to_string(), json!(100));
  
  let options = EmailOptions::new("template_id", recipient)
    .with_data(email_data);
  
  match api.send_email(options).await {
    Ok(response) => println!("Email sent successfully: {:?}", response),
    Err(e) => eprintln!("Failed to send email: {}", e),
  }
  
  Ok(())
}
```

### With Email Attachments

```rust , no_run
use send_with_us::{Api, Attachment, ApiClient};
use send_with_us::types::{EmailOptions, Recipient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let api = Api::with_api_key("YOUR_API_KEY");
  let recipient = Recipient::new("user@example.com");
  
  let attachment = Attachment::from_path("path/to/file.pdf").await?;
  
  let options = EmailOptions::new("template_id", recipient)
    .with_files(vec![attachment]);
  
  api.send_email(options).await?;
  
  Ok(())
}
```

### Custom Configuration

```rust , no_run
use send_with_us::{Api, Config, ApiClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let config = Config::new("YOUR_API_KEY")
    .with_url("https://custom-api.sendwithus.com")
    .with_api_version("2")
    .with_debug(true);
  
  let api = Api::new(config);
  
  let templates = api.list_templates().await?;
  println!("Templates: {:?}", templates);
  
  Ok(())
}
```


### Working with Templates

```rust , no_run
use send_with_us::{Api, ApiClient, types::{TemplateOptions}};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let api = Api::with_api_key("YOUR_API_KEY");
  
  let template = TemplateOptions {
    name: "Welcome Email".to_string(),
    subject: "Welcome to Our Service".to_string(),
    html: "<html><body>Welcome, {{name}}!</body></html>".to_string(),
    text: "Welcome, {{name}}!".to_string(),
    preheader: Some("Welcome to our service".to_string()),
    amp_html: None,
  };

  let result = api.create_template(template).await?;

  Ok(())
}
```

### Error Handling

```rust , no_run
use send_with_us::{Api, ApiClient, Error, types::{EmailOptions, Recipient}};
#[tokio::main]

async fn main() {
  let api = Api::with_api_key("YOUR_API_KEY");
  let recipient = Recipient::new("test@example.com");
  let options = EmailOptions::new("", recipient); // Empty template ID

  match api.send_email(options).await {
    Ok(response) => println!("Email sent: {:?}", response),
    Err(Error::MissingTemplateId) => eprintln!("Error: Template ID is required"),
    Err(Error::InvalidCredentials) => eprintln!("Error: Invalid API key"),
    Err(Error::ConnectionFailed) => eprintln!("Error: Could not connect to SendWithUs API"),
    Err(err) => eprintln!("Error: {}", err),
  }
}
```

## API Documentation

The client provides methods for all SendWithUs API endpoints:

- `send_email` - Send a templated email
- `list_templates` - List all templates
- `render` - Render a template with data
- `create_template` - Create a new template
- `list_drip_campaigns` - List all drip campaigns
- `start_on_drip_campaign` - Start a recipient on a drip campaign
- `remove_from_drip_campaign` - Remove a recipient from a drip campaign
- And many more...

See the API documentation for complete details.

## Logging

This library provides optional logging integration via the `tracing` crate. To enable it, add the `logging` feature to your dependency:

```toml
[dependencies]
send_with_us = { version = "0.1.0", features = ["logging"] }
```

When the `logging` feature is enabled, the library will log API operations, request details, and responses at appropriate trace levels. This can be helpful for debugging and monitoring API usage.

For development and testing, the tracing crate is included as a dev-dependency, allowing example code to use it without requiring it for production usage.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
