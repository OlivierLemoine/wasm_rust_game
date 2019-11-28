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
        System $struct_name:ident
        Uses [
            $($mutability:ident $component:ident as $var:ident)*
        ]
        Does [$($code:tt)*]
    ) => {
        struct $struct_name;
        impl<'a> System<'a> for $struct_name {
            type SystemData = (
                $(get_storage!($mutability $component),)*
            );
            fn run(&mut self, (
                $(get_name!($mutability $var),)*
            ): Self::SystemData) {
                analyse_lang!{$($code)*}
            }
        }
    };
}

macro_rules! get_storage {
    (static $storage:ident) => {
        WriteStorage<'a, $storage>
    };
    (mut $storage:ident) => {
        WriteStorage<'a, $storage>
    };
}

macro_rules! get_name {
    (mut $var:ident) => {
        mut $var
    };
    (static $var:ident) => {
        $var
    }
}

macro_rules! analyse_lang {
    ($($rest:tt)*) => {
        stringify!($($rest)*);
    };
    (Foreach [
        $($mutability:ident $var:ident as $local_var:ident),*
    ] => { $($code:tt)* }; $($rest:tt)*) => {
        for (
            $($local_var),*
        ) in (
            $(&$mutability $var),*
        ).join(){
            analyse_lang!{$($code)*}
        }
        analyse_lang!{$($rest)*}
    };
}

object! {
    Declare test
    With [
        { Transform offset=[0 0] }
        { Collisions }
    ]
}

logic! {
    System TestSystem
    Uses [
        mut Transform as transforms
    ]
    Does [
        Foreach [
            mut transforms as t,
        ] => {

        };
    ]
}
