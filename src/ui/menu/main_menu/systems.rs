use bevy::app::AppExit;
use bevy::prelude::*;

use crate::resources::ApplicationState;

use super::components::*;
use super::constants::*;

pub fn render_menu_background() {
    println!("Spawn menu background");
}

pub fn add_menu_music() {
    println!("Start some music");
}

pub fn render_menu_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    main_menu_setup(&mut commands, &asset_server);
}

fn get_button_bundle(texture_handle: Handle<Image>) -> ButtonBundle {
    ButtonBundle {
        image: UiImage {
            texture: texture_handle.clone(),
            ..default()
        },
        style: DEFAULT_BUTTON_STYLE,
        background_color: NORMAL_BUTTON_TEXT_COLOR.into(),
        ..default()
    }
}

fn get_text_bundle(button_text: &str, asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![TextSection::new(
                button_text,
                get_text_style(asset_server, 30.0, NORMAL_BUTTON_TEXT_COLOR),
            )],
            alignment: TextAlignment::Center,
            ..default()
        },
        ..default()
    }
}

// TODO: Сделать функцию spawn_button, вынести её в сервисы
// понять, как выносить компонент, связующий бандл в аргумент monkaHmm

fn spawn_play_button(
    object: &mut ChildBuilder,
    texture_handle: Handle<Image>,
    asset_server: &Res<AssetServer>,
) {
    object
        .spawn((get_button_bundle(texture_handle), PlayButton {}))
        .with_children(|parent| {
            parent.spawn(get_text_bundle("Play", asset_server));
        });
}

fn spawn_settings_button(
    object: &mut ChildBuilder,
    texture_handle: Handle<Image>,
    asset_server: &Res<AssetServer>,
) {
    object
        .spawn((get_button_bundle(texture_handle), SettingsButton {}))
        .with_children(|parent| {
            parent.spawn(get_text_bundle("Settings", asset_server));
        });
}

fn spawn_exit_button(
    object: &mut ChildBuilder,
    texture_handle: Handle<Image>,
    asset_server: &Res<AssetServer>,
) {
    object
        .spawn((get_button_bundle(texture_handle), ExitButton {}))
        .with_children(|parent| {
            parent.spawn(get_text_bundle("Exit", asset_server));
        });
}

pub fn main_menu_setup(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let texture_handle = asset_server.load("sprites/button.png");
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    gap: Size::new(Val::Px(8.0), Val::Px(15.0)),
                    ..default()
                },
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Space jump",
                                get_text_style(asset_server, 60.0, TITLE_TEXT_COLOR),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        style: Style {
                            margin: UiRect {
                                bottom: Val::Px(30.0),
                                ..default()
                            },
                            ..default()
                        },
                        ..default()
                    });
                });
            // Play Button
            spawn_play_button(parent, texture_handle.clone(), asset_server);
            spawn_settings_button(parent, texture_handle.clone(), asset_server);
            spawn_exit_button(parent, texture_handle.clone(), asset_server);
        })
        .id();

    return main_menu_entity;
}

pub fn destroy_menu_background() {
    println!("Destroy menu background");
}

pub fn remove_menu_music() {
    println!("Remove menu music");
}

pub fn destroy_menu_ui() {
    println!("Destroy menu ui");
}

pub fn play_button_interaction(
    mut game_state: ResMut<NextState<ApplicationState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON_COLOR.into();
                game_state.set(ApplicationState::Game);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn exit_button_interaction(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ExitButton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
