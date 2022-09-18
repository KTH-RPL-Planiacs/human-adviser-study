use bevy::prelude::*;

use crate::{AppState, FontAssets};

use super::{
    BUTTON_TEXT, DISABLED_BUTTON, HOVERED_BUTTON, NORMAL_BUTTON, PART_ID_LEN, PRESSED_BUTTON,
};

#[derive(Component)]
pub struct MenuMainUI;

#[derive(Component)]
pub enum MenuMainBtn {
    Start,
}

pub struct ParticipantId(pub String);

#[derive(Component)]
pub struct ButtonEnabled(bool);

#[derive(Component)]
pub struct ParticipantIdText;

pub fn setup_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    // participant id resource
    commands.insert_resource(ParticipantId("".to_owned()));
    // ui camera
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(MenuMainUI);

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
            // part id display
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: format!("Please type your {}-digit ID!\n", PART_ID_LEN),
                                style: TextStyle {
                                    font: font_assets.default_font.clone(),
                                    font_size: 40.0,
                                    color: BUTTON_TEXT,
                                },
                            },
                            TextSection {
                                value: "".to_owned(),
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
                })
                .insert(ParticipantIdText);

            // start button
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(16.)),
                        padding: UiRect::all(Val::Px(16.)),
                        ..Default::default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::from_section(
                            "Start",
                            TextStyle {
                                font: font_assets.default_font.clone(),
                                font_size: 40.0,
                                color: BUTTON_TEXT,
                            },
                        ),
                        ..Default::default()
                    });
                })
                .insert(MenuMainBtn::Start)
                .insert(ButtonEnabled(false));
        })
        .insert(MenuMainUI);
}

pub fn update_part_id(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut lobby_id: ResMut<ParticipantId>,
) {
    let lid = &mut lobby_id.0;
    for ev in char_evr.iter() {
        if lid.len() < PART_ID_LEN && ev.char.is_ascii_digit() {
            lid.push(ev.char);
        }
    }
    if keys.just_pressed(KeyCode::Back) {
        let mut chars = lid.chars();
        chars.next_back();
        *lid = chars.as_str().to_owned();
    }
}

pub fn update_part_id_display(
    mut query: Query<&mut Text, With<ParticipantIdText>>,
    lobby_id: ResMut<ParticipantId>,
) {
    for mut text in query.iter_mut() {
        text.sections[1].value = lobby_id.0.clone();
    }
}

pub fn update_start_btn(
    text_query: Query<&Text, With<ParticipantIdText>>,
    mut btn_query: Query<&mut ButtonEnabled, With<MenuMainBtn>>,
) {
    let mut lobby_id_complete = false;
    for text in text_query.iter() {
        if text.sections[1].value.len() == PART_ID_LEN {
            lobby_id_complete = true;
            break;
        }
    }

    for mut enabled in btn_query.iter_mut() {
        enabled.0 = lobby_id_complete;
    }
}

pub fn btn_visuals(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, Option<&ButtonEnabled>),
        With<MenuMainBtn>,
    >,
) {
    for (interaction, mut color, enabled) in interaction_query.iter_mut() {
        let changeable = match enabled {
            Some(e) => e.0,
            None => true,
        };
        if changeable {
            match *interaction {
                Interaction::Clicked => {
                    *color = PRESSED_BUTTON.into();
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON.into();
                }
            }
        } else {
            *color = DISABLED_BUTTON.into();
        }
    }
}

pub fn btn_listeners(
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &MenuMainBtn, Option<&ButtonEnabled>),
        Changed<Interaction>,
    >,
) {
    for (interaction, btn, enabled) in interaction_query.iter_mut() {
        let clickable = match enabled {
            Some(e) => e.0,
            None => true,
        };

        if !clickable {
            continue;
        }

        if let Interaction::Clicked = *interaction {
            match btn {
                MenuMainBtn::Start => {
                    state.set(AppState::Study).expect("Could not change state.");
                }
            }
        }
    }
}

pub fn cleanup_ui(query: Query<Entity, With<MenuMainUI>>, mut commands: Commands) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
