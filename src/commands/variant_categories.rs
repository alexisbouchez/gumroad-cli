use crate::cli::{OutputFormat, VariantCategoriesCommand};
use crate::client::GumroadClient;
use crate::error::Result;
use crate::models::variant_category::VariantCategoryRow;
use crate::output::{print_output, print_single};

pub async fn execute(
    client: &GumroadClient,
    cmd: &VariantCategoriesCommand,
    format: OutputFormat,
) -> Result<()> {
    match cmd {
        VariantCategoriesCommand::List { product_id } => {
            let categories = client.list_variant_categories(product_id).await?;
            let rows: Vec<VariantCategoryRow> =
                categories.into_iter().map(VariantCategoryRow::from).collect();
            print_output(&rows, format);
        }
        VariantCategoriesCommand::Get { product_id, id } => {
            let category = client.get_variant_category(product_id, id).await?;
            let row = VariantCategoryRow::from(category);
            print_single(&row, format);
        }
        VariantCategoriesCommand::Create { product_id, title } => {
            let params = [("title", title.as_str())];
            let category = client
                .create_variant_category(product_id, &params)
                .await?;
            let row = VariantCategoryRow::from(category);
            print_single(&row, format);
        }
        VariantCategoriesCommand::Update {
            product_id,
            id,
            title,
        } => {
            let params = [("title", title.as_str())];
            let category = client
                .update_variant_category(product_id, id, &params)
                .await?;
            let row = VariantCategoryRow::from(category);
            print_single(&row, format);
        }
        VariantCategoriesCommand::Delete { product_id, id } => {
            client.delete_variant_category(product_id, id).await?;
            println!("Variant category {id} deleted.");
        }
    }
    Ok(())
}
