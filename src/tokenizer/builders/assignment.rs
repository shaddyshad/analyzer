use super::{PyEntity, EntityType};
use tendril::StrTendril;
use std::collections::HashMap;


#[derive(Debug)]
pub enum Variable {
    Local(StrTendril),
    Context(StrTendril),
    Object(StrTendril, StrTendril)
}

#[derive(Debug)]
pub enum FunctionCall {
    Function(StrTendril),
    MethodCall(StrTendril, StrTendril),
    ContextCall(StrTendril)
}

#[derive(Debug)]
pub enum ObjProp {
    Context(StrTendril),
    Object(StrTendril, StrTendril)
}

#[derive(Debug)]
pub enum Values {
    Str(StrTendril),
    Integer(u32),
    List(Vec<Values>),
    Dict(HashMap<StrTendril, Values>),
    Tuple(Vec<Values>),
    Float(f32),
    Call(FunctionCall),
    Property(ObjProp),
    Undefined
}


#[derive(Debug)]
pub struct Assignment {
    variable: Option<Variable>,
    value: Option<Values>,
    line_no: u32,
    depth: u32
}

impl Assignment {
    pub fn new(line_no: u32, depth: u32) -> Self {
        Self {
            line_no,
            depth,
            variable: None,
            value: None
        }
    }

    pub fn set_variable(&mut self, variable: Variable){
        self.variable = Some(variable);
    }

    pub fn set_value(&mut self, value: Values){
        self.value = Some(value);
    }
}

impl PyEntity for Assignment {
    fn get_type(&self) -> EntityType {
        EntityType::new("assignment")
    }

    fn get_line(&self) -> u32 {
        self.line_no
    }

    fn get_depth(&self) -> u32 {
        self.depth
    }
}

