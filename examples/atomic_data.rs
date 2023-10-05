use common_traits::*;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct MyVec<T> {
    data: Vec<T>,
}

pub trait Get<T: Boolean> {
    type Item: Copy;
    fn get(&self, index: usize) -> Self::Item;
}

impl<T: Atomic> Get<True> for MyVec<T>
where
    T::NonAtomicType: Copy,
{
    type Item = T::NonAtomicType;
    fn get(&self, index: usize) -> Self::Item {
        self.data[index].load(Ordering::SeqCst)
    }
}

impl<T: NonAtomic + Copy> Get<False> for MyVec<T> {
    type Item = T;
    fn get(&self, index: usize) -> Self::Item {
        self.data[index]
    }
}

fn get<T: Boolean, V: Get<T>>(v: &V, index: usize) -> V::Item {
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
