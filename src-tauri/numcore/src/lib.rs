//!
//! # Numcore: a math interpreter and evaluator
//! **Numcore** allows you to easily **parse** strings into math expressions
//! and **evaluate** them.
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
//! * No panicking
//!
//! Much more will be implemented in future releases!
//!
//! ## Values
//! **Values** are contained inside the [Value enum](Value), which provides useful functions
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
//! assert_eq!(value.as_vector(), vec![Value::Float(1.0)]);
//! ```
//!
//! Note that, even thought the initial value was a float, it has been **cast** into ints and bools. This
//! was possible since the value had no decimal part and it was a one. If these conditions were not
//! met, the cast would have failed.
//!
//! ## Operators
//! **Binary** operators:
//!
//! | Operator | Description | Precedence |
//! |----------|-------------|------------|
//! | ^  | Exponentiation                                       | 90 |
//! | /  | Division                                             | 70 |
//! | *  | Multiplication                                       | 70 |
//! | %  | Modulo                                               | 70 |
//! | +  | Sum                                                  | 60 |
//! | -  | Subtraction                                          | 60 |
//! | <  | Less than                                            | 50 |
//! | >  | Greater than                                         | 50 |
//! | <= | Less or equal to                                     | 50 |
//! | >= | Greater or equal to                                  | 50 |
//! | == | Equal to                                             | 40 |
//! | != | Not equal to                                         | 40 |
//! | && | Logical AND                                          | 30 |
//! | &#124;&#124; | Logical OR                                 | 20 |
//! | ,  | Aggregation. Creates vectors                         | 10 |
//! | =  | Assignment. Used for functions and vars declarations | 0  |
//!
//! **Unary** operators:
//!
//! | Operator | Description | Precedence |
//! |----------|-------------|------------|
//! | ! | Logical NOT | 80 |
//! | - | Negation    | 60 |
//!
//! ## Functions
//!
//! | Function | Parameters Amount          | Description                                                   |
//! |----------|----------------------------|---------------------------------------------------------------|
//! | `min`    | >=1                        | Returns the minimum value.                                    |
//! | `max`    | >=1                        | Returns the maximum value.                                    |
//! | `floor`  | 1                          | Returns the greatest lower integer.                           |
//! | `ceil`   | 1                          | Returns the lowest greater integer.                           |
//! | `round`  | 1                          | Returns the rounded integer.                                  |
//! | `ln`     | 1                          | Returns the natural log of the number.                        |
//! | `log`    | 2 (base, arg)              | Returns the logarithm of the number with the specified base.  |
//! | `exp`    | 1                          | Returns e^(arg).                                              |
//! | `rand`   | 2 (min, max)               | Returns a random float between the two number specified.      |
//! | `branch` | 3 (condition, true, false) | Returns the sine of the angle.                                |
//! | `sin`    | 1                          | Returns the sine of the angle.                                |
//! | `cos`    | 1                          | Returns the cosine of the angle.                              |
//! | `tan`    | 1                          | Returns the tangent of the angle.                             |
//! | `asin`   | 1                          | Returns the arcsine of the angle.                             |
//! | `acos`   | 1                          | Returns the arccosine of the angle.                           |
//! | `atan`   | 1                          | Returns the arctangent of the angle.                          |
//! | `sinh`   | 1                          | Returns the hyperbolic sine of the angle.                     |
//! | `cosh`   | 1                          | Returns the hyperbolic cosine of the angle.                   |
//! | `tanh`   | 1                          | Returns the hyperbolic tangent of the angle.                  |
//! | `asinh`  | 1                          | Returns the hyperbolic arcsine of the angle.                  |
//! | `acosh`  | 1                          | Returns the hyperbolic arccosine of the angle.                |
//! | `atanh`  | 1                          | Returns the hyperbolic arctangent of the angle.               |
//! | `re`     | 1                          | Returns the natural part of the number.                       |
//! | `im`     | 1                          | Returns the imaginary part of the number.                     |
//! | `polar`  | 1                          | Returns the polar form (r, theta) of the complex number.      |
//! | `arg`    | 1                          | Returns the principal arg of the number.                      |
//! | `norm`   | 1                          | Returns the length of the vector (re, im).                    |
//!
//! ## Context
//!
//! [Contexts](Context) allows you keep track of **user-defined functions** and **variables**, as well
//! as settings. They can be created as follows:
//!
//! ```rust
//! use numcore;
//!
//! let mut my_context = numcore::Context::new();
//!
//! // Add a variable to the context
//! let res = numcore::eval_with_mutable_context("a = 2", &mut my_context);
//!
//! assert_eq!(res, None);
//!
//! // Read the variable
//! let res = numcore::eval_with_mutable_context("a", &mut my_context);
//!
//! assert_eq!(res, Value::Int(2));
//! ```
//!
//! ### Serde
//!
//! You can use the optional feature `serde_support` to let all the public structs
//! derive  [`Serialize`](https://docs.rs/serde/1.0.71/serde/trait.Serializer.html) and
//! [`Deserialize`](https://docs.rs/serde/1.0.71/serde/trait.Serializer.html).
//!
//! ```rust
//! [dependencies]
//! num = { version = "<version>", features = [ "serde_support" ] }
//! ```
//!
//! ## License and contribution
//! Differently from Numerus, Numcore is licensed under a **MIT License**.
//!
//! Numcore is currently under heavy development, but feel free to open issues and
//! pull requests for any problems or ideas you come up with.
//!
//! **IMPORTANT**: after staging files use the `npm run commit` command to commit and then
//! follow the prompts. Use all **lowercase** and **imperative** descriptions. This
//! ensures that all commits follow the same format and help to keep a clean commit history.
//!

#[cfg(feature = "serde_support")]
extern crate serde;

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
