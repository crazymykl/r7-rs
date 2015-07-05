use std::cell::RefCell;
use std::rc::Rc;

use lisp_value::{LispValue, LispResult};
use lisp_environment::LispEnvironment;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LispFunction {
    args: Vec<String>,
    varargs: Option<String>,
    body: Vec<LispValue>,
    closure: Rc<RefCell<LispEnvironment>>
}

impl LispFunction {
    pub fn new(env: &LispEnvironment, args: &[LispValue], body: &[LispValue]) -> LispFunction {
        LispFunction {
            args: args.iter().map(|x| x.to_string()).collect(),
            varargs: None,
            body: body.iter().cloned().collect(),
            closure: Rc::new(RefCell::new(env.clone()))
        }
    }

    pub fn new_with_varargs(env: &LispEnvironment, args: &[LispValue],
                            varargs: LispValue, body: &[LispValue]) -> LispFunction {
        LispFunction {
            args: args.iter().map(|x| x.to_string()).collect(),
            varargs: Some(varargs.to_string()),
            body: body.iter().cloned().collect(),
            closure: Rc::new(RefCell::new(env.clone()))
        }
    }

    pub fn arg_list(&self) -> String {
        let varargs = match self.varargs {
            Some(ref name) => format!(" . {}", name),
            None           => "".into()
        };

        format!("{}{}", self.args.join(", "), varargs)
    }

    pub fn call(&self, src_env: &LispEnvironment, params: &[LispValue]) -> LispResult {
        let mut env = src_env.merge(&self.closure.borrow());

        for (name, value) in self.args.iter().zip(params) {
            env.set(name, value.clone());
        }

        if let Some(ref name) = self.varargs {
            let values = params.iter()
                .skip(self.args.len())
                .cloned()
                .collect::<Vec<_>>();
            env.set(name, LispValue::List(values))
        }

        let (result, world) = env.eval_many(&self.body);
        let mut new_env = self.closure.borrow_mut();
        *new_env = world;
        result
    }

}
