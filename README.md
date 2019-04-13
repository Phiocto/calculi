# Calculi
'calculi' is a crate used to algebraically solve equations with unknown variables for a given outcome.

It is also able to solve equations when all unknown variables are given and perform other calculus functions.

# Features
These are the current features of this crate (see [Future Features](#future-features) for more info on future features).

* Attempt to algebraically solve equations
* Solve equations with given variables
* Calculate derivatives (and supply them in string/component form)


# Examples
```rust
let eq1 = calculi::Equation::new("x - 2 * a + 4 ^ b");

assert_eq!(eq1.solve_for(10.0, vec![("a", 4.5), ("b", 1.0)]).1, 15.0);


let eq2 = calculi::Equation::new("max(x + 3, root(y, 3), 1) + ln(exp(3))");

assert_eq!(eq2.solve_with(vec![("x", 2.0), ("y", 27.0)]).to_float().unwrap(), 8.0);


let eq3 = calculi::Equation::new("x ^ 3");

assert_eq!(eq3.derive().text, "3 * x ^ 2")
```
---

See the documentation for more info on how to use this crate and its functions.

# Future Features<a name="future-features"></a>
If you want to contribute these are some things you can contribute.

* Calculate primitives (and supply them in string/component form)
* Inverse functions
* Possibly limits (when derivatives and integrals are implemented)
* More calculus...

Fixing possible bugs, shortening code (without making it less efficient) or making it more efficient is of course always welcome.
