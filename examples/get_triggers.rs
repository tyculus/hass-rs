use hass_rs::client::HassClient;
use std::env::var;
use std::sync::OnceLock;

static HASS_URL: OnceLock<String> = OnceLock::new();
static TOKEN: OnceLock<String> = OnceLock::new();
static DEVICE_ID: OnceLock<String> = OnceLock::new();

#[tokio::main]
async fn main() {
    let url = HASS_URL.get_or_init(|| {
        var("HASS_URL").expect("please set up the HASS_URL=ws://home.example.com:8123 env variable before running this")
    });
    let token = TOKEN.get_or_init(|| {
        var("HASS_TOKEN").expect("please set up the HASS_TOKEN env variable before running this")
    });
    let device_id = DEVICE_ID.get_or_init(|| {
        var("DEVICE_ID").expect("please set up the DEVICE_ID env variable before running this. You can find these in the device registry.")
    });

    // let device_id = "0c1af90f906f04d97049d9ea3235d048";

    println!("Connecting to - {}", url);
    let mut client = HassClient::new(url).await.expect("Failed to connect");

    client
        .auth_with_longlivedtoken(token)
        .await
        .expect("Not able to authenticate");

    println!("WebSocket connection and authentication works\n");

    println!("Getting the triggers:\n");
    let cmd = client
        .get_trigger_list(device_id)
        .await
        .expect("Unable to retrieve the triggers");
    println!("triggers: {:#?}\n", cmd);
}
