use anyhow::Result;
use btclib::types::Transaction;
use clap::{Parser, Subcommand};
use core::{Config, Core, FeeConfig, FeeType, Recipient};
use cursive::vec;
use kanal::bounded;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::Arc;
use tasks::{handle_transactions, ui_task, update_balance, update_utxos};
use tokio::time::{self, Duration};
use tracing::*;
use util::{big_mode_btc, generate_dummy_config, setup_panic_hook, setup_tracing};

mod core;
mod tasks;
mod ui;
mod util;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(short, long, value_name = "FILE", default_value_os_t = PathBuf::from("wallet_config.toml"))]
    config: PathBuf,
    #[arg(short, long, value_name = "ADDRESS")]
    node: Option<String>,
}
#[derive(Subcommand)]
enum Commands {
    GenerateConfig {
        #[arg(short, long, value_name = "FILE", default_value_os_t = PathBuf::from("wallet_config.toml"))]
        output: PathBuf,
    },
}
// async fn update_utxos(core: Arc<Core>) {
//     // ..
// }
// async fn handle_transactions(rx: kanal::AsyncReceiver<Transaction>, core: Arc<Core>) {
//     // ..
// }
// async fn run_cli(core: Arc<Core>) -> Result<()> {
//     loop {
//         print!(">");
//         io::stdout().flush()?;
//         let mut input = String::new();
//         io::stdin().read_line(&mut input)?;
//         let parts: Vec<&str> = input.trim().split_whitespace().collect();
//         if parts.is_empty() {
//             continue;
//         }
//         match parts[0] {
//             "balance" => {
//                 println!("Current balance: {} satoshis", core.get_balance());
//             }
//             "send" => {
//                 if parts.len() != 3 {
//                     println!("Usage: send <recipient> <amount>");
//                     continue;
//                 }
//                 let recipient = parts[1];
//                 let amount: u64 = parts[2].parse()?;
//                 let recipient_key = core
//                     .config
//                     .contacts
//                     .iter()
//                     .find(|x| x.name == recipient)
//                     .ok_or_else(|| anyhow::anyhow!("Recipient not found"))?
//                     .load()?
//                     .key;
//                 if let Err(e) = core.fetch.utxos().await {
//                     println!("failed to fetch utxos: {e}");
//                 };
//                 let transaction = core.create_transaction(&recipient_key, amount).await?;
//                 core.tx_sender.send(transaction).await?;
//                 println!("Transaction sent successfully");
//                 core.fetch_utxos().await?;
//             }
//         }
//     }
//     Ok(())
// }
// fn generate_dummy_config(path: &PathBuf) -> Result<()> {
//     // ..
//     Ok(())
// }
#[tokio::main]
async fn main() -> Result<()> {
    setup.tracing()?;
    setup_panic_hook();
    info!("Starting wallet application");
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::GenerateConfig { output }) => {
            debug!("Generating dummy config at: {:?}", output);
            return generate_dummy_config(output);
        }
        None => {}
    }
    info!("Loading config from: {:?}", cli.config);
    // let config_path = cli
    //     .config
    //     .unwrap_or_else(|| PathBuf::from("wallet_config.toml"));
    // let mut core = Core::load(config_path.clone())?;
    let mut core = Core::load(cli.config.clone()).await?;
    if let Some(node) = cli.node {
        info!("Overriding default node with: {}", node);
        core.config.default_node = node;
    }
    let (tx_sender, tx_receiver) = kanal::bounded(10);
    // core.tx_sender = tx_sender.clone_async();
    core.tx.sender = tx.sender;
    let core = Arc::new(core);
    info!("Starting background taskes");
    let balance_content = TextContent::new(big_mode_btc(&core));
    tokio::select! {
       _ = ui_task(core.clone(), balance_content.clone()).await => (),
       _ = update_utxos(core.clone()).await => (),
       _ = handle_transactions(tx_receiver.clone_async(), core.clone()).await => (),
       _ = update_balance(core.clone(), balance_content).await => (),
    }
    // tokio::spawn(update_utxos(core.clone()));
    // tokio::spawn(handle_transactions(tx_receiver.clone_async(), core.clone()));
    // run_cli(core).await?;
    info!("Application shutting down");
    Ok(())
}
fn generate_dummy_config(path: &PathBuf) -> Result<()> {
    let dummy_config = Config {
        my_keys: vec![],
        contacts: vec![
            Recipient {
                name: "Alice".to_string(),
                key: PathBuf::from("alice.pub.pem"),
            },
            Recipient {
                name: "Bob".to_string(),
                key: PathBuf::from("bob.pub.pem"),
            },
        ],
        default_node: "127.0.0.1:9000".to_string(),
        fee_config: FeeConfig {
            fee_type: FeeType::Percent,
            value: 0.1,
        },
    };
    let config_str = toml::to_string_pretty(&dummy_config)?;
    std::fs::write(path, config_str)?;
    println!("Dummy config generated at: {}", path.display());
    Ok(())
}

// async fn update_utxos(core: Arc<Core>) {
//     let mut interval = time::interval(Duration::from_sec(20));
//     loop {
//         interval.tick().await;
//         if let Err(e) = core.fetch_utxos().await {
//             eprintln!("Failed to update UTXOs: {}", e);
//         }
//     }
// }
// async fn handle_transactions(rx: kanal::AsyncReceiver<Transaction>, core: Arc<Core>) {
//     while let Ok(transaction) = rx.recv().await {
//         if let Err(e) = core.send_transaction(transaction).await {
//             eprintln!("Failed to send transaction: {}", e);
//         }
//     }
// }
