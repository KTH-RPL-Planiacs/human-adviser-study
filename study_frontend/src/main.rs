use bevy::prelude::*;
use study_shared_types::GameResults;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(keyboard_input_system)
        .run();
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::A) {
        info!("'A' just pressed");
        let result = GameResults { participant_id: 7 };

        // create request - hacky json encoding
        let result_json = format!("{{\"participant_id\": {}}}", result.participant_id);
        let body = JsValue::from_str(&result_json);
        let headers = Headers::new().unwrap();
        headers.set("content-type", "application/json").unwrap();
        let url = format!("http://127.0.0.1:3030/data");
        let mut opts = RequestInit::new();
        opts.method("POST")
            .mode(RequestMode::Cors)
            .body(Some(&body))
            .headers(&headers);
        let request = Request::new_with_str_and_init(&url, &opts).unwrap();

        // send request
        let window = web_sys::window().unwrap();
        spawn_local(async move {
            match JsFuture::from(window.fetch_with_request(&request)).await {
                Ok(resp_value) => {
                    assert!(resp_value.is_instance_of::<Response>());
                    let _resp: Response = resp_value.dyn_into().unwrap();
                    // TODO: do something with the response
                }
                Err(e) => error!("Could not send results: {:?}", e),
            }
        });
    }
}
