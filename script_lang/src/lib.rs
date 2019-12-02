mod var;
pub use var::*;

#[macro_export]
macro_rules! lang {
    ($($rest:tt)*) => {
        __get_line!{[] $($rest)*}
    };
    () => {};
}

#[macro_export]
macro_rules! step {
    ($($rest:tt)*) => {
        stringify!($($rest)*);
        __get_line!{$($rest)*}
    };
}

#[macro_export]
macro_rules! __get_line {
    ([] if ($($cond:tt)*) {$($actions:tt)*} else {$($actions2:tt)*} $($rest:tt)*) => {
        if __parse_action!($($cond)*).cast() {
            __get_line!{[] $($actions)*}
        } else {
            __get_line!{[] $($actions2)*}
        }
        __get_line!{[] $($rest)*}
    };
    ([] if ($($cond:tt)*) {$($actions:tt)*} $($rest:tt)*) => {
        if $($cond)* {
            __get_line!{[] $($actions)*}
        }
        __get_line!{[] $($rest)*}
    };
    ([$($curr_line:tt)*] ; $($rest:tt)*) => {
        __parse_line!($($curr_line)*);
        __get_line!{[] $($rest)*}
    };
    ([$($curr_line:tt)*] $token:tt $($rest:tt)*) => {
        __get_line!{[$($curr_line)* $token] $($rest)*}
    };
    ([]) => {};
}

#[macro_export]
macro_rules! __parse_line {
    (get $var:ident$(.$attr:ident)* = $($rest:tt)*) => {
        $var$(.$attr)* = __parse_action!($($rest)*)
    };
    (ext $var:ident$(.$attr:ident)* = $($rest:tt)*) => {
        $var$(.$attr)* = __parse_action!($($rest)*).cast()
    };
    ($var:ident = $($rest:tt)*) => {
        let mut $var = __parse_action!($($rest)*)
    };
    ($var:ident$(.$attr:ident)* = $($rest:tt)*) => {
        $var$(.get_nested__(String::from(stringify!($attr))))*.update_nested__(&__parse_action!($($rest)*))
    };
    ($($rest:tt)*) => {
        __parse_action!($($rest)*)
    };
}

#[macro_export]
macro_rules! __parse_action {
    ($var1:tt$(.$attr1:tt)* == $var2:tt$(.$attr2:tt)*) => {
        Var__::from(&(__parse_action!($var1$(.$attr1)*) == __parse_action!($var2$(.$attr2)*)))
    };
    ([$($value:tt),*]) => {
        Var__::from(vec![
            $(&__parse_action!($value),)*
        ])
    };
    ({$($key:tt: $value:tt),*}) => {
        Var__::Null$(.obj__(String::from(stringify!($key)), &__parse_action!($value)))*
    };
    ($value:expr) => {
        Var__::from(&$value)
    };
    ($var:ident$(.$attr:ident)*) => {
        &$var$(.get_nested__(String::from(stringify!($attr))))*
    };
    (ext $var:ident$(.$attr:ident)*) => {
        Var__::from(&$var$(.$attr)*)
    }
}

fn test() {
    lang! {
        a = 2;

        if (a == 2) {
            get a = 3;
        } else {
            get a = 5;
        }
    }
}
