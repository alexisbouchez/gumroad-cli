use crate::cli::{OfferCodesCommand, OutputFormat};
use crate::client::GumroadClient;
use crate::error::Result;
use crate::models::offer_code::OfferCodeRow;
use crate::output::{print_output, print_single};

pub async fn execute(
    client: &GumroadClient,
    cmd: &OfferCodesCommand,
    format: OutputFormat,
) -> Result<()> {
    match cmd {
        OfferCodesCommand::List { product_id } => {
            let codes = client.list_offer_codes(product_id).await?;
            let rows: Vec<OfferCodeRow> = codes.into_iter().map(OfferCodeRow::from).collect();
            print_output(&rows, format);
        }
        OfferCodesCommand::Get { product_id, id } => {
            let code = client.get_offer_code(product_id, id).await?;
            let row = OfferCodeRow::from(code);
            print_single(&row, format);
        }
        OfferCodesCommand::Create {
            product_id,
            name,
            amount_off,
            offer_type,
            max_purchase_count,
            universal,
        } => {
            let amount_str = amount_off.to_string();
            let max_str = max_purchase_count.to_string();
            let universal_str = universal.to_string();
            let params = [
                ("name", name.as_str()),
                ("amount_off", &amount_str),
                ("offer_type", offer_type.as_str()),
                ("max_purchase_count", &max_str),
                ("universal", &universal_str),
            ];
            let code = client.create_offer_code(product_id, &params).await?;
            let row = OfferCodeRow::from(code);
            print_single(&row, format);
        }
        OfferCodesCommand::Update {
            product_id,
            id,
            max_purchase_count,
        } => {
            let mut params_owned: Vec<(&str, String)> = vec![];
            if let Some(m) = max_purchase_count {
                params_owned.push(("max_purchase_count", m.to_string()));
            }
            let params: Vec<(&str, &str)> =
                params_owned.iter().map(|(k, v)| (*k, v.as_str())).collect();
            let code = client.update_offer_code(product_id, id, &params).await?;
            let row = OfferCodeRow::from(code);
            print_single(&row, format);
        }
        OfferCodesCommand::Delete { product_id, id } => {
            client.delete_offer_code(product_id, id).await?;
            println!("Offer code {id} deleted.");
        }
    }
    Ok(())
}
