//! Contains everything you need to create your own built-in function.
//!
//! ## Creating your own function
//!
//! To create your own function declare your `fn` function and then convert it
//! to a `Function` struct. This module provides useful functions and macros to
//! easily create a simple one.
//!
//! ### Declaring and creating a function
//!
//! The first to do is declaring a function, you can do this using the `decl_func!`
//! macro.
//!
//! This macro takes three parameters:
//! - the function name,
//! - the function type,
//! - a predicate, which is the actual function that takes a value as input,
//! - the target type, a `ValueType`.
//!
//! Mark your function as `FunctionType::Trig` if it expects an angle.
//! Mark your function as `FunctionType::InverseTrig` if returns an angle if you
//! use `type_wrapper` (included in `decl_func!`).
//!
//! All angles are automatically converted to radians at function call.
//!
//! The predicate expects a `EvalResult<Value>` as output.
//!
//! Remember that functions name can only be alphabetic strings.
//!
//! Then use the `create_func!` macro to create a `Function` object. It takes
//! the function name and a `Arguments` as parameters. Then pass the created
//! object to `builtin::add_built_in_function`
//!
//!
//! ```
//! use num_parser::{*, function::*};
//!  
//! fn main() {
//!     // Declare the function
//!     decl_func!(
//!         // Function name
//!         addone,
//!         // Function type
//!         FunctionType::Std,
//!         // Predicate
//!         | v: Value | {
//!             Value::add(v, Value::from(1))
//!         },
//!         // The target type. This is the type at which the value will be converted to
//!         ValueType::ComplexType
//!     );
//!
//!     // Add the function to the built-in ones.
//!     builtin::add_built_in_function(
//!         create_func!(addone, Arguments::Const(1))
//!     );
//!
//!     assert_eq!(eval("addone(1)").unwrap(), Value::from(2));
//!
//! }
//! ```
//!
//! ### Reading multiple parameters
//!
//! You can read multiple parameters by specifying `ValueType::VectorType` as your
//! target type and using the `read_vec_values!` macro. It takes the input value as
//! first parameter, and the any other identifier will be assigned a value.
//!
//! ```
//! # use num_parser::{*, function:: *};
//! #
//! # fn main() {
//! #
//! decl_func!(
//!     // Function name
//!     addtriplet,
//!     // Function type
//!     FunctionType::Std,
//!     // Predicate
//!     | v: Value | {
//!         read_vec_values!(v, foo, bar, baz);
//!         // foo, bar, baz are automatically declared. They are `&Value` of
//!         // unknown type.
//!
//!         Value::add(
//!             Value::add(foo.clone(), bar.clone())?,
//!             baz.clone()
//!         )
//!     },
//!     // Use VectorType as target
//!     ValueType::VectorType
//! );
//!
//! builtin::add_built_in_function(
//!     create_func!(addtriplet, Arguments::Const(3))
//! );
//!
//! assert_eq!(eval("addtriplet(1,2,3)").unwrap(), Value::from(6));
//! # }
//! ```
//!
//! ### Creating a function with a dynamic argument counts
//!
//! You can create a function with a dynamic arguments count specifying
//! `Arguments::Dynamic` in the `create_func!` macro and by reading the arguments
//! manually.
//!
//! Example:
//! ```
//! # use num_parser::{*, function:: *};
//! #
//! # fn main() {
//! #
//! decl_func!(
//!     min,
//!     FunctionType::Std,
//!     |v: Value| {
//!         let vec = v.as_vector();
//!         let mut min = vec[0].as_float()?;
//!         for elem in vec {
//!             if elem.as_float()? < min {
//!                 min = elem.as_float()?;
//!             }
//!         }
//!         Ok(Value::Float(min))
//!     },
//!     ValueType::VectorType
//! );
//!
//! builtin::add_built_in_function(
//!     create_func!(min, Arguments::Dynamic)
//! );
//!
//! assert_eq!(eval("min(-2,3,8)").unwrap(), Value::from(-2));
//! #
//! #
//! # }
//! ```
//!

pub mod builtin;

use crate::{
    objects::Expression,
    out::{ErrorType, EvalResult},
    settings::{self, AngleUnit},
    value::{valuetype::ValueType, Value},
    Context,
};

/// A function object. You can pass this object to `builtin::add_built_in_function` to
/// make it available in all evaluations.
#[derive(Clone)]
pub struct Function {
    /// The identifier needed to call this function.
    pub func_identifier: &'static str,
    /// The actual function.
    pub func: fn(&Vec<Box<Expression>>, &Context, u32) -> EvalResult<Value>,
    /// The function arguments type.
    pub args: Arguments,
}

/// Contains the possible expected parameters for a function.
#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Arguments {
    /// Expects a constant number of arguments.
    Const(usize),
    /// Expects any amount greater than one.
    Dynamic,
}

/// The function type. Handles angle conversions.
#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FunctionType {
    /// A function that does not need conversion.
    Std,
    /// A function that expects an angle.
    Trig,
    /// A function that returns an angle.
    InverseTrig,
}

impl Function {
    /// Creates a new function with the specified data.
    pub fn new(
        func_identifier: &'static str,
        func: fn(&Vec<Box<Expression>>, &Context, u32) -> EvalResult<Value>,
        args: Arguments,
    ) -> Self {
        Self {
            func_identifier,
            func,
            args,
        }
    }

    /// Call a function.
    pub fn call(
        &self,
        arguments: &Vec<Box<Expression>>,
        context: &Context,
        scope: Option<&Context>,
        depth: u32,
    ) -> EvalResult<Value> {
        match self.args {
            Arguments::Const(count) => {
                if arguments.len() != count {
                    return Err(ErrorType::WrongFunctionArgumentsAmount {
                        func_name: self.func_identifier.to_owned(),
                        expected: count as u8,
                        given: arguments.len() as u8,
                    });
                }
            }
            _ => (),
        }

        let mut joined_context = context.clone();
        joined_context.join_with(context);
        if let Some(c) = scope {
            joined_context.join_with(c);
        }

        (self.func)(arguments, &joined_context, depth)
    }
}

/// A function wrapper around a predicate to handle types.
///
/// This function does essentially three things:
/// 1. Converts the input value to the target type.
/// 2. Executes the predicate and insert the output into a value.
/// 3. Tries to convert it back to the original value type, if it is
///    of lower complexity.
pub fn type_wrapper<P, T>(
    value: Value,
    func_type: FunctionType,
    target_type: ValueType,
    context: &Context,
    mut predicate: P,
) -> EvalResult<Value>
where
    P: FnMut(Value) -> EvalResult<T>,
    Value: From<T>,
{
    let original_type = value.get_type();

    let value = value.as_type(&target_type)?;
    // Input angle conversion
    let value = match func_type {
        FunctionType::Trig => {
            AngleUnit::convert_value(context.angle_unit, AngleUnit::Radian, value)?
        }
        _ => value,
    };

    let result = Value::from(predicate(value)?);
    // Output angle conversion
    let result = match func_type {
        FunctionType::InverseTrig => {
            AngleUnit::convert_value(AngleUnit::Radian, context.angle_unit, result)?
        }
        _ => result,
    };

    Ok(result.try_as_type(original_type))
}

/// Returns the output value(s) of the function arguments.
///
/// Arguments are actually expressions. This function calculates all of them and returns a `Value`,
/// which can be either a `VectorType` with all the values inside, or any other type if the argument
/// was just one.
pub fn unbox_parameters(
    arguments: &Vec<Box<Expression>>,
    context: &Context,
    depth: u32,
) -> EvalResult<Value> {
    Expression::Union(arguments.clone()).eval(context, None, depth)
}

/// Given a function name, a `FunctionType`, a predicate and a target `ValueType` declares a function. It generates
/// a wrapper that handles types and unbox parameters.
///
/// The predicate expects a `Value` as parameter and an `EvalResult<Value>` as output.
///
/// ## Examples
/// ```
/// // Remember to import everything under the `function` module.
/// use num_parser::{*, function::*};
///
/// decl_func!(
///     // Function name
///     hypotenuse,
///     // Function type
///     FunctionType::Std,
///     // Predicate
///     |v: Value| {
///         // Read the contained data using read_vec_values
///         read_vec_values!(v, a, b);
///         // Convert the data to the desired type
///         let a_as_number = a.as_float()?;
///         let b_as_number = b.as_float()?;
///
///         Ok(
///             Value::Float(
///                 a_as_number.powi(2) + b_as_number.powi(2)
///             )
///         )
///     },
///     // Expect a vector as input, as we need multiple parameters.
///     ValueType::VectorType
/// );
/// ```
///
/// ### Generated code
///
/// ```
/// use num_parser::{*, function::*};
///
/// fn hypotenuse(arguments: &Vec<Box<Expression>>, context: &Context, depth: u32) -> EvalResult<Value> {
///     let unboxed = unbox_parameters(arguments, context, depth)?;
///     type_wrapper(
///         unboxed,
///         FunctionType::Std,
///         ValueType::VectorType,
///         context,
///         |v: Value| {
///             read_vec_values!(v, a, b);
///
///             let a_as_number = a.as_float()?;
///             let b_as_number = b.as_float()?;
///     
///             Ok(
///                 Value::Float(
///                     a_as_number.powi(2) + b_as_number.powi(2)
///                 )
///             )
///         }    
///     )
/// }
/// ```
///
#[macro_export]
macro_rules! decl_func {
    ( $identifier:ident, $func_type:expr, $predicate:expr, $target:expr ) => {
        fn $identifier(
            arguments: &Vec<Box<Expression>>,
            context: &Context,
            depth: u32,
        ) -> EvalResult<Value> {
            let unboxed = unbox_parameters(arguments, context, depth)?;
            type_wrapper(unboxed, $func_type, $target, context, $predicate)
        }
    };
}

/// Given a `Value` of type `ValueType::VectorType` it declares variables with the provided names.
///
/// The generated code may return and `Err(ErrorType)` and should be executed in a function that expects
/// an `EvalResult<...>` as result.
///
/// ## Examples
/// ```
/// use num_parser::{*, function:: *};
///
/// fn read_values() -> EvalResult<()> {
///     
///     let vec_values = Value::Vector(vec![
///         Value::from(1),
///         Value::from(7),
///         Value::from(-9)
///     ]);
///     
///     read_vec_values!(vec_values, foo, bar, baz);
///     
///     assert_eq!(foo, &Value::from(1));
///     assert_eq!(bar, &Value::from(7));
///     assert_eq!(baz, &Value::from(-9));
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! read_vec_values {
    ( $vec:expr, $($x:ident),* ) => {
        let vec = $vec.as_vector();
        let mut iter = vec.iter();

        $(
            let $x = match iter.next() {
                Some(value) => value,
                None => return Err(ErrorType::InternalError { message: "failed to retrieve function parameters".to_owned() })
            };
        )*
    };
}

/// Creates a `Function` object from an actual function and an `Arguments` object.
///
/// See `Function::new` for additional information.
///
/// The generated `Function` has the same name as the provided function. The provided function
/// needs `&Vec<Box<Expression>>`, a `&Context` and a u32 (for depth controls) as parameters and
/// returns an `EvalResult<Values>`. You can easily declare one using the `decl_func!` macro.
///
/// ## Examples
/// ```
/// use num_parser::{*, function::*};
///
/// // Declare the function
/// decl_func!(
///     // Function name
///     hypotenuse,
///     // Function type
///     FunctionType::Std,
///     // Predicate
///     |v: Value| {
///         // ...
/// #        // Read the contained data using read_vec_values
/// #        read_vec_values!(v, a, b);
/// #        // Convert the data to the desired type
/// #        let a_as_number = a.as_float()?;
/// #        let b_as_number = b.as_float()?;
/// #
/// #        Ok(
/// #            Value::Float(
/// #                a_as_number.powi(2) + b_as_number.powi(2)
/// #            )
/// #        )
///     },
///     // Expect a vector as input, as we need multiple parameters.
///     ValueType::VectorType
/// );
///
/// let hyp_func = create_func!(hypotenuse, Arguments::Const(2));
///
/// builtin::add_built_in_function(hyp_func);
///
///
/// ```
#[macro_export]
macro_rules! create_func {
    ( $func:ident, $args:expr ) => {
        Function::new(stringify!($func), $func, $args)
    };
}

/// Converts angle units and executes the function. Useful for functions that take angles as inputs.
pub fn convert_angle_and_execute<P>(
    value: Value,
    from: settings::AngleUnit,
    to: settings::AngleUnit,
    mut predicate: P,
) -> EvalResult<Value>
where
    P: FnMut(Value) -> EvalResult<Value>,
{
    let converted = AngleUnit::convert_value(from, to, value)?;
    predicate(converted)
}

/// Executes the function and converts the output. Useful for functions that return angles.
pub fn execute_and_convert_angle<P>(
    value: Value,
    from: settings::AngleUnit,
    to: settings::AngleUnit,
    mut predicate: P,
) -> EvalResult<Value>
where
    P: FnMut(Value) -> EvalResult<Value>,
{
    let result = predicate(value)?;
    AngleUnit::convert_value(from, to, result)
}
