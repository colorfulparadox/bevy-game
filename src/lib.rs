
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

pub struct Game {
    app: App,
}

impl Game {
    pub fn new(window: WindowData) -> Game {
        let mut app = App::new();
        app
            .add_plugins(
                DefaultPlugins
                    .set(ImagePlugin::default_nearest())
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

        Game {
            app: app,
        }
    }

    pub fn run(&mut self) {
        self.app.run();
    }
}
