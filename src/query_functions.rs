pub mod tables;

use self::tables::{BulkPrices, Catalogue, ClientList, Orders};
use sqlx::{mysql::MySqlPool, Row};

pub async fn show_tables(pool: &MySqlPool) -> Result<Vec<String>, sqlx::Error> {
    let query = "show tables";
    println!("{query}");
    let tables = return_first_column(pool, query.to_string()).await?;
    Ok(tables)
}

pub async fn describe_table(pool: &MySqlPool, table_name: &str) -> Result<(), sqlx::Error> {
    let query = format!("desc {table_name}");
    println!("{query}");
    let result = sqlx::query_as::<_, tables::TableStruct>(&query)
        .fetch_all(pool)
        .await?;
    for block in result {
        println!("{:?}", block);
    }
    Ok(())
}

pub async fn discounted_average_price(
    discount: usize,
    pool: &MySqlPool,
) -> Result<(), sqlx::Error> {
    let average_price = sqlx::query("select CAST(AVG(price) AS FLOAT) AS newprice from catalogue")
        .fetch_one(pool)
        .await?;
    let average_price: f32 = average_price.try_get("newprice").unwrap();
    let new_price = (average_price * (100 - discount) as f32) / 100.0_f32;
    println!("{}", new_price);
    Ok(())
}

pub async fn display_table(
    pool: &MySqlPool,
    table_name: &str,
    query: String,
) -> Result<(), sqlx::Error> {
    println!("{query}");
    let bulk_result;
    let cat_result;
    let client_result;
    let order_result;
    match table_name {
        "bulk_prices" => {
            bulk_result = sqlx::query_as::<_, tables::BulkPrices>(&query)
                .fetch_all(pool)
                .await?;
            for block in bulk_result {
                println!("{:?}", block);
            }
        }
        "catalogue" => {
            cat_result = sqlx::query_as::<_, tables::Catalogue>(&query)
                .fetch_all(pool)
                .await?;
            for block in cat_result {
                println!("{:?}", block);
            }
        }
        "client_list" => {
            client_result = sqlx::query_as::<_, tables::ClientList>(&query)
                .fetch_all(pool)
                .await?;
            for block in client_result {
                println!("{:?}", block);
            }
        }
        "orders" => {
            order_result = sqlx::query_as::<_, tables::Orders>(&query)
                .fetch_all(pool)
                .await?;
            for block in order_result {
                println!("{:?}", block);
            }
        }
        &_ => println!("how did we get here?"),
    }
    Ok(())
}

pub async fn show_column_names(
    pool: &MySqlPool,
    table_name: &String,
) -> Result<Vec<String>, sqlx::Error> {
    let query = format!("desc {}", table_name);
    println!("{query}");
    let columns = return_first_column(pool, query).await?;
    Ok(columns)
}

pub async fn return_first_column(
    pool: &MySqlPool,
    query: String,
) -> Result<Vec<String>, sqlx::Error> {
    let mut number = 1;
    let mut tables: Vec<String> = vec![];
    let result = sqlx::query(&query).fetch_all(pool).await?;
    for table in result {
        let table_name: String = table.get(0);
        println!("{}.{}", number, table_name);
        number += 1;
        tables.push(table_name);
    }
    Ok(tables)
}

pub async fn show_in_range(
    pool: &MySqlPool,
    table_name: &str,
    column_name: String,
    symbol: Vec<&str>,
    number: Vec<usize>,
    inclusion_symbol: &str,
) -> Result<(), sqlx::Error> {
    let query = gen_query(table_name, column_name, symbol, number, inclusion_symbol);
    println!("{query}");
    let result = sqlx::query_as::<_, Catalogue>(&query)
        .fetch_all(pool)
        .await?;
    if result.is_empty() {
        println!("nothing in this table satisfies the range");
        Ok(())
    } else {
        println!("Product\t\t\tPrice");
        for block in result {
            let product_name = block.product_name;
            let product_price = block.price;
            let mut tabs;
            if product_name.chars().count() <= 7 {
                tabs = "\t\t\t".to_string();
            } else if product_name.chars().count() >= 12 {
                tabs = "\t".to_string();
            } else {
                tabs = "\t\t".to_string();
            }
            println!("{}{}{}", product_name, tabs, product_price);
            tabs.clear();
        }
        Ok(())
    }
}

pub fn gen_query(
    table_name: &str,
    column_name: String,
    range_symbols: Vec<&str>,
    range_nums: Vec<usize>,
    inclusion_symbol: &str,
) -> String {
    let mut query = format!("Select * from {} where", table_name);
    if !inclusion_symbol.is_empty() {
        if inclusion_symbol.eq("NOT") {
            query = format!("{query} {} {} {} {}",inclusion_symbol, column_name, range_symbols.get(0).unwrap(), range_nums.get(0).unwrap());
        } else {
            query = format!(
                "{query} {} {} {} {}",
                inclusion_symbol,
                column_name,
                range_symbols.get(1).unwrap(),
                range_nums.get(1).unwrap()
            );
        }
    }
    query
}

pub async fn insert_into(pool: &MySqlPool, table_name: &str) {
    match table_name {
        "bulk_prices" => {
            let new_value = tables::BulkPrices::new_with_input();
            let query = bulk_query(
                table_name,
                new_value,
                next_id(pool, table_name).await.unwrap(),
            );
            sqlx::query(&query).execute(pool).await.unwrap();
        }
        "catalogue" => {
            let new_value = tables::Catalogue::new_with_input();
            let query = cat_query(
                table_name,
                new_value,
                next_id(pool, table_name).await.unwrap(),
            );
            sqlx::query(&query).execute(pool).await.unwrap();
        }
        "client_list" => {
            let new_value = tables::ClientList::new_with_input();
            let query = client_query(
                table_name,
                new_value,
                next_id(pool, table_name).await.unwrap(),
            );
            sqlx::query(&query).execute(pool).await.unwrap();
        }
        "orders" => {
            let new_value = tables::Orders::new_with_input();
            let query = order_query(
                table_name,
                new_value,
                next_id(pool, table_name).await.unwrap(),
            );
            sqlx::query(&query).execute(pool).await.unwrap();
        }
        &_ => println!("how did we get here?"),
    }
}

fn bulk_query(table_name: &str, tcurts: BulkPrices, id: u32) -> String {
    format!(
        "insert into {} 
        values
        ({},'{}',{},'{}',{})",
        table_name,
        id as i32,
        tcurts.ItemName.trim(),
        tcurts.BulkPrice,
        tcurts.DistributorInfo.trim(),
        tcurts.BAV
    )
}
fn cat_query(table_name: &str, tcurts: Catalogue, id: u32) -> String {
    format!(
        "insert into {} values({},'{}','{}','{}','{}','{}','{}','{}',{})",
        table_name,
        id,
        tcurts.product_name.trim(),
        tcurts.manufacturer.trim(),
        tcurts.category.trim(),
        tcurts.distribute_date,
        tcurts.short_specs.trim(),
        tcurts.full_specs.trim(),
        tcurts.price,
        tcurts.CAV
    )
}
fn client_query(table_name: &str, tcurts: ClientList, id: u32) -> String {
    format!(
        "insert into {} values({},'{}','{}','{}','{}','{}')",
        table_name,
        id,
        tcurts.full_name.trim(),
        tcurts.Birthday,
        tcurts.Address.trim(),
        tcurts.phone_number.trim(),
        tcurts.email.trim()
    )
}
fn order_query(table_name: &str, tcurts: Orders, id: u32) -> String {
    format!(
        "insert into {} values({},{},{},{},'{}','{}','{}','{}')",
        table_name,
        id as i32,
        tcurts.CustomerID.unwrap(),
        tcurts.ItemID.unwrap(),
        tcurts.BulkId.unwrap(),
        tcurts.OrderDate,
        tcurts.PayStatus.trim(),
        tcurts.DeliveryStatus.trim(),
        tcurts.OrderFullfilmentDate
    )
}
async fn next_id(pool: &MySqlPool, table_name: &str) -> Result<u32, sqlx::Error> {
    let result = sqlx::query(format!("select count(*) from {}", table_name).as_str())
        .fetch_one(pool)
        .await?;
    let count: i64 = result.try_get("count(*)").unwrap();
    Ok((count + 1) as u32)
}
pub async fn update_info1(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    let query = format!("update bulk_prices set ItemName = 'lmao even' WHERE BIC = 8");
    println!("{query}");
    sqlx::query(&query).execute(pool).await?;
    Ok(())
}
pub async fn update_info2(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    let query = format!("update catalogue set category = 'persian' where product_name = 'mane'");
    println!("{query}");
    sqlx::query(&query).execute(pool).await?;
    Ok(())
}
pub async fn update_info3(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    let query =
        format!("update client_list set email = 'FaaS@mymail.com' where full_name = 'Sarah Afton'");
    println!("{query}");
    sqlx::query(&query).execute(pool).await?;
    Ok(())
}
