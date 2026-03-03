use crate::cli::{OutputFormat, ProductsCommand};
use crate::client::GumroadClient;
use crate::error::Result;
use crate::models::product::ProductRow;
use crate::output::{print_output, print_single};

pub async fn execute(
    client: &GumroadClient,
    cmd: &ProductsCommand,
    format: OutputFormat,
) -> Result<()> {
    match cmd {
        ProductsCommand::List => {
            let products = client.list_products().await?;
            let rows: Vec<ProductRow> = products.into_iter().map(ProductRow::from).collect();
            print_output(&rows, format);
        }
        ProductsCommand::Get { id } => {
            let product = client.get_product(id).await?;
            let row = ProductRow::from(product);
            print_single(&row, format);
        }
        ProductsCommand::Create {
            name,
            price,
            description,
            url,
        } => {
            let mut params: Vec<(&str, String)> = vec![("name", name.clone())];
            if let Some(p) = price {
                params.push(("price", p.to_string()));
            }
            if let Some(d) = description {
                params.push(("description", d.clone()));
            }
            if let Some(u) = url {
                params.push(("url", u.clone()));
            }
            let param_refs: Vec<(&str, &str)> =
                params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            let product = client.create_product(&param_refs).await?;
            let row = ProductRow::from(product);
            print_single(&row, format);
        }
        ProductsCommand::Update {
            id,
            name,
            price,
            description,
            url,
        } => {
            let mut params: Vec<(&str, String)> = vec![];
            if let Some(n) = name {
                params.push(("name", n.clone()));
            }
            if let Some(p) = price {
                params.push(("price", p.to_string()));
            }
            if let Some(d) = description {
                params.push(("description", d.clone()));
            }
            if let Some(u) = url {
                params.push(("url", u.clone()));
            }
            let param_refs: Vec<(&str, &str)> =
                params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            let product = client.update_product(id, &param_refs).await?;
            let row = ProductRow::from(product);
            print_single(&row, format);
        }
        ProductsCommand::Delete { id } => {
            client.delete_product(id).await?;
            println!("Product {id} deleted.");
        }
        ProductsCommand::Enable { id } => {
            let product = client.enable_product(id).await?;
            let row = ProductRow::from(product);
            print_single(&row, format);
        }
        ProductsCommand::Disable { id } => {
            let product = client.disable_product(id).await?;
            let row = ProductRow::from(product);
            print_single(&row, format);
        }
    }
    Ok(())
}
