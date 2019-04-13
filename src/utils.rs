use super::component::Component;
use super::operators::Operator;

pub(crate) fn create_unary(operator: Operator, component: Component) -> Component {
  Component::Function {
    operator,
    values: vec![component],
  }
}

pub(crate) fn create_binary(operator: Operator, left: Component, right: Component) -> Component {
  Component::Function {
    operator,
    values: vec![left, right],
  }
}