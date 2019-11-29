use engine::prelude::*;
use engine::*;

// logic! {
//     System TestSystem
//     Uses [
//         mut Transform as transforms
//     ]
//     Does [
//         Foreach [mut transforms as t] => {
//             // t.position.x = 2.0;
//             // a = 2;
//             // a = 3;
//             // print a;
//         };
//     ]
// }

pub struct Test{
    a: i32,
}

fn main() {
    let mut a = Test{
        a: 0,
    };
    __analyse_lang! {
        // b = a.a;
        ext a.a = 3;
    }
}
