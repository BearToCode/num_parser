use super::value::valuetype::ValueType;

pub enum EvalResult<T> {
    Ok(T),
    EvalError(ErrorType),
}

pub enum ErrorType {
    TypeError {
        expected: ValueType,
        given: ValueType,
    },
}
