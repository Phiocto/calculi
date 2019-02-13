# Calculi
'calculi' is a crate used to algebraically solve equations with unknown variables for a given outcome.

It is also able to solve equations when all unknown variables are given and perform other calculus functions.

# Examples
```rust
let eq1 = calculi::Equation::new("x - 2 * a + 4 ^ b");
assert_eq!(eq1.solve_for(10.0, vec![("a", 4.5), ("b", 1.0)]).1, 15.0);

let eq2 = calculi::Equation::new("max(x + 3, root(y, 3), 1) + ln(exp(3))");
assert_eq!(eq2.solve_with(vec![("x", 2.0), ("y", 27.0)]).to_float().unwrap(), 8.0);
```
---

Look at [Equation](equation/struct.Equation.html) for all the equation functions.

Look at [Component](component/enum.Component.html) for the component functions that can be used on the component which [Equation::solve_with](equation/struct.Equation.html#method.solve_with) returns.

Look at [Operators](enum.Operator.html) for all available operators.

# Future Features
If you want to contribute these are some things you can contribute.

* Calculate derivatives (and supply them in string form)
* Calculate integrals (and supply them in string form)
* Possibly limits (when derivatives and integrals are integrated)
* More calculus...

Fixing possible bugs, shortening code (without making it less efficient) or making it more efficient is of course always welcome.
