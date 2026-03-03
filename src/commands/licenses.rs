use crate::cli::{LicensesCommand, OutputFormat};
use crate::client::GumroadClient;
use crate::error::Result;
use crate::models::license::LicenseRow;
use crate::output::print_single;

pub async fn execute(
    client: &GumroadClient,
    cmd: &LicensesCommand,
    format: OutputFormat,
) -> Result<()> {
    match cmd {
        LicensesCommand::Verify {
            product_id,
            license_key,
            increment_uses_count,
        } => {
            let license = client
                .verify_license(product_id, license_key, *increment_uses_count)
                .await?;
            let row = LicenseRow::from(license);
            print_single(&row, format);
        }
        LicensesCommand::Enable {
            product_id,
            license_key,
        } => {
            let license = client.enable_license(product_id, license_key).await?;
            let row = LicenseRow::from(license);
            print_single(&row, format);
        }
        LicensesCommand::Disable {
            product_id,
            license_key,
        } => {
            let license = client.disable_license(product_id, license_key).await?;
            let row = LicenseRow::from(license);
            print_single(&row, format);
        }
        LicensesCommand::DecrementUses {
            product_id,
            license_key,
        } => {
            let license = client
                .decrement_license_uses(product_id, license_key)
                .await?;
            let row = LicenseRow::from(license);
            print_single(&row, format);
        }
        LicensesCommand::Rotate {
            product_id,
            license_key,
        } => {
            let license = client.rotate_license(product_id, license_key).await?;
            let row = LicenseRow::from(license);
            print_single(&row, format);
        }
    }
    Ok(())
}
