use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gumroad", about = "CLI for the Gumroad API v2")]
pub struct Cli {
    /// Access token (overrides env and config)
    #[arg(long, global = true, env = "GUMROAD_ACCESS_TOKEN")]
    pub access_token: Option<String>,

    /// Output format
    #[arg(long, global = true, default_value = "table")]
    pub output: OutputFormat,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
}

#[derive(Subcommand)]
pub enum Command {
    /// Show authenticated user info
    User,

    /// Manage products
    #[command(subcommand)]
    Products(ProductsCommand),

    /// Manage product custom fields
    #[command(subcommand)]
    CustomFields(CustomFieldsCommand),

    /// Manage product offer codes
    #[command(subcommand)]
    OfferCodes(OfferCodesCommand),

    /// Manage product variant categories
    #[command(subcommand)]
    VariantCategories(VariantCategoriesCommand),

    /// Manage variants within a category
    #[command(subcommand)]
    Variants(VariantsCommand),

    /// List product SKUs
    #[command(subcommand)]
    Skus(SkusCommand),

    /// Manage subscribers
    #[command(subcommand)]
    Subscribers(SubscribersCommand),

    /// Manage sales
    #[command(subcommand)]
    Sales(SalesCommand),

    /// Manage payouts
    #[command(subcommand)]
    Payouts(PayoutsCommand),

    /// Manage license keys
    #[command(subcommand)]
    Licenses(LicensesCommand),

    /// Manage resource subscriptions (webhooks)
    #[command(subcommand)]
    ResourceSubscriptions(ResourceSubscriptionsCommand),

    /// Authentication
    #[command(subcommand)]
    Auth(AuthCommand),
}

// --- Products ---

#[derive(Subcommand)]
pub enum ProductsCommand {
    /// List all products
    List,
    /// Get a product by ID
    Get {
        /// Product ID
        id: String,
    },
    /// Create a new product
    Create {
        /// Product name
        #[arg(long)]
        name: String,
        /// Price in cents (USD)
        #[arg(long)]
        price: Option<u64>,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// URL-friendly permalink
        #[arg(long)]
        url: Option<String>,
    },
    /// Update a product
    Update {
        /// Product ID
        id: String,
        /// Product name
        #[arg(long)]
        name: Option<String>,
        /// Price in cents (USD)
        #[arg(long)]
        price: Option<u64>,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// URL-friendly permalink
        #[arg(long)]
        url: Option<String>,
    },
    /// Delete a product
    Delete {
        /// Product ID
        id: String,
    },
    /// Enable (publish) a product
    Enable {
        /// Product ID
        id: String,
    },
    /// Disable (unpublish) a product
    Disable {
        /// Product ID
        id: String,
    },
}

// --- Custom Fields ---

#[derive(Subcommand)]
pub enum CustomFieldsCommand {
    /// List custom fields for a product
    List {
        /// Product ID
        #[arg(long)]
        product_id: String,
    },
    /// Create a custom field
    Create {
        /// Product ID
        #[arg(long)]
        product_id: String,
        /// Field name
        #[arg(long)]
        name: String,
        /// Whether the field is required
        #[arg(long, default_value_t = false)]
        required: bool,
    },
    /// Update a custom field
    Update {
        /// Product ID
        #[arg(long)]
        product_id: String,
        /// Custom field name (used as identifier)
        #[arg(long)]
        name: String,
        /// Whether the field is required
        #[arg(long)]
        required: bool,
    },
    /// Delete a custom field
    Delete {
        /// Product ID
        #[arg(long)]
        product_id: String,
        /// Custom field name
        #[arg(long)]
        name: String,
    },
}

// --- Offer Codes ---

#[derive(Subcommand)]
pub enum OfferCodesCommand {
    /// List offer codes for a product
    List {
        /// Product ID
        #[arg(long)]
        product_id: String,
    },
    /// Get an offer code
    Get {
        /// Product ID
        #[arg(long)]
        product_id: String,
        /// Offer code ID
        id: String,
    },
    /// Create an offer code
    Create {
        /// Product ID
        #[arg(long)]
        product_id: String,
        /// Offer code name
        #[arg(long)]
        name: String,
        /// Discount amount (cents or percent depending on type)
        #[arg(long)]
        amount_off: u64,
        /// "cents" or "percent"
        #[arg(long, default_value = "cents")]
        offer_type: String,
        /// Max number of uses (0 = unlimited)
        #[arg(long, default_value_t = 0)]
        max_purchase_count: u64,
        /// Universal (applies to all products)
        #[arg(long, default_value_t = false)]
        universal: bool,
    },
    /// Update an offer code
    Update {
        /// Product ID
        #[arg(long)]
        product_id: String,
        /// Offer code ID
        id: String,
        /// Max number of uses
        #[arg(long)]
        max_purchase_count: Option<u64>,
    },
    /// Delete an offer code
    Delete {
        /// Product ID
        #[arg(long)]
        product_id: String,
        /// Offer code ID
        id: String,
    },
}

// --- Variant Categories ---

#[derive(Subcommand)]
pub enum VariantCategoriesCommand {
    /// List variant categories for a product
    List {
        /// Product ID
        #[arg(long)]
        product_id: String,
    },
    /// Get a variant category
    Get {
        /// Product ID
        #[arg(long)]
        product_id: String,
        /// Variant category ID
        id: String,
    },
    /// Create a variant category
    Create {
        /// Product ID
        #[arg(long)]
        product_id: String,
        /// Category title
        #[arg(long)]
        title: String,
    },
    /// Update a variant category
    Update {
        /// Product ID
        #[arg(long)]
        product_id: String,
        /// Variant category ID
        id: String,
        /// New title
        #[arg(long)]
        title: String,
    },
    /// Delete a variant category
    Delete {
        /// Product ID
        #[arg(long)]
        product_id: String,
        /// Variant category ID
        id: String,
    },
}

// --- Variants ---

#[derive(Subcommand)]
pub enum VariantsCommand {
    /// List variants in a category
    List {
        /// Product ID
        #[arg(long)]
        product_id: String,
        /// Variant category ID
        #[arg(long)]
        variant_category_id: String,
    },
    /// Get a variant
    Get {
        /// Product ID
        #[arg(long)]
        product_id: String,
        /// Variant category ID
        #[arg(long)]
        variant_category_id: String,
        /// Variant ID
        id: String,
    },
    /// Create a variant
    Create {
        /// Product ID
        #[arg(long)]
        product_id: String,
        /// Variant category ID
        #[arg(long)]
        variant_category_id: String,
        /// Variant name
        #[arg(long)]
        name: String,
        /// Price difference in cents
        #[arg(long)]
        price_difference: Option<i64>,
        /// Max purchase count (0 = unlimited)
        #[arg(long)]
        max_purchase_count: Option<u64>,
    },
    /// Update a variant
    Update {
        /// Product ID
        #[arg(long)]
        product_id: String,
        /// Variant category ID
        #[arg(long)]
        variant_category_id: String,
        /// Variant ID
        id: String,
        /// Variant name
        #[arg(long)]
        name: Option<String>,
        /// Price difference in cents
        #[arg(long)]
        price_difference: Option<i64>,
        /// Max purchase count
        #[arg(long)]
        max_purchase_count: Option<u64>,
    },
    /// Delete a variant
    Delete {
        /// Product ID
        #[arg(long)]
        product_id: String,
        /// Variant category ID
        #[arg(long)]
        variant_category_id: String,
        /// Variant ID
        id: String,
    },
}

// --- SKUs ---

#[derive(Subcommand)]
pub enum SkusCommand {
    /// List SKUs for a product
    List {
        /// Product ID
        #[arg(long)]
        product_id: String,
    },
}

// --- Subscribers ---

#[derive(Subcommand)]
pub enum SubscribersCommand {
    /// List subscribers for a product
    List {
        /// Product ID
        #[arg(long)]
        product_id: String,
    },
    /// Get a subscriber by ID
    Get {
        /// Subscriber ID
        id: String,
    },
}

// --- Sales ---

#[derive(Subcommand)]
pub enum SalesCommand {
    /// List sales (paginated)
    List {
        /// Filter sales after this date (YYYY-MM-DD)
        #[arg(long)]
        after: Option<String>,
        /// Filter sales before this date (YYYY-MM-DD)
        #[arg(long)]
        before: Option<String>,
        /// Product ID filter
        #[arg(long)]
        product_id: Option<String>,
        /// Email filter
        #[arg(long)]
        email: Option<String>,
        /// Page key for pagination
        #[arg(long)]
        page_key: Option<String>,
    },
    /// Get a sale by ID
    Get {
        /// Sale ID
        id: String,
    },
    /// Mark a sale as shipped
    MarkAsShipped {
        /// Sale ID
        id: String,
        /// Tracking URL
        #[arg(long)]
        tracking_url: Option<String>,
    },
    /// Refund a sale
    Refund {
        /// Sale ID
        id: String,
        /// Amount in cents to refund (omit for full refund)
        #[arg(long)]
        amount_cents: Option<u64>,
    },
    /// Resend receipt for a sale
    ResendReceipt {
        /// Sale ID
        id: String,
    },
}

// --- Payouts ---

#[derive(Subcommand)]
pub enum PayoutsCommand {
    /// List payouts (paginated)
    List {
        /// Page number
        #[arg(long)]
        page: Option<u32>,
    },
    /// Get a payout by ID
    Get {
        /// Payout ID
        id: String,
    },
    /// Show upcoming payout
    Upcoming,
}

// --- Licenses ---

#[derive(Subcommand)]
pub enum LicensesCommand {
    /// Verify a license key
    Verify {
        /// Product ID
        #[arg(long)]
        product_id: String,
        /// License key
        #[arg(long)]
        license_key: String,
        /// Increment uses count
        #[arg(long, default_value_t = true)]
        increment_uses_count: bool,
    },
    /// Enable a license
    Enable {
        /// Product ID
        #[arg(long)]
        product_id: String,
        /// License key
        #[arg(long)]
        license_key: String,
    },
    /// Disable a license
    Disable {
        /// Product ID
        #[arg(long)]
        product_id: String,
        /// License key
        #[arg(long)]
        license_key: String,
    },
    /// Decrement uses count
    DecrementUses {
        /// Product ID
        #[arg(long)]
        product_id: String,
        /// License key
        #[arg(long)]
        license_key: String,
    },
    /// Rotate (generate new) license key
    Rotate {
        /// Product ID
        #[arg(long)]
        product_id: String,
        /// License key
        #[arg(long)]
        license_key: String,
    },
}

// --- Resource Subscriptions ---

#[derive(Subcommand)]
pub enum ResourceSubscriptionsCommand {
    /// List resource subscriptions
    List,
    /// Create a resource subscription
    Create {
        /// Resource name (e.g. "sale", "refund", "cancellation", etc.)
        #[arg(long)]
        resource_name: String,
        /// Post URL (webhook endpoint)
        #[arg(long)]
        post_url: String,
    },
    /// Delete a resource subscription
    Delete {
        /// Resource subscription ID
        id: String,
    },
}

// --- Auth ---

#[derive(Subcommand)]
pub enum AuthCommand {
    /// Store your access token
    Login,
}
