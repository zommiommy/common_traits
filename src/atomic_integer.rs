use crate::{AtomicNumber, Integer, SignedInt, UnsignedInt};

pub trait AtomicInteger: AtomicNumber
where
    Self::NonAtomicType: Integer,
{
}

pub trait AtomicSignedInt: AtomicInteger
where
    Self::NonAtomicType: SignedInt,
{
}

pub trait AtomicUnsignedInt: AtomicInteger
where
    Self::NonAtomicType: UnsignedInt,
{
}
