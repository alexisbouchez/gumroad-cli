use crate::cli::{OutputFormat, SubscribersCommand};
use crate::client::GumroadClient;
use crate::error::Result;
use crate::models::subscriber::SubscriberRow;
use crate::output::{print_output, print_single};

pub async fn execute(
    client: &GumroadClient,
    cmd: &SubscribersCommand,
    format: OutputFormat,
) -> Result<()> {
    match cmd {
        SubscribersCommand::List { product_id } => {
            let subscribers = client.list_subscribers(product_id).await?;
            let rows: Vec<SubscriberRow> =
                subscribers.into_iter().map(SubscriberRow::from).collect();
            print_output(&rows, format);
        }
        SubscribersCommand::Get { id } => {
            let subscriber = client.get_subscriber(id).await?;
            let row = SubscriberRow::from(subscriber);
            print_single(&row, format);
        }
    }
    Ok(())
}
