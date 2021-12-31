use gdnative::prelude::*;

/// Loads a Godot Node by path, being absolute to res://
/// 
/// Returns a temporary reference to a node `TRef<'static, Node>`.
/// 
/// An `&str` like `godot/my_folder/my_scene.tscn` provided on the argument
/// w'd be used to `res://godot/my_folder/my_scene.tscn`
pub fn load_node(path: &str) -> TRef<'static, Node> {
    // Concatenate the path to the Godot's root path identifier
    let path_from_res = &("res://".to_string() + path)[..];

    // Returning the Node
    return unsafe { ResourceLoader::godot_singleton()
        .load(path_from_res, "", false)
        .unwrap()
        .assume_safe()
        .cast::<PackedScene>()
        .unwrap()
        .instance(0)
        .unwrap()
        .assume_safe() 
    }
}


/// Loads a Godot Texture by path, being absolute to res://
/// 
/// Returns a reference to a node `Ref<Texture>`.
/// 
/// An `&str` like `godot/my_folder/my_scene.tscn` provided on the fn argument
/// w'd be used to `res://godot/my_folder/my_scene.tscn`
pub fn load_texture(path: &str) -> Ref<Texture> {
    // Concatenate the path to the Godot's root path identifier
    let path_from_res = &("res://".to_string() + path)[..];

    // Returning the Node
    return unsafe { ResourceLoader::godot_singleton()
        .load(path_from_res, "", false)
        .unwrap()
        .assume_safe()
        .cast::<Texture>()
        .unwrap()
        .assume_shared()
    };
}