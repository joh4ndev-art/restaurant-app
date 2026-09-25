pub mod category_repo;
pub mod order_repo;
pub mod payment_repo;
pub mod product_repo;
pub mod reports_repo;
pub mod settings_repo;
pub mod table_repo;

pub use category_repo::CategoryRepository;
pub use order_repo::OrderRepository;
pub use payment_repo::PaymentRepository;
pub use product_repo::ProductRepository;
pub use reports_repo::ReportsRepository;
pub use settings_repo::SettingsRepository;
pub use table_repo::TableRepository;