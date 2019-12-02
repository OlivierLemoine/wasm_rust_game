#[macro_export]
macro_rules! object_builder {
    (
        Declare $struct_name:ident
        With [ $( { $($component_list:tt)* } )* ]
    ) => {
        mod $struct_name{
            use super::*;
            pub fn new(world: &mut World) -> EntityBuilder {
                world.create_entity()$( .with( __unfold_component!( $($component_list)* ) ) )*
            }
        }
    };
}

#[macro_export]
macro_rules! __unfold_component {
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
            $($mutability:ident $component:ident as $var:ident),*
        ]
        Does [$($code:tt)*]
    ) => {
        mashup! {
            $(
                m["var_" $var] = var_$var;
            )*
        }
        struct $struct_name;
        impl<'a> System<'a> for $struct_name {
            type SystemData = (
                $(__get_storage!($mutability $component))*
            );
            m!{
                fn run(&mut self, (
                    $(
                        __get_name!($mutability "var_" $var)
                    ),*
                ): Self::SystemData) {
                    for (
                        $($var),*
                    ) in (
                        $(
                            &__get_name!($mutability "var_" $var)
                        ),*
                    ).join() {
                        lang!{$($code)*}
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! __get_storage {
    (static $storage:ident) => {
        ReadStorage<'a, $storage>
    };
    (mut $storage:ident) => {
        WriteStorage<'a, $storage>
    };
    (global $storage:ident) => {
        Write<'a, $storage>
    }
}

#[macro_export]
macro_rules! __get_name {
    (mut $var:ident) => {
        mut $var
    };
    (static $var:ident) => {
        $var
    }
}

// logic! {
//     System TestSys
//     Uses [
//         Collisions as c,
//         Transform as t,
//         RigidBody as rb,
//         Player as p,
//         Sprite as s
//     ]
//     Does [
//     ]
// }
