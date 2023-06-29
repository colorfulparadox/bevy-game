
use bevy::prelude::*;

#[derive(Debug)]
pub struct WindowData {
    title: String,
    width: f32,
    height: f32,
    resizeable: bool,
}

impl Default for WindowData {
    fn default() -> WindowData {
        WindowData {
            title: "Game".to_string(),
            width: 1200.,
            height: 800.,
            resizeable: true,
        }
    }
}

pub fn new_game(window: WindowData) -> App {
    let app = App::new();
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: window.title.into(),
                        resolution: (window.width, window.height).into(),
                        resizable: window.resizeable,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        );
     app
}
