use std::collections::HashMap;

use super::component::{Component, Prec};
use super::derive;
use super::operators::{Operator, Operator::*};
use super::parser;

/// The equation struct containing the equation text and the parsed component.Component.
///
/// Various functions can be executed on this equation to solve it or it's variables.

#[derive(Debug)]
pub struct Equation {
  /// The equation in string form
  pub text: String,
  /// The equation in a component tree
  pub expression: Component,
}

impl From<Component> for Equation {
  fn from(expression: Component) -> Self {
    Equation {
      text: expression.to_string(),
      expression,
    }
  }
}

impl Equation {
  /// Creates a new equation from an equation in string form
  ///
  /// # Examples
  /// ```
  /// let eq = calculi::Equation::new("a * sqrt(x + 1)");
  /// ```
  pub fn new<T: Into<String>>(text: T) -> Equation {
    let text = text.into();
    let expression = Self::solve_component(&HashMap::new(), &parser::parse(&text));
    Equation { text, expression }
  }

  // fn single_unknown_variable(epxr: &Component) -> bool {
  //   false
  // }

  // fn brute_force_solution(expr: &Component, outcome: Prec) -> Option<(Component, Prec)> {
  //   if Self::single_unknown_variable(expr) {
  //     return None;
  //   }

  //   None
  // }

  fn apply_function(operator: &Operator, values: &[Component]) -> Option<Component> {
    if values.is_empty() {
      return None;
    }

    // Unary operators
    if values.len() == 1 {
      if let Component::Number(f) = &values[0] {
        return Some(match operator {
          Sin => Component::Number(f.sin()),
          Cos => Component::Number(f.cos()),
          Tan => Component::Number(f.tan()),
          Sec => Component::Number(1.0 / f.cos()),
          Csc => Component::Number(1.0 / f.sin()),
          Cot => Component::Number(1.0 / f.tan()),
          Abs => Component::Number(f.abs()),
          Floor => Component::Number(f.floor()),
          Round => Component::Number(f.round()),
          Ceil => Component::Number(f.ceil()),
          Exp => Component::Number(f.exp()),
          Ln => Component::Number(f.ln()),
          Sqrt => Component::Number(f.sqrt()),
          _ => return None,
        });
      }
    }

    // Apply binary operator to components if they are both numbers
    if values.len() == 2 {
      if let (Component::Number(f1), Component::Number(f2)) = (&values[0], &values[1]) {
        match operator {
          Add => return Some(Component::Number(f1 + f2)),
          Subtract => return Some(Component::Number(f1 - f2)),
          Multiply => return Some(Component::Number(f1 * f2)),
          Divide => return Some(Component::Number(f1 / f2)),
          Modulo => return Some(Component::Number(f1 % f2)),
          Exponent | Pow => return Some(Component::Number(f1.powf(*f2))),
          Log => return Some(Component::Number(f1.log(*f2))),
          Root => return Some(Component::Number(f1.powf(1.0 / *f2))),
          _ => (),
        }
      }
    }

    None
  }

  // Attempt to solve component with given variables
  fn solve_component(vars: &HashMap<&str, Prec>, component: &Component) -> Component {
    match component {
      // Attempt to retrieve variable value
      Component::Variable(c) => {
        if vars.contains_key(c.as_str()) {
          Component::Number(vars[c.as_str()])
        } else {
          Component::Variable(c.to_string())
        }
      }

      Component::Number(f) => Component::Number(*f),

      // Attempt to solve binary component
      Component::Function { operator, values } => {
        // Retrieve value of left and right component
        let values: Vec<_> = values
          .iter()
          .map(|x| Self::solve_component(vars, x))
          .collect();
        
        let solved = Self::apply_function(&operator, &values);

        if solved.is_some() {
          solved.unwrap()
        } else {
          // Return original binary component if simplifying failed
          Component::Function {
            operator: operator.clone(),
            values,
          }
        }
      }
      _ => Component::End,
    }
  }

  // Inverts operator so the unknown component gets closer to a solution
  // Example:
  // 5 * x - 3 = 7        : unsolved
  // 5 * x = 7 + 3 = 10   : solve 1
  // x = 10 / 5 = 2       : solve 2
  fn invert_operator(
    operator: Operator,
    outcome: Prec,
    values: Vec<Component>,
  ) -> (Component, Prec) {
    let mut values = values;

    if values.len() == 1 {
      return (
        values.into_iter().next().unwrap(),
        match operator {
          Sin => outcome.asin(),
          Cos => outcome.acos(),
          Tan => outcome.atan(),
          Sec => 1.0 / outcome.asin(),
          Csc => 1.0 / outcome.acos(),
          Cot => 1.0 / outcome.atan(),
          Exp => outcome.ln(),
          Ln => outcome.exp(),
          Sqrt => outcome.exp2(),
          _ => outcome,
        },
      );
    } else if values.len() == 2 {
      let mut maybe_num = None;
      let mut pos_left = false;
      let mut iter = values.into_iter();
      let (left, right) = (iter.next().unwrap(), iter.next().unwrap());

      // Retrieve possible number from binary component
      if let Component::Number(f) = left {
        maybe_num = Some(f);
        pos_left = true;
      } else if let Component::Number(f) = right {
        maybe_num = Some(f);
      }

      if let Some(f) = maybe_num {
        return (
          if pos_left { right } else { left },
          Self::invert_binary(&operator, outcome, f, pos_left),
        );
      } else {
        values = vec![left, right];
      }
    }

    (Component::Function { operator, values }, outcome)
  }

  // Invert binary component, see invert component
  // This function exists to prevent a huge cyclomatic complexity
  fn invert_binary(operator: &Operator, outcome: Prec, f: Prec, pos_left: bool) -> Prec {
    match operator {
      Add => outcome - f,

      Subtract => {
        if pos_left {
          f - outcome
        } else {
          outcome + f
        }
      }

      Multiply => outcome / f,

      Divide => {
        if pos_left {
          f / outcome
        } else {
          outcome * f
        }
      }

      Exponent | Pow => {
        if pos_left {
          outcome.log(f)
        } else {
          outcome.powf(1.0 / f)
        }
      }

      Log => {
        if pos_left {
          f.powf(1.0 / outcome)
        } else {
          f.powf(outcome)
        }
      }

      _ => outcome,
    }
  }

  // Solve component with an unknown variable for given outcome algebraically
  fn solve(expr: Component, outcome: Prec) -> (Component, Prec) {
    match expr {
      Component::Variable(c) => (Component::Variable(c), outcome),
      Component::Number(f) => (Component::Number(f), outcome),

      // Attempt to apply algebraic rules to binary component if it contains a number
      Component::Function { operator, values } => Self::invert_operator(operator, outcome, values),
      _ => (Component::End, 0.0),
    }
  }

  /// Get the derivative of an equation
  ///
  /// # Examples
  /// ```
  /// let eq = calculi::Equation::new("x^sin(x)").derive();
  ///
  /// assert_eq!(eq.text, "x ^ sin(x) * (cos(x) * ln(x) + sin(x) * 1 / x)");
  /// ```
  pub fn derive(&self) -> Equation {
    Equation::from(parser::simplify(derive::derive_component(&self.expression)))
  }

  /// Get the output of an equation with the given variable definitions
  ///
  /// # Examples
  /// ```
  /// let eq = calculi::Equation::new("a * sqrt(x + 1)");
  ///
  /// assert_eq!(eq.solve_with(vec![("a", 2.0), ("x", 8.0)]).to_float().unwrap(), 6.0);
  /// ```
  pub fn solve_with<'a>(&self, vars_raw: impl IntoIterator<Item = (&'a str, Prec)>) -> Component {
    let vars: HashMap<_, _> = vars_raw.into_iter().collect();
    Self::solve_component(&vars, &self.expression)
  }

  /// Attempt to solve equation that contains an unknown variable
  /// Returns left over outcome and expression if solving failed
  ///
  /// # Examples
  /// ```
  /// let eq = calculi::Equation::new("a * sqrt(x + 1)");
  ///
  /// let solved = eq.solve_for(9.0, vec![("x", 8.0)]);
  ///
  /// assert_eq!(solved.0.to_string(), "a");
  /// assert_eq!(solved.1, 3.0);
  /// ```

  // TODO: Make solve_for with outcome as a component
  pub fn solve_for<'a>(
    &self,
    outcome: Prec,
    vars: impl IntoIterator<Item = (&'a str, Prec)>,
  ) -> (Component, Prec) {
    let c = self.solve_with(vars);
    let mut expr = Self::solve(c, outcome);

    // Attempt to apply algebra while a binary component appears
    while let Component::Function { .. } = &expr.0 {
      let last = expr.0.to_string();
      expr = Self::solve(expr.0, expr.1);
      if last == expr.0.to_string() {
        break;
      }
    }

    expr
  }
}
