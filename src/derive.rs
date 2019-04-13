use super::utils::*;
use super::component::Component;
use super::operators::Operator::*;

fn chain_rule(left: Component, right: &Component) -> Component {
  create_binary(Multiply, left, derive_component(right))
}

pub fn derive_component(expr: &Component) -> Component {
  match expr {
    Component::Number(_) => Component::Number(0.0),
    Component::Variable(_) => Component::Number(1.0),

    Component::Function { operator, values } => match operator {
      Add => create_binary(
        Add,
        derive_component(&values[0]),
        derive_component(&values[1]),
      ),

      Subtract => create_binary(
        Subtract,
        derive_component(&values[0]),
        derive_component(&values[1]),
      ),

      Multiply => create_binary(
        Add,
        chain_rule(values[0].clone(), &values[1]),
        chain_rule(values[1].clone(), &values[0]),
      ),

      Exponent | Pow => {
        // x^n
        if let Component::Number(f) = &values[1] {
          chain_rule(
            create_binary(
              Multiply,
              Component::Number(*f),
              create_binary(Exponent, values[0].clone(), Component::Number(f - 1.0)),
            ),
            &values[0],
          )
        // n^x
        } else if let Component::Number(f) = &values[0] {
          create_binary(
            Multiply,
            create_unary(Ln, Component::Number(*f)),
            chain_rule(expr.clone(), &expr),
          )
        } else {
          // x^x
          create_binary(
            Multiply,
            expr.clone(),
            create_binary(
              Add,
              create_binary(
                Multiply,
                derive_component(&values[1]),
                create_unary(Ln, values[0].clone()),
              ),
              create_binary(
                Multiply,
                values[1].clone(),
                create_binary(Divide, derive_component(&values[0]), values[0].clone()),
              ),
            ),
          )
        }
      }

      Log => chain_rule(
        create_binary(
          Divide,
          derive_component(&values[0]),
          create_binary(
            Multiply,
            create_unary(Ln, values[1].clone()),
            values[0].clone(),
          ),
        ),
        &values[0],
      ),

      Ln => chain_rule(
        create_binary(Divide, derive_component(&values[0]), values[0].clone()),
        &values[0],
      ),

      Sin => chain_rule(create_unary(Cos, values[0].clone()), &values[0]),

      Cos => chain_rule(
        create_binary(
          Multiply,
          Component::Number(-1.0),
          create_unary(Sin, values[0].clone()),
        ),
        &values[0],
      ),

      Tan => chain_rule(
        create_binary(
          Pow,
          create_unary(Sec, values[0].clone()),
          Component::Number(2.0),
        ),
        &values[0],
      ),

      Sec => chain_rule(
        create_binary(
          Multiply,
          create_unary(Sec, values[0].clone()),
          create_unary(Tan, values[0].clone()),
        ),
        &values[0],
      ),

      Csc => chain_rule(
        create_binary(
          Multiply,
          Component::Number(-1.0),
          create_binary(
            Multiply,
            create_unary(Csc, values[0].clone()),
            create_unary(Cot, values[0].clone()),
          ),
        ),
        &values[0],
      ),

      Cot => chain_rule(
        create_binary(
          Multiply,
          Component::Number(-1.0),
          create_binary(
            Pow,
            create_unary(Csc, values[0].clone()),
            Component::Number(2.0),
          ),
        ),
        &values[0],
      ),

      _ => Component::End,
    },
    _ => Component::End,
  }
}
