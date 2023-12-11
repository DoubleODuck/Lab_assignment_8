use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use std::{env, io::stdin};

pub mod query_functions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("");
    let pool = MySqlPool::connect(&database_url).await?;
    println!("Connected to Database suspc");
    println!("1. Show all tables");
    println!("2. Show the structure of a table");
    println!("3. Show the contents of a table");
    println!("4. Show the average price of a product with a discount");
    println!("5. Filter table catalogue by price range(>, <, =)");
    println!("6. Filter table catalogue by price range(AND, OR, NOT)");
    println!("7. Sort table by column");
    println!("8. Insert new values");
    println!("9. update statement 1");
    println!("10. update statement 2");
    println!("11. update statement 3");
    let choice = input_as_number("Choose your action");
    match choice {
        1 => {
            query_functions::show_tables(&pool).await?;
        }
        2 => {
            let table_name = table_choice(&pool).await?;
            if !table_name.is_empty() {
                query_functions::describe_table(&pool, &table_name).await?;
            } else {
                println!("invalid choice")
            }
        }
        3 => {
            let table_name = table_choice(&pool).await?;
            if !table_name.is_empty() {
                let query = format!("Select * from {}", table_name);
                query_functions::display_table(&pool, &table_name, query).await?;
            } else {
                println!("invalid choice")
            }
        }
        4 => {
            let discount = input_as_number("Enter percentage");
            query_functions::discounted_average_price(discount, &pool).await?;
        }
        5 => {
            let table_name = "catalogue".to_string();
            let column_name = "price".to_string();
            if !column_name.is_empty() {
                let range_symbol = range_choice();
                if !range_symbol.is_empty() {
                    let range_num = input_as_number("give me range number");
                    query_functions::show_in_range(
                        &pool,
                        &table_name,
                        column_name,
                        vec![&range_symbol],
                        vec![range_num],
                        "",
                    )
                    .await?;
                } else {
                    println!("invalid choice")
                }
            } else {
                println!("invalid choice")
            }
        }
        6 => {
            let table_name = "catalogue".to_string();
            let column_name = "price".to_string();
            if !column_name.is_empty() {
                let logic = inclusion_choice();
                if logic.eq("AND") || logic.eq("OR") {
                    println!("Choose range numbers");
                    let mut range_nums = vec![input_as_number("lesser than"), input_as_number("more than")];
                    range_nums.sort();
                    if !(range_nums.get(0).unwrap() == &usize::MAX)
                        && !(range_nums.get(1).unwrap() == &usize::MAX)
                    {
                        query_functions::show_in_range(
                            &pool,
                            &table_name,
                            column_name,
                            vec![">=", "<="],
                            range_nums,
                            logic,
                        )
                        .await?;
                    } else {
                        println!("invalid choice")
                    }
                } else {
                    println!("invalid choice")
                }
            } else {
                println!("invalid choice")
            }
        }
        7 => {
            let table_name = table_choice(&pool).await?;
            let column_name = column_choice(&pool, &table_name).await?;
            let order_by = sort_option();
            if !table_name.is_empty() && !column_name.is_empty() && !order_by.is_empty() {
                let query = format!(
                    "Select * from {} order by {} {}",
                    table_name, column_name, order_by
                );
                query_functions::display_table(&pool, &table_name, query).await?;
            } else {
                println!("invalid choice");
            }
        }
        8=>{
            let table_name = table_choice(&pool).await?;
            query_functions::insert_into(&pool, &table_name).await;
        }
        9=> query_functions::update_info1(&pool).await?,
        10=> query_functions::update_info2(&pool).await?,
        11=> query_functions::update_info3(&pool).await?,
        _ => println!("invalid choice"),
    }
    Ok(())
}
fn input_as_number(prompt: &str) -> usize {
    let input = match user_input(prompt).trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("invalid input");
            usize::MAX
        }
    };
    input
}
fn user_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{prompt}");
    stdin().read_line(&mut input).expect("failed to read line");
    input
}
async fn table_choice(pool: &MySqlPool) -> Result<String, sqlx::Error> {
    let tables = query_functions::show_tables(&pool).await?;
    let position = input_as_number("enter the position of your table");
    if position <= tables.len() {
        let table_name = tables.get(position - 1).unwrap().clone();
        Ok(table_name)
    } else {
        println!("invalid choice");
        Ok("".to_string())
    }
}
async fn column_choice(pool: &MySqlPool, table_name: &String) -> Result<String, sqlx::Error> {
    if !table_name.is_empty() {
        let columns = query_functions::show_column_names(&pool, table_name).await?;
        let position = input_as_number("enter the position of your column");
        if position <= columns.len() {
            let column_name: String = columns.get(position - 1).unwrap().clone();
            Ok(column_name)
        } else {
            println!("invalid choice");
            Ok("".to_string())
        }
    } else {
        Ok("".to_string())
    }
}

fn range_choice() -> &'static str {
    let range = vec![">", ">=", "<", "<=", "="];
    choose_from_vec(range)
}

fn inclusion_choice() -> &'static str {
    let inclusion = vec!["AND", "OR", "NOT"];
    choose_from_vec(inclusion)
}

fn sort_option() -> &'static str {
    let inclusion = vec!["asc", "desc"];
    choose_from_vec(inclusion)
}

fn choose_from_vec(vector: Vec<&str>) -> &str {
    let mut number = 1;
    for content in &vector[..] {
        println!("{number}. {content}");
        number += 1;
    }
    let choice = input_as_number("Choose logic");
    if choice <= vector.len() {
        vector.get(choice - 1).unwrap()
    } else {
        println!("invalid choice");
        ""
    }
}
