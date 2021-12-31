use gdnative::prelude::*;
use crate::loader::loader::load_texture;

/// Returns a new instanciated `TRef<Sprite>` with an asset as a GFX already loaded,
/// simplifiying the process of create a new sprite, load the asset as a resource
/// and setting it as a texture of the sprite in a single method call
pub fn sprite_with_asset(sprite_node_name: &str, path: &str) -> TRef<'static, Sprite> {
    // Creates a new TRef to a sprite
    let sprite = unsafe { Sprite::new().assume_shared().assume_safe() };
    // Loading the GFX assets
    let sprite_asset = load_texture(path);
    // Setting a name identifier (to Godot) as a node name identifier
    sprite.set_name(sprite_node_name);
    // Attaching it to the sprite
    sprite.set_texture(sprite_asset);
    // returning the sprite
    sprite
}