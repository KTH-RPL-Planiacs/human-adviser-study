use study_shared_types::GameResults;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let game_result = GameResults { participant_id: 3 };
    let client = reqwest::Client::new();

    let res = client
        .post("http://127.0.0.1:3030/data")
        .json(&game_result)
        .send()
        .await?;

    println!("{:?}", res);

    Ok(())
}
