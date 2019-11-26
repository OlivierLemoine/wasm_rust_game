#![allow(non_snake_case)]

macro_rules! generate_kp {
    ($($key: ident),*) => {
        #[derive(Default)]
        pub struct KeyPress {
            $(
                $key: bool,
            )*
        }

        impl KeyPress {
            pub fn update_from_str(&mut self, val: &str, new_val: bool) {
                match val {
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

generate_kp!(KeyA, KeyD, Space, ShiftLeft);
