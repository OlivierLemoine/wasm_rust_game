use crate::builder::*;
use crate::components::*;
use math::Vec2;
use specs::prelude::*;

#[macro_export]
macro_rules! logic {
    (
        Declare $struct_name:ident
        With [ $( { $component_mut:ident $component_name:ident is $($component_list:tt)* } )* ]
    ) => {
        #[derive(Default)]
        struct $struct_name;
        impl $struct_name {
            pub fn new(world: &mut World) -> EntityBuilder {
                world.create_entity()$( .with( unfold_component!( $($component_list)* ) ) )*
            }
        }
        impl<'a> System<'a> for $struct_name {
            type SystemData = (
                $(get_storage!($($component_mut $component_list)*),)*
                // Read<'a, engine::KeyPress>,
                // ReadStorage<'a, Collisions>,
                // WriteStorage<'a, Transform>,
                // WriteStorage<'a, RigidBody>,
                // WriteStorage<'a, Player>,
                // WriteStorage<'a, Sprite>,
            );
            fn run(
                &mut self,
                (
                    $(mut $component_name,)*
                ): Self::SystemData,
            ) {}
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
}

macro_rules! get_storage {
    (static $storage:tt $($rest:tt)*) => {
        WriteStorage<'a, $storage>
    };
    (mut $storage:tt $($rest:tt)*) => {
        WriteStorage<'a, $storage>
    };
}

logic! {
    Declare Test
    With [
        { mut transforms is Transform offset=[0 0] }
    ]
}
