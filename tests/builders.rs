use send_with_us::Config;
use send_with_us::types::{CustomerOptions, DripCampaignOptions, RenderOptions, TemplateOptions};
use serde_json::json;
use std::collections::HashMap;

#[test]
fn template_options_builds_from_outside_the_crate() {
  let options = TemplateOptions::new("Welcome", "Hi {{name}}", "<p>Hi</p>", "Hi")
    .with_preheader("Preview")
    .with_amp_html("<amp>Hi</amp>");

  assert_eq!(options.name, "Welcome");
  assert_eq!(options.subject, "Hi {{name}}");
  assert_eq!(options.html, "<p>Hi</p>");
  assert_eq!(options.text, "Hi");
  assert_eq!(options.preheader.as_deref(), Some("Preview"));
  assert_eq!(options.amp_html.as_deref(), Some("<amp>Hi</amp>"));
}

#[test]
fn drip_campaign_options_builds_from_outside_the_crate() {
  let bare = DripCampaignOptions::new("customer@example.com");
  assert_eq!(bare.recipient_address, "customer@example.com");
  assert!(bare.email_data.is_none() && bare.tags.is_none() && bare.locale.is_none());

  let data = HashMap::from([("name".to_string(), json!("John"))]);
  let options = DripCampaignOptions::new("customer@example.com")
    .with_email_data(data.clone())
    .with_tags(vec!["new-user".to_string()])
    .with_locale("en-US");

  assert_eq!(options.email_data, Some(data));
  assert_eq!(options.tags, Some(vec!["new-user".to_string()]));
  assert_eq!(options.locale.as_deref(), Some("en-US"));
}

#[test]
fn customer_options_builds_from_outside_the_crate() {
  let bare = CustomerOptions::new("customer@example.com");
  assert_eq!(bare.email, "customer@example.com");
  assert!(bare.data.is_none() && bare.locale.is_none());

  let data = HashMap::from([("plan".to_string(), json!("premium"))]);
  let options = CustomerOptions::new("customer@example.com")
    .with_data(data.clone())
    .with_locale("fr-CA");

  assert_eq!(options.data, Some(data));
  assert_eq!(options.locale.as_deref(), Some("fr-CA"));
}

#[test]
fn render_options_builds_from_outside_the_crate() {
  let data = HashMap::from([("order_id".to_string(), json!("12345"))]);

  let bare = RenderOptions::new("tem_123", data.clone());
  assert_eq!(bare.template, "tem_123");
  assert_eq!(bare.template_data, data);
  assert!(!bare.strict);
  assert!(bare.version_id.is_none() && bare.locale.is_none());

  let options = RenderOptions::new("tem_123", data)
    .with_version_id("ver_1")
    .with_strict(true)
    .with_locale("en-US");

  assert_eq!(options.version_id.as_deref(), Some("ver_1"));
  assert!(options.strict);
  assert_eq!(options.locale.as_deref(), Some("en-US"));
}

#[test]
fn config_client_stub_is_settable_from_outside_the_crate() {
  let config = Config::new("api-key").with_client_stub("my-app/1.0");

  assert_eq!(config.client_stub, "my-app/1.0");
}
