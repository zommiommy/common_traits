use core::result::Result;

pub trait ResultMonad<T, E>: Into<Result<T, E>> {
    /// Trick to be able to 
    type BiTypeConstructor<T1, E1>: ResultMonad<T1, E1, BiTypeConstructor<T, E> = Self>;

    fn is_ok(&self) -> bool;
    fn is_err(&self) -> bool;
    fn map<T2>(self, f: impl FnOnce(T) -> T2) -> Self::BiTypeConstructor<T2, E>;
    fn map_err<E2>(self, f: impl FnOnce(E) -> E2) -> Self::BiTypeConstructor<T, E2>;
    fn unwrap(self) -> T
    where 
        E: core::fmt::Debug;
    fn unwrap_err(self) -> E
    where
        T: core::fmt::Debug;
    // ... all the other methods from Result
}

impl<T, E> ResultMonad<T, E> for Result<T, E> {
    type BiTypeConstructor<T1, E1> = Result<T1, E1>;
    fn is_ok(&self) -> bool {
        self.is_ok()
    }
    fn is_err(&self) -> bool {
        self.is_err()
    }
    fn map<T2>(self, f: impl FnOnce(T) -> T2) -> Self::BiTypeConstructor<T2, E> {
        <Result<T, E>>::map(self, f)
    }
    fn map_err<E2>(self, f: impl FnOnce(E) -> E2) -> Self::BiTypeConstructor<T, E2> {
        <Result<T, E>>::map_err(self, f)
    }
    fn unwrap(self) -> T 
    where
        E: core::fmt::Debug,
    {
        <Result<T, E>>::unwrap(self)
    }
    fn unwrap_err(self) -> E 
    where
        T: core::fmt::Debug,
    {
        <Result<T, E>>::unwrap_err(self)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
/// An always successful result
pub struct ConstOk<T, E>(T, core::marker::PhantomData<E>);

impl<T, E> ResultMonad<T, E> for ConstOk<T, E> {
    type BiTypeConstructor<T1, E1> = ConstOk<T1, E1>;
    fn is_ok(&self) -> bool {
        true
    }
    fn is_err(&self) -> bool {
        false
    }
    fn map<T2>(self, f: impl FnOnce(T) -> T2) -> Self::BiTypeConstructor<T2, E> {
        ConstOk(f(self.0), core::marker::PhantomData)
    }
    fn map_err<E2>(self, _f: impl FnOnce(E) -> E2) -> Self::BiTypeConstructor<T, E2> {
        ConstOk(self.0, core::marker::PhantomData)
    }
    fn unwrap(self) -> T
        where 
            E: core::fmt::Debug {
        self.0
    }
    fn unwrap_err(self) -> E
        where 
            T: core::fmt::Debug {
        panic!("Cannot unwrap Err on ConstOk: {:?}", self.0);
    }
}
impl<T, E> Into<Result<T, E>> for ConstOk<T, E> {
    fn into(self) -> Result<T, E> {
        Result::Ok(self.0)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
/// An always failing result
pub struct ConstErr<T, E>(E, core::marker::PhantomData<T>);

impl<T, E> ResultMonad<T, E> for ConstErr<T, E> {
    type BiTypeConstructor<T1, E1> = ConstErr<T1, E1>;
    fn is_ok(&self) -> bool {
        false
    }
    fn is_err(&self) -> bool {
        true
    }
    fn map<T2>(self, _f: impl FnOnce(T) -> T2) -> Self::BiTypeConstructor<T2, E> {
        ConstErr(self.0, core::marker::PhantomData)
    }
    fn map_err<E2>(self, f: impl FnOnce(E) -> E2) -> Self::BiTypeConstructor<T, E2> {
        ConstErr(f(self.0), core::marker::PhantomData)
    }
    fn unwrap(self) -> T
        where 
            E: core::fmt::Debug {
        panic!("Cannot unwrap Ok on ConstErr: {:?}", self.0);
    }
    fn unwrap_err(self) -> E
        where 
            T: core::fmt::Debug {
        self.0
    }
}
impl<T, E> Into<Result<T, E>> for ConstErr<T, E> {
    fn into(self) -> Result<T, E> {
        Result::Err(self.0)
    }
}