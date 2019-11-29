use std::collections::BTreeMap;
use std::fmt;

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
    ($var:ident = { $($value:tt)* }; $($rest:tt)*) => {
        let mut $var = __parse_object!($($value)*);
        __analyse_lang!{$($rest)*}
    };
    ($var:ident = [ $($value:tt)* ]; $($rest:tt)*) => {
        let mut $var = __parse_array!($($value)*);
        __analyse_lang!{$($rest)*}
    };
    ($var:ident = $var2:ident $(.$attr:ident)*; $($rest:tt)*) => {
        let mut $var = $var2$(.get_nested__(String::from(stringify!($attr))))*;
        __analyse_lang!{$($rest)*}
    };
    ($var:ident = $value:expr; $($rest:tt)*) => {
        let mut $var = Var__::from(&$value);
        __analyse_lang!{$($rest)*}
    };
    ($var:tt $(.$attr:tt)* = { $($value:tt)* }; $($rest:tt)*) => {
        $var$(.get_nested__(String::from(stringify!($attr))))*.update_nested__(&__parse_object!($($value)*));
        __analyse_lang!{$($rest)*}
    };
    ($var:tt $(.$attr:tt)* = [ $($value:tt)* ]; $($rest:tt)*) => {
        $var$(.get_nested__(String::from(stringify!($attr))))*.update_nested__(&__parse_array!($($value)*));
        __analyse_lang!{$($rest)*}
    };
    ($var:tt $(.$attr:tt)* = $var2:ident $(.$attr2:ident)*; $($rest:tt)*) => {
        $var$(.get_nested__(String::from(stringify!($attr))))*.update_nested__(&$var2$(.get_nested__(String::from(stringify!($attr2))))*);
        __analyse_lang!{$($rest)*}
    };
    ($var:tt $(.$attr:tt)* = $value:expr; $($rest:tt)*) => {
        $var$(.get_nested__(String::from(stringify!($attr))))*.update_nested__(&Var__::from(&$value));
        __analyse_lang!{$($rest)*}
    };
    // (ext $var:tt $(.$attr:tt)* = $value:expr; $($rest:tt)*) => {
    //     $var $(.$attr)* = __expand_value!($value);
    //     __analyse_lang!{$($rest)*}
    // };
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
macro_rules! __parse_object {
    ($($key:ident : $($value:tt)*),*) => {
        Var__::Null
            $(
                .obj__(String::from(stringify!($key)),  &__parse_value!($($value)*))
            )*
    };
    () => {
        Var__::Null
    }
}

#[macro_export]
macro_rules! __parse_value {
    ([$($value:tt)*]) => {
        __parse_array!($($value)*)
    };
    ({$($value:tt)*}) => {
        __parse_object!($($value)*)
    };
    ($value:expr) => {
        Var__::from(&$value);
    };
}

#[derive(Clone)]
pub enum Var__ {
    Null,
    String(String),
    Number(f64),
    Object(BTreeMap<String, Var__>),
    Array(Vec<Var__>),
}
impl Var__ {
    pub fn obj__(mut self, key: String, val: &Var__) -> Var__ {
        match &mut self {
            Var__::Object(tree) => {
                tree.insert(key, val.clone());
            }
            _ => {
                let mut tree = BTreeMap::new();
                tree.insert(key, val.clone());
                self = Var__::Object(tree);
            }
        };
        self
    }
    pub fn get_nested__(&mut self, key: String) -> &mut Var__ {
        match self {
            Var__::Object(tree) => {
                if !tree.contains_key(&key) {
                    tree.insert(key.clone(), Var__::Null);
                }
                tree.get_mut(&key).unwrap()
            }
            _ => panic!(),
        }
    }
    pub fn update_nested__(&mut self, val: &Var__) {
        std::mem::replace(self, val.clone());
    }
}
impl From<&f64> for Var__ {
    fn from(v: &f64) -> Self {
        Var__::Number(*v as f64)
    }
}
impl From<&f32> for Var__ {
    fn from(v: &f32) -> Self {
        Var__::Number(*v as f64)
    }
}
impl From<&i64> for Var__ {
    fn from(v: &i64) -> Self {
        Var__::Number(*v as f64)
    }
}
impl From<&i32> for Var__ {
    fn from(v: &i32) -> Self {
        Var__::Number(*v as f64)
    }
}
impl From<&&str> for Var__ {
    fn from(s: &&str) -> Self {
        Var__::String(String::from(*s))
    }
}
impl From<&Var__> for Var__ {
    fn from(v: &Var__) -> Self {
        v.clone()
    }
}
impl fmt::Display for Var__ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = match self {
            Var__::Null => format!("null"),
            Var__::String(s) => format!("\"{}\"", s),
            Var__::Number(n) => format!("{}", n),
            Var__::Array(v) => format!(
                "[{}]",
                v.iter()
                    .map(|v| format!("{}", v))
                    .fold(String::new(), |a, c| format!("{}{},", a, c))
            ),

            Var__::Object(t) => format!(
                "{{{}}}",
                t.iter()
                    .map(|(key, value)| format!("{}: {}", key, value))
                    .fold(String::new(), |a, c| format!("{}{},", a, c))
            ),
        };
        write!(f, "{}", &res)
    }
}
