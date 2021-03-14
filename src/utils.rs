use gdnative::prelude::*;
use gdnative::{Ref, GodotObject};
use std::ops::Deref;

// pub fn find_level(node: &Node) -> Ref<Node> {
//     return node.find_parent(GodotString::from_str("Level"))
//         .expect("Should find Level");
// }

pub fn find_node<T>(owner: &Node, name: &str) -> Ref<T, Shared>
    where T: GodotObject
{
    owner
        .find_node(GodotString::from_str(name), false, false)
        .expect(format!("Failed to find `{}` node", name).deref())
        .to_variant()
        .try_to_object::<T>()
        .expect(format!("Failed to cast `{}` node", name).deref())
}