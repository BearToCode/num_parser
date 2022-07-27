//!
//! # A math interpreter and evaluator
//! Numcore allows you to easily parse strings into math expressions
//! and evaluate them.
//!
//! Numcore is part of [Numerus](https://github.com/BearToCode/Numerus).
//!
//! ## Features
//! * Binary and unary operators
//! * Supports multiple value types:
//!     * Bool,
//!     * Int,
//!     * Float,
//!     * [Complex](num::complex::Complex64),
//!     * Vector
//! * Built-in functions
//! * Built-in constants
//! * User-defined functions: `f(x,y) = xsin(y)+ysin(x)`
//! * User-defined var: `a = pi/2` or `b = a+2`
//! * Understands ambiguous syntax, like: `g(x) = pisinx`
//! * Serde support
//!
//! Much more will be implemented in future releases!
//!
//! ## Values
//! Values are contained inside the [Value enum](Value), which provides useful functions
//! to access the contained data:
//!
//! ```rust
//! use numcore::Value;
//!
//! let value = Value::Float(1.0);
//!
//! assert_eq!(value.as_bool().unwrap(), true);
//! assert_eq!(value.as_int().unwrap(), 1);
//! assert_eq!(value.as_float().unwrap(), 1.0);
//! assert_eq!(value.as_complex().unwrap(), num::complex::Complex::new(1.0, 0.0));
//! assert_eq!(value.as_vector(), vec![1.0]);
//! ```
//!
//! Note that, even thought the initial value was a float, it has been cast into ints and bools. This
//! was possible since the value had no decimal part and it was a one. If these conditions were not
//! met, the cast would have failed.
//!
//! ## Operators
//! Binary operators:
//!
//! | Operator | Description | Precedence |
//! |----------|-------------|------------|
//! | / | Division                                 | 40 |
//! | * | Multiplication                           | 40 |
//! | + | Sum                                      | 30 |
//! | - | Subtraction                              | 30 |
//! | , | Aggregation. Creates vectors             | 20 |
//! | = | Used for functions and vars declarations | 10 |
//!
//! Unary operators:
//!
//! | Operator | Description | Precedence |
//! |----------|-------------|------------|
//! | - | Negation | 30 |
//!
//! ## Functions
//!
//! | Function | Parameters Amount | Description                       |
//! |----------|-------------------|-----------------------------------|
//! | `sin`    | 1                 | Returns the sine of the angle.    |
//! | `cos`    | 1                 | Returns the cosine of the angle.  |
//! | `tan`    | 1                 | Returns the tangent of the angle. |
//!
//! ## Context
//!
//! [Contexts](Context) allows you keep track of user-defined functions and variables, as well
//! as settings. They can be created as follows:
//!
//! ```rust
//! use numcore;
//!
//! let mut my_context = numcore::Context::new();
//!
//! // Add a variable to the context
//! let res = numcore::eval_with_mutable_context("a = 2", my_context);
//!
//! assert_eq!(res, None);
//!
//! let res = numcore::eval_with_mutable_context("a", my_context);
//!
//! assert_eq!(res, Value::Int(2));
//! ```
//!
//! ### Serde
//!
//! ## License
//!
//!

extern crate num;

mod api;

mod context;
mod function;
mod interpreter;
mod objects;
mod operators;
mod out;
mod token;
mod tree;
mod value;

pub use crate::{
    api::*,
    context::*,
    out::*,
    value::{valuetype::*, Value},
};
