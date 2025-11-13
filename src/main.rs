use pushers::{Channel, Config, Pusher, PusherError}; // Adjusted to use crate name
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), PusherError> {
    let mut config_builder = Config::builder() // Use the builder pattern
        .app_id("app-id")
        .key("app-key")
        .secret("app-secret")
        .host("127.0.0.1") // e.g. "eu", "ap1"
        .port(6001)
        .use_tls(false)
        .timeout(std::time::Duration::from_secs(5)); // Optional

    // For encrypted channels:
    // config_builder = config_builder
    // .encryption_master_key_base64("YOUR_BASE64_ENCRYPTION_MASTER_KEY")?;

    let config = config_builder.build()?; // Build the config

    let pusher = Pusher::new(config)?; // Pass the built config
                                       //
    let channels = vec![Channel::from_string("my-channel")?]; // Use Channel type
    let event_name = "new-message";
    let data = json!({ "text": "Hello from Rust!" });

    match pusher.trigger(&channels, event_name, data, None).await { // Pass data directly
        Ok(response) => {
            println!("Event triggered! Status: {}", response.status());
            // You might want to consume the response body, e.g., response.text().await?
        }
        Err(e) => eprintln!("Error triggering event: {:?}", e),
    }

    // Your application logic here...
    Ok(())
}
