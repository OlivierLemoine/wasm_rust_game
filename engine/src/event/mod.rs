macro_rules! generate_kp {
    ($($key: ident),*) => {
        #[derive(Default)]
        pub struct KeyPress {
            $($key: bool,)*
        }

        impl KeyPress {
            pub fn update_from_str(&mut self, val: &str, new_val: bool) {
                match val {
                    "w" => self.w = new_val,
                    $(stringify!{$key} => self.$key = new_val,)*
                    _ => {}
                }
            }

            $(
                pub fn $key(&self) -> bool {
                    self.$key
                }
            )*
        }
    };
}

generate_kp!(w, a, s, d);

// #[derive(Default)]
// pub struct KeyPress {
//     w: bool,
//     a: bool,
//     s: bool,
//     d: bool,
// }

// impl KeyPress {
//     pub fn update_from_str(&mut self, val: &str, new_val: bool) {
//         match val {
//             "w" => self.w = new_val,
//             "a" => self.a = new_val,
//             "s" => self.s = new_val,
//             "d" => self.d = new_val,
//             _ => {}
//         }
//     }
// }
