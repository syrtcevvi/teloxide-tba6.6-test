use std::error::Error;

use dotenvy::dotenv;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use teloxide_v6_5::{
    adaptors::{throttle::Limits, Throttle},
    prelude::*,
    types::InputFile,
};

type Bot = Throttle<teloxide_v6_5::Bot>;
type Result = std::result::Result<(), Box<dyn Error>>;

#[derive(Deserialize, Serialize)]
struct EnvConfig {
    pub user_id: UserId,
}

lazy_static! {
    static ref ENV_CONFIG: EnvConfig =
        envy::from_env::<EnvConfig>().expect("Unable to build ENV_CONFIG");
    static ref BOT: Bot = teloxide_v6_5::Bot::from_env().throttle(Limits::default());
}

#[tokio::main]
async fn main() -> Result {
    dotenv().ok();
    pretty_env_logger::init();

    log::info!("The program started");

    // I wanted just to check how the method `send_sticker` works in TBA 6.5
    BOT.send_sticker(
        ENV_CONFIG.user_id,
        InputFile::file("data/teloxide-logo.webp"),
    )
    .await?;

    /*
        Due to the breaking change in TBA 7.2 (which removes `is_audio` and `is_animated`)
        the method `get_sticker_set` fails, because it misses these fields while deserialization

        Uncomment the following to check, provide the sticker name, created by your bot
    */
    // let sticker_set_name = Some("<YOUR STICKER SET NAME>".to_owned());
    // if let Some(sticker_set_name) = sticker_set_name {
    //     let sticker_set = BOT.get_sticker_set(sticker_set_name).await?;
    // }

    Ok(())
}
