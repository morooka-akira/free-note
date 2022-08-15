use std::rc::Rc;

use downcast_rs::{impl_downcast, Downcast};

pub type ObjectType = String;
pub const INTEGER_OBJ: &str = "INTEGER";
pub const BOOLEAN_OBJ: &str = "BOOLEAN";
pub const NULL_OBJ: &str = "NULL";
pub const RETURN_VALUE_OBJ: &str = "RETURN_VALUE";

pub trait Object: Downcast {
    fn obj_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}
impl_downcast!(Object);

// ------------------------------

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

// ------------------------------

pub struct Boolean {
    pub value: bool,
}

impl Object for Boolean {
    fn obj_type(&self) -> ObjectType {
        BOOLEAN_OBJ.to_string()
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

pub const TRUE: Boolean = Boolean { value: true };
pub const FALSE: Boolean = Boolean { value: false };

// ------------------------------

pub struct Null {}

impl Object for Null {
    fn obj_type(&self) -> ObjectType {
        NULL_OBJ.to_string()
    }

    fn inspect(&self) -> String {
        "null".to_string()
    }
}

pub const NULL: Null = Null {};

// ------------------------------
pub struct ReturnValue {
    pub value: Rc<dyn Object>,
}

impl Object for ReturnValue {
    fn obj_type(&self) -> ObjectType {
        RETURN_VALUE_OBJ.to_string()
    }
    fn inspect(&self) -> String {
        self.value.inspect()
    }
}
