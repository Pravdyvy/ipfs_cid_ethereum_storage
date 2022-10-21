use std::path::PathBuf;

use clap::Parser;

use ipfs_cid_ethereum_storage::{ipfs_store,prepare_code,deploy_ethereum_contract,ethereum_store};
//use web3::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///Path to artifacts and file
    #[arg(short = 'P', value_name = "PATH", value_hint = clap::ValueHint::DirPath)]
    path: PathBuf,
}

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let source_path = args.path.join("source.txt");
    let cid = ipfs_store(source_path.as_path()).await?;
    println!("CID is {:?}", cid);
    let (bytecode, json) = prepare_code(args.path.as_path())?;
    let (accounts, contract) = deploy_ethereum_contract(bytecode.as_str(), json.as_bytes()).await?;
    let cid_returned = ethereum_store(accounts, contract, cid).await?;
    println!("CID returned is {:?}", cid_returned);
    Ok(())
}
