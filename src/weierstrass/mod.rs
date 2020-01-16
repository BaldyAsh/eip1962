#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub(crate) enum CurveType {
    Generic,
    AIsMinus3,
    AIsZero,
    BIsZero,
}

use crate::field::SizedPrimeField;
use crate::representation::ElementRepr;
use crate::traits::FieldElement;
use crate::traits::ZeroAndOne;

pub trait CurveParameters {
    type BaseFieldElement: FieldElement + ZeroAndOne;
    fn params(&self) -> <Self::BaseFieldElement as ZeroAndOne>::Params;
}

use crate::fp::Fp;
pub(crate) struct CurveOverFpParameters<'a, FE: ElementRepr, F: SizedPrimeField<Repr = FE>> {
    pub(crate) field: &'a F,
}
impl<'a, FE: ElementRepr, F: SizedPrimeField<Repr = FE>> CurveParameters for CurveOverFpParameters<'a, FE, F> {
    type BaseFieldElement = Fp<'a, FE, F>;
    fn params(&self) -> <Self::BaseFieldElement as ZeroAndOne>::Params {
        self.field
    }
}
impl<'a, FE: ElementRepr, F: SizedPrimeField<Repr = FE>> CurveOverFpParameters<'a, FE, F> {
    pub(crate) fn new(field: &'a F) -> Self {
        Self {
            field
        }
    }
}

use crate::extension_towers::fp2;
pub(crate) struct CurveOverFp2Parameters<'a, FE: ElementRepr, F: SizedPrimeField<Repr = FE>> {
    pub(crate) field: &'a fp2::Extension2<'a, FE, F>,
}
impl<'a, FE: ElementRepr, F: SizedPrimeField<Repr = FE>> CurveParameters for CurveOverFp2Parameters<'a, FE, F> {
    type BaseFieldElement = fp2::Fp2<'a, FE, F>;
    fn params(&self) -> <Self::BaseFieldElement as ZeroAndOne>::Params {
        self.field
    }
}
impl<'a, FE: ElementRepr, F: SizedPrimeField<Repr = FE>> CurveOverFp2Parameters<'a, FE, F> {
    pub(crate) fn new(field: &'a fp2::Extension2<'a, FE, F>) -> Self {
        Self {
            field
        }
    }
}

use crate::extension_towers::fp3;
pub(crate) struct CurveOverFp3Parameters<'a, FE: ElementRepr, F: SizedPrimeField<Repr = FE>> {
    pub(crate) field: &'a fp3::Extension3<'a, FE, F>,
}
impl<'a, FE: ElementRepr, F: SizedPrimeField<Repr = FE>> CurveParameters for CurveOverFp3Parameters<'a, FE, F> {
    type BaseFieldElement = fp3::Fp3<'a, FE, F>;
    fn params(&self) -> <Self::BaseFieldElement as ZeroAndOne>::Params {
        self.field
    }
}
impl<'a, FE: ElementRepr, F: SizedPrimeField<Repr = FE>> CurveOverFp3Parameters<'a, FE, F> {
    pub(crate) fn new(field: &'a fp3::Extension3<'a, FE, F>) -> Self {
        Self {
            field
        }
    }
}


pub trait Group: Sized + Clone {
    fn add_assign(&mut self, other: &Self);
    fn add_assign_mixed(&mut self, other: &Self);
    fn sub_assign(&mut self, other: &Self);
    fn negate(&mut self);
    fn double(&mut self);
    fn mul<S: AsRef<[u64]>>(&self, exp: S) -> Self;
    fn wnaf_mul<S: crate::representation::IntoWnaf>(&self, exp: S) -> Self;
    fn is_zero(&self) -> bool;
    fn check_correct_subgroup(&self) -> bool;
}

pub mod curve;