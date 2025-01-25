/*! Rigs, rings, and modules over them.

Lots of people have their own versions of a trait hierarchy for abstract
algebra; see [`noether`](https://crates.io/crates/noether) and links therein.
Our aim is not to make the most complete or general hierarchy but just to meet
our own needs, which currently is a bit of [commutative algebra](super::alg),
especially polynomial algebras over rings. So we avoid slicing the salomi too
thin with minor concepts like magmas and semigroups. We also take the category
theorist's attitude that rigs are respectable concept that do not deserve to be
denigrated as "semirings".

Besides the hierarchy of traits, the data structure provided here for [linear
combinations](Combination) and [monomials](Monomial). These are actually the
same data structure, but with different notation!
 */

use num_traits::{One, Zero};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg};

use duplicate::duplicate_item;

/// A commutative monoid, written additively.
pub trait AdditiveMonoid: Add<Output = Self> + Zero {}

#[duplicate_item(T; [f32]; [f64]; [i32]; [i64]; [u32]; [u64]; [usize])]
impl AdditiveMonoid for T {}

/** An abelian group, written additively.

Though logically redundant, this trait should also extend `Sub<Output = Self>`.
So far I've been too lazy to make this change since the extra trait cannot be
automatically derived without macro magic.
 */
pub trait AbGroup: AdditiveMonoid + Neg<Output = Self> {}

#[duplicate_item(T; [f32]; [f64]; [i32]; [i64])]
impl AbGroup for T {}

/// A monoid, written multiplicatively.
pub trait Monoid: Mul<Output = Self> + One {}

#[duplicate_item(T; [f32]; [f64]; [i32]; [i64]; [u32]; [u64]; [usize])]
impl Monoid for T {}

/// A commutative monoid, written multiplicatively.
pub trait CommMonoid: Monoid {}

#[duplicate_item(T; [f32]; [f64]; [i32]; [i64]; [u32]; [u64]; [usize])]
impl CommMonoid for T {}

/// A rig, also known as a semiring.
pub trait Rig: Monoid + AdditiveMonoid {}

#[duplicate_item(T; [f32]; [f64]; [i32]; [i64]; [u32]; [u64]; [usize])]
impl Rig for T {}

/// A commutative rig, also known as a commutative semiring.
pub trait CommRig: Rig + CommMonoid {}

#[duplicate_item(T; [f32]; [f64]; [i32]; [i64]; [u32]; [u64]; [usize])]
impl CommRig for T {}

/// A ring, assumed to be unital.
pub trait Ring: Rig + AbGroup {}

#[duplicate_item(T; [f32]; [f64]; [i32]; [i64])]
impl Ring for T {}

/// A commutative ring, assumed to be unital.
pub trait CommRing: Ring + CommRig {}

#[duplicate_item(T; [f32]; [f64]; [i32]; [i64])]
impl CommRing for T {}

/// A module over a commutative rig.
pub trait RigModule: AdditiveMonoid + Mul<Self::Rig, Output = Self> {
    /// Base rig for the module.
    type Rig: CommRig;
}

/// A module over a commutative ring.
pub trait Module: RigModule<Rig = Self::Ring> + AbGroup {
    /// Base ring for the module.
    type Ring: CommRing;
}

/** A formal linear combination.

This data structure is for linear combinations of indeterminates/variables
(`Var`) with coefficients (`Coef`) valued in a [rig](Rig) or at least in an
[additive monoid](AdditiveMonoid). For example, the coefficients could be
natural numbers, integers, real numbers, or nonnegative real numbers.

Linear combinations are the data structure for free modules. That is, for any
rig R, the free R-module on a set consists of formal R-linear combinations on
elements of the set.
 */
#[derive(Clone, Default, PartialEq, Eq)]
pub struct Combination<Var, Coef>(BTreeMap<Var, Coef>);

impl<Var, Coef> Combination<Var, Coef>
where
    Var: Ord,
{
    /// Generating combination on a single variable.
    pub fn generator(var: Var) -> Self
    where
        Coef: One,
    {
        Combination([(var, Coef::one())].into_iter().collect())
    }
}

/** Pretty print the combination using ASCII.

Intended for debugging/testing rather than any serious use.
 */
impl<Var, Coef> Display for Combination<Var, Coef>
where
    Var: Display,
    Coef: Display + PartialEq + One,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pairs = self.0.iter();
        let fmt_scalar_mul = |f: &mut std::fmt::Formatter<'_>, coef: &Coef, var: &Var| {
            if !coef.is_one() {
                let coef = coef.to_string();
                if coef.chars().all(|c| c.is_alphabetic())
                    || coef.chars().all(|c| c.is_ascii_digit() || c == '.')
                {
                    write!(f, "{} ", coef)?;
                } else {
                    write!(f, "({}) ", coef)?;
                }
            }
            write!(f, "{}", var)
        };
        if let Some((var, coef)) = pairs.next() {
            fmt_scalar_mul(f, coef, var)?;
        } else {
            write!(f, "0")?;
        }
        for (var, coef) in pairs {
            write!(f, " + ")?;
            fmt_scalar_mul(f, coef, var)?;
        }
        Ok(())
    }
}

impl<Var, Coef> AddAssign for Combination<Var, Coef>
where
    Var: Ord,
    Coef: Add<Output = Coef>,
{
    fn add_assign(&mut self, rhs: Self) {
        _add_assign(&mut self.0, rhs.0)
    }
}

fn _add_assign<K, V>(lhs: &mut BTreeMap<K, V>, rhs: BTreeMap<K, V>)
where
    K: Ord,
    V: Add<Output = V>,
{
    for (var, b) in rhs {
        if let Some(a) = lhs.remove(&var) {
            lhs.insert(var, a + b);
        } else {
            lhs.insert(var, b);
        }
    }
}

impl<Var, Coef> Add for Combination<Var, Coef>
where
    Var: Ord,
    Coef: Add<Output = Coef>,
{
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl<Var, Coef> Zero for Combination<Var, Coef>
where
    Var: Ord,
    Coef: Add<Output = Coef>,
{
    fn zero() -> Self {
        Combination(Default::default())
    }

    fn is_zero(&self) -> bool {
        self.0.is_empty()
    }
}

impl<Var, Coef> AdditiveMonoid for Combination<Var, Coef>
where
    Var: Ord,
    Coef: AdditiveMonoid,
{
}

impl<Var, Coef> Mul<Coef> for Combination<Var, Coef>
where
    Var: Ord,
    Coef: Clone + Default + Mul<Output = Coef>,
{
    type Output = Self;

    fn mul(mut self, a: Coef) -> Self {
        for coef in self.0.values_mut() {
            *coef = std::mem::take(coef) * a.clone();
        }
        self
    }
}

impl<Var, Coef> RigModule for Combination<Var, Coef>
where
    Var: Ord,
    Coef: Clone + Default + CommRig,
{
    type Rig = Coef;
}

impl<Var, Coef> Neg for Combination<Var, Coef>
where
    Var: Ord,
    Coef: Default + Neg<Output = Coef>,
{
    type Output = Self;

    fn neg(mut self) -> Self {
        for coef in self.0.values_mut() {
            *coef = std::mem::take(coef).neg();
        }
        self
    }
}

impl<Var, Coef> AbGroup for Combination<Var, Coef>
where
    Var: Ord,
    Coef: Default + AbGroup,
{
}

impl<Var, Coef> Module for Combination<Var, Coef>
where
    Var: Ord,
    Coef: Clone + Default + CommRing,
{
    type Ring = Coef;
}

/** A monomial in several variables.

This data structure is for monomials in several indeterminates/variables
(`Var`), having exponents (`Exp`) valued in an [additive
monoid](AdditiveMonoid). Most standardly, the exponents will be natural numbers
(say `u32` or `u64`), in which case the monomials in a set of variables, under
their usual multiplication, comprise the free commutative monoid on that set.
Other types of coefficents are also allowed, such as negative integers as in
Laurent polynomials, or real numbers as in
[S-systems](https://doi.org/10.1016/0895-7177(88)90553-5).

Monomials have exactly the same underlying data structure as
[combinations](Combination), but are written multiplicatively rather than
additively.
 */
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Monomial<Var, Exp>(BTreeMap<Var, Exp>);

impl<Var, Exp> Monomial<Var, Exp>
where
    Var: Ord,
{
    /// Generating monomial on a single variable.
    pub fn generator(var: Var) -> Self
    where
        Exp: One,
    {
        Monomial([(var, Exp::one())].into_iter().collect())
    }
}

/// Pretty print the monomial using ASCII.
impl<Var, Exp> Display for Monomial<Var, Exp>
where
    Var: Display,
    Exp: Display + PartialEq + One,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pairs = self.0.iter();
        let fmt_power = |f: &mut std::fmt::Formatter<'_>, var: &Var, exp: &Exp| {
            write!(f, "{}", var)?;
            if !exp.is_one() {
                let exp = exp.to_string();
                if exp.len() == 1 {
                    write!(f, "^{}", exp)?;
                } else {
                    write!(f, "^{{{}}}", exp)?;
                }
            }
            Ok(())
        };
        if let Some((var, exp)) = pairs.next() {
            fmt_power(f, var, exp)?;
        } else {
            write!(f, "1")?;
        }
        for (var, exp) in pairs {
            write!(f, " ")?;
            fmt_power(f, var, exp)?;
        }
        Ok(())
    }
}

impl<Var, Exp> MulAssign for Monomial<Var, Exp>
where
    Var: Ord,
    Exp: Add<Output = Exp>,
{
    fn mul_assign(&mut self, rhs: Self) {
        _add_assign(&mut self.0, rhs.0)
    }
}

impl<Var, Exp> Mul for Monomial<Var, Exp>
where
    Var: Ord,
    Exp: Add<Output = Exp>,
{
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self {
        self *= rhs;
        self
    }
}

impl<Var, Exp> One for Monomial<Var, Exp>
where
    Var: Ord,
    Exp: Add<Output = Exp>,
{
    fn one() -> Self {
        Monomial(Default::default())
    }

    fn is_one(&self) -> bool {
        self.0.is_empty()
    }
}

impl<Var, Exp> Monoid for Monomial<Var, Exp>
where
    Var: Ord,
    Exp: AdditiveMonoid,
{
}

impl<Var, Exp> CommMonoid for Monomial<Var, Exp>
where
    Var: Ord,
    Exp: AdditiveMonoid,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combinations() {
        let x = Combination::generator('x');
        let y = Combination::generator('y');
        assert_eq!(x.to_string(), "x");
        assert_eq!((x.clone() * 2u32 + y.clone() * 3u32).to_string(), "2 x + 3 y");
        assert_eq!((x.clone() + y.clone() + y + x).to_string(), "2 x + 2 y");

        assert_eq!(Combination::<char, u32>::zero().to_string(), "0");

        let x = Combination::generator('x');
        assert_eq!((x.clone() * -1i32).to_string(), "(-1) x");
        assert_eq!(x.neg().to_string(), "(-1) x");
    }

    #[test]
    fn monomials() {
        let x = Monomial::<_, u32>::generator('x');
        let y = Monomial::<_, u32>::generator('y');
        assert_eq!(x.clone().to_string(), "x");
        assert_eq!((x.clone() * y.clone() * y * x).to_string(), "x^2 y^2");

        assert_eq!(Monomial::<char, u32>::one().to_string(), "1");
    }
}
