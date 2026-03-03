use tabled::settings::Style;
use tabled::{Table, Tabled};

use crate::cli::OutputFormat;

pub fn print_output<T: Tabled + serde::Serialize>(items: &[T], format: OutputFormat) {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(items).unwrap());
        }
        OutputFormat::Table => {
            if items.is_empty() {
                println!("No results.");
                return;
            }
            let table = Table::new(items).with(Style::rounded()).to_string();
            println!("{table}");
        }
    }
}

pub fn print_single<T: serde::Serialize + Tabled>(item: &T, format: OutputFormat) {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(item).unwrap());
        }
        OutputFormat::Table => {
            let table = Table::new(std::iter::once(item))
                .with(Style::rounded())
                .to_string();
            println!("{table}");
        }
    }
}
