use downcast_rs::{impl_downcast, Downcast};

pub type ObjectType = String;
pub const INTEGER_OBJ: &str = "INTEGER";

pub trait Object: Downcast {
    fn obj_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}
impl_downcast!(Object);

pub struct Integer {
    pub value: i64,
}

impl Object for Integer {
    fn obj_type(&self) -> ObjectType {
        INTEGER_OBJ.to_string()
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}
