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
            $($component:ident as $var:ident),*
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
                $(WriteStorage<'a, $component>),*
            );
            m!{
                fn run(&mut self, (
                    $(
                        mut "var_" $var
                    ),*
                ): Self::SystemData) {
                    for (
                        $($var),*
                    ) in (
                        $(
                            &mut "var_" $var
                        ),*
                    ).join() {
                        lang!{$($code)*}
                    }
                }
            }
        }
    };
}
