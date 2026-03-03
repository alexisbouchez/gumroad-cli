use crate::cli::{OutputFormat, ResourceSubscriptionsCommand};
use crate::client::GumroadClient;
use crate::error::Result;
use crate::models::resource_subscription::ResourceSubscriptionRow;
use crate::output::{print_output, print_single};

pub async fn execute(
    client: &GumroadClient,
    cmd: &ResourceSubscriptionsCommand,
    format: OutputFormat,
) -> Result<()> {
    match cmd {
        ResourceSubscriptionsCommand::List => {
            let subs = client.list_resource_subscriptions().await?;
            let rows: Vec<ResourceSubscriptionRow> =
                subs.into_iter().map(ResourceSubscriptionRow::from).collect();
            print_output(&rows, format);
        }
        ResourceSubscriptionsCommand::Create {
            resource_name,
            post_url,
        } => {
            let params = [
                ("resource_name", resource_name.as_str()),
                ("post_url", post_url.as_str()),
            ];
            let sub = client.create_resource_subscription(&params).await?;
            let row = ResourceSubscriptionRow::from(sub);
            print_single(&row, format);
        }
        ResourceSubscriptionsCommand::Delete { id } => {
            client.delete_resource_subscription(id).await?;
            println!("Resource subscription {id} deleted.");
        }
    }
    Ok(())
}
