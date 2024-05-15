use axum::{response::IntoResponse, Json};
use bdk::{
    blockchain::ElectrumBlockchain, database::SqliteDatabase, electrum_client::Client,
    wallet::AddressIndex, SyncOptions, Wallet,
};
use serde::{Deserialize, Serialize};

use crate::{app_error::AppError, setup::setup};

#[derive(Serialize, Deserialize)]
struct AddressResponse {
    address: String,
    index: u32,
}

pub async fn handler() -> Result<impl IntoResponse, AppError> {
    let descriptor = setup();
    let wallet = Wallet::new(
        &descriptor,
        None,
        bdk::bitcoin::Network::Testnet,
        SqliteDatabase::new("test.db"),
    )?;

    let client = Client::new("ssl://electrum.blockstream.info:60002")?;
    let blockchain = ElectrumBlockchain::from(client);

    wallet.sync(&blockchain, SyncOptions::default())?;

    println!("Descriptor balance: {} SAT", wallet.get_balance()?);
    let address = wallet.get_address(AddressIndex::New)?;
    dbg!(address);

    let response = AddressResponse {
        address: "test".to_string(),
        index: 0,
    };
    Ok(Json(response))
}
