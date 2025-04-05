use crate::attachment::Attachment;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents an email recipient with an email address and optional name.
///
/// Recipients are used in the `EmailOptions` struct to specify who will
/// receive an email, including primary recipients, CC, and BCC.
///
/// # Examples
///
/// ```
/// use send_with_us::types::Recipient;
///
/// let recipient = Recipient::new("user@example.com");
///
/// let recipient = Recipient::new("user@example.com")
///   .with_name("John Doe");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Recipient {
  /// Recipient's email address
  pub address: String,

  /// Recipient's name (optional)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
}

impl Recipient {
  /// Creates a new recipient with the specified email address.
  ///
  /// # Arguments
  /// * `address` - The recipient's email address
  ///
  /// # Returns
  /// A new Recipient instance with the provided email and no name
  ///
  /// # Examples
  ///
  /// ```
  /// use send_with_us::types::Recipient;
  ///
  /// let recipient = Recipient::new("user@example.com");
  /// assert_eq!(recipient.address, "user@example.com");
  /// assert_eq!(recipient.name, None);
  /// ```
  pub fn new(address: impl Into<String>) -> Self {
    Self {
      address: address.into(),
      name: None,
    }
  }

  /// Sets the recipient's name.
  ///
  /// This method uses the builder pattern, allowing for method chaining.
  ///
  /// # Arguments
  /// * `name` - The recipient's name
  ///
  /// # Returns
  /// Self with the updated name for method chaining
  ///
  /// # Examples
  ///
  /// ```
  /// use send_with_us::types::Recipient;
  ///
  /// let recipient = Recipient::new("user@example.com")
  ///   .with_name("John Doe");
  ///     
  /// assert_eq!(recipient.address, "user@example.com");
  /// assert_eq!(recipient.name, Some("John Doe".to_string()));
  /// ```
  pub fn with_name(mut self, name: impl Into<String>) -> Self {
    self.name = Some(name.into());
    self
  }
}

/// Represents email sender information including address, name, and reply-to.
///
/// The Sender struct is used in `EmailOptions` to specify who the email
/// appears to be from, which can differ from the actual sending address
/// managed by SendWithUs.
///
/// # Examples
///
/// ```
/// use send_with_us::types::Sender;
///
/// // Create a sender with just an email address
/// let sender = Sender::new("support@company.com");
///
/// // Create a sender with a name
/// let sender = Sender::new("noreply@company.com")
///   .with_name("Company Support");
///
/// // Create a sender with a name and reply-to address
/// let sender = Sender::new("noreply@company.com")
///   .with_name("Company Support")
///   .with_reply_to("support@company.com");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sender {
  /// Sender's email address
  pub address: String,

  /// Sender's name (optional)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,

  /// Reply-to address (optional)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub reply_to: Option<String>,
}

impl Sender {
  /// Creates a new sender with the specified email address.
  ///
  /// # Arguments
  /// * `address` - The sender's email address
  ///
  /// # Returns
  /// A new Sender instance with the provided email address
  ///
  /// # Examples
  ///
  /// ```
  /// use send_with_us::types::Sender;
  ///
  /// let sender = Sender::new("support@company.com");
  /// assert_eq!(sender.address, "support@company.com");
  /// assert_eq!(sender.name, None);
  /// assert_eq!(sender.reply_to, None);
  /// ```
  pub fn new(address: impl Into<String>) -> Self {
    Self {
      address: address.into(),
      name: None,
      reply_to: None,
    }
  }

  /// Sets the sender's name.
  ///
  /// This method uses the builder pattern, allowing for method chaining.
  ///
  /// # Arguments
  /// * `name` - The sender's name
  ///
  /// # Returns
  /// Self with the updated name for method chaining
  ///
  /// # Examples
  ///
  /// ```
  /// use send_with_us::types::Sender;
  ///
  /// let sender = Sender::new("support@company.com")
  ///   .with_name("Company Support Team");
  ///     
  /// assert_eq!(sender.name, Some("Company Support Team".to_string()));
  /// ```
  pub fn with_name(mut self, name: impl Into<String>) -> Self {
    self.name = Some(name.into());
    self
  }

  /// Sets the reply-to address.
  ///
  /// This address will be used when recipients click "Reply" in their email client.
  ///
  /// # Arguments
  /// * `reply_to` - The email address to use for replies
  ///
  /// # Returns
  /// Self with the updated reply-to for method chaining
  ///
  /// # Examples
  ///
  /// ```
  /// use send_with_us::types::Sender;
  ///
  /// let sender = Sender::new("noreply@company.com")
  ///   .with_reply_to("support@company.com");
  ///     
  /// assert_eq!(sender.reply_to, Some("support@company.com".to_string()));
  /// ```
  pub fn with_reply_to(mut self, reply_to: impl Into<String>) -> Self {
    self.reply_to = Some(reply_to.into());
    self
  }
}

/// Represents the complete set of options for sending an email through SendWithUs.
///
/// This struct is the primary interface for configuring emails to be sent using the
/// SendWithUs API. It includes the template ID, recipient information, dynamic data,
/// and various optional settings like CC/BCC recipients, attachments, and more.
///
/// Uses the builder pattern to create and configure email options in a fluent interface.
///
/// # Examples
///
/// ```
/// use send_with_us::types::{EmailOptions, Recipient, Sender};
/// use std::collections::HashMap;
/// use serde_json::json;
///
/// let recipient = Recipient::new("user@example.com").with_name("User Name");
/// let email = EmailOptions::new("template-id-123", recipient);
///
/// let recipient = Recipient::new("customer@example.com").with_name("Customer");
/// let sender = Sender::new("support@company.com").with_name("Support Team");
///
/// let mut data = HashMap::new();
/// data.insert("name".to_string(), json!("John"));
/// data.insert("order_id".to_string(), json!("12345"));
///
/// let email = EmailOptions::new("template-id-123", recipient)
///   .with_data(data)
///   .with_sender(sender)
///   .with_cc(vec![Recipient::new("manager@company.com")])
///   .with_locale("en-US")
///   .with_tags(vec!["welcome".to_string(), "new-user".to_string()]);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmailOptions {
  /// Email template ID
  pub email_id: String,

  /// Email recipient
  pub recipient: Recipient,

  /// Dynamic email data for template variables
  #[serde(rename = "email_data", skip_serializing_if = "Option::is_none")]
  pub data: Option<HashMap<String, serde_json::Value>>,

  /// Email sender information
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sender: Option<Sender>,

  /// CC recipients
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cc: Option<Vec<Recipient>>,

  /// BCC recipients
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bcc: Option<Vec<Recipient>>,

  /// File attachments
  #[serde(skip_serializing_if = "Option::is_none")]
  pub files: Option<Vec<Attachment>>,

  /// ESP account identifier
  #[serde(skip_serializing_if = "Option::is_none")]
  pub esp_account: Option<String>,

  /// Template version name
  #[serde(skip_serializing_if = "Option::is_none")]
  pub version_name: Option<String>,

  /// Custom email headers
  #[serde(skip_serializing_if = "Option::is_none")]
  pub headers: Option<HashMap<String, String>>,

  /// Tags for email categorization
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tags: Option<Vec<String>>,

  /// Locale for internationalization
  #[serde(skip_serializing_if = "Option::is_none")]
  pub locale: Option<String>,
}

impl EmailOptions {
  /// Creates new email options with a template ID and recipient.
  ///
  /// # Arguments
  /// * `email_id` - The SendWithUs template ID to use
  /// * `recipient` - The primary recipient of the email
  ///
  /// # Returns
  /// A new EmailOptions instance with the specified template and recipient
  ///
  /// # Examples
  ///
  /// ```
  /// use send_with_us::types::{EmailOptions, Recipient};
  ///
  /// let recipient = Recipient::new("user@example.com");
  /// let options = EmailOptions::new("template-123", recipient);
  ///
  /// assert_eq!(options.email_id, "template-123");
  /// ```
  pub fn new(email_id: impl Into<String>, recipient: Recipient) -> Self {
    Self {
      email_id: email_id.into(),
      recipient,
      data: None,
      sender: None,
      cc: None,
      bcc: None,
      files: None,
      esp_account: None,
      version_name: None,
      headers: None,
      tags: None,
      locale: None,
    }
  }

  /// Sets dynamic data for the email template.
  ///
  /// This data is used to replace variables in the template with actual values.
  ///
  /// # Arguments
  /// * `data` - HashMap of template variable names to values
  ///
  /// # Returns
  /// Self with the added template data for method chaining
  ///
  /// # Examples
  ///
  /// ```
  /// use send_with_us::types::{EmailOptions, Recipient};
  /// use std::collections::HashMap;
  /// use serde_json::json;
  ///
  /// let recipient = Recipient::new("user@example.com");
  ///
  /// let mut data = HashMap::new();
  /// data.insert("first_name".to_string(), json!("John"));
  /// data.insert("last_name".to_string(), json!("Doe"));
  /// data.insert("items".to_string(), json!(["item1", "item2"]));
  ///
  /// let options = EmailOptions::new("template-123", recipient)
  ///     .with_data(data);
  /// ```
  pub fn with_data(mut self, data: HashMap<String, serde_json::Value>) -> Self {
    self.data = Some(data);
    self
  }

  /// Sets the sender information for the email.
  ///
  /// # Arguments
  /// * `sender` - The sender information
  ///
  /// # Returns
  /// Self with the added sender for method chaining
  ///
  /// # Examples
  ///
  /// ```
  /// use send_with_us::types::{EmailOptions, Recipient, Sender};
  ///
  /// let recipient = Recipient::new("user@example.com");
  /// let sender = Sender::new("support@company.com")
  ///     .with_name("Support Team");
  ///
  /// let options = EmailOptions::new("template-123", recipient)
  ///     .with_sender(sender);
  /// ```
  pub fn with_sender(mut self, sender: Sender) -> Self {
    self.sender = Some(sender);
    self
  }

  /// Adds CC (carbon copy) recipients to the email.
  ///
  /// # Arguments
  /// * `cc` - Vector of CC recipients
  ///
  /// # Returns
  /// Self with the added CC recipients for method chaining
  ///
  /// # Examples
  ///
  /// ```
  /// use send_with_us::types::{EmailOptions, Recipient};
  ///
  /// let recipient = Recipient::new("user@example.com");
  /// let cc1 = Recipient::new("manager@company.com");
  /// let cc2 = Recipient::new("support@company.com").with_name("Support Team");
  ///
  /// let options = EmailOptions::new("template-123", recipient)
  ///     .with_cc(vec![cc1, cc2]);
  /// ```
  pub fn with_cc(mut self, cc: Vec<Recipient>) -> Self {
    self.cc = Some(cc);
    self
  }

  /// Adds BCC (blind carbon copy) recipients to the email.
  ///
  /// # Arguments
  /// * `bcc` - Vector of BCC recipients
  ///
  /// # Returns
  /// Self with the added BCC recipients for method chaining
  ///
  /// # Examples
  ///
  /// ```
  /// use send_with_us::types::{EmailOptions, Recipient};
  ///
  /// let recipient = Recipient::new("user@example.com");
  /// let bcc = Recipient::new("archive@company.com");
  ///
  /// let options = EmailOptions::new("template-123", recipient)
  ///     .with_bcc(vec![bcc]);
  /// ```
  pub fn with_bcc(mut self, bcc: Vec<Recipient>) -> Self {
    self.bcc = Some(bcc);
    self
  }

  /// Adds file attachments to the email.
  ///
  /// # Arguments
  /// * `files` - Vector of file attachments
  ///
  /// # Returns
  /// Self with the added attachments for method chaining
  ///
  /// # Examples
  ///
  /// ```
  /// use send_with_us::types::{EmailOptions, Recipient};
  /// use send_with_us::Attachment;
  ///
  /// # fn example() {
  /// let recipient = Recipient::new("user@example.com");
  ///
  /// // In a real application, you would load these attachments from files
  /// let attachment = Attachment::from_bytes(b"test content", "test.txt");
  ///
  /// let options = EmailOptions::new("template-123", recipient)
  ///   .with_files(vec![attachment]);
  /// # }
  /// ```
  pub fn with_files(mut self, files: Vec<Attachment>) -> Self {
    self.files = Some(files);
    self
  }

  /// Sets the ESP (Email Service Provider) account to use.
  ///
  /// Some SendWithUs configurations allow for multiple ESP integrations.
  /// This option specifies which one to use for this email.
  ///
  /// # Arguments
  /// * `esp_account` - The ESP account identifier
  ///
  /// # Returns
  /// Self with the ESP account set for method chaining
  pub fn with_esp_account(mut self, esp_account: impl Into<String>) -> Self {
    self.esp_account = Some(esp_account.into());
    self
  }

  /// Sets a specific template version to use.
  ///
  /// # Arguments
  /// * `version_name` - The template version name
  ///
  /// # Returns
  /// Self with the template version set for method chaining
  pub fn with_version_name(mut self, version_name: impl Into<String>) -> Self {
    self.version_name = Some(version_name.into());
    self
  }

  /// Sets custom email headers.
  ///
  /// # Arguments
  /// * `headers` - HashMap of header names to values
  ///
  /// # Returns
  /// Self with the custom headers set for method chaining
  ///
  /// # Examples
  ///
  /// ```
  /// use send_with_us::types::{EmailOptions, Recipient};
  /// use std::collections::HashMap;
  ///
  /// let recipient = Recipient::new("user@example.com");
  ///
  /// let mut headers = HashMap::new();
  /// headers.insert("X-Custom-Header".to_string(), "Custom Value".to_string());
  /// headers.insert("X-Priority".to_string(), "1".to_string());
  ///
  /// let options = EmailOptions::new("template-123", recipient)
  ///   .with_headers(headers);
  /// ```
  pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
    self.headers = Some(headers);
    self
  }

  /// Adds tags to the email for categorization and tracking.
  ///
  /// # Arguments
  /// * `tags` - Vector of tag strings
  ///
  /// # Returns
  /// Self with the tags added for method chaining
  ///
  /// # Examples
  ///
  /// ```
  /// use send_with_us::types::{EmailOptions, Recipient};
  ///
  /// let recipient = Recipient::new("user@example.com");
  ///
  /// let options = EmailOptions::new("template-123", recipient)
  ///   .with_tags(vec!["welcome".to_string(), "new-user".to_string()]);
  /// ```
  pub fn with_tags(mut self, tags: Vec<String>) -> Self {
    self.tags = Some(tags);
    self
  }

  /// Sets the locale for internationalization.
  ///
  /// This can be used to select language-specific template versions.
  ///
  /// # Arguments
  /// * `locale` - The locale code (e.g., "en-US", "fr-CA")
  ///
  /// # Returns
  /// Self with the locale set for method chaining
  ///
  /// # Examples
  ///
  /// ```
  /// use send_with_us::types::{EmailOptions, Recipient};
  ///
  /// let recipient = Recipient::new("user@example.com");
  ///
  /// let options = EmailOptions::new("template-123", recipient)
  ///   .with_locale("fr-CA");
  /// ```
  pub fn with_locale(mut self, locale: impl Into<String>) -> Self {
    self.locale = Some(locale.into());
    self
  }
}

/// Options for creating or updating email templates in SendWithUs.
///
/// This struct is used when creating new templates or updating existing ones.
/// It contains all the required template content in both HTML and text formats,
/// along with metadata like the template name and subject line.
///
/// # Examples
///
/// ```
/// use send_with_us::types::TemplateOptions;
///
/// let template = TemplateOptions {
///   name: "Welcome Email".to_string(),
///   subject: "Welcome to Our Service".to_string(),
///   html: "<html><body>Welcome, {{name}}!</body></html>".to_string(),
///   text: "Welcome, {{name}}!".to_string(),
///   preheader: Some("Welcome to our service".to_string()),
///   amp_html: None,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplateOptions {
  /// Template name (displayed in the SendWithUs dashboard)
  pub name: String,

  /// Email subject line (can include template variables)
  pub subject: String,

  /// HTML content of the email (can include template variables)
  pub html: String,

  /// Plain text content of the email (can include template variables)
  pub text: String,

  /// Preheader text (preview text shown in email clients)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub preheader: Option<String>,

  /// AMP HTML content for supported email clients
  #[serde(skip_serializing_if = "Option::is_none")]
  pub amp_html: Option<String>,
}

/// Options for adding recipients to a drip campaign.
///
/// Drip campaigns are sequences of automated emails sent over time.
/// This struct is used when adding a recipient to a drip campaign,
/// with optional dynamic data, tags, and locale settings.
///
/// # Examples
///
/// ```
/// use send_with_us::types::DripCampaignOptions;
/// use std::collections::HashMap;
/// use serde_json::json;
///
/// let options = DripCampaignOptions {
///   recipient_address: "customer@example.com".to_string(),
///   email_data: None,
///   tags: None,
///   locale: None,
/// };
///
/// let mut email_data = HashMap::new();
/// email_data.insert("name".to_string(), json!("John"));
///
/// let options = DripCampaignOptions {
///   recipient_address: "customer@example.com".to_string(),
///   email_data: Some(email_data),
///   tags: Some(vec!["new-user".to_string()]),
///   locale: Some("en-US".to_string()),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DripCampaignOptions {
  /// Email address of the recipient to add to the campaign
  pub recipient_address: String,

  /// Dynamic data for email template variables
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email_data: Option<HashMap<String, serde_json::Value>>,

  /// Tags for categorization and tracking
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tags: Option<Vec<String>>,

  /// Locale for internationalization
  #[serde(skip_serializing_if = "Option::is_none")]
  pub locale: Option<String>,
}

/// Options for creating or managing customers in SendWithUs.
///
/// The CustomerOptions struct is used when creating or updating customer
/// records in SendWithUs, which can be used to track email engagement
/// and store custom data about your customers.
///
/// # Examples
///
/// ```
/// use send_with_us::types::CustomerOptions;
/// use std::collections::HashMap;
/// use serde_json::json;
///
/// let customer = CustomerOptions {
///   email: "customer@example.com".to_string(),
///   data: None,
///   locale: None,
/// };
///
/// let mut data = HashMap::new();
/// data.insert("first_name".to_string(), json!("John"));
/// data.insert("last_name".to_string(), json!("Doe"));
/// data.insert("plan".to_string(), json!("premium"));
///
/// let customer = CustomerOptions {
///   email: "customer@example.com".to_string(),
///   data: Some(data),
///   locale: Some("en-US".to_string()),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomerOptions {
  /// Customer's email address (primary identifier)
  pub email: String,

  /// Optional custom data associated with the customer
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<HashMap<String, serde_json::Value>>,

  /// Locale for internationalization
  #[serde(skip_serializing_if = "Option::is_none")]
  pub locale: Option<String>,
}

/// Options for rendering an email template without sending it.
///
/// This struct is used with the render API endpoint to preview
/// how a template will look with specific data. This is useful
/// for testing templates and generating email content for use
/// in other systems.
///
/// # Examples
///
/// ```
/// use send_with_us::types::RenderOptions;
/// use std::collections::HashMap;
/// use serde_json::json;
///
/// let mut template_data = HashMap::new();
/// template_data.insert("name".to_string(), json!("John"));
/// template_data.insert("order_id".to_string(), json!("12345"));
///
/// let options = RenderOptions {
///   template: "template-id".to_string(),
///   version_id: Some("version-id".to_string()),
///   template_data,
///   strict: true,
///   locale: Some("en-US".to_string()),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RenderOptions {
  /// ID of the template to render
  pub template: String,

  /// Optional specific version of the template to render
  #[serde(skip_serializing_if = "Option::is_none")]
  pub version_id: Option<String>,

  /// Data to use when rendering template variables
  pub template_data: HashMap<String, serde_json::Value>,

  /// Whether to use strict mode for variable replacement
  /// (error on missing variables vs. leaving them as is)
  pub strict: bool,

  /// Locale for template internationalization
  #[serde(skip_serializing_if = "Option::is_none")]
  pub locale: Option<String>,
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::json;

  #[test]
  fn test_recipient() {
    let recipient = Recipient::new("test@example.com");
    assert_eq!(recipient.address, "test@example.com");
    assert_eq!(recipient.name, None);

    let recipient = Recipient::new("test@example.com").with_name("Test User");
    assert_eq!(recipient.address, "test@example.com");
    assert_eq!(recipient.name, Some("Test User".to_string()));
  }

  #[test]
  fn test_sender() {
    let sender = Sender::new("sender@example.com");
    assert_eq!(sender.address, "sender@example.com");
    assert_eq!(sender.name, None);
    assert_eq!(sender.reply_to, None);

    let sender = Sender::new("sender@example.com").with_name("Sender Name");
    assert_eq!(sender.address, "sender@example.com");
    assert_eq!(sender.name, Some("Sender Name".to_string()));
    assert_eq!(sender.reply_to, None);

    let sender = Sender::new("sender@example.com").with_reply_to("reply@example.com");
    assert_eq!(sender.address, "sender@example.com");
    assert_eq!(sender.name, None);
    assert_eq!(sender.reply_to, Some("reply@example.com".to_string()));

    let sender = Sender::new("sender@example.com")
      .with_name("Sender Name")
      .with_reply_to("reply@example.com");

    assert_eq!(sender.address, "sender@example.com");
    assert_eq!(sender.name, Some("Sender Name".to_string()));
    assert_eq!(sender.reply_to, Some("reply@example.com".to_string()));
  }

  #[test]
  fn test_email_options() {
    let recipient = Recipient::new("recipient@example.com");
    let options = EmailOptions::new("template-123", recipient.clone());

    assert_eq!(options.email_id, "template-123");
    assert_eq!(options.recipient.address, "recipient@example.com");
    assert_eq!(options.data, None);
    assert_eq!(options.sender, None);
    assert_eq!(options.cc, None);
    assert_eq!(options.bcc, None);
    assert_eq!(options.files, None);
    assert_eq!(options.esp_account, None);
    assert_eq!(options.version_name, None);
    assert_eq!(options.headers, None);
    assert_eq!(options.tags, None);
    assert_eq!(options.locale, None);

    let mut email_data = HashMap::new();
    email_data.insert("name".to_string(), json!("John Doe"));
    email_data.insert("order_id".to_string(), json!(12345));

    let options = EmailOptions::new("template-123", recipient.clone()).with_data(email_data);

    assert_eq!(options.data.as_ref().unwrap()["name"], "John Doe");
    assert_eq!(options.data.as_ref().unwrap()["order_id"], 12345);

    let sender = Sender::new("sender@example.com").with_name("Sender Name");
    let options = EmailOptions::new("template-123", recipient.clone()).with_sender(sender);

    assert_eq!(
      options.sender.as_ref().unwrap().address,
      "sender@example.com"
    );
    assert_eq!(
      options.sender.as_ref().unwrap().name,
      Some("Sender Name".to_string())
    );

    let cc1 = Recipient::new("cc1@example.com");
    let cc2 = Recipient::new("cc2@example.com").with_name("CC Recipient");
    let bcc = Recipient::new("bcc@example.com");

    let options = EmailOptions::new("template-123", recipient.clone())
      .with_cc(vec![cc1, cc2])
      .with_bcc(vec![bcc]);

    assert_eq!(options.cc.as_ref().unwrap().len(), 2);
    assert_eq!(options.cc.as_ref().unwrap()[0].address, "cc1@example.com");
    assert_eq!(options.cc.as_ref().unwrap()[1].address, "cc2@example.com");
    assert_eq!(
      options.cc.as_ref().unwrap()[1].name,
      Some("CC Recipient".to_string())
    );

    assert_eq!(options.bcc.as_ref().unwrap().len(), 1);
    assert_eq!(options.bcc.as_ref().unwrap()[0].address, "bcc@example.com");

    let custom_headers = HashMap::from([("Header-X".into(), "Some value".into())]);

    let options = EmailOptions::new("template-123", recipient)
      .with_esp_account("esp-123")
      .with_files(vec![Attachment::from_bytes(
        b"File contents",
        "cooldoc.pdf",
      )])
      .with_headers(custom_headers)
      .with_version_name("version-name")
      .with_locale("en-US")
      .with_tags(vec!["tag1".to_string(), "tag2".to_string()]);

    assert_eq!(options.esp_account, Some("esp-123".to_string()));
    assert_eq!(options.version_name, Some("version-name".to_string()));
    assert_eq!(options.locale, Some("en-US".to_string()));
    assert_eq!(options.tags.as_ref().unwrap()[0], "tag1");
    assert_eq!(options.tags.as_ref().unwrap()[1], "tag2");
  }

  #[test]
  fn test_drip_campaign_options() {
    let options = DripCampaignOptions {
      recipient_address: "recipient@example.com".to_string(),
      email_data: None,
      tags: None,
      locale: None,
    };

    assert_eq!(options.recipient_address, "recipient@example.com");
    assert_eq!(options.email_data, None);
    assert_eq!(options.tags, None);
    assert_eq!(options.locale, None);

    let mut email_data = HashMap::new();
    email_data.insert("foo".to_string(), json!("bar"));

    let options = DripCampaignOptions {
      recipient_address: "recipient@example.com".to_string(),
      email_data: Some(email_data),
      tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
      locale: Some("fr-CA".to_string()),
    };

    assert_eq!(options.recipient_address, "recipient@example.com");
    assert_eq!(options.email_data.as_ref().unwrap()["foo"], "bar");
    assert_eq!(options.tags.as_ref().unwrap()[0], "tag1");
    assert_eq!(options.tags.as_ref().unwrap()[1], "tag2");
    assert_eq!(options.locale, Some("fr-CA".to_string()));
  }

  #[test]
  fn test_template_options() {
    let options = TemplateOptions {
      name: "Template Name".to_string(),
      subject: "Email Subject".to_string(),
      html: "<html>Content</html>".to_string(),
      text: "Plain text content".to_string(),
      preheader: Some("Preheader text".to_string()),
      amp_html: None,
    };

    assert_eq!(options.name, "Template Name");
    assert_eq!(options.subject, "Email Subject");
    assert_eq!(options.html, "<html>Content</html>");
    assert_eq!(options.text, "Plain text content");
    assert_eq!(options.preheader, Some("Preheader text".to_string()));
    assert_eq!(options.amp_html, None);
  }

  #[test]
  fn test_render_options() {
    let mut template_data = HashMap::new();
    template_data.insert("name".to_string(), json!("John"));
    template_data.insert("items".to_string(), json!(["item1", "item2"]));

    let options = RenderOptions {
      template: "template-id".to_string(),
      version_id: Some("version-id".to_string()),
      template_data,
      strict: true,
      locale: Some("en-US".to_string()),
    };

    assert_eq!(options.template, "template-id");
    assert_eq!(options.version_id, Some("version-id".to_string()));
    assert_eq!(options.template_data["name"], "John");
    assert_eq!(options.template_data["items"], json!(["item1", "item2"]));
    assert!(options.strict);
    assert_eq!(options.locale, Some("en-US".to_string()));
  }

  #[test]
  fn test_customer_options() {
    let options = CustomerOptions {
      email: "customer@example.com".to_string(),
      data: None,
      locale: None,
    };

    assert_eq!(options.email, "customer@example.com");
    assert_eq!(options.data, None);
    assert_eq!(options.locale, None);

    let mut data = HashMap::new();
    data.insert("first_name".to_string(), json!("John"));
    data.insert("last_name".to_string(), json!("Doe"));
    data.insert("age".to_string(), json!(30));

    let options = CustomerOptions {
      email: "customer@example.com".to_string(),
      data: Some(data),
      locale: Some("en-US".to_string()),
    };

    assert_eq!(options.email, "customer@example.com");
    assert_eq!(options.data.as_ref().unwrap()["first_name"], "John");
    assert_eq!(options.data.as_ref().unwrap()["last_name"], "Doe");
    assert_eq!(options.data.as_ref().unwrap()["age"], 30);
    assert_eq!(options.locale, Some("en-US".to_string()));
  }
}
