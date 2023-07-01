use bevy::prelude::*;

use crate::resources::GameState;

use super::components::*;

pub const NORMAL_BUTTON_TEXT_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn get_text_style(asset_server: &Res<AssetServer>, font_size: f32, color: Color) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/CyrillicPixel.ttf"),
        font_size: font_size,
        color: color,
    }
}

pub const DEFAULT_BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(280.0), Val::Px(80.0)),
    ..Style::DEFAULT
};

pub fn render_menu_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    pause_menu_setup(&mut commands, &asset_server);
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

fn spawn_resume_button(
    object: &mut ChildBuilder,
    texture_handle: Handle<Image>,
    asset_server: &Res<AssetServer>,
) {
    object
        .spawn((get_button_bundle(texture_handle), ResumeButton {}))
        .with_children(|parent| {
            parent.spawn(get_text_bundle("Resume", asset_server));
        });
}

pub fn pause_menu_setup(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let texture_handle = asset_server.load("sprites/button.png");

    let pause_menu_entity = commands
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
                background_color: Color::rgb(0.0, 0.5, 0.5).into(),
                transform: Transform::from_xyz(0.0, 0.0, 10.0),
                ..default()
            },
            PauseMenu {},
        ))
        .with_children(|parent| {
            // Resume Button
            spawn_resume_button(parent, texture_handle.clone(), asset_server);
        })
        .id();

    return pause_menu_entity;
}

pub fn destroy_menu_ui(mut commands: Commands, pause_menu_query: Query<Entity, With<PauseMenu>>) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn();
    }
}

pub fn resume_button_interaction(
    mut game_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ResumeButton>)>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                game_state.set(GameState::Game);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn settings_button_interaction(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<SettingsButton>)>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {}
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn set_pause_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if game_state.0 == GameState::Game {
            game_state_next_state.set(GameState::PauseMenu);
        } else if game_state.0 != GameState::PauseMenu {
            game_state_next_state.set(GameState::Game);
        }
    }
}
