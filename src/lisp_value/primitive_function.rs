use lisp_value::{LispValue, LispResult};
use std::rc::Rc;
use std::fmt;

pub type LispPrimitiveFunction = Rc<Fn(&[LispValue]) -> LispResult>;

#[derive(Clone)]
pub struct PrimitiveFunction {
    name: String,
    func: LispPrimitiveFunction
}

impl PrimitiveFunction {
    pub fn new(name: &str, function: LispPrimitiveFunction) -> PrimitiveFunction {
        PrimitiveFunction {
            name: name.to_string(),
            func: function
        }
    }

    pub fn call(&self, args: &[LispValue]) -> LispResult {
        (self.func)(args)
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
