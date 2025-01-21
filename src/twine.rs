
use std::{ffi::{CStr, CString, OsStr, OsString}, path::{Path, PathBuf}};

#[derive(Debug)]
/// like `std::borrow::Cow` but that also supports most rust strings.
pub enum TwineCow<'a> {
    Byte(u8),
    Char(char),
    Borrowed(&'a str),
    Owned(String),
    BorrowedVec(&'a [char]),
    OwnedBytes(Vec<u8>),
    BorrowedBytes(&'a [u8]),
    OwnedVec(Vec<char>),
    BorrowedPath(&'a Path),
    OwnedPath(PathBuf),
    BorrowedOs(&'a OsStr),
    OwnedOs(OsString),
    CBorrowed(&'a CStr),
    COwned(CString),
}

impl<'a> core::fmt::Display for TwineCow<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TwineCow::Byte(value) => write!(f, "{}", value),
            TwineCow::Char(value) => write!(f, "{}", value),

            TwineCow::Borrowed(value) => write!(f, "{}", value),
            TwineCow::Owned(value) => write!(f, "{}", value),

            TwineCow::OwnedBytes(value) => write!(f, "{}", String::from_utf8_lossy(value)),
            TwineCow::BorrowedBytes(value) => write!(f, "{}", String::from_utf8_lossy(value)),

            TwineCow::BorrowedVec(value) => write!(f, "{}", value.iter().collect::<String>()),
            TwineCow::OwnedVec(value) => write!(f, "{}", value.iter().collect::<String>()),

            TwineCow::BorrowedPath(value) => write!(f, "{}", value.display()),
            TwineCow::OwnedPath(value) => write!(f, "{}", value.display()),

            TwineCow::BorrowedOs(value) => write!(f, "{}", value.to_string_lossy()),
            TwineCow::OwnedOs(value) => write!(f, "{}", value.to_string_lossy()),

            TwineCow::CBorrowed(value) => write!(f, "{}", value.to_string_lossy()),
            TwineCow::COwned(value) => write!(f, "{}", value.to_string_lossy()),
        }
    }
}

impl<'a> From<u8> for TwineCow<'a> {
    fn from(value: u8) -> Self {
        TwineCow::Byte(value)
    }
}
impl<'a> From<char> for TwineCow<'a> {
    fn from(value: char) -> Self {
        TwineCow::Char(value)
    }
}
impl<'a> From<&'a str> for TwineCow<'a> {
    fn from(value: &'a str) -> Self {
        TwineCow::Borrowed(value)
    }
}
impl<'a> From<String> for TwineCow<'a> {
    fn from(value: String) -> Self {
        TwineCow::Owned(value)
    }
}
impl<'a> From<&'a [char]> for TwineCow<'a> {
    fn from(value: &'a [char]) -> Self {
        TwineCow::BorrowedVec(value)
    }
}
impl<'a> From<Vec<char>> for TwineCow<'a> {
    fn from(value: Vec<char>) -> Self {
        TwineCow::OwnedVec(value)
    }
}
impl<'a> From<&'a [u8]> for TwineCow<'a> {
    fn from(value: &'a [u8]) -> Self {
        TwineCow::BorrowedBytes(value)
    }
}
impl<'a> From<Vec<u8>> for TwineCow<'a> {
    fn from(value: Vec<u8>) -> Self {
        TwineCow::OwnedBytes(value)
    }
}
impl<'a> From<&'a Path> for TwineCow<'a> {
    fn from(value: &'a Path) -> Self {
        TwineCow::BorrowedPath(value)
    }
}
impl<'a> From<PathBuf> for TwineCow<'a> {
    fn from(value: PathBuf) -> Self {
        TwineCow::OwnedPath(value)
    }
}
impl<'a> From<&'a OsStr> for TwineCow<'a> {
    fn from(value: &'a OsStr) -> Self {
        TwineCow::BorrowedOs(value)
    }
}
impl<'a> From<OsString> for TwineCow<'a> {
    fn from(value: OsString) -> Self {
        TwineCow::OwnedOs(value)
    }
}
impl<'a> From<&'a CStr> for TwineCow<'a> {
    fn from(value: &'a CStr) -> Self {
        TwineCow::CBorrowed(value)
    }
}
impl<'a> From<CString> for TwineCow<'a> {
    fn from(value: CString) -> Self {
        TwineCow::COwned(value)
    }
}

impl<'a> TwineCow<'a> {
    /// Like clone, but convert all Owned parts to borrowed parts for a cheaper.
    pub fn clone_weak(&self) -> Self {
        match self {
            TwineCow::Byte(value) => TwineCow::Byte(*value),
            TwineCow::Char(value) => TwineCow::Char(*value),

            TwineCow::Borrowed(value) => TwineCow::Borrowed(value),
            TwineCow::Owned(value) => TwineCow::Borrowed(&value),

            TwineCow::OwnedBytes(value) => TwineCow::BorrowedBytes(&value),
            TwineCow::BorrowedBytes(value) => TwineCow::BorrowedBytes(value),

            TwineCow::BorrowedVec(value) => TwineCow::BorrowedVec(value),
            TwineCow::OwnedVec(value) => TwineCow::BorrowedVec(&value),

            TwineCow::BorrowedPath(value) => TwineCow::BorrowedPath(value),
            TwineCow::OwnedPath(value) => TwineCow::BorrowedPath(&value),

            TwineCow::BorrowedOs(value) => TwineCow::BorrowedOs(value),
            TwineCow::OwnedOs(value) => TwineCow::BorrowedOs(&value),

            TwineCow::CBorrowed(value) => TwineCow::CBorrowed(value),
            TwineCow::COwned(value) => TwineCow::CBorrowed(value),
        }
    
    } 
}

impl<'a> ToOwned for TwineCow<'a> {
    type Owned = TwineCow<'static>;

    fn to_owned(&self) -> TwineCow<'static> {
        match self {
            TwineCow::Byte(value) => TwineCow::Byte(*value),
            TwineCow::Char(value) => TwineCow::Char(*value),

            TwineCow::Borrowed(value) => TwineCow::Owned((*value).to_owned()),
            TwineCow::BorrowedBytes(value) => TwineCow::OwnedBytes((*value).to_owned()),
            TwineCow::BorrowedVec(value) => TwineCow::OwnedVec((*value).to_owned()),
            TwineCow::BorrowedPath(value) => TwineCow::OwnedPath((*value).to_owned()),
            TwineCow::BorrowedOs(value) => TwineCow::OwnedOs((*value).to_owned()),
            TwineCow::CBorrowed(value) => TwineCow::COwned((*value).to_owned()),

            TwineCow::Owned(value) => TwineCow::Owned(value.clone()),
            TwineCow::OwnedBytes(value) => TwineCow::OwnedBytes(value.clone()),
            TwineCow::OwnedVec(value) => TwineCow::OwnedVec(value.clone()),
            TwineCow::OwnedPath(value) => TwineCow::OwnedPath(value.clone()),
            TwineCow::OwnedOs(value) => TwineCow::OwnedOs(value.clone()),
            TwineCow::COwned(value) => TwineCow::COwned(value.clone()),
        }
    }
}

/// Twine - A lightweight data structure for efficiently representing the 
/// concatenation of temporary values as strings ported from LLVM.
/// 
/// A Twine is a kind of rope, it represents a concatenated string using a 
/// binary-tree, where the string is the preorder of the nodes. Since the Twine 
/// can be efficiently rendered into a buffer when its result is used, it avoids 
/// the cost of generating temporary values for intermediate string results – 
/// particularly in cases when the Twine result is never required. By explicitly 
/// tracking the type of leaf nodes, we can also avoid the creation of temporary 
/// strings for conversions operations (such as appending an integer to a 
/// string).
/// 
/// Unlike LLVM's Twine which is unsafe as it uses raw pointers and it's
/// explicitly designed to be used fro temporary values and not to be stored, 
/// This version is safe and can be stored and used as a regular string because
/// we can force the Twine lifetime to be covariant with all the lifetimes of
/// the strings it contains.
/// 
/// # Reference
/// - <https://llvm.org/doxygen/classllvm_1_1Twine.html#details>
#[derive(Clone, Default)]
pub struct Twine<'a> {
    tree: Vec<TwineCow<'a>>,
}

impl<'a> Twine<'a> {
    /// Like clone, but convert all Owned parts to borrowed parts for a cheaper
    /// clone.
    pub fn weak_clone(&self) -> Twine<'_> {
        todo!();
    }

    /// Clone all borrowed parts to owned parts so that its lifetime can be 
    /// extended to static.
    pub fn into_owned(self) -> Twine<'static> {
        todo!();
    }

    /// Append the given Twine to this one.
    pub fn push(&mut self, rhs: TwineCow<'a>) {
        todo!();
    }
}

impl<'a> core::fmt::Display for Twine<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!();
    }
}

impl<'a> From<TwineCow<'a>> for Twine<'a> {
    fn from(value: TwineCow<'a>) -> Self {
        let mut twine = Self::default();
        twine.push(value);
        twine
    }
}
impl<'a> From<char> for Twine<'a> {
    fn from(value: char) -> Self {
        let mut twine = Self::default();
        twine.push(TwineCow::Char(value));
        twine
    }
}
impl<'a> From<&'a str> for Twine<'a> {
    fn from(value: &'a str) -> Self {
        let mut twine = Self::default();
        twine.push(TwineCow::Borrowed(value));
        twine
    }
}
impl<'a> From<&'a [char]> for Twine<'a> {
    fn from(value: &'a [char]) -> Self {
        let mut twine = Self::default();
        twine.push(TwineCow::BorrowedVec(value));
        twine
    }
}
impl<'a> From<&'a CStr> for Twine<'a> {
    fn from(value: &'a CStr) -> Self {
        let mut twine = Self::default();
        twine.push(TwineCow::CBorrowed(value));
        twine
    }
}
impl<'a> From<String> for Twine<'a> {
    fn from(value: String) -> Self {
        let mut twine = Self::default();
        twine.push(TwineCow::Owned(value));
        twine
    }
}
impl<'a> From<Vec<char>> for Twine<'a> {
    fn from(value: Vec<char>) -> Self {
        let mut twine = Self::default();
        twine.push(TwineCow::OwnedVec(value));
        twine
    }
}
impl<'a> From<CString> for Twine<'a> {
    fn from(value: CString) -> Self {
        let mut twine = Self::default();
        twine.push(TwineCow::COwned(value));
        twine
    }
}

impl<'a> core::ops::AddAssign<TwineCow<'a>> for Twine<'a> {
    fn add_assign(&mut self, rhs: TwineCow<'a>) {
        self.push(rhs);
    }
}
impl<'a> core::ops::AddAssign<char> for Twine<'a> {
    fn add_assign(&mut self, rhs: char) {
        self.push(TwineCow::Char(rhs));
    }
}
impl<'a> core::ops::AddAssign<&'a str> for Twine<'a> {
    fn add_assign(&mut self, rhs: &'a str) {
        self.push(TwineCow::Borrowed(rhs));
    }
}
impl<'a> core::ops::AddAssign<&'a [char]> for Twine<'a> {
    fn add_assign(&mut self, rhs: &'a [char]) {
        self.push(TwineCow::BorrowedVec(rhs));
    }
}
impl<'a> core::ops::AddAssign<&'a CStr> for Twine<'a> {
    fn add_assign(&mut self, rhs: &'a CStr) {
        self.push(TwineCow::CBorrowed(rhs));
    }
}
impl<'a> core::ops::AddAssign<String> for Twine<'a> {
    fn add_assign(&mut self, rhs: String) {
        self.push(TwineCow::Owned(rhs));
    }
}
impl<'a> core::ops::AddAssign<Vec<char>> for Twine<'a> {
    fn add_assign(&mut self, rhs: Vec<char>) {
        self.push(TwineCow::OwnedVec(rhs));
    }
}
impl<'a> core::ops::AddAssign<CString> for Twine<'a> {
    fn add_assign(&mut self, rhs: CString) {
        self.push(TwineCow::COwned(rhs));
    }
}



