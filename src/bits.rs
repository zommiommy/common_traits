/// Shared constants of both atomic and non numerical types
pub trait Bits: Sized + Send + Sync {
    /// Number of bits in the UnsignedInt
    const BITS: usize;
    /// Number of bytes in the UnsignedInt
    const BYTES: usize;
    /// The byte array that can be use to build the value. It will always be
    ///  `[u8; Self::BYTES]`
    type Bytes: AsRef<[u8]> + AsMut<[u8]> + Copy + Default;
}
