//  * This file provides several traits that, when implemented on a `gdrust` object, 
//  * automatically provides some kind of **2D** movement, like normal 2D motion with keys, 
//  * with keys and orientation with mouse, Pokémon like tile-based movement...

use gdnative::prelude::*;

use crate::config::{
    game_config::CharacterConfiguration,
    input_mapping::MotionKeybindings
};


/// A way to implement motion over a `KinematicBody2D`, controlling the 0 to MAX SPEED movement
/// with the keyboard's keys (provided by the Godot's Input singleton) and the player's orientation
/// (or where it's looking) control by tracking where the mouse it's located on the screen.
/// 
/// Method need to have an `input` Godot Singleton, in order to avoid the overhead fn call on every
/// `_process` or `_physics_process` iteration, or to avoi. 
pub trait KeysMotionMouseDirection {
    fn move_character(
        self: &mut Self,
        input_param: Option<&Input>,
        motion: &mut Vector2,
        // TODO The next two params should be contained on a bigger struct, 'cause the game config 
        // probably will increase the number of structs needed to hold it's various configurations.
        // So, having just one, for example, `game_config: GameConfiguration` struct that will hold
        // the inner ones w'd be better for the user when calling this method.
        // The drawback it's that you will have to fullfill every single config even if you don't have
        // or we can right it based in Rust optional type for initialization, and the drawback just 
        // will be having more fun unwraping data.
        player_config: CharacterConfiguration,
        keybinding: MotionKeybindings
    ) {
        // If there's Some(input) value inside self.input, unwrap it
        if let Some(input) = input_param { 
            // Control the vertical motion
            if Input::is_action_pressed( input, keybinding.get_up_keybinding() ) 
                && !Input::is_action_pressed( input, keybinding.get_down_keybinding() ) {
                motion.y -= player_config.get_move_speed();
            }
            else if Input::is_action_pressed( input, keybinding.get_down_keybinding() ) 
                && !Input::is_action_pressed( input, keybinding.get_up_keybinding() ) {
                motion.y += player_config.get_move_speed();
            }
            else {
                motion.y = 0.0;
            }

            // Control the horizontal motion
            if Input::is_action_pressed( input, keybinding.get_left_keybinding() ) 
                && !Input::is_action_pressed( input, keybinding.get_right_keybinding() ) {
                motion.x -= player_config.get_move_speed();
            }
            else if Input::is_action_pressed( input, keybinding.get_right_keybinding() ) 
                && !Input::is_action_pressed( input, keybinding.get_left_keybinding() ) {
                motion.x += player_config.get_move_speed();
            }
            else {
                motion.x = 0.0;
            }   
        }
    }
}