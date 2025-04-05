use send_with_us::types::{EmailOptions, Recipient};
use send_with_us::{Api, ApiClient};
use serde_json::json;
use std::collections::HashMap;
use std::env;

use dotenv::dotenv;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();

  let filter = EnvFilter::new("debug");
  let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();
  tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

  let api_key =
    env::var("SENDWITHUS_API_KEY").expect("SENDWITHUS_API_KEY must be set in .env file");
  let template_id =
    env::var("SENDWITHUS_TEMPLATE_ID").expect("SENDWITHUS_TEMPLATE_ID must be set in .env file");

  let api = Api::with_api_key(&api_key);
  let recipient = Recipient::new("user@example.com").with_name("Example User");

  let mut email_data = HashMap::new();
  email_data.insert("recipient_name".to_string(), json!("Example User"));

  // Example of creating an attachment (commented out to avoid requiring a real file)
  // use send_with_us::Attachment;
  // let attachment = Attachment::from_path("path/to/file.pdf").await?;

  let options = EmailOptions::new(&template_id, recipient).with_data(email_data);
  // Uncomment to use with attachment
  // .with_files(vec![attachment]);

  match api.send_email(options).await {
    Ok(response) => tracing::info!("Email sent successfully: {:?}", response),
    Err(e) => tracing::error!("Failed to send email: {}", e),
  }

  Ok(())
}
