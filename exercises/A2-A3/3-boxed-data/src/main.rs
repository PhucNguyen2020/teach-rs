/// Below you find a small start of a data type modelling the abstract syntax tree for an expression,
/// and a small evaluator function.
///
/// Please extend this evaluator in the following ways:
///
/// - Add support for multiplication and division
///
/// - Since division can fail, the function eval needs to return an Option<i64>, where None indicates that a division by
///   zero has occurred. Make sure that errors are propagated correctly (hint: use the ? syntax).
///
/// - We have added the the form "Summation(Vec<Expr>)", representing the sum of a list of expressions.
/// Question: why can we get away with Vec<Expr> enough in that case, instead of Box<Vec<Expr>> ?

#[derive(Debug)]
enum Expr {
    Const(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Var,
    Summation(Vec<Expr>),
}

// inject these two identifiers directly into the current namespace
use Expr::Const;
use Expr::Var;
use Expr::Summation;

// These are convenience functions, so you don't have to type "Box::new" as often
// when building test-data types
fn add(x: Expr, y: Expr) -> Expr {
    Expr::Add(Box::new(x), Box::new(y))
}

fn sub(x: Expr, y: Expr) -> Expr {
    Expr::Sub(Box::new(x), Box::new(y))
}

fn mul(x: Expr, y: Expr) -> Expr {
    todo!()
}

fn div(x: Expr, y: Expr) -> Expr {
    todo!()
}

// ...

fn eval(expr: &Expr, var: i64) -> i64 {  // this should return an Option<i64>
    use Expr::*;
    match expr {
        Const(k) => *k,
        Var => var,
        Add(lhs, rhs) => eval(lhs, var) + eval(rhs, var),
        Sub(lhs, rhs) => eval(lhs, var) - eval(rhs, var),

	Summation(exprs) => {
	    let mut acc = 0;
	    for e in exprs {
		acc += eval(e, var)
	    }
	    acc
	}
    }
}

fn main() {
    let test = |expr| {
        let value = rand::random::<i8>() as i64;
        println!("{:?} with Var = {} ==> {}", &expr, value, eval(&expr, value));
    };

    test(Const(5));
    test(Var);
    test(sub(Var, Const(5)));
    test(add(sub(Var, Const(5)), Const(5)));
    test(Summation(vec![Var, Const(1)]));
}

// If you have time let and want to code more Rust: you can extend this exercise endlessly; one idea would be adding a Sigma(from,to,expr)
// constructor to Expr which computes the equivalent of (in LaTeX notation) \sum_{Var = from}^{to} expr; i.e. Sigma(Const(1), Const(5), Var) should be
// equivalent to Summation(vec![Const(1), Const(2), Const(3), Const(4), Const(5)]).
