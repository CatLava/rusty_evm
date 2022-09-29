// EVM setup, all I want to do is get an account balance from my wallet 
// https://github.com/tmsdev82/rust-web3-basics-tutorial/blob/main/Cargo.toml
use std::env;
use std::str::FromStr;

use web3::types::{ Address, H160, U256 };
use web3::contract::{Contract, Options};
#[tokio::main]
async fn main() -> web3::Result<()> {
    println!("Hello, world!");
    dotenv::dotenv().ok();
    // connecting to an RPC node to query the blockchain
    let http_transport = web3::transports::Http::new(&env::var("HTTP_INFURA").unwrap())?;

    let websockets = web3::transports::WebSocket::new(&env::var("GOERLI_INFURA").unwrap()).await?;
    let web3 = web3::Web3::new(websockets);
    println!("http: {:?}", web3);

    // tell the chain I'm looking for accounts in this vector
    let mut accounts = web3.eth().accounts().await?;
    accounts.push(H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
    println!("accounts in: {:?}", accounts);

    let wei_conversion: U256 = U256::exp10(18);
    for account in accounts {
        let balance = web3.eth().balance(account , None).await?;
        println!("Eth Balance for {:?}: is {:?}", account, balance.checked_div(wei_conversion).unwrap() );
    }

    // next will be query of an ERC contract
    // In this case it is for Weth on Goerli
    let weth_addr = Address::from_str("0xB4FBF271143F4FBf7B91A5ded31805e42b2208d6").unwrap();
    let token_contract = Contract::from_json(web3.eth(), weth_addr, include_bytes!("weth_abi.json")).unwrap();
    // Note this requires a type definition in order to unwrap;
    let test: String = token_contract
        .query("name", (), None, Options::default(), None).await.unwrap();
      

    let total_supply: U256 = token_contract
        .query("totalSupply", (), None, Options::default(), None)
        .await
        .unwrap();
    println!("token Name: {}; total supply: {}", test, total_supply);

    Ok(())
}
