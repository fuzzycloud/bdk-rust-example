use std::env;

pub fn setup() -> String {
    dotenv::from_filename(".env").ok();
    let descriptor = env::var("WALLET_DESCRIPTOR");
    match descriptor {
        Ok(descriptor) => descriptor,
        Err(_) => "errorrrrrrrrrr".to_string(),
    }
}
