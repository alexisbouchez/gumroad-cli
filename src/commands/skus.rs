use crate::cli::{OutputFormat, SkusCommand};
use crate::client::GumroadClient;
use crate::error::Result;
use crate::models::sku::SkuRow;
use crate::output::print_output;

pub async fn execute(
    client: &GumroadClient,
    cmd: &SkusCommand,
    format: OutputFormat,
) -> Result<()> {
    match cmd {
        SkusCommand::List { product_id } => {
            let skus = client.list_skus(product_id).await?;
            let rows: Vec<SkuRow> = skus.into_iter().map(SkuRow::from).collect();
            print_output(&rows, format);
        }
    }
    Ok(())
}
