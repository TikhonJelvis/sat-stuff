use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Not;

/// A SAT variable.
///
/// Under the hood, variables are indexed from `1` (to let us
/// represent logical negation with numeric negation).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Var(i32);

impl Var {
    pub fn index(self) -> usize {
        (self.0 - 1) as usize
    }
}

/// A SAT literal, which is a variable (`Var`) that may or may not be
/// negated.
///
/// Literals can be negated with the `!` operator. Under the hood,
/// negation is represented with numeric negation (`-3` is
/// interpreted as ¬x₃).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Lit(i32);

impl Lit {
    /// Return the variable in this literal.
    pub fn var(self) -> Var {
        Var(self.0.abs())
    }

    /// Is this literal negated?
    pub fn negated(self) -> bool {
        self.0 < 0
    }
}

impl Not for Lit {
    type Output = Lit;

    fn not(self) -> Self::Output {
        Lit(-self.0)
    }
}

/// A clause is a disjunction of literals (variables that can be negated).
///
/// Clauses can be empty (which means the entire formula is
/// unsatisfiable. Examples:
///
///  * ()
///  * (x₁)
///  * (x₁ ∨ ¬x₂ ∨ x₃)
pub type Clause = Vec<Lit>;

/// A formula is a conjunction of zero or more clauses. (An empty
/// formula is satisfiable by definition.)
pub type Formula = Vec<Clause>;

/// An assignment from variables to true, false or unknown (`None`).
pub struct Assignment(Vec<Option<bool>>);

impl Assignment {
    /// Return the value of the given literal, taking negation into
    /// account.
    pub fn check(&self, lit: Lit) -> Option<bool> {
        self[lit.var()].map(|val| if lit.negated() { !val } else { val })
    }

    /// Returns whether the given clause is a "unit clause" where
    /// every variable but one is false.
    pub fn is_unit(&self, clause: &Clause) -> bool {
        let mut unit = false;

        for lit in clause {
            match self[lit.var()] {
                None => {
                    if unit {
                        unit = false;
                        break;
                    } else {
                        unit = true;
                    }
                }
                Some(false) => {}
                Some(true) => {
                    unit = false;
                    break;
                }
            }
        }

        unit
    }
}

impl Index<Var> for Assignment {
    type Output = Option<bool>;

    fn index(&self, key: Var) -> &Self::Output {
        &self.0[key.index()]
    }
}

impl IndexMut<Var> for Assignment {
    fn index_mut(&mut self, key: Var) -> &mut Self::Output {
        &mut self.0[key.index()]
    }
}
