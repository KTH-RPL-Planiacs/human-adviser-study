use bevy::prelude::*;
use study_shared_types::GameResults;
use web_sys::{Request, RequestInit, RequestMode, Response};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(keyboard_input_system)
        .run();
}

/// This system prints 'A' key state
fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::A) {
        info!("'A' just pressed");
        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.mode(RequestMode::Cors);

        let url = "http://127.0.0.1:3030/data";
    }
}

/*
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
*/
