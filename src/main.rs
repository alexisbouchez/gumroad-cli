mod cli;
mod client;
mod commands;
mod config;
mod error;
mod models;
mod output;

use clap::Parser;

use cli::{Cli, Command};
use client::GumroadClient;
use config::resolve_token;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Command::Auth(cmd) => commands::auth::execute(cmd),
        _ => {
            let token = match resolve_token(cli.access_token.as_deref()) {
                Ok(t) => t,
                Err(e) => e.exit(),
            };
            let client = GumroadClient::new(token);
            run_command(&cli, &client).await
        }
    };

    if let Err(e) = result {
        e.exit();
    }
}

async fn run_command(cli: &Cli, client: &GumroadClient) -> error::Result<()> {
    let format = cli.output;
    match &cli.command {
        Command::User => commands::user::execute(client, format).await,
        Command::Products(cmd) => commands::products::execute(client, cmd, format).await,
        Command::CustomFields(cmd) => commands::custom_fields::execute(client, cmd, format).await,
        Command::OfferCodes(cmd) => commands::offer_codes::execute(client, cmd, format).await,
        Command::VariantCategories(cmd) => {
            commands::variant_categories::execute(client, cmd, format).await
        }
        Command::Variants(cmd) => commands::variants::execute(client, cmd, format).await,
        Command::Skus(cmd) => commands::skus::execute(client, cmd, format).await,
        Command::Subscribers(cmd) => commands::subscribers::execute(client, cmd, format).await,
        Command::Sales(cmd) => commands::sales::execute(client, cmd, format).await,
        Command::Payouts(cmd) => commands::payouts::execute(client, cmd, format).await,
        Command::Licenses(cmd) => commands::licenses::execute(client, cmd, format).await,
        Command::ResourceSubscriptions(cmd) => {
            commands::resource_subscriptions::execute(client, cmd, format).await
        }
        Command::Auth(_) => unreachable!(),
    }
}
