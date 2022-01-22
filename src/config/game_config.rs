/// Provides a data holder to store the internal configuration 
/// of a character.
/// 
/// Is intented to be used with `in-game` constant values.
/// 
/// For example, the `move_speed` member could be initialized with some value like:
/// ```pub const MOVE_SPEED: f32 = 300.0``` 
/// passing the `MOVE_SPEED` constant to the `move_speed` member. Even if in a piece of 
/// code we modify (multiply, modify, substract...) the move speed value as:
/// ```
/// let character_config = CharacterConfiguration { MOVE_SPEED, 400 };
/// let move_speed_bonus = character_config.get_move_speed() * 2;
/// ```
/// the original value it's always a constant value.
pub struct CharacterConfiguration {
    pub move_speed: f32,
    pub max_speed: f32,
    pub friction: f32
}