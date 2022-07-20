use super::{
    out::{ErrorType, EvalResult},
    value::Value,
};

pub struct Function {
    function: Box<dyn ClonableFunc>,
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Self {
            function: self.function.clone(),
        }
    }
}

impl Function {
    pub fn new<Func>(function: Func) -> Self
    where
        Func: Fn(&Value) -> EvalResult<Value>,
        Func: Send + Sync + 'static,
        Func: Clone,
    {
        Self {
            function: Box::new(function) as _,
        }
    }
}

trait ClonableFunc
where
    Self: Fn(&Value) -> EvalResult<Value>,
    Self: Send + Sync + 'static,
{
    fn clone(&self) -> Box<dyn ClonableFunc>;
}

impl<Func> ClonableFunc for Func
where
    Func: Fn(&Value) -> EvalResult<Value>,
    Func: Send + Sync + 'static,
    Func: Clone,
{
    fn clone(&self) -> Box<dyn ClonableFunc> {
        Box::new(self.clone()) as _
    }
}
