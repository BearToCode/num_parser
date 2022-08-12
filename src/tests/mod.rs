use crate::*;
use num::complex::Complex64;

// VALUES

#[test]
fn int() {
    assert_eq!(eval("1").unwrap(), Value::from(1));
    assert_eq!(eval("0").unwrap(), Value::from(0));
    assert_eq!(eval("-1").unwrap(), Value::from(-1));
}

#[test]
fn bool() {
    assert_eq!(eval("1").unwrap(), Value::from(true));
    assert_eq!(eval("true").unwrap(), Value::from(true));
    assert_eq!(eval("0").unwrap(), Value::from(false));
    assert_eq!(eval("false").unwrap(), Value::from(false));
}

#[test]
fn float() {
    assert_eq!(eval("1.5").unwrap(), Value::from(1.5));
    assert_eq!(eval("-8.32").unwrap(), Value::from(-8.32));
}

#[test]
fn complex() {
    assert_eq!(eval("5+3i").unwrap(), Value::from(Complex64::new(5.0, 3.0)));
    assert_eq!(eval("i").unwrap(), Value::from(Complex64::new(0.0, 1.0)));
}

#[test]
fn vector() {
    assert_eq!(
        eval("(1,2,3)").unwrap(),
        Value::Vector(vec![Value::from(1), Value::from(2), Value::from(3)])
    );
    assert_eq!(
        eval("(-1,-2,-3)").unwrap(),
        Value::Vector(vec![Value::from(-1), Value::from(-2), Value::from(-3)])
    );
    assert_eq!(
        eval("((1,2,3),(4,5,6),(7,8,9))").unwrap(),
        Value::Vector(vec![
            Value::Vector(vec![Value::from(1), Value::from(2), Value::from(3),]),
            Value::Vector(vec![Value::from(4), Value::from(5), Value::from(6),]),
            Value::Vector(vec![Value::from(7), Value::from(8), Value::from(9),]),
        ])
    );
    assert_eq!(
        eval("(1,(2,(3,4)))").unwrap(),
        Value::Vector(vec![
            Value::from(1),
            Value::Vector(vec![
                Value::from(2),
                Value::Vector(vec![Value::from(3), Value::from(4)]),
            ]),
        ])
    );
}

// OPERATORS

#[test]
fn exponentiation() {
    assert_eq!(eval("2^3").unwrap(), Value::from(8));
    assert_eq!(eval("10^(-1)").unwrap(), Value::from(0.1));
    assert_eq!(eval("e^(pi*i)").unwrap(), Value::from(-1));
    assert_eq!(eval("(1,2,3)^2").unwrap(), Value::from(vec![1, 4, 9]));
    assert_eq!(eval("2^(1,2,3)").unwrap(), Value::from(vec![2, 4, 8]));
}

#[test]
#[should_panic]
fn exponentiation_panic() {
    eval("(1,2)^(1,2,3)").unwrap();
}

#[test]
fn division() {
    assert_eq!(eval("1/2").unwrap(), Value::from(0.5));
    assert_eq!(eval("-1/2/5").unwrap(), Value::from(-0.1));
    assert_eq!(eval("2+4i").unwrap(), Value::from(Complex64::new(2.0, 4.0)));
    assert_eq!(
        eval("1/(1,2,4)").unwrap(),
        Value::from(vec![1.0, 0.5, 0.25])
    );
}

#[test]
#[should_panic]
fn division_panic() {
    eval("(1,2)/(1,2,3)").unwrap();
}

#[test]
fn multiplication() {
    assert_eq!(eval("-1*1").unwrap(), Value::from(-1));
    assert_eq!(eval("1+2+3").unwrap(), Value::from(6));
    assert_eq!(
        eval("(3+4i)*(1+2i)").unwrap(),
        Value::from(Complex64::new(-5.0, 10.0))
    );
    assert_eq!(eval("1+(1,2,3)").unwrap(), Value::from(vec![2, 3, 4]));
}

#[test]
#[should_panic]
fn multiplication_panic() {
    eval("(1,2)*(1,2,3)").unwrap();
}

#[test]
fn modulo() {
    assert_eq!(eval("4%2").unwrap(), Value::from(0));
    assert_eq!(eval("5.5%1").unwrap(), Value::from(0.5));
    assert_eq!(
        eval("(14+7i)%(4+5i)").unwrap(),
        Value::from(Complex64::new(1.0, 1.0))
    );
    assert_eq!(eval("15%(2,5,7)").unwrap(), Value::from(vec![1, 0, 1]));
}

#[test]
#[should_panic]
fn modulo_panic() {
    eval("(1,2)%(1,2,3)").unwrap();
    eval("2%0").unwrap();
}

#[test]
fn sum() {
    assert_eq!(eval("1+1").unwrap(), Value::from(2));
    assert_eq!(eval("1+2+3").unwrap(), Value::from(6));
    assert_eq!(eval("1+(1,2,3)").unwrap(), Value::from(vec![2, 3, 4]));
}

#[test]
#[should_panic]
fn sum_panic() {
    eval("(1,2)+(1,2,3)").unwrap();
}

#[test]
fn subtraction() {
    assert_eq!(eval("1-2").unwrap(), Value::from(-1));
    assert_eq!(eval("1-2-3").unwrap(), Value::from(-4));
    assert_eq!(eval("1-(1,2,3)").unwrap(), Value::from(vec![0, -1, -2]));
}

#[test]
#[should_panic]
fn subtraction_panic() {
    eval("(1,2)-(1,2,3)").unwrap();
}

#[test]
fn less_than() {
    assert_eq!(eval("1<2").unwrap(), Value::from(true));
    assert_eq!(eval("0<1").unwrap(), Value::from(true));
    assert_eq!(eval("1<1").unwrap(), Value::from(false));
    assert_eq!(
        eval("1<(-4,2,1)").unwrap(),
        Value::from(vec![false, true, false])
    );
}

#[test]
#[should_panic]
fn less_than_panic() {
    eval("(1,2)<(1,2,3)").unwrap();
    eval("i<2").unwrap();
}

#[test]
fn greater_than() {
    assert_eq!(eval("1>2").unwrap(), Value::from(false));
    assert_eq!(eval("0>1").unwrap(), Value::from(false));
    assert_eq!(eval("1>1").unwrap(), Value::from(false));
    assert_eq!(
        eval("1>(-4,2,1)").unwrap(),
        Value::from(vec![true, false, false])
    );
}

#[test]
#[should_panic]
fn greater_than_panic() {
    eval("(1,2)>(1,2,3)").unwrap();
    eval("i>2").unwrap();
}

#[test]
fn less_or_equal_to() {
    assert_eq!(eval("1<=2").unwrap(), Value::from(true));
    assert_eq!(eval("0<=1").unwrap(), Value::from(true));
    assert_eq!(eval("1<=1").unwrap(), Value::from(true));
    assert_eq!(
        eval("1<=(-4,2,1)").unwrap(),
        Value::from(vec![false, true, true])
    );
}

#[test]
#[should_panic]
fn less_or_equal_to_panic() {
    eval("(1,2)<=(1,2,3)").unwrap();
    eval("i<=2").unwrap();
}

#[test]
fn greater_or_equal_to() {
    assert_eq!(eval("1>=2").unwrap(), Value::from(false));
    assert_eq!(eval("0>=1").unwrap(), Value::from(false));
    assert_eq!(eval("1>=1").unwrap(), Value::from(true));
    assert_eq!(
        eval("1>=(-4,2,1)").unwrap(),
        Value::from(vec![true, false, true])
    );
}

#[test]
#[should_panic]
fn greater_or_equal_to_panic() {
    eval("(1,2)>=(1,2,3)").unwrap();
    eval("i>=2").unwrap();
}

#[test]
fn equal_to() {
    assert_eq!(eval("1==2").unwrap(), Value::from(false));
    assert_eq!(eval("-1==1").unwrap(), Value::from(false));
    assert_eq!(eval("3==3").unwrap(), Value::from(true));
    assert_eq!(eval("(1,2,3)==(1,2,3)").unwrap(), Value::from(true));
}

#[test]
#[should_panic]
fn equal_to_panic() {
    eval("(1,2)==(1,2,3)").unwrap();
}

#[test]
fn not_equal_to() {
    assert_eq!(eval("1!=2").unwrap(), Value::from(true));
    assert_eq!(eval("-1!=1").unwrap(), Value::from(true));
    assert_eq!(eval("3!=3").unwrap(), Value::from(false));
    assert_eq!(eval("(1,2,3)!=(1,2,3)").unwrap(), Value::from(false));
}

#[test]
#[should_panic]
fn not_equal_to_panic() {
    eval("(1,2)!=(1,2,3)").unwrap();
}

#[test]
fn logical_and() {
    assert_eq!(eval("false&&true").unwrap(), Value::from(false));
    assert_eq!(eval("true&&false").unwrap(), Value::from(false));
    assert_eq!(eval("true&&true").unwrap(), Value::from(true));
    assert_eq!(eval("false&&false").unwrap(), Value::from(false));
}

#[test]
#[should_panic]
fn logical_and_panic() {
    eval("1.2&&true").unwrap();
}

#[test]
fn logical_or() {
    assert_eq!(eval("false||true").unwrap(), Value::from(true));
    assert_eq!(eval("true||false").unwrap(), Value::from(true));
    assert_eq!(eval("true||true").unwrap(), Value::from(true));
    assert_eq!(eval("false||false").unwrap(), Value::from(false));
}

#[test]
#[should_panic]
fn logical_or_panic() {
    eval("1.2||true").unwrap();
}

// VARS DECLARATIONS

#[test]
fn var() {
    let mut context = Context::default();

    eval_with_mutable_context("a = 2", &mut context).unwrap();

    assert_eq!(
        eval_with_static_context("a", &context).unwrap(),
        Value::from(2)
    );
}

#[test]
#[should_panic]
fn invalid_var_panic() {
    let mut context = Context::default();

    eval_with_mutable_context("ab = 2", &mut context).unwrap();
}

// FUNCTIONS DECLARATIONS

#[test]
fn func() {
    let mut context = Context::default();

    eval_with_mutable_context("f(x) = xcos(x)", &mut context).unwrap();

    assert_eq!(
        eval_with_static_context("f(pi)", &context).unwrap(),
        Value::from(-3.14159265)
    );
}

#[test]
#[should_panic]
fn unknown_func_panic() {
    let mut context = Context::default();

    eval_with_mutable_context("f(x) = g(x)", &mut context).unwrap();

    eval_with_static_context("f(1)", &context).unwrap();
}

#[test]
#[should_panic]
fn invalid_func_panic() {
    let mut context = Context::default();

    eval_with_mutable_context("f(x) g(x) = 2", &mut context).unwrap();
}

#[test]
fn recursion() {
    let mut context = Context::default();

    eval_with_mutable_context("f(x) = branch(x<=2, 1, f(x-1)+f(x-2))", &mut context).unwrap();

    assert_eq!(
        eval_with_static_context("f(10)", &context).unwrap(),
        Value::from(55)
    );
    eval_with_static_context("f(20)", &context).unwrap_err();
}

// FUNCTIONS

#[test]
fn min() {
    assert_eq!(
        eval("min(4,-1.5,5.344,2.7,-6,9.2)").unwrap(),
        Value::from(-6)
    );
}

#[test]
fn max() {
    assert_eq!(
        eval("max(4,-1.5,5.344,2.7,-6,9.2)").unwrap(),
        Value::from(9.2)
    );
}

#[test]
fn floor() {
    assert_eq!(eval("floor(1.34)").unwrap(), Value::from(1));
}

#[test]
fn ceil() {
    assert_eq!(eval("ceil(1.34)").unwrap(), Value::from(2));
}

#[test]
fn round() {
    assert_eq!(eval("round(1.2)").unwrap(), Value::from(1));
    assert_eq!(eval("round(1.5001)").unwrap(), Value::from(2));
    assert_eq!(eval("round(1.5)").unwrap(), Value::from(2));
}

#[test]
fn abs() {
    assert_eq!(eval("abs(1.5)").unwrap(), Value::from(1.5));
    assert_eq!(eval("abs(-1.5)").unwrap(), Value::from(1.5));
    assert_eq!(eval("abs(4+3i)").unwrap(), Value::from(5));
}

#[test]
fn sqrt() {
    assert_eq!(eval("sqrt(2)").unwrap(), eval("2^(0.5)").unwrap());
    assert_eq!(eval("sqrt(4+3i)").unwrap(), eval("(4+3i)^(0.5)").unwrap());
}

#[test]
fn ln() {
    assert_eq!(eval("ln(e^3.5)").unwrap(), Value::from(3.5));
    assert_eq!(eval("ln(e^(-1.5))").unwrap(), Value::from(-1.5));
    assert_eq!(
        eval("ln(-1)").unwrap(),
        Value::from(Complex64::new(0.0, 3.14159265))
    );
    assert_eq!(
        eval("ln(i)").unwrap(),
        Value::from(Complex64::new(0.0, 1.57079633))
    );
}

#[test]
fn log() {
    assert_eq!(eval("log(3, 9^2)").unwrap(), Value::from(4));
}

#[test]
fn exp() {
    assert_eq!(eval("exp(ln(4))").unwrap(), Value::from(4));
}

#[test]
fn branch() {
    assert_eq!(eval("branch(true, 1, 2)").unwrap(), Value::from(1));
    assert_eq!(eval("branch(false, 1, 2)").unwrap(), Value::from(2));
}

#[test]
fn sin() {
    let context = Context::new(
        settings::Rounding::default(),
        settings::AngleUnit::Degree,
        settings::DepthLimit::default(),
    );
    assert_eq!(
        eval_with_static_context("sin(30)", &context).unwrap(),
        Value::from(0.5)
    );
    assert_eq!(
        eval_with_static_context("sin(30)", &context).unwrap(),
        Value::from(0.5)
    );
}

#[test]
fn cos() {
    let context = Context::new(
        settings::Rounding::default(),
        settings::AngleUnit::Degree,
        settings::DepthLimit::default(),
    );
    assert_eq!(
        eval_with_static_context("cos(60)", &context).unwrap(),
        Value::from(0.5)
    );
}

#[test]
fn tan() {
    let context = Context::new(
        settings::Rounding::default(),
        settings::AngleUnit::Degree,
        settings::DepthLimit::default(),
    );
    assert_eq!(
        eval_with_static_context("tan(45)", &context).unwrap(),
        Value::from(1)
    );
}

#[test]
fn asin() {
    let context = Context::new(
        settings::Rounding::default(),
        settings::AngleUnit::Degree,
        settings::DepthLimit::default(),
    );
    assert_eq!(
        eval_with_static_context("asin(0.5)", &context).unwrap(),
        Value::from(30)
    );
}

#[test]
fn acos() {
    let context = Context::new(
        settings::Rounding::default(),
        settings::AngleUnit::Degree,
        settings::DepthLimit::default(),
    );
    assert_eq!(
        eval_with_static_context("acos(0.5)", &context).unwrap(),
        Value::from(60)
    );
}

#[test]
fn atan() {
    let context = Context::new(
        settings::Rounding::default(),
        settings::AngleUnit::Degree,
        settings::DepthLimit::default(),
    );
    assert_eq!(
        eval_with_static_context("atan(1)", &context).unwrap(),
        Value::from(45)
    );
}

#[test]
fn sinh_asinh() {
    assert_eq!(eval("asinh(sinh(1))").unwrap(), Value::from(1));
    assert_eq!(eval("asinh(sinh(pi))").unwrap(), Value::from(3.14159265));
}
#[test]
fn cosh_acosh() {
    assert_eq!(eval("acosh(cosh(1))").unwrap(), Value::from(1));
    assert_eq!(eval("acosh(cosh(pi))").unwrap(), Value::from(3.14159265));
}
#[test]
fn tanh_atanh() {
    // Requires less precision, otherwise the test would fail
    let context = Context::new(
        settings::Rounding::Round(4),
        settings::AngleUnit::default(),
        settings::DepthLimit::default(),
    );
    assert_eq!(
        eval_with_static_context("atanh(tanh(1))", &context).unwrap(),
        Value::from(1)
    );
    assert_eq!(
        eval_with_static_context("atanh(tanh(pi))", &context).unwrap(),
        Value::from(3.1416)
    );
}

#[test]
fn re() {
    assert_eq!(eval("re(1)").unwrap(), Value::from(1));
    assert_eq!(eval("re(3+2i)").unwrap(), Value::from(3));
}

#[test]
fn im() {
    assert_eq!(eval("im(1)").unwrap(), Value::from(0));
    assert_eq!(eval("im(3+2i)").unwrap(), Value::from(2));
}

#[test]
fn polar() {
    assert_eq!(
        eval("polar(1+sqrt(3)i)").unwrap(),
        Value::from(vec![
            2.0,
            1.04719755 // 30 deg
        ])
    );
}

#[test]
fn arg() {
    let context = Context::new(
        settings::Rounding::default(),
        settings::AngleUnit::Degree,
        settings::DepthLimit::default(),
    );
    assert_eq!(
        eval_with_static_context("arg(1+i)", &context).unwrap(),
        Value::from(0.78539816)
    );
}

#[test]
fn norm() {
    assert_eq!(eval("norm(3+4i)").unwrap(), Value::from(5));
}
