//  * This file provides several traits that, when implemented on a `gdrust` object, 
//  * automatically provides some kind of **2D** movement, like normal 2D motion with keys, 
//  * with keys and orientation with mouse, Pokémon like tile-based movement...

use gdnative::prelude::*;


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
        player_config: CharacterConfiguration
    ) {
        // If there's Some(input) value inside self.input, unwrap it
        if let Some(input) = input_param { 
            // Control the vertical motion
            if Input::is_action_pressed(input, "move_up") 
                && !Input::is_action_pressed(input, "move_down")  {
                motion.y -= player_config.move_speed;
            }
            else if Input::is_action_pressed(input, "move_down") 
                && !Input::is_action_pressed(input, "move_up")  {
                motion.y += player_config.move_speed;
            }
            else {
                motion.y = 0.0;
            }

            // Control the horizontal motion
            if Input::is_action_pressed(input, "move_left") 
                && !Input::is_action_pressed(input, "move_right")  {
                motion.x -= player_config.move_speed;
            }
            else if Input::is_action_pressed(input, "move_right") 
                && !Input::is_action_pressed(input, "move_left")  {
                motion.x += player_config.move_speed;
            }
            else {
                motion.x = 0.0;
            }   
        }
    }
}