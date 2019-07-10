use std::io;

use crate::error::VerificationResult;

pub trait Reader<'r>: Sized + Verifiable {
    fn as_slice(&self) -> &[u8];
    fn from_slice<'a: 'r>(slice: &'a [u8]) -> VerificationResult<Self>;
}

pub trait Verifiable {
    fn verify(slice: &[u8]) -> VerificationResult<()>;
}

pub trait Builder {
    type Output;
    fn calc_len(&self) -> usize;
    fn write<W: io::Write>(&self, writer: &mut W) -> io::Result<()>;
    fn build(&self) -> io::Result<Self::Output>;
}