use crate::cli::{OutputFormat, VariantsCommand};
use crate::client::GumroadClient;
use crate::error::Result;
use crate::models::variant::VariantRow;
use crate::output::{print_output, print_single};

pub async fn execute(
    client: &GumroadClient,
    cmd: &VariantsCommand,
    format: OutputFormat,
) -> Result<()> {
    match cmd {
        VariantsCommand::List {
            product_id,
            variant_category_id,
        } => {
            let variants = client
                .list_variants(product_id, variant_category_id)
                .await?;
            let rows: Vec<VariantRow> = variants.into_iter().map(VariantRow::from).collect();
            print_output(&rows, format);
        }
        VariantsCommand::Get {
            product_id,
            variant_category_id,
            id,
        } => {
            let variant = client
                .get_variant(product_id, variant_category_id, id)
                .await?;
            let row = VariantRow::from(variant);
            print_single(&row, format);
        }
        VariantsCommand::Create {
            product_id,
            variant_category_id,
            name,
            price_difference,
            max_purchase_count,
        } => {
            let mut params_owned: Vec<(&str, String)> = vec![("name", name.clone())];
            if let Some(p) = price_difference {
                params_owned.push(("price_difference", p.to_string()));
            }
            if let Some(m) = max_purchase_count {
                params_owned.push(("max_purchase_count", m.to_string()));
            }
            let params: Vec<(&str, &str)> =
                params_owned.iter().map(|(k, v)| (*k, v.as_str())).collect();
            let variant = client
                .create_variant(product_id, variant_category_id, &params)
                .await?;
            let row = VariantRow::from(variant);
            print_single(&row, format);
        }
        VariantsCommand::Update {
            product_id,
            variant_category_id,
            id,
            name,
            price_difference,
            max_purchase_count,
        } => {
            let mut params_owned: Vec<(&str, String)> = vec![];
            if let Some(n) = name {
                params_owned.push(("name", n.clone()));
            }
            if let Some(p) = price_difference {
                params_owned.push(("price_difference", p.to_string()));
            }
            if let Some(m) = max_purchase_count {
                params_owned.push(("max_purchase_count", m.to_string()));
            }
            let params: Vec<(&str, &str)> =
                params_owned.iter().map(|(k, v)| (*k, v.as_str())).collect();
            let variant = client
                .update_variant(product_id, variant_category_id, id, &params)
                .await?;
            let row = VariantRow::from(variant);
            print_single(&row, format);
        }
        VariantsCommand::Delete {
            product_id,
            variant_category_id,
            id,
        } => {
            client
                .delete_variant(product_id, variant_category_id, id)
                .await?;
            println!("Variant {id} deleted.");
        }
    }
    Ok(())
}
