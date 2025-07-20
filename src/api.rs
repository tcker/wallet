use reqwest::blocking::Client;
use crate::types::AptosResource;

pub fn get_balance(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://fullnode.mainnet.aptoslabs.com/v1/accounts/{}/resources",
        address
    );

    let client = Client::new();
    let resources: Vec<AptosResource> = client.get(&url).send()?.json()?;

    for res in resources {
        if res.resource_type.contains("CoinStore") && res.resource_type.contains("AptosCoin") {
            if let Some(value) = res.data.get("coin").and_then(|c| c.get("value")) {
                println!("Balance: {}", value);
                break;
            }
        }
    }

    Ok(())
}
