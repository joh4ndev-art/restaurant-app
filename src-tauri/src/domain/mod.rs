pub mod category;
pub mod order;
pub mod payment;
pub mod product;
pub mod reports;
pub mod table;
pub mod user;

pub use category::{Category, CreateCategoryInput};
pub use order::{
    Order, OrderItem, OrderItemStatus, OrderStatus, SendOrderInput, SendOrderItemInput,
};
pub use payment::{PayOrderInput, Payment, PaymentMethod, PaymentSummary};
pub use product::{CreateProductInput, Product, UpdateProductInput};
pub use reports::{ReportsBundle, ReportsInput, SalesByDay, SalesSummary, TopProduct};
pub use table::{CreateTableInput, Table, TableStatus};
pub use user::{
    ChangePinInput, CreateUserInput, LoginInput, Role, UpdateUserInput, User,
};