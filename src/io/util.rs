#[cfg(feature = "nightly")]
use core::mem::MaybeUninit;
#[cfg(feature = "nightly")]
use std::io::ReadBuf;

use crate::io::{ErrorKind, Read, Write};

#[cfg(feature = "nightly")]
pub fn copy<R: ?Sized, W: ?Sized, const S: usize>(
    reader: &mut R,
    writer: &mut W,
) -> crate::io::Result<u64>
where
    R: Read,
    W: Write,
{
    let mut buf = [MaybeUninit::<u8>::uninit(); S];

    let mut read_buf = ReadBuf::uninit(&mut buf);
    let mut written = 0;
    loop {
        match reader.read_buf(&mut read_buf) {
            Ok(()) => {
                if read_buf.filled().is_empty() {
                    return Ok(written);
                }
                written += read_buf.filled().len() as u64;
                writer.write_all(read_buf.filled())?;
            }
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
    }
}

#[cfg(not(feature = "nightly"))]
pub fn copy<R: ?Sized, W: ?Sized, const S: usize>(
    reader: &mut R,
    writer: &mut W,
) -> crate::io::Result<u64>
where
    R: Read,
    W: Write,
{
    let mut buf = [0; S];

    let mut written = 0;
    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        writer.write_all(&buf[..len])?;
        written += len as u64;
    }
}
