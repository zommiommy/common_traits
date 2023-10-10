use common_traits::*;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct MyVec<T> {
    data: Vec<T>,
}

pub trait Get {
    type Item: Copy;
    fn get(&self, index: usize) -> Self::Item;
}

impl<T: IsAtomic + GetHelper<T::Atomic>> Get for MyVec<T> {
    type Item = T::Item;
    fn get(&self, index: usize) -> Self::Item {
        GetHelper::get(self, index)
    }
}

pub trait GetHelper<T: Boolean>: Sized {
    type Item: Copy;
    fn get(data: &MyVec<Self>, index: usize) -> Self::Item;
}

impl<T: Atomic> GetHelper<True> for T
where
    T::NonAtomicType: Copy,
{
    type Item = T::NonAtomicType;
    fn get(data: &MyVec<Self>, index: usize) -> Self::Item {
        data.data[index].load(Ordering::SeqCst)
    }
}

impl<T: IntoAtomic + Copy> GetHelper<False> for T {
    type Item = T;
    fn get(data: &MyVec<Self>, index: usize) -> Self::Item {
        data.data[index]
    }
}

fn get<V: Get>(v: &V, index: usize) -> V::Item {
    v.get(index)
}

fn main() {
    let v: MyVec<usize> = MyVec {
        data: vec![1, 2, 3],
    };
    let res = get(&v, 0);
    println!("{:?}", res);

    let v: MyVec<AtomicUsize> = MyVec {
        data: vec![
            AtomicUsize::new(1),
            AtomicUsize::new(2),
            AtomicUsize::new(3),
        ],
    };
    let res = get(&v, 0);
    println!("{:?}", res);
}
