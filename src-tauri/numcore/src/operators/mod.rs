use crate::{
    out::{ErrorType, EvalResult},
    value::{valuetype::ValueType, Value},
};

// Implement operators for values. The values should be converted
// to the highest complex type of the operands.

/// Convert values to a valid value and apply the operation.
fn convert_and_apply<T>(
    lhs: &Value,
    rhs: &Value,
    operation: &mut T,
    operation_name: &'static str,
    target_value_type: ValueType,
    inverse: bool,
) -> EvalResult<Value>
where
    T: FnMut(Value, Value) -> EvalResult<Value>,
{
    // Retrieve the highest complexity, so we will try to convert the output value back to that type
    let highest_complexity = ValueType::highest_complexity(vec![&lhs.to_type(), &rhs.to_type()]);

    // Convert to vectors
    let lhs_as_vector = lhs.as_vector();
    let rhs_as_vector = rhs.as_vector();

    fn apply_to_vector_and_number<T>(
        v: &Vec<Value>,
        n: &Value,
        inverse: bool,
        operation: &mut T,
        operation_name: &'static str,
        target_value_type: ValueType,
    ) -> EvalResult<Vec<Value>>
    where
        T: FnMut(Value, Value) -> EvalResult<Value>,
    {
        let mut out_v = vec![];

        for item in v {
            out_v.push(convert_and_apply(
                &item,
                &n,
                operation,
                operation_name,
                target_value_type,
                inverse,
            )?);
        }

        Ok(out_v)
    }

    fn apply_to_vectors<T>(
        lv: &Vec<Value>,
        rv: &Vec<Value>,
        inverse: bool,
        operation: &mut T,
        operation_name: &'static str,
        target_value_type: ValueType,
    ) -> EvalResult<Vec<Value>>
    where
        T: FnMut(Value, Value) -> EvalResult<Value>,
    {
        let joined = lv.iter().zip(rv.iter());

        let mut out_v = vec![];

        for (l_item, r_item) in joined {
            out_v.push(convert_and_apply(
                l_item,
                r_item,
                operation,
                operation_name,
                target_value_type,
                inverse,
            )?);
        }

        Ok(out_v)
    }

    if lhs_as_vector.len() == 1 && rhs_as_vector.len() == 1 {
        // Convert the values to the right type
        let lhs_converted = lhs.as_type(&target_value_type)?;
        let rhs_converted = rhs.as_type(&target_value_type)?;

        if inverse {
            Ok(operation(rhs_converted, lhs_converted)?.try_as_type(highest_complexity))
        } else {
            Ok(operation(lhs_converted, rhs_converted)?.try_as_type(highest_complexity))
        }
    } else if lhs_as_vector.len() == 1 {
        Ok(Value::Vector(apply_to_vector_and_number(
            &rhs_as_vector,
            lhs,
            !inverse,
            operation,
            operation_name,
            target_value_type,
        )?))
    } else if rhs_as_vector.len() == 1 {
        Ok(Value::Vector(apply_to_vector_and_number(
            &lhs_as_vector,
            rhs,
            inverse,
            operation,
            operation_name,
            target_value_type,
        )?))
    } else if lhs_as_vector.len() == rhs_as_vector.len() {
        if inverse {
            Ok(Value::Vector(apply_to_vectors(
                &rhs_as_vector,
                &lhs_as_vector,
                inverse,
                operation,
                operation_name,
                target_value_type,
            )?))
        } else {
            Ok(Value::Vector(apply_to_vectors(
                &lhs_as_vector,
                &rhs_as_vector,
                inverse,
                operation,
                operation_name,
                target_value_type,
            )?))
        }
    } else {
        Err(ErrorType::MismatchedArrayLengths {
            first: lhs_as_vector.len(),
            second: rhs_as_vector.len(),
            operation_name: operation_name,
        })
    }
}

impl Value {
    pub fn add(self, rhs: Self) -> EvalResult<Self> {
        convert_and_apply(
            &self,
            &rhs,
            &mut |lhs, rhs| Ok(Value::Complex(lhs.as_complex()? + rhs.as_complex()?)),
            "Sum",
            ValueType::ComplexType,
            false,
        )
    }

    pub fn sub(self, rhs: Self) -> EvalResult<Self> {
        convert_and_apply(
            &self,
            &rhs,
            &mut |lhs, rhs| Ok(Value::Complex(lhs.as_complex()? - rhs.as_complex()?)),
            "Subtraction",
            ValueType::ComplexType,
            false,
        )
    }

    pub fn mul(self, rhs: Self) -> EvalResult<Self> {
        convert_and_apply(
            &self,
            &rhs,
            &mut |lhs, rhs| Ok(Value::Complex(lhs.as_complex()? * rhs.as_complex()?)),
            "Multiplication",
            ValueType::ComplexType,
            false,
        )
    }

    pub fn div(self, rhs: Self) -> EvalResult<Self> {
        convert_and_apply(
            &self,
            &rhs,
            &mut |lhs, rhs| Ok(Value::Complex(lhs.as_complex()? / rhs.as_complex()?)),
            "Division",
            ValueType::ComplexType,
            false,
        )
    }

    pub fn negate(self) -> EvalResult<Self> {
        let zero = Value::Int(0);
        convert_and_apply(
            &zero,
            &self,
            &mut |lhs, rhs| Ok(Value::Complex(lhs.as_complex()? - rhs.as_complex()?)),
            "Negation",
            ValueType::ComplexType,
            false,
        )
    }

    pub fn exponentiation(self, rhs: Self) -> EvalResult<Self> {
        convert_and_apply(
            &self,
            &rhs,
            &mut |lhs, rhs| {
                let lhs_as_complex = lhs.as_complex()?;
                let rhs_as_complex = rhs.as_complex()?;

                // a^b = e^(b*ln(a))
                Ok(Value::Complex((rhs_as_complex * lhs_as_complex.ln()).exp()))
            },
            "Exponentiation",
            ValueType::ComplexType,
            false,
        )
    }

    pub fn modulo(self, rhs: Self) -> EvalResult<Self> {
        convert_and_apply(
            &self,
            &rhs,
            &mut |lhs, rhs| Ok(Value::Complex(lhs.as_complex()? % rhs.as_complex()?)),
            "Modulo",
            ValueType::ComplexType,
            false,
        )
    }

    pub fn less_than(self, rhs: Self) -> EvalResult<Self> {
        convert_and_apply(
            &self,
            &rhs,
            &mut |lhs, rhs| Ok(Value::Bool(lhs.as_float()? < rhs.as_float()?)),
            "Less than",
            ValueType::FloatType,
            false,
        )
    }

    pub fn greater_than(self, rhs: Self) -> EvalResult<Self> {
        convert_and_apply(
            &self,
            &rhs,
            &mut |lhs, rhs| Ok(Value::Bool(lhs.as_float()? > rhs.as_float()?)),
            "Greater than",
            ValueType::FloatType,
            false,
        )
    }

    pub fn less_or_equal_to(self, rhs: Self) -> EvalResult<Self> {
        convert_and_apply(
            &self,
            &rhs,
            &mut |lhs, rhs| Ok(Value::Bool(lhs.as_float()? <= rhs.as_float()?)),
            "Less or equal to",
            ValueType::FloatType,
            false,
        )
    }

    pub fn greater_or_equal_to(self, rhs: Self) -> EvalResult<Self> {
        convert_and_apply(
            &self,
            &rhs,
            &mut |lhs, rhs| Ok(Value::Bool(lhs.as_float()? >= rhs.as_float()?)),
            "Greater or equal to",
            ValueType::FloatType,
            false,
        )
    }

    pub fn logical_and(self, rhs: Self) -> EvalResult<Self> {
        convert_and_apply(
            &self,
            &rhs,
            &mut |lhs, rhs| Ok(Value::Bool(lhs.as_bool()? && rhs.as_bool()?)),
            "Logical AND",
            ValueType::BoolType,
            false,
        )
    }

    pub fn logical_or(self, rhs: Self) -> EvalResult<Self> {
        convert_and_apply(
            &self,
            &rhs,
            &mut |lhs, rhs| Ok(Value::Bool(lhs.as_bool()? || rhs.as_bool()?)),
            "Logical OR",
            ValueType::BoolType,
            false,
        )
    }

    pub fn equal_to(self, rhs: Self) -> EvalResult<Self> {
        convert_and_apply(
            &self,
            &rhs,
            &mut |lhs, rhs| Ok(Value::Bool(lhs.as_bool()? == rhs.as_bool()?)),
            "Equal to",
            ValueType::ComplexType,
            false,
        )
    }

    pub fn not_equal_to(self, rhs: Self) -> EvalResult<Self> {
        convert_and_apply(
            &self,
            &rhs,
            &mut |lhs, rhs| Ok(Value::Bool(lhs.as_bool()? != rhs.as_bool()?)),
            "Not equal to",
            ValueType::ComplexType,
            false,
        )
    }

    pub fn not(self) -> EvalResult<Self> {
        convert_and_apply(
            &self,
            &Value::Bool(true),
            &mut |lhs, rhs| Ok(Value::Bool(lhs.as_bool()? != rhs.as_bool()?)),
            "Not",
            ValueType::BoolType,
            false,
        )
    }
}
