use super::out::*;
use super::value::{valuetype::ValueType, Value};

// Implement operators for values. The values should be converted
// to the highest complex type of the operands.

impl Value {
    pub fn add(self, rhs: Self) -> EvalResult<Self> {
        // Maximum complexity between the two types, but cannot be lower than an int.
        let highest_complexity = ValueType::highest_complexity(vec![
            &self.to_type(),
            &rhs.to_type(),
            &ValueType::IntType,
        ]);
        let self_converted = self.as_type(&highest_complexity)?;
        let rhs_converted = rhs.as_type(&highest_complexity)?;

        Ok(match highest_complexity {
            ValueType::IntType => Value::Int(self_converted.as_int()? + rhs_converted.as_int()?),
            ValueType::FloatType => {
                Value::Float(self_converted.as_float()? + rhs_converted.as_float()?)
            }
            ValueType::ComplexType => {
                Value::Complex(self_converted.as_complex()? + rhs_converted.as_complex()?)
            }
            ValueType::VectorType => Value::Vector({
                let self_as_vector = self_converted.as_vector();
                let rhs_as_vector = rhs_converted.as_vector();
                let mut out_vector: Vec<Value> = vec![];

                if self_as_vector.len() != rhs_as_vector.len() {
                    return Err(ErrorType::MismatchedArrayLengths {
                        first: self_as_vector.len(),
                        second: rhs_as_vector.len(),
                        operation_name: "sum",
                    });
                } else {
                    let mut i = 0;
                    while i < self_as_vector.len() {
                        out_vector.push(
                            match Value::add(self_as_vector[i].clone(), rhs_as_vector[i].clone()) {
                                Err(err) => {
                                    return Err(ErrorType::ErrorDuring {
                                        operation_name: "sum of arrays",
                                        error: Box::new(err),
                                    })
                                }
                                Ok(value) => value,
                            },
                        );
                        i += 1;
                    }
                }
                out_vector
            }),
            // This arm will never be executed as ValueType::highest_complexity ensure that.
            _ => unimplemented!(),
        })
    }

    pub fn sub(self, rhs: Self) -> EvalResult<Self> {
        match Value::add(self, Value::mul(rhs, Value::Int(-1))?) {
            // Replace eventual error with the right operation name.
            Ok(value) => Ok(value),
            Err(err) => Err(match err {
                ErrorType::ErrorDuring {
                    operation_name: _,
                    error,
                } => ErrorType::ErrorDuring {
                    operation_name: "subtraction of arrays",
                    error,
                },
                ErrorType::MismatchedArrayLengths {
                    first,
                    second,
                    operation_name: _,
                } => ErrorType::MismatchedArrayLengths {
                    first,
                    second,
                    operation_name: "subtraction",
                },
                other => other,
            }),
        }
    }

    pub fn mul(self, rhs: Self) -> EvalResult<Self> {
        // Maximum complexity between the two types, but cannot be lower than an int.
        let highest_complexity = ValueType::highest_complexity(vec![
            &self.to_type(),
            &rhs.to_type(),
            &ValueType::IntType,
        ]);
        let self_converted = self.as_type(&highest_complexity)?;
        let rhs_converted = rhs.as_type(&highest_complexity)?;

        Ok(match highest_complexity {
            ValueType::IntType => Value::Int(self_converted.as_int()? * rhs_converted.as_int()?),
            ValueType::FloatType => {
                Value::Float(self_converted.as_float()? * rhs_converted.as_float()?)
            }
            ValueType::ComplexType => {
                Value::Complex(self_converted.as_complex()? * rhs_converted.as_complex()?)
            }
            ValueType::VectorType => Value::Vector({
                // TODO: add support for multiplication of a single number and a vector
                let self_as_vector = self_converted.as_vector();
                let rhs_as_vector = rhs_converted.as_vector();
                let mut out_vector: Vec<Value> = vec![];

                if self_as_vector.len() != rhs_as_vector.len() {
                    return Err(ErrorType::MismatchedArrayLengths {
                        first: self_as_vector.len(),
                        second: rhs_as_vector.len(),
                        operation_name: "multiplication",
                    });
                } else {
                    let mut i = 0;
                    while i < self_as_vector.len() {
                        out_vector.push(
                            match Value::mul(self_as_vector[i].clone(), rhs_as_vector[i].clone()) {
                                Err(err) => {
                                    return Err(ErrorType::ErrorDuring {
                                        operation_name: "multiplication of arrays",
                                        error: Box::new(err),
                                    })
                                }
                                Ok(value) => value,
                            },
                        );
                        i += 1;
                    }
                }
                out_vector
            }),
            // This arm will never be executed as ValueType::highest_complexity ensure that.
            _ => unimplemented!(),
        })
    }

    pub fn div(self, rhs: Self) -> EvalResult<Self> {
        // Maximum complexity between the two types, but cannot be lower than a float.
        let highest_complexity = ValueType::highest_complexity(vec![
            &self.to_type(),
            &rhs.to_type(),
            &ValueType::FloatType,
        ]);
        let self_converted = self.as_type(&highest_complexity)?;
        let rhs_converted = rhs.as_type(&highest_complexity)?;

        if highest_complexity != ValueType::VectorType {
            if rhs_converted.as_complex()?.re == 0.0 {
                return Err(ErrorType::DivideByZero {
                    numerator: self_converted,
                });
            }
        }

        Ok(match highest_complexity {
            ValueType::IntType => Value::Int(self_converted.as_int()? / rhs_converted.as_int()?),
            ValueType::FloatType => {
                Value::Float(self_converted.as_float()? / rhs_converted.as_float()?)
            }
            ValueType::ComplexType => {
                Value::Complex(self_converted.as_complex()? / rhs_converted.as_complex()?)
            }
            ValueType::VectorType => Value::Vector({
                let self_as_vector = self_converted.as_vector();
                let rhs_as_vector = rhs_converted.as_vector();
                let mut out_vector: Vec<Value> = vec![];

                if self_as_vector.len() != rhs_as_vector.len() {
                    return Err(ErrorType::MismatchedArrayLengths {
                        first: self_as_vector.len(),
                        second: rhs_as_vector.len(),
                        operation_name: "division",
                    });
                } else {
                    let mut i = 0;
                    while i < self_as_vector.len() {
                        out_vector.push(
                            match Value::div(self_as_vector[i].clone(), rhs_as_vector[i].clone()) {
                                Err(err) => {
                                    return Err(ErrorType::ErrorDuring {
                                        operation_name: "division of arrays",
                                        error: Box::new(err),
                                    })
                                }
                                Ok(value) => value,
                            },
                        );
                        i += 1;
                    }
                }
                out_vector
            }),
            // This arm will never be executed as ValueType::highest_complexity ensure that.
            _ => unimplemented!(),
        })
    }

    pub fn negate(self) -> EvalResult<Self> {
        let highest_complexity =
            ValueType::highest_complexity(vec![&self.to_type(), &ValueType::IntType]);
        let self_converted = self.as_type(&highest_complexity)?;
        match self_converted {
            Self::Bool(_) => Err(ErrorType::InternalError {
                message: "Failed type conversion in negate operator".to_owned(),
            }),
            other => Ok(Value::sub(Value::Int(0), other)?),
        }
    }

    pub fn aggregate(self, rhs: Self) -> Self {
        // Convert both values to a vector and concatenate them
        Value::Vector([self.as_vector(), rhs.as_vector()].concat())
    }
}
