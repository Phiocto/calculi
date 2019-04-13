//! # Calculi
//! 'calculi' is a crate used to algebraically solve equations with unknown variables for a given outcome.
//!
//! It is also able to solve equations when all unknown variables are given and perform other calculus functions.
//!
//! # Examples
//! ```
//! let eq1 = calculi::Equation::new("x - 2 * a + 4 ^ b");
//!
//! assert_eq!(eq1.solve_for(10.0, vec![("a", 4.5), ("b", 1.0)]).1, 15.0);
//!
//!
//! let eq2 = calculi::Equation::new("x + root(y, 3) + ln(exp(3))");
//!
//! assert_eq!(eq2.solve_with(vec![("x", 2.0), ("y", 27.0)]).to_float().unwrap(), 8.0);
//! 
//! 
//! let eq3 = calculi::Equation::new("x ^ 3");
//! 
//! assert_eq!(eq3.derive().text, "3 * x ^ 2")
//! ```
//! ---
//!
//! Look at [Equation](equation/struct.Equation.html) for all the equation functions.
//!
//! Look at [Component](component/enum.Component.html) for the component functions that can be used on the component which [Equation::solve_with](equation/struct.Equation.html#method.solve_with) returns.
//!
//! Look at [Operators](enum.Operator.html) for all available operators.

mod component;
mod derive;
mod equation;
mod operators;
mod parser;
mod utils;

pub use component::Component;
pub use equation::Equation;
pub use operators::Operator;

#[cfg(test)]
mod tests {
  use super::equation::Equation;

  #[test]
  fn it_works() {
    let eq = Equation::new("x - 2 * a + 4 ^ b");

    assert_eq!(
      eq.solve_with(vec![("x", 10.0), ("a", 4.5), ("b", 1.0)])
        .to_float()
        .unwrap(),
      5.0
    );
    assert_eq!(eq.solve_for(10.0, vec![("a", 4.5), ("b", 1.0)]).1, 15.0);
    assert_eq!(Equation::new("(16 + x) / 4").solve_for(8.0, vec![]).1, 16.0);
    assert_eq!(Equation::new("4 ^ x * 3").solve_for(192.0, vec![]).1, 3.0);
    assert_eq!(
      Equation::new("x * 0")
        .solve_with(vec![])
        .to_float()
        .unwrap(),
      0.0
    );

    println!("{:?}", Equation::new(""));
    println!("{:?}", Equation::new("x ^ 3").derive().text);
    println!("{:?}", Equation::new("x^sin(x)").derive().text);
  }
}
