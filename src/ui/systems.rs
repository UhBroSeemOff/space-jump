use bevy::prelude::*;
use super::constants::*;

pub fn get_button_bundle(texture_handle: Handle<Image>) -> ButtonBundle {
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

pub fn get_text_bundle(button_text: &str, asset_server: &Res<AssetServer>) -> TextBundle {
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

pub fn spawn_button(
    object: &mut ChildBuilder,
    texture_handle: Handle<Image>,
    asset_server: &Res<AssetServer>,
    button_text: &str,
) {
    object
        // TODO: здесь прокинуть бандл компонент
        // .spawn((get_button_bundle(texture_handle), ResumeButton {}))
        .spawn(get_button_bundle(texture_handle))
        .with_children(|parent| {
            parent.spawn(get_text_bundle(button_text, asset_server));
        });
}
