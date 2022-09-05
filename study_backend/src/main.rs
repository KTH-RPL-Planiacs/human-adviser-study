use dotenv::dotenv;
use mysql_async::{prelude::*, OptsBuilder};
use study_shared_types::GameResults;
use warp::{http, Filter};

fn db_url() -> OptsBuilder {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let database_name = std::env::var("DATABASE_NAME").expect("DATABASE_NAME must be set.");
    let database_port = std::env::var("DATABASE_PORT")
        .expect("DATABASE_PORT must be set.")
        .parse::<u16>()
        .expect("DATABASE_PORT is not a u16.");
    let user = std::env::var("DATABASE_USER").expect("DATABASE_USER must be set.");
    let pass = std::env::var("DATABASE_PASS").expect("DATABASE_PASS must be set.");

    OptsBuilder::default()
        .ip_or_hostname(database_url)
        .tcp_port(database_port)
        .user(Some(user))
        .pass(Some(pass))
        .db_name(Some(database_name))
}

async fn insert_user_data(id: u32) -> Result<impl warp::Reply, warp::Rejection> {
    let game_result = GameResults { id };

    println!("{:?}", game_result);

    let pool = mysql_async::Pool::new(db_url());
    let mut conn = match pool.get_conn().await {
        Ok(c) => c,
        Err(_) => return Ok(http::StatusCode::INTERNAL_SERVER_ERROR),
    };

    // make sure the table exists
    match r"CREATE TABLE if not exists study_data (
        participant_id int not null
    )"
    .ignore(&mut conn)
    .await
    {
        Ok(_) => (),
        Err(_) => return Ok(http::StatusCode::INTERNAL_SERVER_ERROR),
    }

    if let Err(_) = conn.disconnect().await {
        return Ok(http::StatusCode::INTERNAL_SERVER_ERROR);
    }
    if let Err(_) = pool.disconnect().await {
        return Ok(http::StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok(http::StatusCode::INTERNAL_SERVER_ERROR)
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().expect("Could not find .env file!");

    let pool = mysql_async::Pool::new(db_url());
    let mut conn = pool.get_conn().await?;

    // make sure the table exists
    r"CREATE TABLE if not exists study_data (
        participant_id int not null
    )"
    .ignore(&mut conn)
    .await?;

    conn.disconnect().await?;
    pool.disconnect().await?;

    let post_user_data = warp::post()
        .and(warp::path("data"))
        // Only accept bodies smaller than 16kb...
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(insert_user_data);

    warp::serve(post_user_data)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}

/*
let database_url = "FILLMEIN"; // Todo: fill in

    let pool = mysql_async::Pool::new(database_url);
    let mut conn = pool.get_conn().await?;

    // Create a table
    r"CREATE TEMPORARY TABLE if not exists payment (
        customer_id int not null,
        amount int not null,
        account_name text
    )"
    .ignore(&mut conn)
    .await?;

    // Save payments
    r"INSERT INTO payment (customer_id, amount, account_name)
      VALUES (:customer_id, :amount, :account_name)"
        .with(payments.iter().map(|payment| {
            params! {
                "customer_id" => payment.customer_id,
                "amount" => payment.amount,
                "account_name" => payment.account_name.as_ref(),
            }
        }))
        .batch(&mut conn)
        .await?;

    // Load payments from the database. Type inference will work here.
    let loaded_payments = "SELECT customer_id, amount, account_name FROM payment"
        .with(())
        .map(&mut conn, |(customer_id, amount, account_name)| Payment {
            customer_id,
            amount,
            account_name,
        })
        .await?;

    // Dropped connection will go to the pool
    drop(conn);

    // The Pool must be disconnected explicitly because
    // it's an asynchronous operation.
    pool.disconnect().await?;

    println!("There are {} payments.", loaded_payments.len());
*/
