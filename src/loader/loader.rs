use gdnative::prelude::*;

/// Loads a Godot resource (a scene, an asset...) from a given path
/// absolute to res://
/// 
/// Returns a temporary reference to a node `TRef<'static, Node>`.
/// 
/// An `&str` like `godot/my_folder/my_scene.tscn` provided on the argument
/// w'd be used to `res://godot/my_folder/my_scene.tscn`
pub fn load_resource(path: &str) -> TRef<'static, Node> {
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