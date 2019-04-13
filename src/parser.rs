use std::iter::Peekable;
use std::str::Chars;

use super::component::{Component, Prec};
use super::operators;
use super::operators::{Operator, Operator::*};
use super::utils::*;

enum Simplified {
  Left,
  Right,
  Component(Component),
  None,
}

fn is_operator(c: char) -> bool {
  match c {
    '+' | '-' | '*' | '/' | '%' | '^' => true,
    _ => false,
  }
}

// Checks if character is a floating point digit
fn is_digit(c: char) -> bool {
  (c >= '0' && c <= '9') || c == '.'
}

// Parses a component out of a peekable iterator of characters
fn parse_component(chars: &mut Peekable<Chars>) -> Component {
  let mut maybe_num = String::new();
  let mut maybe_var = String::new();

  while let Some(c) = chars.peek() {
    if is_digit(*c) {
      maybe_num.push(*c);
    } else if !maybe_num.is_empty() {
      break;
    }
    // Parse parenthesis
    else if *c == '(' {
      chars.next();
      let first = parse_component(chars);
      let first = parse_binary(chars, 0, first);

      // Operator function
      // Syntax:
      // FUNCTION(par1, par2, ..., parn)
      if !maybe_var.is_empty() {
        let mut values = vec![first];

        while let Some(c) = chars.peek() {
          if *c == ')' {
            chars.next();
            return Component::Function {
              operator: Operator::from(maybe_var.to_lowercase().as_str()),
              values,
            };
          }

          let par = parse_component(chars);
          let par = parse_binary(chars, 0, par);
          values.push(par);
        }

      // Normal parenthesis
      } else {
        return first;
      }
    } else if !is_operator(*c) && *c != ',' && *c != ')' {
      maybe_var.push(*c);
    } else if !maybe_var.is_empty() {
      break;
    }
    chars.next();
  }

  if !maybe_num.is_empty() {
    return Component::Number(maybe_num.parse::<Prec>().unwrap());
  }
  if !maybe_var.is_empty() {
    return Component::Variable(maybe_var);
  }
  Component::End
}

pub fn simplify(component: Component) -> Component {
  match component {
    Component::Function { operator, values } => {
      if values.len() == 2 {
        let mut iter = values.into_iter();
        let (left, right) = (iter.next().unwrap(), iter.next().unwrap());
        let simple = simplify_binary(&operator, &left, &right);

        let left = simplify(left);
        let right = simplify(right);

        match simple {
          Simplified::Left => left,
          Simplified::Right => right,
          Simplified::Component(component) => component,
          Simplified::None => create_binary(operator, left, right),
        }
      } else {
        Component::Function { operator, values }
      }
    }
    _ => component,
  }
}

#[allow(clippy::float_cmp)]
fn simplify_binary(operator: &Operator, left: &Component, right: &Component) -> Simplified {
  if operator.compare(&Multiply) || operator.compare(&Exponent) {
    if left.to_float().unwrap_or(1.0) == 0.0 {
      // 0 * x or 0 ^ x, becomes 0
      return Simplified::Component(Component::Number(0.0));
    } else if right.to_float().unwrap_or(1.0) == 0.0 {
      if operator.compare(&Multiply) {
        // x * 0, becomes 0
        return Simplified::Component(Component::Number(0.0));
      } else {
        // x ^ 0, becomes 1
        return Simplified::Component(Component::Number(1.0));
      }
    } else if left.to_float().unwrap_or(0.0) == 1.0 {
      if operator.compare(&Multiply) {
        // 1 * x, becomes x
        return Simplified::Right;
      } else {
        // 1 ^ x, becomes 1
        return Simplified::Component(Component::Number(1.0));
      }
    } else if right.to_float().unwrap_or(0.0) == 1.0 {
      // x * 1 or x ^ 1, becomes x
      return Simplified::Left;
    }
  } else if operator.compare(&Add) || operator.compare(&Subtract) {
    if left.to_float().unwrap_or(1.0) == 0.0 && operator.compare(&Add) {
      // 0 + x, becomes x
      return Simplified::Right;
    } else if right.to_float().unwrap_or(1.0) == 0.0 {
      // x - 0 or x + 0, becomes x
      return Simplified::Left;
    }
  } else if operator.compare(&Divide) && right.to_float().unwrap_or(0.0) == 1.0 {
    // x / 1, becomes x
    return Simplified::Left;
  } else {
    return Simplified::None;
  }

  Simplified::None
}

// Parses a binary component (right applied by operator to left)
fn parse_binary(chars: &mut Peekable<Chars>, prev_prec: i8, left: Component) -> Component {
  let mut left = left;
  loop {
    // Skips current character if it is not an operator
    if let Some(c) = chars.peek() {
      if !is_operator(*c) && *c != ')' {
        chars.next();
      }
    }

    let c = chars.peek();
    // Gets precedence of current operator
    let prec = operators::get_precedence(c);

    // If current operator is less important than the previous one, return the previous component
    if prec < prev_prec {
      return left;
    }

    let c = *c.unwrap();
    let mut right = parse_component(chars);


    let new_prec = operators::get_precedence(chars.peek());

    // Create new binary component if current operator precedence is higher than the previous one
    if prec < new_prec {
      right = parse_binary(chars, prec + 1, right);
      if let Component::End = right {
        return Component::End;
      }
    }

    let operator = Operator::from(c);

    // Check some constant expressions to attempt to shorten equation
    match simplify_binary(&operator, &left, &right) {
      Simplified::Left => return left,
      Simplified::Right => return right,
      Simplified::Component(component) => return component,
      Simplified::None => (),
    }

    left = create_binary(operator, left, right);
  }
}

/// Parses component from an equation in string form
pub fn parse(raw: &str) -> Component {
  // Removes all spaces from string
  let string = raw
    .chars()
    .filter(|x| !x.is_whitespace())
    .collect::<String>();

  let mut chars = string.chars().peekable();

  let left = parse_component(&mut chars);
  let mut comp = parse_binary(&mut chars, 0, left);
  while chars.size_hint().0 > 0 {
    chars.next();
    comp = parse_binary(&mut chars, 0, comp);
  }
  comp
}
