use std::collections::BTreeMap;

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
            $($mutability:ident $component:ident as $var:ident)*
        ]
        Does [$($code:tt)*]
    ) => {
        struct $struct_name;
        impl<'a> System<'a> for $struct_name {
            type SystemData = (
                $(__get_storage!($mutability $component),)*
            );
            fn run(&mut self, (
                $(__get_name!($mutability $var),)*
            ): Self::SystemData) {
                __analyse_lang!{$($code)*}
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

#[macro_export]
macro_rules! __analyse_lang {
    (Foreach [
        $($mutability:ident $var:ident as $local_var:ident),*
    ] => { $($code:tt)* }; $($rest:tt)*) => {
        for (
            $($local_var),*
        ) in (
            $(&$mutability $var),*
        ).join(){
            __analyse_lang!{$($code)*}
        }
        __analyse_lang!{$($rest)*}
    };
    (print $var:ident; $($rest:tt)*) => {
        println!("{}", $var);
        __analyse_lang!{$($rest)*}
    };
    ($var:ident = [ $($value:tt),* ]; $($rest:tt)*) => {
        let mut $var = __parse_array!($($value),*);
        __analyse_lang!{$($rest)*}
    };
    ($var:ident = $value:expr; $($rest:tt)*) => {
        let mut $var = Var__::Number($value as f64);
        __analyse_lang!{$($rest)*}
    };
    ($var:tt $(.$attr:tt)* = $value:expr; $($rest:tt)*) => {
        $var $(.$attr)* = __expand_value!($value);
        __analyse_lang!{$($rest)*}
    };
    () => {};
}

#[macro_export]
macro_rules! __parse_array {
    ($($value:tt),*) => {
        Var__::Array(vec![
            $(__parse_value!($value)),*
        ])
    };
}

#[macro_export]
macro_rules! __parse_value {
    ([$($value:tt),*]) => {
        __parse_array!($($value),*)
    };
    ($value:expr) => {
        Var__::Number($value as f64)
    };
    () => {
        Var__::Null
    };
}

pub enum Var__ {
    Null,
    Number(f64),
    Object(BTreeMap<String, Var__>),
    Array(Vec<Var__>),
}
