use sqlx::{sqlite, types::Json, SqlitePool};
use std::env;
use std::fmt::format;
use std::sync::Arc;
use teloxide::prelude::*;
use tokio::sync::Mutex;

#[derive(Clone)]
struct RaroescuchaConfig {
    pool: Arc<Mutex<SqlitePool>>,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();
    log::info!("Starting raroescucha bot...");
    let conn = Arc::new(Mutex::new(
        sqlite::SqlitePool::connect(&env::var("DATABASE_URL").expect("DATABASE_URL is not set"))
            .await
            .unwrap(),
    ));
    let dep_data = RaroescuchaConfig { pool: conn };

    sqlx::query("CREATE TABLE IF NOT EXISTS messages (id INTEGER, message jsonb)")
        .execute(&*dep_data.pool.lock().await)
        .await
        .unwrap();

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .inspect(|u: Update| {
            log::trace!("{u:#?}"); // Print the update to the console with inspect
        })
        .branch(
            Update::filter_message().branch(
                dptree::filter(|msg: Message| {
                    msg.chat.is_group() || msg.chat.is_supergroup() || msg.chat.is_private()
                    //                        || (msg.from().unwrap().id == UserId(51739298))
                })
                .endpoint(store_message),
            ),
        );

    // Dispatcher::builder(bot, handler).enable_ctrlc_handler().build().dispatch().await?;
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![dep_data])
        .build()
        .dispatch()
        .await;
}

async fn store_message(
    dep_data: RaroescuchaConfig,
    bot: Bot,
    msg: Message,
) -> Result<(), teloxide::RequestError> {
    let msg_id = msg.id.0;
    let serialized_message: Json<_> = msg.try_into().unwrap();

    sqlx::query("INSERT INTO messages (id, message) VALUES (?, ?)")
        .bind(msg_id)
        .bind(serialized_message)
        .execute(&*dep_data.pool.lock().await)
        .await
        .expect(format!("Error inserting message ({}) into database", msg_id).as_str());

    log::info!("Message stored in the database");

    Ok(())
}
