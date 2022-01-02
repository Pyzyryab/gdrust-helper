//  * This file provides several traits that, when implemented on a `gdrust` object, 
//  * automatically provides some kind of **2D** movement, like normal 2D motion with keys, 
//  * with keys and orientation with mouse, Pokémon like tile-based movement...


/// A way to implement motion over a `KinematicBody2D`, controlling the 0 to MAX SPEED movement
/// with the keyboard's keys (provided by the Godot's Input singleton) and the player's orientation
/// (or where it's looking) control by tracking where the mouse it's located on the screen
pub trait KeysWithMouse {
    // TODO Pending impl
}