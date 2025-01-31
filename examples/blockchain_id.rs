use {
    fireblocks_sdk::{
        ClientBuilder,
        apis::Api,
    },
    std::{fs::File, io::Read, time::Duration},
};
use fireblocks_sdk::apis::blockchains_assets_beta_api::ListBlockchainsParams;
use fireblocks_sdk::apis::blockchains_assets_beta_api::GetBlockchainByIdParams;

fn load_secret() -> anyhow::Result<Vec<u8>> {
    std::env::var("FIREBLOCKS_SECRET").ok().map_or_else(
        || {
            let secret = std::env::var("FIREBLOCKS_SECRET_FILE")
                .expect("failed find secret key in FIREBLOCKS_SECRET or FIREBLOCKS_SECRET_FILE");
            let mut file = File::open(secret).expect("file not found");
            let mut secret: String = String::new();
            file.read_to_string(&mut secret)
                .expect("something went wrong reading the file");
            Ok(secret.into_bytes())
        },
        |secret| Ok(secret.into_bytes()),
    )
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let api_key = std::env::var("FIREBLOCKS_API_KEY")?;
    let secret = load_secret()?;
    let client = ClientBuilder::new(&api_key, &secret)
        .with_sandbox()
        .with_timeout(Duration::from_secs(10))
        .with_connect_timeout(Duration::from_secs(5))
        .build()?;

    //// TODO: seems to be returning 403 forbidden
    //let params = ListBlockchainsParams{
    //    protocol: Some("ETH".to_string()),
    //    deprecated: Some(false),
    //    test: Some(true),
    //    page_cursor: None,
    //    page_size: None,
    //};
    //let blockchains = client.apis().blockchains_assets_beta_api().list_blockchains(params).await?;
    //dbg!(blockchains);

    let param = GetBlockchainByIdParams{
        id: "ETH".to_string(),
    };

    let blockchain = client.apis().blockchains_assets_beta_api().get_blockchain_by_id(param).await?;
    dbg!(blockchain);

    Ok(())
}
