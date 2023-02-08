use common_traits::*;

#[derive(Debug)]
/// a 1D vector
pub struct Vector<T: Number>(Vec<T>);

impl<T: Number> Vector<T> {
   #[inline]
   pub fn dot_product<T2: Number, RT: Number>(&self, other: &Vector<T2>) -> RT
   where
       T: To<RT>,
       T2: To<T>,
   {
        // Check compatability of the vectors
        assert_eq!(self.0.len(), other.0.len());

        // Compute the dot product
        let mut accum = T::ZERO;
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            accum = (*a).mul_add(b.to(), accum);
        }

        accum.to()
   }
}

fn main() {
    let x: Vector<f32> = Vector(vec![1.0, 2.0, 3.0]);
    let w: Vector<u8> = Vector(vec![3, 2, 1]);

    let res: u16 = x.dot_product(&w);

    println!("{:?}", res);
}