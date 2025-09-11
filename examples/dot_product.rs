use common_traits::*;

#[inline]
pub fn dot_product<MT: Number, RT: Number, A, B>(a: A, b: B) -> RT
where
    A: Sequence,
    B: Sequence,
    A::Item: To<MT>,
    B::Item: To<MT>,
    MT: To<RT>,
    RT: To<MT>,
{
    // Check compatability of the vectors
    assert_eq!(a.len(), b.len());

    // Compute the dot product
    let mut accum = RT::ZERO;
    for (a, b) in a.iter().zip(b.iter()) {
        accum = (a.to()).mul_add(b.to(), accum.to()).to();
    }

    accum
}

#[cfg(any(feature = "alloc", feature = "std"))]
fn main() {
    let x: Vec<f32> = vec![1.0, 2.0, 3.0];
    let w: Vec<u8> = vec![3, 2, 1];
    let res: u16 = dot_product::<f64, _, _, _>(&x, &w);
    println!("{:?}", res);
}

#[cfg(not(any(feature = "alloc", feature = "std")))]
fn main() {
    let x: [f32; 3] = [1.0, 2.0, 3.0];
    let w: [u8; 3] = [3, 2, 1];
    let res: u16 = dot_product::<f64, _, _, _>(&x, &w);
    println!("{:?}", res);
}
