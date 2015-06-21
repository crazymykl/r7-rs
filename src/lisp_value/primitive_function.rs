use lisp_value::{LispValue, LispResult};
use std::rc::Rc;
use std::fmt;

pub type LispFunction = Rc<Fn(&[LispValue]) -> LispResult>;

#[derive(Clone)]
pub struct PrimitiveFunction {
    name: String,
    pub func: LispFunction
}

impl PrimitiveFunction {
    pub fn new(name: &str, function: LispFunction) -> PrimitiveFunction {
        PrimitiveFunction {
            name: name.to_string(),
            func: function
        }
    }
}

impl PartialEq for PrimitiveFunction {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for PrimitiveFunction {}

impl fmt::Debug for PrimitiveFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: <primitive function>", self.name)
    }
}
