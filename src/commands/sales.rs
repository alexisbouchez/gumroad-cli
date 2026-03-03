use crate::cli::{OutputFormat, SalesCommand};
use crate::client::GumroadClient;
use crate::error::Result;
use crate::models::sale::SaleRow;
use crate::output::{print_output, print_single};

pub async fn execute(
    client: &GumroadClient,
    cmd: &SalesCommand,
    format: OutputFormat,
) -> Result<()> {
    match cmd {
        SalesCommand::List {
            after,
            before,
            product_id,
            email,
            page_key,
        } => {
            let mut params: Vec<(&str, &str)> = vec![];
            if let Some(a) = after {
                params.push(("after", a.as_str()));
            }
            if let Some(b) = before {
                params.push(("before", b.as_str()));
            }
            if let Some(p) = product_id {
                params.push(("product_id", p.as_str()));
            }
            if let Some(e) = email {
                params.push(("email", e.as_str()));
            }
            if let Some(pk) = page_key {
                params.push(("page_key", pk.as_str()));
            }
            let page = client.list_sales(&params).await?;
            let rows: Vec<SaleRow> = page.sales.into_iter().map(SaleRow::from).collect();
            print_output(&rows, format);
            if let Some(key) = page.next_page_key {
                eprintln!("Next page key: {key}");
            }
        }
        SalesCommand::Get { id } => {
            let sale = client.get_sale(id).await?;
            let row = SaleRow::from(sale);
            print_single(&row, format);
        }
        SalesCommand::MarkAsShipped { id, tracking_url } => {
            let sale = client
                .mark_sale_as_shipped(id, tracking_url.as_deref())
                .await?;
            let row = SaleRow::from(sale);
            print_single(&row, format);
        }
        SalesCommand::Refund { id, amount_cents } => {
            let amount_str = amount_cents.map(|a| a.to_string());
            let sale = client
                .refund_sale(id, amount_str.as_deref())
                .await?;
            let row = SaleRow::from(sale);
            print_single(&row, format);
        }
        SalesCommand::ResendReceipt { id } => {
            client.resend_receipt(id).await?;
            println!("Receipt resent for sale {id}.");
        }
    }
    Ok(())
}
