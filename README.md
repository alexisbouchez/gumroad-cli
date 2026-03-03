# gumroad-cli

A command-line interface for the [Gumroad API v2](https://app.gumroad.com/api). Manage products, sales, payouts, licenses, subscribers, and more from your terminal.

## Installation

### From source

```sh
cargo install --path .
```

### Build locally

```sh
git clone https://github.com/alexisbouchez/gumroad-cli.git
cd gumroad-cli
cargo build --release
# Binary is at target/release/gumroad
```

## Authentication

You need a [Gumroad API access token](https://app.gumroad.com/settings/advanced#application-form). The CLI resolves your token in this order:

1. `--access-token` flag
2. `GUMROAD_ACCESS_TOKEN` environment variable
3. Config file (`~/.config/gumroad/config.toml`)

### Save your token to the config file

```sh
gumroad auth login
# Prompts for your access token and saves it
```

### Use an environment variable

```sh
export GUMROAD_ACCESS_TOKEN="your-token-here"
gumroad user
```

### Pass it directly

```sh
gumroad --access-token "your-token-here" user
```

## Output Formats

All commands support `--output table` (default) and `--output json`:

```sh
gumroad products list
gumroad products list --output json
```

## Commands

### User

```sh
# Show authenticated user info
gumroad user
```

### Products

```sh
# List all products
gumroad products list

# Get a product by ID
gumroad products get <PRODUCT_ID>

# Create a product
gumroad products create --name "My Product" --price 999 --description "A great product"

# Update a product
gumroad products update <PRODUCT_ID> --name "New Name" --price 1999

# Delete a product
gumroad products delete <PRODUCT_ID>

# Publish a product
gumroad products enable <PRODUCT_ID>

# Unpublish a product
gumroad products disable <PRODUCT_ID>
```

### Sales

```sh
# List sales (paginated)
gumroad sales list

# Filter by date range
gumroad sales list --after 2024-01-01 --before 2024-12-31

# Filter by product or email
gumroad sales list --product-id <PRODUCT_ID> --email customer@example.com

# Paginate through results (next page key is printed to stderr)
gumroad sales list --page-key <PAGE_KEY>

# Get a specific sale
gumroad sales get <SALE_ID>

# Mark a sale as shipped
gumroad sales mark-as-shipped <SALE_ID>
gumroad sales mark-as-shipped <SALE_ID> --tracking-url "https://tracking.example.com/123"

# Refund a sale (full refund)
gumroad sales refund <SALE_ID>

# Partial refund (amount in cents)
gumroad sales refund <SALE_ID> --amount-cents 500

# Resend receipt email
gumroad sales resend-receipt <SALE_ID>
```

### Payouts

```sh
# List payouts
gumroad payouts list
gumroad payouts list --page 2

# Get a specific payout
gumroad payouts get <PAYOUT_ID>

# Show upcoming payout
gumroad payouts upcoming
```

### Licenses

```sh
# Verify a license key
gumroad licenses verify --product-id <PRODUCT_ID> --license-key <KEY>

# Verify without incrementing the uses count
gumroad licenses verify --product-id <PRODUCT_ID> --license-key <KEY> --increment-uses-count false

# Enable a license
gumroad licenses enable --product-id <PRODUCT_ID> --license-key <KEY>

# Disable a license
gumroad licenses disable --product-id <PRODUCT_ID> --license-key <KEY>

# Decrement uses count
gumroad licenses decrement-uses --product-id <PRODUCT_ID> --license-key <KEY>

# Rotate (generate new) license key
gumroad licenses rotate --product-id <PRODUCT_ID> --license-key <KEY>
```

### Subscribers

```sh
# List subscribers for a product
gumroad subscribers list --product-id <PRODUCT_ID>

# Get a specific subscriber
gumroad subscribers get <SUBSCRIBER_ID>
```

### Custom Fields

```sh
# List custom fields for a product
gumroad custom-fields list --product-id <PRODUCT_ID>

# Create a custom field
gumroad custom-fields create --product-id <PRODUCT_ID> --name "Company" --required

# Update a custom field
gumroad custom-fields update --product-id <PRODUCT_ID> --name "Company" --required

# Delete a custom field
gumroad custom-fields delete --product-id <PRODUCT_ID> --name "Company"
```

### Offer Codes

```sh
# List offer codes
gumroad offer-codes list --product-id <PRODUCT_ID>

# Get a specific offer code
gumroad offer-codes get --product-id <PRODUCT_ID> <OFFER_CODE_ID>

# Create a dollar-off offer code
gumroad offer-codes create --product-id <PRODUCT_ID> --name "SAVE5" --amount-off 500

# Create a percent-off offer code
gumroad offer-codes create --product-id <PRODUCT_ID> --name "HALF" --amount-off 50 --offer-type percent

# Create with a usage limit
gumroad offer-codes create --product-id <PRODUCT_ID> --name "LIMITED" --amount-off 1000 --max-purchase-count 100

# Update an offer code
gumroad offer-codes update --product-id <PRODUCT_ID> <OFFER_CODE_ID> --max-purchase-count 200

# Delete an offer code
gumroad offer-codes delete --product-id <PRODUCT_ID> <OFFER_CODE_ID>
```

### Variant Categories

```sh
# List variant categories
gumroad variant-categories list --product-id <PRODUCT_ID>

# Get a variant category
gumroad variant-categories get --product-id <PRODUCT_ID> <CATEGORY_ID>

# Create a variant category
gumroad variant-categories create --product-id <PRODUCT_ID> --title "Size"

# Update a variant category
gumroad variant-categories update --product-id <PRODUCT_ID> <CATEGORY_ID> --title "Color"

# Delete a variant category
gumroad variant-categories delete --product-id <PRODUCT_ID> <CATEGORY_ID>
```

### Variants

```sh
# List variants in a category
gumroad variants list --product-id <PRODUCT_ID> --variant-category-id <CATEGORY_ID>

# Get a variant
gumroad variants get --product-id <PRODUCT_ID> --variant-category-id <CATEGORY_ID> <VARIANT_ID>

# Create a variant
gumroad variants create --product-id <PRODUCT_ID> --variant-category-id <CATEGORY_ID> --name "Large" --price-difference 200

# Update a variant
gumroad variants update --product-id <PRODUCT_ID> --variant-category-id <CATEGORY_ID> <VARIANT_ID> --name "XL"

# Delete a variant
gumroad variants delete --product-id <PRODUCT_ID> --variant-category-id <CATEGORY_ID> <VARIANT_ID>
```

### SKUs

```sh
# List SKUs for a product
gumroad skus list --product-id <PRODUCT_ID>
```

### Resource Subscriptions (Webhooks)

```sh
# List resource subscriptions
gumroad resource-subscriptions list

# Subscribe to sale events
gumroad resource-subscriptions create --resource-name sale --post-url "https://example.com/webhooks/sale"

# Available resource names: sale, refund, dispute, dispute_won,
# cancellation, subscription_updated, subscription_ended,
# subscription_restarted, subscription_duration_changed

# Delete a subscription
gumroad resource-subscriptions delete <SUBSCRIPTION_ID>
```

### Auth

```sh
# Save your access token to ~/.config/gumroad/config.toml
gumroad auth login
```

## API Coverage

This CLI covers the entire Gumroad API v2:

| Resource | Endpoints |
|---|---|
| User | get |
| Products | list, get, create, update, delete, enable, disable |
| Custom Fields | list, create, update, delete |
| Offer Codes | list, get, create, update, delete |
| Variant Categories | list, get, create, update, delete |
| Variants | list, get, create, update, delete |
| SKUs | list |
| Subscribers | list, get |
| Sales | list, get, mark-as-shipped, refund, resend-receipt |
| Payouts | list, get, upcoming |
| Licenses | verify, enable, disable, decrement-uses, rotate |
| Resource Subscriptions | list, create, delete |

## License

Apache-2.0
