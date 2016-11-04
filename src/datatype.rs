#![allow(dead_code)]

use std::fmt;

// Reference ID counter.
static mut ref_id_counter: u32 = 0;

/// A data type.
#[derive(Debug, Clone)]
pub enum DataType {
    Nil,
    Void,
    Boolean(bool),
    Number(f64),
    StaticString(String),
    Function,
    Table,
    Tuple,
    UserData,
    Coroutine,
    TailCallRequest,
    YieldRequest,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.clone().to_error_type_string())
    }
}

impl DataType {
    /// Determines whether the specified data type can have type metatables.
    pub fn can_have_type_metatables(data_type: DataType) -> bool {
        match data_type {
            DataType::Nil |
            DataType::Void |
            DataType::Boolean(_) |
            DataType::Number(_) |
            DataType::StaticString(_) |
            DataType::Function => true,
            _ => false,
        }
    }

    /// Converts the data type to a string.
    /// Used by the `type(...)` Lua function.
    pub fn to_error_type_string(self) -> String {
        let result: String = match self {
            DataType::Void => format!("no value"),
            DataType::Tuple |
            DataType::TailCallRequest |
            DataType::YieldRequest => format!("internal<{:?}>", self),
            _ => format!("{:?}", self),
        };
        result.to_lowercase()
    }
}

/// A dynamic value.
#[derive(Debug, Clone)]
pub struct Value {
    pub ref_id: u32,
    pub read_only: bool,
    pub data_type: DataType,
    hash_code: i32,
}

impl Default for Value {
    fn default() -> Value {
        let ref_id: u32;
        unsafe {
            ref_id = ref_id_counter;
            ref_id_counter += 1;
        }
        Value {
            ref_id: ref_id,
            read_only: false,
            hash_code: -1,
            data_type: DataType::Void,
        }
    }
}

impl Value {
    fn new_nil() -> Value {
        Value { data_type: DataType::Nil, ..Default::default() }
    }
    fn new_boolean(val: bool) -> Value {
        Value { data_type: DataType::Boolean(val), ..Default::default() }
    }
    fn new_number(val: f64) -> Value {
        Value { data_type: DataType::Number(val), ..Default::default() }
    }
    fn new_string(val: String) -> Value {
        Value { data_type: DataType::StaticString(val), ..Default::default() }
    }
}