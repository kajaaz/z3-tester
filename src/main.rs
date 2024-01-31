extern crate z3;

use z3::{Config, Context, Solver, Symbol};
use z3::ast::Int;

fn main() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    // Declaration of symbolic variables
    let x = Int::new_const(&ctx, Symbol::String("x".to_string()));
    let y = Int::new_const(&ctx, Symbol::String("y".to_string()));

    let three = Int::from_i64(&ctx, 3);
    let nine = Int::from_i64(&ctx, 9);

    // Setting constraint 1 : x + y > 3
    let sum = Int::add(&ctx, &[&x, &y]);
    let condition1 = sum.gt(&three);

    // Setting constraint 2 : x < 9
    let condition2 = x.lt(&nine);

    solver.assert(&condition1);
    solver.assert(&condition2);

    match solver.check() {
    	z3::SatResult::Sat => {
        	println!("SATISFIABLE");
        	let model = solver.get_model().unwrap();

        	let x_val = model.eval(&x).unwrap().as_i64().unwrap();
        	let y_val = model.eval(&y).unwrap().as_i64().unwrap();

        	println!("Solution: x = {}, y = {}", x_val, y_val);
   	}
    	z3::SatResult::Unsat => {
        	println!("UNSATISFIABLE");
    	}
    	z3::SatResult::Unknown => {
   	     println!("UNKNOWN");
   	}
    }
}
