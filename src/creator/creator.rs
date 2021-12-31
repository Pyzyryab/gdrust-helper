use gdnative::prelude::*;
use crate::loader::loader::load_texture;

/// Returns a new instanciated `TRef<Sprite>` with an asset already loaded,
/// simplifiying the process of create a new sprite, load the asset as a resource
/// and setting it as a texture of the sprite in a single method call
fn sprite_with_asset(path: &str) -> TRef<Sprite> {
    // Creates a new TRef to a sprite
    let sprite = unsafe { Sprite::new().assume_shared().assume_safe() };
    // Loading the GFX assets
    let sprite_asset = load_texture(path);
    // Attaching it to the sprite
    sprite.set_texture(sprite_asset);
    // returning the sprite
    sprite
}