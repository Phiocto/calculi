use std::fmt;

use super::operators;
use super::operators::Operator;

pub type Prec = f32;

/// The possible equation components
#[derive(Debug, Clone)]
pub enum Component {
  Variable(String),
  Number(Prec),
  Function {
    operator: Operator,
    values: Vec<Component>,
  },
  End,
}

impl fmt::Display for Component {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(&self.to_text_prec(0))
  }
}

impl Component {
  // Converts component to a readable text
  fn to_text_prec(&self, prev_prec: i8) -> String {
    match self {
      Component::Variable(c) => c.to_string(),
      Component::Number(f) => f.to_string(),
      Component::Function { operator, values } => {
        let op_value = operator.to_string();

        // Standard binary operator
        // Checks if parenthesis surround expression with operator precedence
        if op_value.len() == 1 {
          let prec = operators::get_precedence(Some(&op_value.chars().next().unwrap()));
          format!(
            "{}{} {} {}{}",
            if prec < prev_prec { "(" } else { "" },
            values[0].to_text_prec(prec),
            op_value,
            values[1].to_text_prec(prec),
            if prec < prev_prec { ")" } else { "" }
          )

        // Function operator function
        } else {
          let mut parameters = values[0].to_string();
          for x in values.iter().skip(1) {
            parameters = format!("{}, {}", parameters, x.to_string());
          }
          format!("{}({})", op_value, parameters)
        }
      }
      _ => String::from(""),
    }
  }

  /// Attempts to convert the component to a float if it is a number
  /// Returns None if component is not a number
  ///
  /// # Examples
  /// ```
  /// let eq = calculi::Equation::new("a * sqrt(x + 1)");
  ///
  /// assert_eq!(eq.solve_with(vec![("a", 2.0), ("x", 8.0)]).to_float().unwrap(), 6.0);
  /// ```
  pub fn to_float(&self) -> Option<Prec> {
    match self {
      Component::Number(f) => Some(*f),
      _ => None,
    }
  }
}
