// EVM setup, all I want to do is get an account balance from my wallet 
// https://github.com/tmsdev82/rust-web3-basics-tutorial/blob/main/Cargo.toml
use std::env;
use std::str::FromStr;

use web3::types::{ H160, U256 };
#[tokio::main]
async fn main() -> web3::Result<()> {
    println!("Hello, world!");
    dotenv::dotenv().ok();
    println!(" test {:?}", &env::var("GOERLI_INFURA").unwrap() );
    // connecting to an RPC node to query the blockchain
    let websockets = web3::transports::WebSocket::new(&env::var("GOERLI_INFURA").unwrap()).await?;
    let web3 = web3::Web3::new(websockets);

    // tell the chain I'm looking for accounts in this vector
    let mut accounts = web3.eth().accounts().await?;
    accounts.push(H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
    println!("accounts in: {:?}", accounts);

    let wei_conversion: U256 = U256::exp10(18);
    for account in accounts {
        let balance = web3.eth().balance(account , None).await?;
        println!("Eth Balance for {:?}: is {:?}", account, balance.checked_div(wei_conversion).unwrap() );
    }

    Ok(())
}
