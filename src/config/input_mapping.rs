/// Provides a way to store constant values that sets the
/// identifier of a keybing in the Godot editor.
/// 
/// For example, if you go under: 
/// ```
/// Project Configuration -> Input Map
/// ```
/// and you want to use the `W` key to move up in the screen along the `X`
/// axis your character, you will put it an identifier like ´up` or `move_up`
/// or whatever.
/// 
/// But when you need to access that value on code, you need to tell it to the 
/// `Input::godot_singleton()` what binding you are looking for.
/// 
/// So it seems obvious that you must declare some code constant to track
/// the literal that maps that binding created on the editor.
/// 
/// This struct provides a programatically way to store that information
/// in a safe way, and avoids the use of literals to the minimum, which is
/// when the constant value it's created.
/// 
/// 
/// The usage of example would be something like:
/// 
/// ```
/// File: `constants.rs`
/// 
/// pub const UP: &'static str = "move_up";
/// pub const DOWN: &'static str = "move_down";
/// pub const LEFT: &'static str = "move_left";
/// pub const RIGHT: &'static str = "move_right";
///
/// pub const MOTION_KEYBINDINGS: MotionKeybindings = 
///     MotionKeybindings {
///        up: MOVE_SPEED,
///        down: MAX_SPEED,
///        left: LEFT,
///        right: RIGHT
///     };
/// ```
/// 
/// This struct provides `read-only` access to it's data members.
pub struct MotionKeybindings<'a> {
    pub up: &'a str,
    pub down: &'a str,
    pub left: &'a str,
    pub right: &'a str,
}