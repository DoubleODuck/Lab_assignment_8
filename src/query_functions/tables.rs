use super::super::{input_as_number, user_input};
use chrono::naive::NaiveDate;
#[derive(sqlx::FromRow, Debug)]
pub struct Orders {
    pub(crate) id: i32,
    pub(crate) CustomerID: Option<i32>,
    pub(crate) ItemID: Option<u32>,
    pub(crate) BulkId: Option<i32>,
    pub(crate) OrderDate: NaiveDate,
    pub(crate) PayStatus: String,
    pub(crate) DeliveryStatus: String,
    pub(crate) OrderFullfilmentDate: NaiveDate,
}
impl Orders {
    // Constructor function with user input
    pub fn new_with_input() -> Self {
        Orders {
            id: 32,
            CustomerID: Some(input_as_number("Enter Customer ID:") as i32),
            ItemID: Some(input_as_number("Enter Item ID:") as u32),
            BulkId: Some(input_as_number("Enter Bulk ID:") as i32),
            OrderDate: NaiveDate::parse_from_str(
                user_input("Enter Order Date:").as_str().trim(),
                "%Y-%m-%d",
            )
            .unwrap(),
            PayStatus: user_input("Enter Pay Status:"),
            DeliveryStatus: user_input("Enter Delivery Status:"),
            OrderFullfilmentDate: NaiveDate::parse_from_str(
                user_input("Enter Order Fulfillment Date:").as_str().trim(),
                "%Y-%m-%d",
            )
            .unwrap(),
        }
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct BulkPrices {
    pub(crate) BIC: i32,
    pub(crate) ItemName: String,
    pub(crate) BulkPrice: i32,
    pub(crate) DistributorInfo: String,
    pub(crate) BAV: bool,
}

impl BulkPrices {
    // Constructor function with user input
    pub fn new_with_input() -> Self {
        BulkPrices {
            BIC: 32,
            ItemName: user_input("Enter Item Name:"),
            BulkPrice: input_as_number("Enter Bulk Price:") as i32,
            DistributorInfo: user_input("Enter Distributor Info:"),
            BAV: input_as_number("Enter BAV (1 for true, 0 for false):") != 0,
        }
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct Catalogue {
    pub(crate) CIC: Option<u32>,
    pub(crate) product_name: String,
    pub(crate) manufacturer: String,
    pub(crate) category: String,
    pub(crate) distribute_date: u32,
    pub(crate) short_specs: String,
    pub(crate) full_specs: String,
    pub(crate) price: f32,
    pub(crate) CAV: bool,
}
impl Catalogue {
    // Constructor function with user input
    pub fn new_with_input() -> Self {
        Catalogue {
            CIC: Some(32),
            product_name: user_input("Enter Product Name:"),
            manufacturer: user_input("Enter Manufacturer:"),
            category: user_input("Enter Category:"),
            distribute_date: input_as_number("Enter Distribute Date:") as u32,
            short_specs: user_input("Enter Short Specs:"),
            full_specs: user_input("Enter Full Specs:"),
            price: input_as_number("Enter Price:") as f32,
            CAV: input_as_number("Enter CAV (1 for true, 0 for false):") != 0,
        }
    }
}
#[derive(sqlx::FromRow, Debug)]
pub struct ClientList {
    pub(crate) ClientID: i32,
    pub(crate) full_name: String,
    pub(crate) Birthday: NaiveDate,
    pub(crate) Address: String,
    pub(crate) phone_number: String,
    pub(crate) email: String,
}
impl ClientList {
    // Constructor function with user input
    pub fn new_with_input() -> Self {
        ClientList {
            ClientID: 32,
            full_name: user_input("Enter Full Name:"),
            Birthday: NaiveDate::parse_from_str(user_input("Enter Birthday:").as_str().trim(), "%Y-%m-%d")
                .unwrap(),
            Address: user_input("Enter Address:"),
            phone_number: user_input("Enter Phone Number:"),
            email: user_input("Enter Email:"),
        }
    }
}
#[derive(sqlx::FromRow, Debug)]
pub struct TableStruct {
    Field: String,
    Type: String,
    Null: String,
    Key: String,
    Default: String,
    xtra: String,
}
