///  * This file provides several traits that, when implemented on a `gdrust` object, 
///  * automatically provides some kind of **2D** movement, like normal 2D motion with keys, 
///  * with keys and orientation with mouse, Pokémon like tile-based movement, 
///  * mechanical 2D simple motion...

use gdnative::prelude::*;

use crate::config::{
    game_config::CharacterConfiguration,
    input_mapping::MotionKeybindings
};


/// A way to implement motion over a `KinematicBody2D`, controlling the 0 to MAX SPEED movement
/// with the keyboard's keys (provided by the Godot's Input singleton) and the player's orientation
/// (or where it's looking) control by tracking where the mouse it's located on the screen.
/// 
/// Method needs to receive a couple of structs, one representing the actual player configuration
/// (like it's SPEED, MAX SPEED, FRICTION constant values) and another that will contains
/// constants that holds the keybindings configured on the Godot's editor
pub trait KeysMotionMouseDirection {
    fn move_character(
        self: &Self,
        // TODO The next two params should be contained on a bigger struct, 'cause the game config 
        // probably will increase the number of structs needed to hold it's various configurations.
        // So, having just one, for example, `game_config: GameConfiguration` struct that will hold
        // the inner ones w'd be better for the user when calling this method.
        // The drawback it's that you will have to fullfill every single config even if you don't have
        // or we can right it based in Rust optional type for initialization, and the drawback just 
        // will be having more fun unwraping data.
        player_config: CharacterConfiguration,
        keybinding: MotionKeybindings 
    ) -> Vector2 {

        // Allocate variables to track the actual motion based on an input received
        let input: &Input = Input::godot_singleton();
        let mut motion: Vector2 = Vector2::ZERO;

        // Control the vertical motion
        if Input::is_action_pressed( input, keybinding.up, false ) 
            && !Input::is_action_pressed( input, keybinding.down, false ) {
            motion.y -= player_config.move_speed;
        }
        else if Input::is_action_pressed( input, keybinding.down, false ) 
            && !Input::is_action_pressed( input, keybinding.up, false ) {
            motion.y += player_config.move_speed;
        }
        else {
            motion.y = 0.0;
        }

        // Control the horizontal motion
        if Input::is_action_pressed( input, keybinding.left, false ) 
            && !Input::is_action_pressed( input, keybinding.right, false ) {
            motion.x -= player_config.move_speed;
        }
        else if Input::is_action_pressed( input, keybinding.right, false ) 
            && !Input::is_action_pressed( input, keybinding.left, false ) {
            motion.x += player_config.move_speed;
        }
        else {
            motion.x = 0.0;
        }

        // Returns the result of the computation
        motion
    }
}