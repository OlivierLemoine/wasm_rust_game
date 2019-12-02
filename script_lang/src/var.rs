use std::collections::BTreeMap;
use std::fmt;
// use std::ops::
use std::cmp::Ordering;

pub trait Cast<T> {
    fn cast(&self) -> T;
}

#[derive(Clone)]
pub enum Var__ {
    Null,
    Bool(bool),
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
impl From<&bool> for Var__ {
    fn from(b: &bool) -> Self {
        Var__::Bool(*b)
    }
}
impl From<Vec<&Var__>> for Var__ {
    fn from(v: Vec<&Var__>) -> Self {
        Var__::Array(v.iter().map(|&e: &&Var__| e.clone()).collect())
        // Var__::Array(v)
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
            Var__::Bool(b) => format!("{}", b),
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

impl Cast<f64> for Var__ {
    fn cast(&self) -> f64 {
        match self {
            Var__::Number(v) => *v,
            _ => panic!(),
        }
    }
}
impl Cast<f32> for Var__ {
    fn cast(&self) -> f32 {
        match self {
            Var__::Number(v) => *v as f32,
            _ => panic!(),
        }
    }
}
impl Cast<i64> for Var__ {
    fn cast(&self) -> i64 {
        match self {
            Var__::Number(v) => *v as i64,
            _ => panic!(),
        }
    }
}
impl Cast<i32> for Var__ {
    fn cast(&self) -> i32 {
        match self {
            Var__::Number(v) => *v as i32,
            _ => panic!(),
        }
    }
}
impl Cast<bool> for Var__ {
    fn cast(&self) -> bool {
        match self {
            Var__::Bool(b) => *b,
            _ => panic!(),
        }
    }
}
impl Cast<String> for Var__ {
    fn cast(&self) -> String {
        match self {
            Var__::String(s) => s.clone(),
            _ => panic!(),
        }
    }
}
impl Cast<Var__> for Var__ {
    fn cast(&self) -> Var__ {
        self.clone()
    }
}
impl PartialEq for Var__ {
    fn eq(&self, other: &Var__) -> bool {
        match (self, other) {
            (Var__::Null, Var__::Null) => true,
            (Var__::Bool(b1), Var__::Bool(b2)) => b1 == b2,
            (Var__::Number(n1), Var__::Number(n2)) => n1 == n2,
            (Var__::String(s1), Var__::String(s2)) => s1 == s2,
            (Var__::Array(a1), Var__::Array(a2)) => {
                a1.iter().zip(a2).map(|(e1, e2)| e1 == e2).all(|v| v)
            }
            (Var__::Object(o1), Var__::Object(o2)) => o1 == o2,
            _ => false,
        }
    }
}
impl PartialOrd for Var__ {
    fn partial_cmp(&self, other: &Var__) -> Option<Ordering> {
        match (self, other) {
            (Var__::Null, Var__::Null) => Some(Ordering::Equal),
            (Var__::Number(n1), Var__::Number(n2)) => n1.partial_cmp(n2),
            _ => None,
        }
    }
}
