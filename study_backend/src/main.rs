use dotenv::dotenv;
use log::{error, info};
use mysql_async::{prelude::*, OptsBuilder};
use study_shared_types::{AdviserMode, GameResults};
use warp::{
    http::{self},
    Filter,
};

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
    info!("Received a result: {:?}", game_result);

    let pool = mysql_async::Pool::new(db_url());
    let mut conn = match pool.get_conn().await {
        Ok(c) => c,
        Err(e) => {
            error!("Could not connect: {}", e);
            return Ok(http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let query = r"INSERT INTO study_data (participant_id, adviser_mode, steps_taken, safety_violated, human_burgers, robot_burgers)
      VALUES (:participant_id, :adviser_mode, :steps_taken, :safety_violated, :human_burgers, :robot_burgers)"
        .with(params! {
            "participant_id" => game_result.participant_id,
            "adviser_mode" => AdviserMode::from_num(game_result.adviser_mode).to_string(),
            "steps_taken" => game_result.steps_taken,
            "safety_violated" => game_result.safety_violated,
            "human_burgers" => game_result.human_burgers,
            "robot_burgers" => game_result.robot_burgers,
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

    info!("Result insertion succeeded.");
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
        participant_id int primary key,
        adviser_mode ENUM('LeastLimiting', 'NextMove', 'None'),
        steps_taken int not null,
        safety_violated int not null,
        human_burgers int not null,
        robot_burgers int not null        
    )"
    .ignore(&mut conn)
    .await?;

    conn.disconnect().await?;
    pool.disconnect().await?;

    // POST DATABASE ENTRY
    let post_user_data = warp::post()
        .and(warp::path("data"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(insert_user_data);

    // CORS settings
    let cors = warp::cors()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["POST"])
        .allow_any_origin();

    warp::serve(post_user_data.with(cors))
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
