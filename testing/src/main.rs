use engine::prelude::*;
use engine::*;

logic! {
    System TestSystem
    Uses [
        mut Transform as transforms
    ]
    Does [
        Foreach [mut transforms as t] => {
            // t.position.x = 2.0;
            // a = 2;
            // a = 3;
            // print a;
        };
    ]
}

fn main() {
    __analyse_lang! {
        t = [];
        a = 2;
        a = 3;
    }
}