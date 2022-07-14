// use super::Function;
// use super::Type;

// pub struct Sine<'a> {
//     argument: &'a dyn Function,
// }

// impl Function for Sine<'_> {
//     fn fn_type(&self) -> Type {
//         Type::Default { name: "Sum" }
//     }
//     fn eval(&self, values: &std::collections::HashMap<&str, f64>) -> f64 {
//         self.argument.eval(values).sin()
//     }
// }

// pub struct Cosine<'a> {
//     argument: &'a dyn Function,
// }

// impl Function for Cosine<'_> {
//     fn fn_type(&self) -> Type {
//         Type::Default { name: "Cos" }
//     }
//     fn eval(&self, values: &std::collections::HashMap<&str, f64>) -> f64 {
//         self.argument.eval(values).cos()
//     }
// }

// pub struct Tangent<'a> {
//     argument: &'a dyn Function,
// }

// impl Function for Tangent<'_> {
//     fn fn_type(&self) -> Type {
//         Type::Default { name: "Tan" }
//     }
//     fn eval(&self, values: &std::collections::HashMap<&str, f64>) -> f64 {
//         self.argument.eval(values).tan()
//     }
// }
