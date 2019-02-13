use std::fmt;

/// These are all the functions/operators that can be used in an equation.
/// 
/// The operators are: Add (+), Subtract (-), Multiply (*), Divide(/), Modulo (%), Exponent(^).
/// 
/// All syntax is the same for the other functions but lowercase.

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Exponent,
    Max,
    Min,
    Pow,
    Log,
    Sin,
    Cos,
    Tan,
    Abs,
    Floor,
    Round,
    Ceil,
    Root,
    Exp,
    Ln,
    Sqrt,
    Error,
}

use Operator::*;

impl From<&str> for Operator {
    fn from(c: &str) -> Operator {
        match c {
            "+" => Add,
            "-" => Subtract,
            "*" => Multiply,
            "/" => Divide,
            "%" => Modulo,
            "^" => Exponent,
            "max" => Max,
            "min" => Min,
            "pow" => Pow,
            "log" => Log,
            "sin" => Sin,
            "cos" => Cos,
            "tan" => Tan,
            "abs" => Abs,
            "floor" => Floor,
            "round" => Round,
            "ceil" => Ceil,
            "root" => Root,
            "exp" => Exp,
            "ln" => Ln,
            "sqrt" => Sqrt,
            _ => Error,
        }
    }
}

impl From<char> for Operator {
    fn from(c: char) -> Operator {
        Operator::from(c.to_string().as_str())
    }
}

impl Operator {
    /// Compare the type of this operator to the given operator
    pub fn compare(&self, other: &Operator) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(
            match self {
                Add => "+",
                Subtract => "-",
                Multiply => "*",
                Divide => "/",
                Modulo => "%",
                Exponent => "^",
                Max => "max",
                Min => "min",
                Pow => "pow",
                Log => "logab",
                Sin => "sin",
                Cos => "cos",
                Tan => "tan",
                Abs => "abs",
                Floor => "floor",
                Round => "round",
                Ceil => "ceil",
                Root => "root",
                Exp => "exp",
                Ln => "ln",
                Sqrt => "sqrt",
                Error => "error",
            }
        )
    }
}

// Get precedence (importance) of an operator
pub fn get_precedence(c: Option<&char>) -> i8 {
    match c {
        Some(c) => match c {
            '+' => 1,
            '-' => 1,
            '*' => 3,
            '/' => 3,
            '%' => 3,
            '^' => 5,
            _ => -1,
        },
        None => -1,
    }
}
