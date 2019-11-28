use engine::prelude::*;
use engine::*;

logic! {
    System TestSystem
    Uses [
        mut Transform as transforms
    ]
    Does [
        Foreach;
        Foreach [
            mut transforms as t,
        ] => {

        };
    ]
}

fn main() {}
