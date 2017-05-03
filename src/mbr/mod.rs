/// MBR was publicly introduced in 1983 with PC DOS 2.0.
///
/// This provides support only for original (basic) MBR layout and modern MBR layout.
/// Support for Windows LDM and other MBR variants that depart from the basic structure may be
/// supported elsewhere.
///
/// UEFI Spec 2.6, 5.2.1 documents the expected format for use with UEFI.
/// 
//use std::convert::{From,Into};
use io_block::{BlockSize};
use io_at::{ReadAt};

pub mod writer;
pub mod header;


/*
impl From<MbrReader> for MbrWriter {}
impl From<MbrReader> for MbrBuilder {}
impl TryFrom<[u8;512]> for MbrReader {}
*/

pub struct MbrReader<T: ReadAt + BlockSize> {
    store: T,
}

impl<T: ReadAt + BlockSize> MbrReader<T> {
    pub fn from_blockdev(back: T) -> Self {
        MbrReader { store: back }
    }

}
