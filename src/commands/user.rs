use crate::cli::OutputFormat;
use crate::client::GumroadClient;
use crate::error::Result;
use crate::models::user::UserRow;
use crate::output::print_single;

pub async fn execute(client: &GumroadClient, format: OutputFormat) -> Result<()> {
    let user = client.get_user().await?;
    let row = UserRow::from(user);
    print_single(&row, format);
    Ok(())
}
