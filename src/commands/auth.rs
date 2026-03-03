use std::io::{self, Write};

use crate::cli::AuthCommand;
use crate::config::Config;
use crate::error::Result;

pub fn execute(cmd: &AuthCommand) -> Result<()> {
    match cmd {
        AuthCommand::Login => {
            print!("Enter your Gumroad access token: ");
            io::stdout().flush()?;
            let mut token = String::new();
            io::stdin().read_line(&mut token)?;
            let token = token.trim().to_string();
            if token.is_empty() {
                println!("No token provided. Aborting.");
                return Ok(());
            }
            let config = Config {
                access_token: Some(token),
            };
            config.save()?;
            println!("Token saved to {:?}", Config::path()?);
            Ok(())
        }
    }
}
