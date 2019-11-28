use crate::builder::*;
use crate::components::*;
use math::Vec2;
use specs::prelude::*;

#[macro_export]
macro_rules! object {
    (
        Declare $struct_name:ident
        With [ $( { $($component_list:tt)* } )* ]
    ) => {
        mod $struct_name{
            use super::*;
            pub fn new(world: &mut World) -> EntityBuilder {
                world.create_entity()$( .with( unfold_component!( $($component_list)* ) ) )*
            }
        }
    };
}

macro_rules! unfold_component {
    (Transform) => {
        Transform::default()
    };
    (Transform offset=[$x:tt $y:tt]) => {
        TransformBuilder::new()
            .position(Vec2::from(($x as f64, $y as f64)))
            .build()
    };
    (Collisions) => {
        Collisions::default()
    };
}

#[macro_export]
macro_rules! logic {
    (
        // $($mutability:ident, )*
    ) => {
        struct $struct_name;
        impl<'a> System<'a> for $struct_name {
            type SystemData = ();
            fn run(&mut self, (): Self::SystemData) {}
        }
    };
}

macro_rules! get_storage {
    (static $storage:tt $($rest:tt)*) => {
        WriteStorage<'a, $storage>
    };
    (mut $storage:tt $($rest:tt)*) => {
        WriteStorage<'a, $storage>
    };
}

object! {
    Declare Test
    With [
        { Transform offset=[0 0] }
        { Collisions }
    ]
}
