use crate::cli::{OutputFormat, PayoutsCommand};
use crate::client::GumroadClient;
use crate::error::Result;
use crate::models::payout::PayoutRow;
use crate::output::{print_output, print_single};

pub async fn execute(
    client: &GumroadClient,
    cmd: &PayoutsCommand,
    format: OutputFormat,
) -> Result<()> {
    match cmd {
        PayoutsCommand::List { page } => {
            let page_str = page.map(|p| p.to_string());
            let mut params: Vec<(&str, &str)> = vec![];
            if let Some(p) = &page_str {
                params.push(("page", p.as_str()));
            }
            let payouts = client.list_payouts(&params).await?;
            let rows: Vec<PayoutRow> = payouts.into_iter().map(PayoutRow::from).collect();
            print_output(&rows, format);
        }
        PayoutsCommand::Get { id } => {
            let payout = client.get_payout(id).await?;
            let row = PayoutRow::from(payout);
            print_single(&row, format);
        }
        PayoutsCommand::Upcoming => {
            let payout = client.get_upcoming_payout().await?;
            let row = PayoutRow::from(payout);
            print_single(&row, format);
        }
    }
    Ok(())
}
