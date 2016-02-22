use lisp_value::{LispValue, LispResult};
use lisp_environment::LispEnvironment;
use std::rc::Rc;
use std::fmt;

pub type LispPrimitiveFunction = Rc<Fn(&[LispValue]) -> LispResult>;

#[derive(Clone)]
pub struct PrimitiveFunction {
    name: String,
    args: Vec<String>,
    varargs: Option<String>,
    func: LispPrimitiveFunction
}

impl PrimitiveFunction {
    pub fn new(name: &str, args: &[&str], varargs: Option<String>,
               function: LispPrimitiveFunction) -> PrimitiveFunction {
        PrimitiveFunction {
            name: name.to_string(),
            args: args.iter().map(|&a| a.into()).collect(),
            varargs: varargs,
            func: function
        }
    }

    pub fn arg_list(&self) -> String {
        let mut args = self.args.clone();

        if let Some(ref varargs) = self.varargs {
            args.push(format!("{}...", varargs))
        };

        format!("{}", args.join(", "))
    }

    pub fn call(&self, _src_env: &LispEnvironment, args: &[LispValue]) -> LispResult {
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
