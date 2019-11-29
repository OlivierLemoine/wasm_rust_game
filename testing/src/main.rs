// use engine::prelude::*;
// use engine::*;

#[macro_export]
macro_rules! lang {
    ($($rest:tt)*) => {
        __get_line!{[] $($rest)*}
    };
    () => {};
}

macro_rules! step {
    ($($rest:tt)*) => {
        stringify!($($rest)*);
        __get_line!{$($rest)*}
    };
}

macro_rules! __get_line {
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
    ($var:tt = $($rest:tt)*) => {
        let $var = 2
    };
    ($($rest:tt)*) => {
        stringify!($($rest)*)
    };
}

fn main() {
    lang!{
        test = 2;
        a = 3;
    }
}
