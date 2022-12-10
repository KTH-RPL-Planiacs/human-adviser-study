use bevy::prelude::*;

use crate::{AppState, FontAssets};

use super::{BUTTON_TEXT, HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};

#[derive(Component)]
pub struct MenuStartUI;

#[derive(Component)]
pub enum MenuStartBtn {
    Start,
}

#[derive(Component)]
pub struct ParticipantIdText;

pub fn setup_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    // ui camera
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(MenuStartUI);

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
                                value: format!("Press the START button to begin!\n"),
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
                .insert(MenuStartBtn::Start);
        })
        .insert(MenuStartUI);
}

pub fn btn_visuals(mut interaction_query: Query<(&Interaction, &mut UiColor), With<MenuStartBtn>>) {
    for (interaction, mut color) in interaction_query.iter_mut() {
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
    }
}

pub fn btn_listeners(
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<(&Interaction, &MenuStartBtn), Changed<Interaction>>,
) {
    for (interaction, btn) in interaction_query.iter_mut() {
        if let Interaction::Clicked = *interaction {
            match btn {
                MenuStartBtn::Start => {
                    state.set(AppState::Study).expect("Could not change state.");
                }
            }
        }
    }
}

pub fn cleanup_ui(query: Query<Entity, With<MenuStartUI>>, mut commands: Commands) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
