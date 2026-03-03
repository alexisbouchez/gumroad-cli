use crate::cli::{CustomFieldsCommand, OutputFormat};
use crate::client::GumroadClient;
use crate::error::Result;
use crate::models::custom_field::CustomFieldRow;
use crate::output::{print_output, print_single};

pub async fn execute(
    client: &GumroadClient,
    cmd: &CustomFieldsCommand,
    format: OutputFormat,
) -> Result<()> {
    match cmd {
        CustomFieldsCommand::List { product_id } => {
            let fields = client.list_custom_fields(product_id).await?;
            let rows: Vec<CustomFieldRow> =
                fields.into_iter().map(CustomFieldRow::from).collect();
            print_output(&rows, format);
        }
        CustomFieldsCommand::Create {
            product_id,
            name,
            required,
        } => {
            let req = required.to_string();
            let params = [("name", name.as_str()), ("required", &req)];
            let field = client.create_custom_field(product_id, &params).await?;
            let row = CustomFieldRow::from(field);
            print_single(&row, format);
        }
        CustomFieldsCommand::Update {
            product_id,
            name,
            required,
        } => {
            let req = required.to_string();
            let params = [("required", req.as_str())];
            let field = client
                .update_custom_field(product_id, name, &params)
                .await?;
            let row = CustomFieldRow::from(field);
            print_single(&row, format);
        }
        CustomFieldsCommand::Delete { product_id, name } => {
            client.delete_custom_field(product_id, name).await?;
            println!("Custom field '{name}' deleted.");
        }
    }
    Ok(())
}
