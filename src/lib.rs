pub mod components;
mod systems;

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

pub struct Game;

#[cfg(debug_assertions)]
use colored::Colorize;

#[cfg(debug_assertions)]
macro_rules! file_setup {
    () => { 
        use std::path::Path;
        use std::fs;     
        let path: &Path = Path::new("assets");
        if !path.is_dir() {
            let result = fs::create_dir("assets");
            result.ok();
        }   
     }
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! log {
    ($( $args:expr ),*) => { 
        println!( $( $args ),* ); 
    }
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! log_warn {
    ($( $args:expr ),*) => { 
        println!( $( $args ).yellow(),* ); 
    }
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! log_alert {
    ($( $args:expr ),*) => { 
        println!( $( $args ).red(),* ); 
    }
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! log_green {
    ($( $args:expr ),*) => { 
        println!( $( $args ).green(),* ); 
    }
}


impl Game {

    pub fn new(window: WindowData) -> App {

        file_setup!();

        let mut app = App::new();

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
        app.add_system(systems::upd_lifetimes);
        return app;
    }

    /*#[cfg(debug_assertions)]
    pub fn file_setup() {
        use std::path::Path;
        use std::fs;
        
        println!("make file!");

        let path: &Path = Path::new("assets");
        if path.is_dir() {
            return;
        }
        
        let result = fs::create_dir("assets");
        result.ok();
    }
    */
}