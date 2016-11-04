pub trait Visitor<T> {
    fn visit_name(&mut self, val: &Name) -> T;
}

pub struct Name(String);