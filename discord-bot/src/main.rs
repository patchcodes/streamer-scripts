use clap::{Parser, Subcommand};
use dotenv::dotenv;
use std::{env, fs};
use crate::discord::Discord;

pub mod discord;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Post { path: String },
    Delete { msg_id: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(debug_assertions) {
        dotenv().ok();
    }

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let channel_id =
        env::var("DISCORD_CHANNEL_ID").expect("Expected a channel ID in the environment");

    let discord = Discord::new(token.as_str(), channel_id.as_str());
    let cli = Args::parse();

    match &cli.command {
        Commands::Post { path } => {
            let contents =
                fs::read_to_string(path).expect("Failed to read message content from file.");

            let message_id = discord.send(&contents).await?;
            println!("{message_id}");
        }

        Commands::Delete { msg_id } => {
            discord
                .delete(msg_id)
                .await
                .expect("Failed to delete message.");
            println!("Message deleted.");
        }
    }

    Ok(())
}
