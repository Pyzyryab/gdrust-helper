use gdnative::prelude::*;

mod loader;
mod creator;
mod motion;

pub use loader::loader as gdloader;
pub use creator::creator as gdcreator;
pub use motion::motion as gdmotion;

/// Just a health_checker to test if the project it's correctly
/// downloaded by Cargo from GitHub in another Godot-Rust project
pub fn health_check_from_github() {
    godot_print!("Hello, Godot-Rust from GitHub!")
}