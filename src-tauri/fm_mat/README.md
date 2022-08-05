
# fm_mat: a math interpreter and evaluator

[![crate](https://img.shields.io/crates/v/fm_mat)](https://crates.io/crates/fm_mat)
[![license](https://img.shields.io/github/license/BearToCode/Fermi)](https://github.com/BearToCode/Fermi/blob/master/src-tauri/fm_mat/LICENSE)
[![docs](https://img.shields.io/docsrs/fm_mat)](https://docs.rs/fm_mat/1.0.0/fm_mat/)

**fm_mat** allows you to easily **parse** strings into math expressions
and **evaluate** them.

fm_mat is part of [Fermi](https://github.com/BearToCode/Fermi).

## Features
* Binary and unary operators
* Supports **multiple value types**:
    * Bool,
    * Int,
    * Float,
    * [Complex](num::complex::Complex64),
    * Vector
* Built-in functions 
* Built-in constants
* **User-defined functions**: `f(x,y) = xsin(y)+ysin(x)`
* **User-defined var**: `a = pi/2` or `b = a+2`
* Define you own functions with **macros**.
* Understands **ambiguous syntax**, like: `g(x) = pisinx`
* **Recursion**: `f(x) = branch(x<=2, 1, f(x-1)+f(x-2))`
* Serde support
* No panicking

Much more will be implemented in future releases!

## Use Guide

Evaluating **simple static expressions**:
```rust
use fm_mat::*;

assert_eq!(eval("2+2").unwrap(), Value::from(4));
assert_eq!(eval("sin(pi)").unwrap(), Value::from(0));
assert_eq!(eval("re(10+3i)").unwrap(), Value::from(10));
```

Using **contexts**:

```rust
use fm_mat::*;

let mut context = Context::default();
// Declaring a function
let res = eval_with_mutable_context(
    "f(x) = branch(x<=2, 1, f(x-1) + f(x-2))",
    &mut context
).unwrap();

// Result is None
assert_eq!(res, None);
// Calling the function. We could just use eval_with_static_context at this point
let res = eval_with_mutable_context("f(10)", &mut context).unwrap();

assert_eq!(res, Some(Value::from(55)));
```

## Values
**Values** are contained inside the [Value enum](Value), which provides useful functions
to access the contained data:

```rust
use fm_mat::Value;

let value = Value::Float(1.0);

assert_eq!(value.as_bool().unwrap(), true);
assert_eq!(value.as_int().unwrap(), 1);
assert_eq!(value.as_float().unwrap(), 1.0);
assert_eq!(value.as_complex().unwrap(), num::complex::Complex::new(1.0, 0.0));
assert_eq!(value.as_vector(), vec![Value::Float(1.0)]);

// Assign type implicitly:
let implicit = Value::from(1.0);

assert_eq!(value, implicit);
```

Note that, even thought the initial value was a float, it has been **cast** into ints and bools. This
was possible since the value had no decimal part and it was a one. If these conditions were not
met, the cast would have failed.

## Operators
**Binary** operators:

| Operator | Description | Precedence |
|----------|-------------|------------|
| ^  | Exponentiation                                       | 90 |
| /  | Division                                             | 70 |
| *  | Multiplication                                       | 70 |
| %  | Modulo                                               | 70 |
| +  | Sum                                                  | 60 |
| -  | Subtraction                                          | 60 |
| <  | Less than                                            | 50 |
| >  | Greater than                                         | 50 |
| <= | Less or equal to                                     | 50 |
| >= | Greater or equal to                                  | 50 |
| == | Equal to                                             | 40 |
| != | Not equal to                                         | 40 |
| && | Logical AND                                          | 30 |
| &#124;&#124; | Logical OR                                 | 20 |
| ,  | Aggregation. Creates vectors                         | 10 |
| =  | Assignment. Used for functions and vars declarations | 0  |

**Unary** operators:

| Operator | Description | Precedence |
|----------|-------------|------------|
| ! | Logical NOT | 80 |
| - | Negation    | 60 |

## Functions

| Function | Parameters Amount          | Description                                                   |
|----------|----------------------------|---------------------------------------------------------------|
| `min`    | >=1                        | Returns the minimum value.                                    |
| `max`    | >=1                        | Returns the maximum value.                                    |
| `floor`  | 1                          | Returns the greatest lower integer.                           |
| `ceil`   | 1                          | Returns the lowest greater integer.                           |
| `round`  | 1                          | Returns the rounded integer.                                  |
| `ln`     | 1                          | Returns the natural log of the number.                        |
| `log`    | 2 (base, arg)              | Returns the logarithm of the number with the specified base.  |
| `exp`    | 1                          | Returns e^(arg).                                              |
| `rand`   | 2 (min, max)               | Returns a random float between the two number specified.      |
| `abs`    | 1                          | Returns the absolute value of a number.                       |
| `branch` | 3 (condition, true, false) | Returns the second argument if the condition is true, the third if it is false. |
| `sin`    | 1                          | Returns the sine of the angle.                                |
| `cos`    | 1                          | Returns the cosine of the angle.                              |
| `tan`    | 1                          | Returns the tangent of the angle.                             |
| `asin`   | 1                          | Returns the arcsine of the angle.                             |
| `acos`   | 1                          | Returns the arccosine of the angle.                           |
| `atan`   | 1                          | Returns the arctangent of the angle.                          |
| `sinh`   | 1                          | Returns the hyperbolic sine of the angle.                     |
| `cosh`   | 1                          | Returns the hyperbolic cosine of the angle.                   |
| `tanh`   | 1                          | Returns the hyperbolic tangent of the angle.                  |
| `asinh`  | 1                          | Returns the hyperbolic arcsine of the angle.                  |
| `acosh`  | 1                          | Returns the hyperbolic arccosine of the angle.                |
| `atanh`  | 1                          | Returns the hyperbolic arctangent of the angle.               |
| `re`     | 1                          | Returns the natural part of the number.                       |
| `im`     | 1                          | Returns the imaginary part of the number.                     |
| `polar`  | 1                          | Returns the polar form (r, theta) of the complex number.      |
| `arg`    | 1                          | Returns the principal arg of the number.                      |
| `norm`   | 1                          | Returns the length of the vector (re, im).                    |

## Context

[Contexts](Context) allows you keep track of **user-defined functions** and **variables**, as well
as settings. They can be created as follows:

```rust
use fm_mat::*;

let mut my_context = Context::new(settings::Rounding::Round(8));
```

### Serde

You can use the optional feature `serde_support` to let all the public structs
derive  [`Serialize`](https://docs.rs/serde/1.0.71/serde/trait.Serializer.html) and
[`Deserialize`](https://docs.rs/serde/1.0.71/serde/trait.Serializer.html).

```rust
[dependencies]
num = { version = "<version>", features = [ "serde_support" ] }
```

## License and contribution
Differently from Fermi, fm_mat is licensed under a **MIT License**.

fm_mat is currently under heavy development, but feel free to open issues and
pull requests for any problems or ideas you come up with.

**IMPORTANT**: after staging files use the `npm run commit` command to commit and then
follow the prompts. Use all **lowercase** and **imperative** descriptions. This
ensures that all commits follow the same format and help to keep a clean commit history.
