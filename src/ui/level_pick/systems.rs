use crate::{resources::ApplicationState, ui::menu::main_menu::constants::get_text_style};
use bevy::prelude::*;

use super::components::*;

// TODO: вынести создание бандла и константы UI в общий файл
pub const BACK_BUTTON_TEXT_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

pub const DEFAULT_BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(280.0), Val::Px(80.0)),
    ..Style::DEFAULT
};

pub fn render_level_pick_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    level_pick_setup(&mut commands, &asset_server);
}

pub fn destroy_level_pick_screen(
    mut commands: Commands,
    level_pick_query: Query<Entity, With<LevelPick>>,
) {
    if let Ok(level_pick_entity) = level_pick_query.get_single() {
        commands.entity(level_pick_entity).despawn_recursive();
    }
}

fn level_pick_setup(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let texture_handle = asset_server.load("sprites/button.png");
    let level_pick_entity = commands
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
            LevelPick {},
        ))
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            });
            // Play Button
            spawn_back_button(parent, texture_handle.clone(), asset_server);
        })
        .id();

    return level_pick_entity;
}

fn spawn_back_button(
    object: &mut ChildBuilder,
    texture_handle: Handle<Image>,
    asset_server: &Res<AssetServer>,
) {
    object
        .spawn((
            ButtonBundle {
                image: UiImage {
                    texture: texture_handle.clone(),
                    ..default()
                },
                style: DEFAULT_BUTTON_STYLE,
                background_color: BACK_BUTTON_TEXT_COLOR.into(),
                ..default()
            },
            BackButton {},
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Back",
                        get_text_style(asset_server, 30.0, BACK_BUTTON_TEXT_COLOR),
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
        });
}

pub fn back_button_interaction(
    mut game_state: ResMut<NextState<ApplicationState>>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                game_state.set(ApplicationState::MainMenu);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn level_button_interaction(
    mut game_state: ResMut<NextState<ApplicationState>>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                game_state.set(ApplicationState::Game);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
