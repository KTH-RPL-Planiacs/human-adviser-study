use bevy::prelude::*;
use study_shared_types::GameResults;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
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
        opts.method("GET");
        opts.mode(RequestMode::NoCors);

        let url = format!("http://127.0.0.1:3030/hello");

        let request = Request::new_with_str_and_init(&url, &opts).unwrap();

        request.headers().set("Accept", "text/plain").unwrap();

        let window = web_sys::window().unwrap();
        spawn_local(async move {
            let resp_value = JsFuture::from(window.fetch_with_request(&request))
                .await
                .unwrap();
            assert!(resp_value.is_instance_of::<Response>());
            let resp: Response = resp_value.dyn_into().unwrap();

            if let Some(resp_string) = resp.as_string() {
                println!("RESPONSE IS {:?}", resp_string);
            }
        });
    }
}
