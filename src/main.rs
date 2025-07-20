mod wallet;
mod api;
mod types;

use wallet::generate_wallet;
use api::get_balance;

fn main() {
    let wallet = generate_wallet();
    let address = wallet.get_address();

    println!("Wallet Address: {}", address);

    if let Err(err) = get_balance(&address) {
        eprintln!("Error: {:?}", err);
    }
}
