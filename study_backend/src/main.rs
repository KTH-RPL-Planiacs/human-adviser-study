use dotenv::dotenv;
use log::error;
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

async fn insert_user_data(game_result: GameResults) -> Result<impl warp::Reply, warp::Rejection> {
    println!("GOT A RESULT {:?}", game_result);

    let pool = mysql_async::Pool::new(db_url());
    let mut conn = match pool.get_conn().await {
        Ok(c) => c,
        Err(e) => {
            error!("Could not connect: {}", e);
            return Ok(http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let query = r"INSERT INTO study_data (participant_id)
      VALUES (:participant_id)"
        .with(params! {
            "participant_id" => game_result.participant_id
        });

    // insert game result data
    if let Err(e) = query.ignore(&mut conn).await {
        error!("Could not insert: {}", e);
        return Ok(http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    if let Err(e) = conn.disconnect().await {
        error!("Could not disconnect: {}", e);
        return Ok(http::StatusCode::INTERNAL_SERVER_ERROR);
    }
    if let Err(e) = pool.disconnect().await {
        error!("Could not connect: {}", e);
        return Ok(http::StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok(http::StatusCode::CREATED)
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().expect("Could not find .env file!");
    pretty_env_logger::init();

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
