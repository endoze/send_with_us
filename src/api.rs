use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::config::Config;
use crate::error::{Error, Result};
use crate::types::{
  CustomerOptions, DripCampaignOptions, EmailOptions, Recipient, RenderOptions, Sender,
  TemplateOptions,
};

/// SendWithUs API client for interacting with the SendWithUs email service.
///
/// This struct provides a complete implementation for making authenticated requests
/// to the SendWithUs API, allowing you to send transactional emails, manage templates,
/// work with drip campaigns, and handle customer data.
///
/// # Examples
///
/// ```
/// use send_with_us::{Api, Config};
///
/// let api = Api::with_api_key("your-api-key");
///
/// let config = Config::new("your-api-key")
///   .with_api_version("2")
///   .with_debug(true);
/// let api = Api::new(config);
/// ```
#[derive(Debug, Clone)]
pub struct Api {
  config: Config,
  client: Client,
}

/// API client trait defining all available SendWithUs operations.
///
/// This trait outlines the complete interface for interacting with the SendWithUs API.
/// It's implemented by the `Api` struct and can be mocked for testing purposes.
///
/// Each method corresponds to a specific API endpoint and operation in the SendWithUs service.
#[async_trait]
#[cfg(not(tarpaulin_include))]
pub trait ApiClient {
  /// Send an email using a template.
  ///
  /// # Arguments
  /// * `options` - Email sending options including template ID, recipient, data, and attachments
  ///
  /// # Returns
  /// API response with send status and tracking information
  async fn send_email(&self, options: EmailOptions) -> Result<Value>;

  /// List all available email templates.
  ///
  /// # Returns
  /// API response with template details including IDs, names, and versions
  async fn list_templates(&self) -> Result<Value>;

  /// Render a template with the provided data.
  ///
  /// # Arguments
  /// * `options` - Template rendering options including template ID and template data
  ///
  /// # Returns
  /// API response with the rendered template content
  async fn render(&self, options: RenderOptions) -> Result<Value>;

  /// Create a new email template.
  ///
  /// # Arguments
  /// * `options` - Template creation options including name, subject, and content
  ///
  /// # Returns
  /// API response with the created template details
  async fn create_template(&self, options: TemplateOptions) -> Result<Value>;

  /// List all available drip campaigns.
  ///
  /// # Returns
  /// API response with campaign details
  async fn list_drip_campaigns(&self) -> Result<Value>;

  /// Start a recipient on a drip campaign.
  ///
  /// # Arguments
  /// * `campaign_id` - ID of the drip campaign
  /// * `options` - Options including recipient address and email data
  ///
  /// # Returns
  /// API response with activation status
  async fn start_on_drip_campaign(
    &self,
    campaign_id: &str,
    options: DripCampaignOptions,
  ) -> Result<Value>;

  /// Remove a recipient from a drip campaign.
  ///
  /// # Arguments
  /// * `campaign_id` - ID of the drip campaign
  /// * `recipient_address` - Email address of the recipient to remove
  ///
  /// # Returns
  /// API response with deactivation status
  async fn remove_from_drip_campaign(
    &self,
    campaign_id: &str,
    recipient_address: &str,
  ) -> Result<Value>;

  /// Get details about a drip campaign.
  ///
  /// # Arguments
  /// * `campaign_id` - ID of the drip campaign
  ///
  /// # Returns
  /// API response with campaign details and statistics
  async fn drip_campaign_details(&self, campaign_id: &str) -> Result<Value>;

  /// Get customer details by email address.
  ///
  /// # Arguments
  /// * `email` - Customer's email address
  ///
  /// # Returns
  /// API response with customer data
  async fn customer_get(&self, email: &str) -> Result<Value>;

  /// Create a new customer record.
  ///
  /// # Arguments
  /// * `options` - Customer options including email and data
  ///
  /// # Returns
  /// API response with creation status
  async fn customer_create(&self, options: CustomerOptions) -> Result<Value>;

  /// Delete a customer record.
  ///
  /// # Arguments
  /// * `email` - Customer's email address
  ///
  /// # Returns
  /// API response with deletion status
  async fn customer_delete(&self, email: &str) -> Result<Value>;

  /// Get email logs for a specific customer.
  ///
  /// # Arguments
  /// * `email` - Customer's email address
  /// * `count` - Optional maximum number of logs to return
  /// * `created_gt` - Optional filter for logs created after this date
  /// * `created_lt` - Optional filter for logs created before this date
  ///
  /// # Returns
  /// API response with email log history
  async fn customer_email_log(
    &self,
    email: &str,
    count: Option<u32>,
    created_gt: Option<String>,
    created_lt: Option<String>,
  ) -> Result<Value>;

  /// Get details for a specific email log.
  ///
  /// # Arguments
  /// * `log_id` - Email log ID
  ///
  /// # Returns
  /// API response with log details
  async fn log(&self, log_id: &str) -> Result<Value>;

  /// Get events for a specific email log.
  ///
  /// # Arguments
  /// * `log_id` - Email log ID
  ///
  /// # Returns
  /// API response with events (sent, opened, clicked, etc.)
  async fn log_events(&self, log_id: &str) -> Result<Value>;

  /// Delete an email template.
  ///
  /// # Arguments
  /// * `template_id` - Template ID to delete
  ///
  /// # Returns
  /// API response with deletion status
  async fn delete_template(&self, template_id: &str) -> Result<Value>;

  /// List all versions of a template.
  ///
  /// # Arguments
  /// * `template_id` - Template ID
  ///
  /// # Returns
  /// API response with version details
  async fn list_template_versions(&self, template_id: &str) -> Result<Value>;

  /// Get a specific template version.
  ///
  /// # Arguments
  /// * `template_id` - Template ID
  /// * `version_id` - Version ID
  ///
  /// # Returns
  /// API response with version details and content
  async fn get_template_version(&self, template_id: &str, version_id: &str) -> Result<Value>;

  /// Update a template version.
  ///
  /// # Arguments
  /// * `template_id` - Template ID
  /// * `version_id` - Version ID
  /// * `options` - Template options with updated content
  ///
  /// # Returns
  /// API response with update status
  async fn update_template_version(
    &self,
    template_id: &str,
    version_id: &str,
    options: TemplateOptions,
  ) -> Result<Value>;

  /// Create a new version of a template.
  ///
  /// # Arguments
  /// * `template_id` - Template ID
  /// * `options` - Template options for the new version
  ///
  /// # Returns
  /// API response with new version details
  async fn create_template_version(
    &self,
    template_id: &str,
    options: TemplateOptions,
  ) -> Result<Value>;

  /// Unsubscribe an email address from all drip campaigns.
  ///
  /// # Arguments
  /// * `email_address` - Email address to unsubscribe
  ///
  /// # Returns
  /// API response with unsubscribe status
  async fn drips_unsubscribe(&self, email_address: &str) -> Result<Value>;
}

impl Api {
  /// Creates a new API client with the specified configuration.
  ///
  /// # Arguments
  /// * `config` - The SendWithUs API configuration
  ///
  /// # Returns
  /// A new Api instance with the specified configuration
  ///
  /// # Examples
  ///
  /// ```
  /// use send_with_us::{Api, Config};
  ///
  /// let config = Config::new("api-key").with_debug(true);
  /// let api = Api::new(config);
  /// ```
  pub fn new(config: Config) -> Self {
    let client = Client::new();
    Self { config, client }
  }

  /// Creates a new API client with just an API key, using default configuration.
  ///
  /// # Arguments
  /// * `api_key` - The SendWithUs API key
  ///
  /// # Returns
  /// A new Api instance with default configuration and the specified API key
  ///
  /// # Examples
  ///
  /// ```
  /// use send_with_us::Api;
  ///
  /// let api = Api::with_api_key("your-api-key");
  /// ```
  pub fn with_api_key(api_key: impl Into<String>) -> Self {
    let config = Config::new(api_key);
    Self::new(config)
  }

  /// Returns a reference to the API configuration.
  ///
  /// # Returns
  /// Reference to the current Config instance
  pub fn config(&self) -> &Config {
    &self.config
  }

  /// Builds the full request URL for a given API endpoint.
  ///
  /// # Arguments
  /// * `endpoint` - The API endpoint path
  ///
  /// # Returns
  /// The complete URL for the specified endpoint
  ///
  /// # Errors
  /// Returns an error if the base URL is not a valid API URL
  fn build_url(&self, endpoint: &str) -> Result<String> {
    let mut base = self.config.url.clone();

    base
      .path_segments_mut()
      .map_err(|_| Error::InvalidApiUrl)?
      .push("api")
      .push(&format!("v{}", self.config.api_version))
      .push(endpoint);

    Ok(base.to_string())
  }

  /// Makes an API request to the SendWithUs API.
  ///
  /// # Arguments
  /// * `method` - HTTP method (GET, POST, etc.)
  /// * `endpoint` - API endpoint path
  /// * `payload` - Optional JSON payload for the request
  ///
  /// # Returns
  /// Deserialized response from the API
  ///
  /// # Type Parameters
  /// * `T` - Type of the request payload
  /// * `R` - Type to deserialize the response into
  ///
  /// # Errors
  /// Returns an error if the request fails, authentication is invalid, or the response cannot be deserialized
  async fn request<T, R>(
    &self,
    method: reqwest::Method,
    endpoint: &str,
    payload: Option<&T>,
  ) -> Result<R>
  where
    T: Serialize + ?Sized,
    R: DeserializeOwned,
  {
    let url = self.build_url(endpoint)?;

    let mut request = self
      .client
      .request(method, &url)
      .header("Content-Type", "application/json")
      .header("X-SWU-API-KEY", &self.config.api_key)
      .header("X-SWU-API-CLIENT", &self.config.client_stub);

    if let Some(data) = payload {
      request = request.json(data);
    }

    if self.config.debug {
      eprintln!("SendWithUs Request: {:?}", request);
    }

    let response = request.send().await.map_err(|e| {
      if e.is_connect() {
        Error::ConnectionFailed
      } else {
        Error::RequestFailed(e)
      }
    })?;

    let status = response.status();
    let body = response.text().await?;

    if self.config.debug {
      eprintln!("SendWithUs Response: {}", body);
    }

    match status {
      StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => {
        serde_json::from_str(&body).map_err(Error::SerializationFailed)
      }
      StatusCode::NOT_FOUND => Err(Error::InvalidEndpoint(endpoint.to_string())),
      StatusCode::FORBIDDEN => Err(Error::InvalidCredentials),
      StatusCode::BAD_REQUEST => Err(Error::InvalidRequest(body)),
      _ => Err(Error::ApiError {
        status: status.as_u16(),
        message: body,
      }),
    }
  }
}

#[async_trait]
#[cfg(not(tarpaulin_include))]
impl ApiClient for Api {
  /// Send an email
  async fn send_email(&self, options: EmailOptions) -> Result<Value> {
    if options.email_id.is_empty() {
      return Err(Error::MissingTemplateId);
    }

    self
      .request(reqwest::Method::POST, "send", Some(&options))
      .await
  }

  /// List all templates
  async fn list_templates(&self) -> Result<Value> {
    self
      .request::<(), _>(reqwest::Method::GET, "emails", None)
      .await
  }

  /// Render a template
  async fn render(&self, options: RenderOptions) -> Result<Value> {
    self
      .request(reqwest::Method::POST, "render", Some(&options))
      .await
  }

  /// Create a new template
  async fn create_template(&self, options: TemplateOptions) -> Result<Value> {
    self
      .request(reqwest::Method::POST, "emails", Some(&options))
      .await
  }

  /// List all drip campaigns
  async fn list_drip_campaigns(&self) -> Result<Value> {
    self
      .request::<(), _>(reqwest::Method::GET, "drip_campaigns", None)
      .await
  }

  /// Start a recipient on a drip campaign
  async fn start_on_drip_campaign(
    &self,
    campaign_id: &str,
    options: DripCampaignOptions,
  ) -> Result<Value> {
    let endpoint = format!("drip_campaigns/{}/activate", campaign_id);
    self
      .request(reqwest::Method::POST, &endpoint, Some(&options))
      .await
  }

  /// Remove a recipient from a drip campaign
  async fn remove_from_drip_campaign(
    &self,
    campaign_id: &str,
    recipient_address: &str,
  ) -> Result<Value> {
    let endpoint = format!("drip_campaigns/{}/deactivate", campaign_id);
    let payload = serde_json::json!({ "recipient_address": recipient_address });
    self
      .request(reqwest::Method::POST, &endpoint, Some(&payload))
      .await
  }

  /// Get drip campaign details
  async fn drip_campaign_details(&self, campaign_id: &str) -> Result<Value> {
    let endpoint = format!("drip_campaigns/{}", campaign_id);
    self
      .request::<(), _>(reqwest::Method::GET, &endpoint, None)
      .await
  }

  /// Get customer details
  async fn customer_get(&self, email: &str) -> Result<Value> {
    let endpoint = format!("customers/{}", email);
    self
      .request::<(), _>(reqwest::Method::GET, &endpoint, None)
      .await
  }

  /// Create a new customer
  async fn customer_create(&self, options: CustomerOptions) -> Result<Value> {
    self
      .request(reqwest::Method::POST, "customers", Some(&options))
      .await
  }

  /// Delete a customer
  async fn customer_delete(&self, email: &str) -> Result<Value> {
    let endpoint = format!("customers/{}", email);
    self
      .request::<(), _>(reqwest::Method::DELETE, &endpoint, None)
      .await
  }

  /// Get customer email logs
  async fn customer_email_log(
    &self,
    email: &str,
    count: Option<u32>,
    created_gt: Option<String>,
    created_lt: Option<String>,
  ) -> Result<Value> {
    let mut params = Vec::new();

    if let Some(count) = count {
      params.push(format!("count={}", count));
    }

    if let Some(created_gt) = created_gt {
      params.push(format!("created_gt={}", created_gt));
    }

    if let Some(created_lt) = created_lt {
      params.push(format!("created_lt={}", created_lt));
    }

    let query_string = if !params.is_empty() {
      format!("?{}", params.join("&"))
    } else {
      String::new()
    };

    let endpoint = format!("customers/{}/logs{}", email, query_string);
    self
      .request::<(), _>(reqwest::Method::GET, &endpoint, None)
      .await
  }

  /// Get email log
  async fn log(&self, log_id: &str) -> Result<Value> {
    let endpoint = format!("logs/{}", log_id);
    self
      .request::<(), _>(reqwest::Method::GET, &endpoint, None)
      .await
  }

  /// Get email log events
  async fn log_events(&self, log_id: &str) -> Result<Value> {
    let endpoint = format!("logs/{}/events", log_id);
    self
      .request::<(), _>(reqwest::Method::GET, &endpoint, None)
      .await
  }

  /// Delete a template
  async fn delete_template(&self, template_id: &str) -> Result<Value> {
    let endpoint = format!("templates/{}", template_id);
    self
      .request::<(), _>(reqwest::Method::DELETE, &endpoint, None)
      .await
  }

  /// List template versions
  async fn list_template_versions(&self, template_id: &str) -> Result<Value> {
    let endpoint = format!("templates/{}/versions", template_id);
    self
      .request::<(), _>(reqwest::Method::GET, &endpoint, None)
      .await
  }

  /// Get template version
  async fn get_template_version(&self, template_id: &str, version_id: &str) -> Result<Value> {
    let endpoint = format!("templates/{}/versions/{}", template_id, version_id);
    self
      .request::<(), _>(reqwest::Method::GET, &endpoint, None)
      .await
  }

  /// Update template version
  async fn update_template_version(
    &self,
    template_id: &str,
    version_id: &str,
    options: TemplateOptions,
  ) -> Result<Value> {
    let endpoint = format!("templates/{}/versions/{}", template_id, version_id);
    self
      .request(reqwest::Method::PUT, &endpoint, Some(&options))
      .await
  }

  /// Create template version
  async fn create_template_version(
    &self,
    template_id: &str,
    options: TemplateOptions,
  ) -> Result<Value> {
    let endpoint = format!("templates/{}/versions", template_id);
    self
      .request(reqwest::Method::POST, &endpoint, Some(&options))
      .await
  }

  /// Unsubscribe from drips
  async fn drips_unsubscribe(&self, email_address: &str) -> Result<Value> {
    if email_address.is_empty() {
      return Err(Error::MissingRecipientAddress);
    }

    let payload = serde_json::json!({ "email_address": email_address });
    self
      .request(reqwest::Method::POST, "drips/unsubscribe", Some(&payload))
      .await
  }
}

/// Helper functions to build email options more easily.
///
/// This module contains utility functions that simplify the creation of common
/// email components such as email data, recipients, and senders.
pub mod helpers {
  use super::*;
  use serde_json::Value;
  use std::collections::HashMap;

  /// Creates a HashMap of email template data from key-value pairs.
  ///
  /// This helper function simplifies the creation of template data for emails.
  /// It accepts any iterable collection of key-value pairs and converts them into
  /// the required HashMap format for email template data.
  ///
  /// # Arguments
  /// * `pairs` - An iterable of key-value pairs where keys can be converted to String
  ///   and values can be converted to serde_json::Value
  ///
  /// # Returns
  /// A HashMap with string keys and JSON values
  ///
  /// # Examples
  ///
  /// ```
  /// use send_with_us::{ApiClient, api::helpers};
  /// use serde_json::json;
  ///
  /// let data = helpers::email_data([
  ///   ("name", json!("John Doe")),
  ///   ("order_id", json!("12345")),
  ///   ("items", json!(["item1", "item2"]))
  /// ]);
  ///
  /// assert_eq!(data["name"], "John Doe");
  /// assert_eq!(data["order_id"], "12345");
  /// ```
  pub fn email_data<K, V>(pairs: impl IntoIterator<Item = (K, V)>) -> HashMap<String, Value>
  where
    K: Into<String>,
    V: Into<Value>,
  {
    pairs
      .into_iter()
      .map(|(k, v)| (k.into(), v.into()))
      .collect()
  }

  /// Creates a Recipient with an email address and optional name.
  ///
  /// # Arguments
  /// * `email` - The recipient's email address
  /// * `name` - Optional recipient name
  ///
  /// # Returns
  /// A configured Recipient instance
  ///
  /// # Examples
  ///
  /// ```
  /// use send_with_us::api::helpers;
  ///
  /// let recipient = helpers::recipient("user@example.com", None::<&str>);
  ///
  /// let recipient = helpers::recipient("user@example.com", Some("John Doe"));
  /// ```
  pub fn recipient(email: impl Into<String>, name: Option<impl Into<String>>) -> Recipient {
    let mut recipient = Recipient::new(email);
    if let Some(name) = name {
      recipient = recipient.with_name(name);
    }
    recipient
  }

  /// Creates a Sender with an email address and optional name and reply-to address.
  ///
  /// # Arguments
  /// * `email` - The sender's email address
  /// * `name` - Optional sender name
  /// * `reply_to` - Optional reply-to email address
  ///
  /// # Returns
  /// A configured Sender instance
  ///
  /// # Examples
  ///
  /// ```
  /// use send_with_us::api::helpers;
  ///
  /// let sender = helpers::sender("support@company.com", None::<&str>, None::<&str>);
  ///
  /// let sender = helpers::sender("support@company.com", Some("Support Team"), None::<&str>);
  ///
  /// let sender = helpers::sender(
  ///   "noreply@company.com",
  ///   Some("Company Name"),
  ///   Some("support@company.com")
  /// );
  /// ```
  pub fn sender(
    email: impl Into<String>,
    name: Option<impl Into<String>>,
    reply_to: Option<impl Into<String>>,
  ) -> Sender {
    let mut sender = Sender::new(email);

    if let Some(name) = name {
      sender = sender.with_name(name);
    }

    if let Some(reply_to) = reply_to {
      sender = sender.with_reply_to(reply_to);
    }

    sender
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::HashMap;

  struct MockApiClient;

  #[async_trait]
  impl ApiClient for MockApiClient {
    async fn send_email(&self, options: EmailOptions) -> Result<Value> {
      if options.email_id.is_empty() {
        return Err(Error::MissingTemplateId);
      }
      Ok(serde_json::json!({"success": true}))
    }

    async fn list_templates(&self) -> Result<Value> {
      Ok(serde_json::json!([
        {"id": "template_1", "name": "Template 1"},
        {"id": "template_2", "name": "Template 2"}
      ]))
    }

    async fn render(&self, options: RenderOptions) -> Result<Value> {
      Ok(serde_json::json!({
        "template": options.template,
        "rendered_template": "<html>Rendered template</html>"
      }))
    }

    async fn create_template(&self, options: TemplateOptions) -> Result<Value> {
      Ok(serde_json::json!({
        "id": "new_template",
        "name": options.name,
        "created": true
      }))
    }

    async fn list_drip_campaigns(&self) -> Result<Value> {
      Ok(serde_json::json!([
        {"id": "campaign_1", "name": "Campaign 1"},
        {"id": "campaign_2", "name": "Campaign 2"}
      ]))
    }

    async fn start_on_drip_campaign(
      &self,
      campaign_id: &str,
      options: DripCampaignOptions,
    ) -> Result<Value> {
      Ok(serde_json::json!({
        "success": true,
        "recipient": options.recipient_address,
        "campaign_id": campaign_id
      }))
    }

    async fn remove_from_drip_campaign(
      &self,
      campaign_id: &str,
      recipient_address: &str,
    ) -> Result<Value> {
      Ok(serde_json::json!({
        "success": true,
        "recipient": recipient_address,
        "campaign_id": campaign_id
      }))
    }

    async fn drip_campaign_details(&self, campaign_id: &str) -> Result<Value> {
      Ok(serde_json::json!({
        "id": campaign_id,
        "name": format!("Campaign {}", campaign_id),
        "details": "Some details"
      }))
    }

    async fn customer_get(&self, email: &str) -> Result<Value> {
      Ok(serde_json::json!({
        "email": email,
        "data": {"name": "Test Customer"}
      }))
    }

    async fn customer_create(&self, options: CustomerOptions) -> Result<Value> {
      Ok(serde_json::json!({
        "success": true,
        "email": options.email
      }))
    }

    async fn customer_delete(&self, email: &str) -> Result<Value> {
      Ok(serde_json::json!({
        "success": true,
        "email": email
      }))
    }

    async fn customer_email_log(
      &self,
      email: &str,
      count: Option<u32>,
      created_gt: Option<String>,
      created_lt: Option<String>,
    ) -> Result<Value> {
      let mut response = serde_json::json!({
        "email": email,
        "logs": []
      });

      if let Some(count) = count {
        response["count"] = serde_json::json!(count);
      }

      if let Some(created_gt) = created_gt {
        response["created_gt"] = serde_json::json!(created_gt);
      }

      if let Some(created_lt) = created_lt {
        response["created_lt"] = serde_json::json!(created_lt);
      }

      Ok(response)
    }

    async fn log(&self, log_id: &str) -> Result<Value> {
      Ok(serde_json::json!({
        "id": log_id,
        "status": "delivered"
      }))
    }

    async fn log_events(&self, log_id: &str) -> Result<Value> {
      Ok(serde_json::json!({
        "log_id": log_id,
        "events": [
          {"type": "sent", "timestamp": "2023-01-01T12:00:00Z"},
          {"type": "delivered", "timestamp": "2023-01-01T12:01:00Z"}
        ]
      }))
    }

    async fn delete_template(&self, template_id: &str) -> Result<Value> {
      Ok(serde_json::json!({
        "success": true,
        "template_id": template_id
      }))
    }

    async fn list_template_versions(&self, template_id: &str) -> Result<Value> {
      Ok(serde_json::json!({
        "template_id": template_id,
        "versions": [
          {"id": "v1", "name": "Version 1"},
          {"id": "v2", "name": "Version 2"}
        ]
      }))
    }

    async fn get_template_version(&self, template_id: &str, version_id: &str) -> Result<Value> {
      Ok(serde_json::json!({
        "template_id": template_id,
        "version_id": version_id,
        "html": "<html>Template content</html>"
      }))
    }

    async fn update_template_version(
      &self,
      template_id: &str,
      version_id: &str,
      options: TemplateOptions,
    ) -> Result<Value> {
      Ok(serde_json::json!({
        "success": true,
        "template_id": template_id,
        "version_id": version_id,
        "name": options.name
      }))
    }

    async fn create_template_version(
      &self,
      template_id: &str,
      options: TemplateOptions,
    ) -> Result<Value> {
      Ok(serde_json::json!({
        "success": true,
        "template_id": template_id,
        "new_version": {
          "id": "new_version",
          "name": options.name
        }
      }))
    }

    async fn drips_unsubscribe(&self, email_address: &str) -> Result<Value> {
      if email_address.is_empty() {
        return Err(Error::MissingRecipientAddress);
      }

      Ok(serde_json::json!({
        "success": true,
        "email": email_address
      }))
    }
  }

  #[tokio::test]
  async fn test_api_initialization() {
    let api = Api::with_api_key("test-api-key");
    assert_eq!(api.config().api_key, "test-api-key");
    assert_eq!(api.config().api_version, "1");

    let custom_config = Config::new("custom-key")
      .with_api_version("2")
      .with_debug(true);

    let api_with_config = Api::new(custom_config);
    assert_eq!(api_with_config.config().api_key, "custom-key");
    assert_eq!(api_with_config.config().api_version, "2");
    assert!(api_with_config.config().debug);
  }

  #[tokio::test]
  async fn test_build_url() {
    let api = Api::with_api_key("api-key");
    let url = api.build_url("test-endpoint").expect("Failed to build URL");
    assert!(url.contains("/api/v1/test-endpoint"));
    assert!(url.starts_with("https://api.sendwithus.com"));
  }

  #[tokio::test]
  async fn test_mock_client_send_email() {
    let mock_client = MockApiClient;

    let recipient = Recipient::new("test@example.com").with_name("Test User");
    let options = EmailOptions::new("template-id", recipient);
    let result = mock_client.send_email(options).await;
    assert!(result.is_ok());

    let recipient = Recipient::new("test@example.com");
    let invalid_options = EmailOptions::new("", recipient);
    let result = mock_client.send_email(invalid_options).await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::MissingTemplateId));
  }

  #[tokio::test]
  async fn test_mock_client_customer_email_log() {
    let mock_client = MockApiClient;

    let result = mock_client
      .customer_email_log("test@example.com", None, None, None)
      .await;
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value["email"], "test@example.com");
    assert!(value.get("count").is_none());

    let result = mock_client
      .customer_email_log("test@example.com", Some(2), None, None)
      .await;
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value["email"], "test@example.com");
    assert_eq!(value["count"], 2);
  }

  #[tokio::test]
  async fn test_mock_client_log() {
    let mock_client = MockApiClient;

    let log_id = "log_TESTTEST123";
    let result = mock_client.log(log_id).await;
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value["id"], log_id);
  }

  #[tokio::test]
  async fn test_mock_client_start_on_drip_campaign() {
    let mock_client = MockApiClient;
    let email = "some@email.stub";
    let campaign_id = "dc_SoMeCampaIGnID";

    let mut email_data = HashMap::new();
    email_data.insert("foo".to_string(), serde_json::json!("bar"));

    let options = DripCampaignOptions {
      recipient_address: email.to_string(),
      email_data: Some(email_data),
      tags: None,
      locale: None,
    };

    let result = mock_client
      .start_on_drip_campaign(campaign_id, options)
      .await;
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value["recipient"], email);
    assert_eq!(value["campaign_id"], campaign_id);
  }

  #[tokio::test]
  async fn test_mock_client_render() {
    let mock_client = MockApiClient;
    let template_id = "template-id";
    let version_id = Some("some-version-id".to_string());
    let locale = Some("fr-CA".to_string());

    let mut template_data = HashMap::new();
    template_data.insert("foo".to_string(), serde_json::json!("bar"));

    let options = RenderOptions {
      template: template_id.to_string(),
      version_id,
      template_data,
      strict: true,
      locale,
    };

    let result = mock_client.render(options).await;
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value["template"], template_id);
    assert_eq!(value["rendered_template"], "<html>Rendered template</html>");
  }

  #[tokio::test]
  async fn test_mock_client_drips_unsubscribe() {
    let mock_client = MockApiClient;

    let result = mock_client.drips_unsubscribe("test@example.com").await;
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value["email"], "test@example.com");

    let result = mock_client.drips_unsubscribe("").await;
    assert!(result.is_err());
    assert!(matches!(
      result.unwrap_err(),
      Error::MissingRecipientAddress
    ));
  }

  #[test]
  fn test_helpers_email_data() {
    let data = helpers::email_data([("name", "John"), ("age", "30")]);

    assert_eq!(data["name"], "John");
    assert_eq!(data["age"], "30");
  }

  #[test]
  fn test_helpers_recipient() {
    let recipient = helpers::recipient("test@example.com", Some("Test User"));

    assert_eq!(recipient.address, "test@example.com");
    assert_eq!(recipient.name, Some("Test User".to_string()));

    let recipient = helpers::recipient("test@example.com", None::<String>);

    assert_eq!(recipient.address, "test@example.com");
    assert_eq!(recipient.name, None);
  }

  #[test]
  fn test_helpers_sender() {
    let sender = helpers::sender(
      "sender@example.com",
      Some("Sender Name"),
      Some("reply@example.com"),
    );

    assert_eq!(sender.address, "sender@example.com");
    assert_eq!(sender.name, Some("Sender Name".to_string()));
    assert_eq!(sender.reply_to, Some("reply@example.com".to_string()));

    let sender = helpers::sender("sender@example.com", None::<String>, None::<String>);

    assert_eq!(sender.address, "sender@example.com");
    assert_eq!(sender.name, None);
    assert_eq!(sender.reply_to, None);
  }
}

#[cfg(test)]
mod request_tests {
  use super::*;
  use mockito::Matcher;
  use reqwest::Client;
  use serde_json::{Value, json};
  use std::net::TcpListener;

  #[tokio::test]
  async fn test_request_success() {
    let mut mock_server = mockito::Server::new_async().await;
    let url = mock_server.url();

    let mock = mock_server
      .mock("GET", "/api/v1/test-endpoint")
      .match_header("Content-Type", "application/json")
      .match_header("X-SWU-API-KEY", "test-api-key")
      .match_header("X-SWU-API-CLIENT", Matcher::Any)
      .with_status(200)
      .with_body(r#"{"success": true, "message": "Test response"}"#)
      .create();

    let mut config = Config::new("test-api-key");
    config.url = url::Url::parse(&url).unwrap();
    let api = Api::new(config);

    let response: Value = api
      .request(reqwest::Method::GET, "test-endpoint", None::<&Value>)
      .await
      .unwrap();

    assert_eq!(response["success"], json!(true));
    assert_eq!(response["message"], json!("Test response"));

    mock.assert();
  }

  #[tokio::test]
  async fn test_request_with_payload() {
    let mut mock_server = mockito::Server::new_async().await;
    let url = mock_server.url();

    let mock = mock_server
      .mock("POST", "/api/v1/test-endpoint")
      .match_header("Content-Type", "application/json")
      .match_header("X-SWU-API-KEY", "test-api-key")
      .match_body(r#"{"data":"test value"}"#)
      .with_status(201)
      .with_body(r#"{"success": true, "data_received": true}"#)
      .create();

    let mut config = Config::new("test-api-key");
    config.url = url::Url::parse(&url).unwrap();
    let api = Api::new(config);

    let payload = json!({"data": "test value"});

    let response: Value = api
      .request(reqwest::Method::POST, "test-endpoint", Some(&payload))
      .await
      .unwrap();

    assert_eq!(response["success"], json!(true));
    assert_eq!(response["data_received"], json!(true));

    mock.assert();
  }

  #[tokio::test]
  async fn test_request_not_found() {
    let mut mock_server = mockito::Server::new_async().await;
    let url = mock_server.url();

    let mock = mock_server
      .mock("GET", "/api/v1/nonexistent-endpoint")
      .with_status(404)
      .with_body("Not Found")
      .create();

    let mut config = Config::new("test-api-key");
    config.url = url::Url::parse(&url).unwrap();
    let api = Api::new(config);

    let result: Result<Value> = api
      .request(reqwest::Method::GET, "nonexistent-endpoint", None::<&Value>)
      .await;

    assert!(result.is_err());

    match result.unwrap_err() {
      Error::InvalidEndpoint(endpoint) => {
        assert_eq!(endpoint, "nonexistent-endpoint");
      }
      err => panic!("Unexpected error: {:?}", err),
    }

    mock.assert();
  }

  #[tokio::test]
  async fn test_request_accepted() {
    let mut mock_server = mockito::Server::new_async().await;
    let url = mock_server.url();

    let mock = mock_server
      .mock("POST", "/api/v1/test-endpoint")
      .match_header("Content-Type", "application/json")
      .match_header("X-SWU-API-KEY", "test-api-key")
      .with_status(202)
      .with_body(r#"{"status": "accepted", "message": "Request accepted"}"#)
      .create();

    let mut config = Config::new("test-api-key");
    config.url = url::Url::parse(&url).unwrap();
    let api = Api::new(config);

    #[derive(Debug, serde::Deserialize)]
    struct TestResponse {
      status: String,
      message: String,
    }

    let response: TestResponse = api
      .request(reqwest::Method::POST, "test-endpoint", None::<&Value>)
      .await
      .unwrap();

    assert_eq!(response.status, "accepted");
    assert_eq!(response.message, "Request accepted");

    mock.assert();
  }

  #[tokio::test]
  async fn test_request_forbidden() {
    let mut mock_server = mockito::Server::new_async().await;
    let url = mock_server.url();

    let mock = mock_server
      .mock("GET", "/api/v1/test-endpoint")
      .with_status(403)
      .with_body("Forbidden")
      .create();

    let mut config = Config::new("test-api-key");
    config.url = url::Url::parse(&url).unwrap();
    let api = Api::new(config);

    let result: Result<Value> = api
      .request(reqwest::Method::GET, "test-endpoint", None::<&Value>)
      .await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::InvalidCredentials));

    mock.assert();
  }

  #[tokio::test]
  async fn test_request_bad_request() {
    let mut mock_server = mockito::Server::new_async().await;
    let url = mock_server.url();
    let error_message = "Missing required field";

    let mock = mock_server
      .mock("POST", "/api/v1/test-endpoint")
      .with_status(400)
      .with_body(error_message)
      .create();

    let mut config = Config::new("test-api-key");
    config.url = url::Url::parse(&url).unwrap();
    let api = Api::new(config);

    let result: Result<Value> = api
      .request(reqwest::Method::POST, "test-endpoint", Some(&json!({})))
      .await;

    assert!(result.is_err());

    match result.unwrap_err() {
      Error::InvalidRequest(message) => {
        assert_eq!(message, error_message);
      }
      err => panic!("Unexpected error: {:?}", err),
    }

    mock.assert();
  }

  #[tokio::test]
  async fn test_request_api_error() {
    let mut mock_server = mockito::Server::new_async().await;
    let url = mock_server.url();
    let error_message = "Internal server error";

    let mock = mock_server
      .mock("GET", "/api/v1/test-endpoint")
      .with_status(500)
      .with_body(error_message)
      .create();

    let mut config = Config::new("test-api-key");
    config.url = url::Url::parse(&url).unwrap();
    let api = Api::new(config);

    let result: Result<Value> = api
      .request(reqwest::Method::GET, "test-endpoint", None::<&Value>)
      .await;

    assert!(result.is_err());

    match result.unwrap_err() {
      Error::ApiError { status, message } => {
        assert_eq!(status, 500);
        assert_eq!(message, error_message);
      }
      err => panic!("Unexpected error: {:?}", err),
    }

    mock.assert();
  }

  #[tokio::test]
  async fn test_request_with_custom_client() {
    let mut mock_server = mockito::Server::new_async().await;
    let url = mock_server.url();

    let mock = mock_server
      .mock("GET", "/api/v1/test-endpoint")
      .match_header("Content-Type", "application/json")
      .match_header("X-SWU-API-KEY", "test-api-key")
      .match_header("X-SWU-API-CLIENT", Matcher::Any)
      .match_header("User-Agent", "test-agent")
      .with_status(200)
      .with_body(r#"{"success": true}"#)
      .create();

    let custom_client = Client::builder().user_agent("test-agent").build().unwrap();

    let mut config = Config::new("test-api-key");
    config.url = url::Url::parse(&url).unwrap();

    let api = Api {
      config,
      client: custom_client,
    };

    let response: Value = api
      .request(reqwest::Method::GET, "test-endpoint", None::<&Value>)
      .await
      .unwrap();

    assert_eq!(response["success"], json!(true));

    mock.assert();
  }

  #[tokio::test]
  async fn test_request_debug_mode() {
    let mut mock_server = mockito::Server::new_async().await;
    let url = mock_server.url();

    let mock = mock_server
      .mock("GET", "/api/v1/test-endpoint")
      .match_header("Content-Type", "application/json")
      .match_header("X-SWU-API-KEY", "test-api-key")
      .with_status(200)
      .with_body(r#"{"success": true}"#)
      .create();

    let mut config = Config::new("test-api-key");
    config.url = url::Url::parse(&url).unwrap();
    config.debug = true;
    let api = Api::new(config);

    let response: Value = api
      .request(reqwest::Method::GET, "test-endpoint", None::<&Value>)
      .await
      .unwrap();

    assert_eq!(response["success"], json!(true));

    mock.assert();
  }

  #[tokio::test]
  async fn test_request_connection_failed() {
    let mut config = Config::new("test-api-key");

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    drop(listener);

    let url = format!("http://127.0.0.1:{}", addr.port());
    config.url = url::Url::parse(&url).unwrap();

    let api = Api::new(config);

    let result: Result<Value> = api
      .request(reqwest::Method::GET, "test-endpoint", None::<&Value>)
      .await;

    assert!(result.is_err());
    match result.unwrap_err() {
      Error::ConnectionFailed => {
        // This is the expected error
      }
      err => panic!("Expected ConnectionFailed error, got: {:?}", err),
    }
  }

  #[tokio::test]
  async fn test_request_failed() {
    let invalid_url = "invalid://example.com";

    let mut config = Config::new("test-api-key");
    config.url = url::Url::parse(invalid_url)
      .unwrap_or_else(|_| url::Url::parse("file:///nonexistent-path-for-testing").unwrap());

    let api = Api::new(config);

    let result: Result<Value> = api
      .request(reqwest::Method::GET, "test-endpoint", None::<&Value>)
      .await;

    assert!(result.is_err());
    match result.unwrap_err() {
      Error::RequestFailed(e) => {
        assert!(!e.is_connect(), "Expected non-connection reqwest error");
      }
      err => panic!("Expected RequestFailed error, got: {:?}", err),
    }
  }
}
