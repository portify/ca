use super::parser::{Expr, Op};
use num::{Signed, ToPrimitive, BigRational};
use num::rational::Ratio;
use num::bigint::ToBigInt;

use super::context::Context;

pub fn evaluate(expression: Expr, context: &mut Context) -> Result<Expr, String> {
	if let Expr::BinaryExpr(lhs, op, rhs) = expression {
		let lhs = evaluate(*lhs, context)?;
		let rhs = evaluate(*rhs, context)?;

		if let Expr::Number(ref lhs_i) = lhs {
			if let Expr::Number(ref rhs_i) = rhs {
				match op {
					Op::Add => return Ok(Expr::Number(lhs_i + rhs_i)),
					Op::Subtract => return Ok(Expr::Number(lhs_i - rhs_i)),
					Op::Multiply => return Ok(Expr::Number(lhs_i * rhs_i)),
					Op::Adjacent => return Ok(Expr::Number(lhs_i * rhs_i)),
					Op::Divide => return Ok(Expr::Number(lhs_i / rhs_i)),
					Op::Modulus => return Ok(Expr::Number(lhs_i % rhs_i)),
					Op::Exponent => {
						if let Some(r) = ratio_power(lhs_i, rhs_i) {
							return Ok(Expr::Number(r));
						}
					},
					Op::Equals => return Ok(Expr::Boolean(lhs_i == rhs_i))
				}
			}
		}

		if let Expr::Name(ref name) = lhs {
			if op == Op::Adjacent {
				if let Some(e) = apply_fn(name, &rhs) {
					return Ok(e);
				}
			}
		}

		return Ok(Expr::BinaryExpr(Box::new(lhs), op, Box::new(rhs)));
	}

	if let Expr::Name(ref name) = expression {
		if let Some(expr) = context.get(name) {
			// FIXME: This use of Clone
			return evaluate(expr, &mut context.evaluate((*name).clone()));
		}
	}

	if let Expr::Tuple(ref items) = expression {
		let mut evaluated = Vec::with_capacity(items.len());

		for item in items.iter() {
			evaluated.push(evaluate((*item).clone(), context)?);
		}

		return Ok(Expr::Tuple(evaluated));
	}

	Ok(expression)
}

fn ratio_power(lhs: &BigRational, rhs: &BigRational) -> Option<BigRational> {
	if !rhs.is_integer() {
		println!("Note: Non-integer exponents ({}) are not supported", rhs);
		return None;
	}

	let power = rhs.numer().to_i32();
	let numer = lhs.numer().to_isize();
	let denom = lhs.denom().to_isize();

	if let (Some(p), Some(n), Some(d)) = (power, numer, denom) {
		let r = Ratio::new(n, d).pow(p);
		let numer = r.numer().to_bigint();
		let denom = r.denom().to_bigint();

		if let (Some(n), Some(d)) = (numer, denom) {
			return Some(BigRational::new(n, d));
		}
	}
	
	None
}

fn apply_fn(name: &String, operand: &Expr) -> Option<Expr> {
	if name == "floor" {
		if let &Expr::Number(ref n) = operand {
			return Some(Expr::Number(n.floor()));
		}
	}
	if name == "ceil" {
		if let &Expr::Number(ref n) = operand {
			return Some(Expr::Number(n.ceil()));
		}
	}
	if name == "round" {
		if let &Expr::Number(ref n) = operand {
			return Some(Expr::Number(n.round()));
		}
	}
	if name == "trunc" {
		if let &Expr::Number(ref n) = operand {
			return Some(Expr::Number(n.trunc()));
		}
	}
	if name == "fract" {
		if let &Expr::Number(ref n) = operand {
			return Some(Expr::Number(n.fract()));
		}
	}
	if name == "abs" {
		if let &Expr::Number(ref n) = operand {
			return Some(Expr::Number(n.abs()));
		}
	}
	None
}
