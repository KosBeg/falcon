//! A very simple, and fast, constant propagation
//!
//! Each location has the known constant values for all variables before
//! execution of that location.
//!
//! Calling Constants::eval() uses the known constant values to replace scalars,
//! and then attempts to evaluate the expression to an `il::Constant`.


use analysis::fixed_point;
use error::*;
use executor::eval;
use il;
use std::collections::{HashMap};
use std::cmp::{Ordering, PartialOrd};


/// Compute constants for the given function
pub fn constants<'r>(function: &'r il::Function)
-> Result<HashMap<il::RefProgramLocation<'r>, Constants>> {
    let constants = fixed_point::fixed_point_forward(ConstantsAnalysis{}, function)?;

    // we're now going to remap constants, so each position holds the values of
    // constants immediately preceeding its execution.

    let mut result = HashMap::new();

    for (location, _) in &constants {
        result.insert(location.clone(),
            location.backward()?
                .into_iter()
                .fold(Constants::new(), |c, location|
                    c.join(&constants[&location])));
    }

    Ok(result)
}


#[allow(dead_code)] // Bottom is never used
#[derive(Clone, Debug, PartialEq)]
enum Constant {
    Top,
    Constant(il::Constant),
    Bottom
}


impl Constant {
    fn get(&self) -> Option<&il::Constant> {
        match *self {
            Constant::Constant(ref constant) => Some(constant),
            Constant::Top |
            Constant::Bottom => None
        }
    }
}


impl PartialOrd for Constant {
    fn partial_cmp(&self, other: &Constant) -> Option<Ordering> {
        match *self {
            Constant::Top => match *other {
                Constant::Top => Some(Ordering::Equal),
                Constant::Constant(_) |
                Constant::Bottom => Some(Ordering::Greater)
            },
            Constant::Constant(ref lc) => match *other {
                Constant::Top => Some(Ordering::Less),
                Constant::Constant(ref rc) =>
                    if lc == rc {
                        Some(Ordering::Equal)
                    }
                    else {
                        None
                    },
                Constant::Bottom => Some(Ordering::Greater),
            },
            Constant::Bottom => match *other {
                Constant::Top |
                Constant::Constant(_) => Some(Ordering::Less),
                Constant::Bottom => Some(Ordering::Equal)
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct Constants {
    constants: HashMap<il::Scalar, Constant>
}


impl PartialOrd for Constants {
    fn partial_cmp(&self, other: &Constants) -> Option<Ordering> {

        if self.constants.len() < other.constants.len() {
            for (ls, lc) in self.constants.iter() {
                if !other.constants
                    .get(ls)
                    .map(|rc| lc <= rc)
                    .unwrap_or(false) {
                    return None;
                }
            }
            Some(Ordering::Less)
        }
        else if self.constants.len() > other.constants.len() {
            for (ls, lc) in other.constants.iter() {
                if !self.constants
                    .get(ls)
                    .map(|rc| lc <= rc)
                    .unwrap_or(false) {
                    return None;
                }
            }
            Some(Ordering::Greater)
        }
        else {
            let mut order = Ordering::Equal;
            for (ls, lc) in &self.constants {
                match other.constants.get(ls) {
                    Some(rc) =>
                        if lc < rc {
                            if order <= Ordering::Equal {
                                order = Ordering::Less;
                            }
                            else {
                                return None;
                            }
                        }
                        else if lc > rc {
                            if order >= Ordering::Equal {
                                order = Ordering::Greater;
                            }
                            else {
                                return None;
                            }
                        },
                    None => {
                        return None;
                    }
                }
            }
            Some(order)
        }
    }
}


impl Constants {
    pub fn new() -> Constants {
        Constants {
            constants: HashMap::new()
        }
    }

    pub fn scalar(&self, scalar: &il::Scalar) -> Option<&il::Constant> {
        self.constants
            .get(scalar)
            .and_then(|constant| constant.get())
    }

    fn set_scalar(&mut self, scalar: il::Scalar, constant: Constant) {
        self.constants.insert(scalar, constant);
    }

    fn top(&mut self) {
        self.constants.iter_mut()
            .for_each(|(_, constant)| *constant = Constant::Top);
    }

    pub fn eval(&self, expression: &il::Expression) -> Option<il::Constant> {
        let expression_scalars = expression.scalars();

        let expression =
            expression_scalars
                .into_iter()
                .fold(Some(expression.clone()), |expression, scalar| 
                    self.scalar(scalar).and_then(|constant|
                        expression.map(|expr|
                            expr.replace_scalar(scalar, &constant.clone().into())
                                .unwrap())
                    )
                )?;
        
        eval(&expression).ok()
    }

    fn join(self, other: &Constants) -> Constants {
        let mut result = self.clone();
        for (scalar, constant) in other.constants.iter() {
            match self.constants.get(scalar) {
                Some(c) => 
                    if c != constant {
                        result.set_scalar(scalar.clone(), Constant::Top);
                    },
                None => result.set_scalar(scalar.clone(), constant.clone())
            }
        }
        result
    }
}


// We require a struct to implement methods for our analysis over.
struct ConstantsAnalysis {}


impl<'r> fixed_point::FixedPointAnalysis<'r, Constants> for ConstantsAnalysis {
    fn trans(&self, location: il::RefProgramLocation<'r>, state: Option<Constants>)
        -> Result<Constants> {

        let mut state = match state {
            Some(state) => state,
            None => Constants::new()
        };

        let state = match location.instruction() {
            Some(instruction) => match *instruction.operation() {
                il::Operation::Assign { ref dst, ref src } => {
                    let constant =
                        state.eval(src)
                            .map(|constant| Constant::Constant(constant))
                            .unwrap_or(Constant::Top);
                    state.set_scalar(dst.clone(), constant);
                    state
                },
                il::Operation::Load { ref dst, .. } => {
                    state.set_scalar(dst.clone(), Constant::Top);
                    state
                },
                il::Operation::Store { .. } |
                il::Operation::Branch { .. } |
                il::Operation::Raise { .. } => {
                    state.top();
                    state
                }
            },
            None => state
        };

        Ok(state)
    }


    fn join(&self, state0: Constants, state1: &Constants)
        -> Result<Constants> {
        
        Ok(state0.join(state1))
    }
}