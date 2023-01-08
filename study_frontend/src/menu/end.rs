use bevy::prelude::*;
use study_shared_types::GameResults;

use crate::FontAssets;

use super::BUTTON_TEXT;

pub fn setup_ui(mut commands: Commands, font_assets: Res<FontAssets>, result: Res<GameResults>) {
    // ui camera
    commands.spawn_bundle(Camera2dBundle::default());

    // root node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::all(Val::Px(0.)),
                flex_direction: FlexDirection::ColumnReverse,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // thank you message
            parent.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Thank you for participating!\n\n".to_owned(),
                            style: TextStyle {
                                font: font_assets.default_font.clone(),
                                font_size: 40.0,
                                color: BUTTON_TEXT,
                            },
                        },
                        TextSection {
                            value: format!("Your ID is:\n"),
                            style: TextStyle {
                                font: font_assets.default_font.clone(),
                                font_size: 40.0,
                                color: BUTTON_TEXT,
                            },
                        },
                        TextSection {
                            value: format!("{}\n\n", result.participant_id),
                            style: TextStyle {
                                font: font_assets.default_font.clone(),
                                font_size: 50.0,
                                color: Color::RED,
                            },
                        },
                        TextSection {
                            value: "Please return to the questionnaire and enter your ID."
                                .to_owned(),
                            style: TextStyle {
                                font: font_assets.default_font.clone(),
                                font_size: 40.0,
                                color: BUTTON_TEXT,
                            },
                        },
                    ],
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

pub fn send_study_data(result: Res<GameResults>) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        warn!("Not sending any data in native mode.");
        info!("Results: {:?}", result)
    }
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::{JsCast, JsValue};
        use wasm_bindgen_futures::{spawn_local, JsFuture};
        use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

        //const DATABASE_BACKEND_URL: &str = "http://127.0.0.1:3030/";
        const DATABASE_BACKEND_URL: &str = "https://study.gschup.dev/";

        info!("Sending Study Results...");

        // create request - hacky json encoding
        let result_json = result.to_json();
        let body = JsValue::from_str(&result_json);
        let headers = Headers::new().unwrap();
        headers.set("content-type", "application/json").unwrap();
        let mut opts = RequestInit::new();
        opts.method("POST")
            .mode(RequestMode::Cors)
            .body(Some(&body))
            .headers(&headers);
        let request = Request::new_with_str_and_init(DATABASE_BACKEND_URL, &opts).unwrap();

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
