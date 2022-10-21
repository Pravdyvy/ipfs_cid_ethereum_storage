use actix_rt::time::sleep;
use ipfs_api_backend_actix::{Error, IpfsApi, IpfsClient};
use std::io::Read;
use std::path::Path;
use std::time::Duration;
use std::{fs::File};
use web3::contract::{Contract, Options};
use web3::transports::Http;
use web3::types::H160;

pub async fn ipfs_store(path: &Path) -> Result<String, Error> {
    let client = IpfsClient::default();
    let file = File::open(path).expect("given file can't be read");
    let resp = client.add(file).await?;
    Ok(resp.name)
}
pub fn prepare_code(path: &Path) -> Result<(String, String), std::io::Error> {
    let bytecode_path = path.join("artifacts/IPFS_CID_Store.bin");
    let json_path = path.join("artifacts/IPFS_CID_Store.abi");

    let mut file_bytecode = File::open(bytecode_path).expect("given bytecode file can't be read");
    let mut file_json = File::open(json_path).expect("given json file can't be read");

    let mut contents_bytecode = String::new();
    let mut contents_json = String::new();

    file_bytecode.read_to_string(&mut contents_bytecode)?;
    file_json.read_to_string(&mut contents_json)?;

    Ok((contents_bytecode, contents_json))
}
pub async fn deploy_ethereum_contract(
    bytecode: &str,
    json: &[u8],
) -> web3::contract::Result<(Vec<H160>, Contract<Http>)> {
    let transport = web3::transports::Http::new("http://127.0.0.1:8545")?;
    let web3 = web3::Web3::new(transport);
    let accounts = web3.eth().accounts().await?;

    let contract = Contract::deploy(web3.eth(), json)?
        .confirmations(0)
        .poll_interval(Duration::from_secs(10))
        .options(Options::with(|opt| opt.gas = Some(1_000_000.into())))
        .execute(bytecode, (), accounts[0])
        .await?;

    Ok((accounts, contract))
}
pub async fn ethereum_store(
    accounts: Vec<H160>,
    contract: Contract<Http>,
    cid: String,
) -> web3::contract::Result<String> {
    contract
        .call("set", (cid,), accounts[0], Options::default())
        .await?;

    sleep(Duration::from_secs(5)).await;

    let cid: String = contract
        .query("get", (), None, Options::default(), None)
        .await?;

    Ok(cid)
}
