use std::collections::HashMap;
use std::f32;

use super::component::Component;
use super::component::Var;
use super::operators::{Operator, Operator::*};
use super::parser;

/// The equation struct containing the equation text and the parsed component.Component.
///
/// Various functions can be executed on this equation to solve it or it's variables.

#[derive(Debug)]
pub struct Equation {
    text: String,
    expression: Component,
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

    // Attempt to apply external binary operator
    // Example:
    // (3 - x) + 2, becomes
    // 5 - x
    // TODO: Change return type to component to add support for other operators than Add and Subtract
    fn apply_external(
        left: &Component,
        right: &Component,
        f: f32,
        inner: &Operator,
        outer: &Operator,
    ) -> (Var, f32, bool) {
        let mut var = String::new();
        let mut sum = 0.0;
        let mut pos_left = false;

        if let (Component::Number(f1), Component::Variable(c)) = (left, right) {
            var = c.to_string();
            match (inner, outer) {
                (Add, Subtract) => sum = f1 - f,
                (Subtract, Subtract) => sum = -f1 + f,
                (Add, Add) => sum = f1 + f,
                (Subtract, Add) => sum = -f1 - f,
                //(Add, Multiply) => sum = -f1 - f,
                //(Subtract, Multiply) => sum = -f1 - f,
                _ => (),
            }
        } else if let (Component::Variable(c), Component::Number(f1)) = (left, right) {
            var = c.to_string();
            pos_left = true;
            match (inner, outer) {
                (Add, Subtract) => sum = f1 - f,
                (Subtract, Subtract) => sum = -f1 - f,
                (Add, Add) => sum = f1 + f,
                (Subtract, Add) => sum = -f1 + f,
                _ => (),
            }
        }

        (var, sum, pos_left)
    }

    fn retrieve_value(values: &[Component], condition: fn(f32, f32) -> bool) -> Option<Component> {
        let mut value = 0.0;
        for (i, x) in values.iter().enumerate() {
            if let Component::Number(f) = x {
                if i == 0 || condition(*f, value) {
                    value = *f;
                }
            } else {
                return None;
            }
        }

        Some(Component::Number(value))
    }

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

            // Attempt to simplify current binary component that still contains unknown variables
            if let (
                Component::Function {
                    operator: op,
                    values,
                },
                Component::Number(f),
            ) = (&values[0], &values[1])
            {
                if (op.compare(&Add) || op.compare(&Subtract))
                    && (operator.compare(&Add) || operator.compare(&Subtract))
                {
                    // Apply internal number to external number
                    let (var, sum, pos_left) =
                        Self::apply_external(&values[0], &values[1], *f, &op, &operator);

                    // Return the correct syntax in a binary component
                    return Some(if sum == 0.0 && !var.is_empty() {
                        Component::Variable(var)
                    } else if pos_left {
                        Component::Function {
                            operator: if sum < 0.0 { Subtract } else { Add },
                            values: vec![Component::Variable(var), Component::Number(sum.abs())],
                        }
                    } else {
                        Component::Function {
                            operator: if sum < 0.0 { Subtract } else { Add },
                            values: vec![Component::Number(sum.abs()), Component::Variable(var)],
                        }
                    });
                }
            }
        }

        // Operators with unlimited amount of parameters
        match operator {
            Max => {
                let value = Self::retrieve_value(values, |x, y| x > y);
                if value.is_some() {
                    return value;
                }
            }
            Min => {
                let value = Self::retrieve_value(values, |x, y| x < y);
                if value.is_some() {
                    return value;
                }
            }
            _ => (),
        }

        None
    }

    // Attempt to solve component with given variables
    fn solve_component(vars: &HashMap<&str, f32>, component: &Component) -> Component {
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
        outcome: f32,
        values: Vec<Component>,
    ) -> (Component, f32) {
        let mut values = values;

        if values.len() == 1 {
            return (
                values.into_iter().next().unwrap(),
                match operator {
                    Sin => outcome.asin(),
                    Cos => outcome.acos(),
                    Tan => outcome.atan(),
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

                        _ => outcome,
                    },
                );
            } else {
                values = vec![left, right];
            }
        }

        match operator {
            Max | Min => {
                let mut count = 0;
                let mut index = 0;

                // Check if only one value is able to match outcome
                // Example:
                // max(x * 2, 4, 5) = 10
                // 4 != 10
                // 5 != 10
                // So:
                // x * 2 = 10
                values.iter().enumerate().for_each(|(i, x)| {
                    if let Component::Number(f) = x {
                        if (*f - outcome).abs() < f32::EPSILON {
                            count += 1;
                        }
                    } else {
                        index = i;
                        count += 1;
                    }
                });

                if count == 1 {
                    return (values.into_iter().nth(index).unwrap(), outcome);
                }
            }
            _ => (),
        }
        
        (Component::Function { operator, values }, outcome)
    }

    // Solve component with an unknown variable for given outcome algebraically
    fn solve(expr: Component, outcome: f32) -> (Component, f32) {
        match expr {
            Component::Variable(c) => (Component::Variable(c), outcome),
            Component::Number(f) => (Component::Number(f), outcome),

            // Attempt to apply algebraic rules to binary component if it contains a number
            Component::Function { operator, values } => {
                Self::invert_operator(operator, outcome, values)
            }
            _ => (Component::End, 0.0),
        }
    }

    /// Get the output of an equation with the given variable definitions
    ///
    /// # Examples
    /// ```
    /// let eq = calculi::Equation::new("a * sqrt(x + 1)");
    ///
    /// assert_eq!(eq.solve_with(vec![("a", 2.0), ("x", 8.0)]).to_float().unwrap(), 6.0);
    /// ```
    pub fn solve_with<'a>(&self, vars_raw: impl IntoIterator<Item = (&'a str, f32)>) -> Component {
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
    pub fn solve_for<'a>(
        &self,
        outcome: f32,
        vars: impl IntoIterator<Item = (&'a str, f32)>,
    ) -> (Component, f32) {
        let mut expr = Self::solve(self.solve_with(vars), outcome);

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
