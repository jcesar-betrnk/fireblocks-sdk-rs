use {
    fireblocks_sdk::{
        ClientBuilder,
        models,
    },
    std::{fs::File, io::Read, time::Duration},
};
use fireblocks_sdk::apis::transactions_api::CreateTransactionParams;

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

    let params = CreateTransactionParams{
        transaction_request: models::TransactionRequest{
            //asset_id: Some("USDT_ERC20".to_string()),
            asset_id: Some("USDT_BSC_TEST".to_string()),
            operation: Some(models::TransactionOperation::Transfer),
            source: Some(models::SourceTransferPeerPath{
                r#type: models::TransferPeerPathType::VaultAccount,
                id: Some("0".to_string()),
                ..Default::default()
            }),
            destination: Some(models::DestinationTransferPeerPath{
                r#type: models::TransferPeerPathType::VaultAccount,
                id: Some("0".to_string()),
                ..Default::default()
            }),
            amount: Some(models::TransactionRequestAmount::String("100.0".to_string())),
            note: Some("Sample transaction from betrnk blockchain engr".to_string()),
            ..Default::default()
        },
        x_end_user_wallet_id: None,
        idempotency_key: None,
    };
    let response = client.transactions_api().create_transaction(params).await?;
    dbg!(response);

    Ok(())
}
