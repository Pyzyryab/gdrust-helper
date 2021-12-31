use gdnative::prelude::*;

mod loader;

pub use loader::loader as gdloader;

/// Just a health_checker to test if the project it's correctly
/// downloaded by Cargo from GitHub in another Godot-Rust project
pub fn health_check_from_github() {
    godot_print!("Hello, Godot-Rust from GitHub!")
}