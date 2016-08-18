/// MBR was publicly introduced in 1983 with PC DOS 2.0.
///
/// This provides support only for original (basic) MBR layout and modern MBR layout.
/// Support for Windows LDM and other MBR variants that depart from the basic structure may be
/// supported elsewhere.
///
///
use std::{time};
use std::convert::{From,Into};
use io_at;
use io_at::{WriteAt,ReadAt};
use io_block::{BlockSize};

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum PartRef {
    /** N partitions before this one. 0 is the current partition. 1 is the one immediately
     * previous. */
    Previous(u32),

    /** N partitions after this one. 0 is the one immediately after. */
    Next(u32),

    /** Partition with number N. 0 is the first partition (all partitions are numbered from 0. */
    Exact(u32),
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum LocSpec {
    /** At the end of a partition */
    AtEndOf(PartRef),

    /** At the start of a partition */
    AtStartOf(PartRef),

    /*
    /** Offset by N bytes from another location */
    pub Offset(LocSpec, i64),

    /** Align the location rounding to the next location divisible by N bytes */
    pub AlignNext(LocSpec, u64),

    /** Align the location rounding to the previous location divisible by N bytes */
    pub AlignPrev(LocSpec, u64),
    */
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum NumSpec {
    Exact(u32),
    /*
    AfterPart(PartRef),
    BeforePart(PartRef),
    */
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum PartSpec {
    Number(NumSpec),
    Start(LocSpec),
    End(LocSpec),
    IsBootable
}

/// Each partition spec (aka request) supplies a series of constraints that should be satisfied by
/// the concrete (relealized, actual) partition. Convertion to a real partition is handled by
/// `MbrBuilder::compile()`.
#[derive(Clone,PartialEq,Eq,Debug)]
struct MbrPartSpec {
    specs: Vec<PartSpec>,
}

/// A physical (real) MBR partition with all associated attributes
#[derive(Clone)]
struct MbrPart {
    number: u32,
    start: u64,
    end: u64
}

impl MbrPart {
    pub fn is_primary(&self) -> bool {
        self.number < 4
    }

    pub fn is_extended(&self) -> bool {
        !self.is_primary()
    }
}

#[derive(Clone,PartialEq,Eq)]
enum MbrBuilderError {
}

/// Allows creating and commiting a new MBR to a WriteAt-able BlockSize-able thing (typically, a
/// block device).
#[derive(Clone)]
struct MbrBuilder {
    bootcode: Option<Vec<u8>>,
    bootcode_2: Option<Vec<u8>>,
    partitions: Option<Vec<MbrPart>>,
    timestamp: Option<time::SystemTime>,
    original_physical_drive: Option<u8>,
    disk_sig: Option<u8>,
}

impl MbrBuilder {
    // TODO: consider determining presense of data prior to writing
    pub fn new() -> Self {
        MbrBuilder {
            bootcode: None,
            bootcode_2: None,
            partitions: None,
            timestamp: None,
            original_physical_drive: None,
            disk_sig: None
        }
    }

    /// MBR contains a block of "bootcode" that is 446 bytes long in classic MBR or 226 bytes long
    /// in modern MBR (for the first half of it).
    ///
    /// This function lets you set the bootcode. Slices less than 446 bytes will be padded with
    /// zeros (this may not be ideal consider carefully).
    ///
    /// Panics:
    ///
    ///  - if code.len() is too long for the type of MBR being constructed.
    pub fn set_bootcode(self, code: &[u8]) -> Self {
        unimplemented!();
        self
    }

    /// In place of some of the bootcode, modern MBR can contain a disk timestamp (seconds,
    /// minutes, hours). The same space may alternately be populated by a OEM loader signature with
    /// NEWLDR.
    ///
    /// This is entirely optional (and probably unlikely to be used
    pub fn set_timestamp(self, ts: time::SystemTime) -> Self {
        unimplemented!();
        self
    }

    /// Considered a piece of the timestamp set by `set_timestamp()`
    ///
    /// `drv` is intended to be a BIOS drive number (0x80 to 0xFF).
    pub fn set_original_physical_drive(self, drv: u8) -> Self {
        unimplemented!();
        self
    }

    /// In modern MBR, bootcode is split into 2 pieces: 1x226 bytes at byte 0, and 1x216 (or 1x222)
    /// at +224 bytes.
    ///
    /// This sets the second part of the bootcode.
    pub fn set_bootcode_part2(self, code: &[u8]) -> Self {
        unimplemented!();
        self
    }

    /// An optional component of the partition table.
    ///
    /// TODO: note the format of `sig` here
    ///
    /// `extra` is normally 0x0000, but may be 0x5A5A to mark the disk as copy protected.
    ///
    /// Adding this element shrinks the 2nd bootcode part (`set_bootcode_part2()`) as it occupies
    /// space at bootcode_part2's end.
    pub fn set_disk_signature(self, sig: u32, extra: u16) -> Self {
        unimplemented!();
        self
    }

    /// Confirm that the MBR specified by our building is buildable, and convert it into a
    /// MbrWriter which may be used to commit the MBR to disk
    pub fn compile(self) -> Result<MbrWriter, MbrBuilderError> {
        unimplemented!();
        Ok(MbrWriter { inner: self })
    }
}

/// A MBR specification that may be directly commited to a device.
struct MbrWriter {
    inner: MbrBuilder,
}

impl MbrWriter {
    /// Commit the MBR we've built up here to a backing store.
    ///
    /// Note that no attempt to preseve the existing contents of the backing store will be made by
    /// _this_ function. Preservation is handled elsewhere by pre-configuring the builder.
    ///
    /// It is recommended that you ensure no unintended changes are made between read & commit.
    pub fn commit<T: WriteAt + BlockSize>(&self, back: T) -> io_at::Result<()> {
        unimplemented!();
        Ok(())
    }

}

/*
impl From<MbrReader> for MbrWriter {}
impl From<MbrReader> for MbrBuilder {}
impl TryFrom<[u8;512]> for MbrReader {}
*/

struct MbrReader<T: ReadAt + BlockSize> {
    store: T,
}

impl<T: ReadAt + BlockSize> MbrReader<T> {
    pub fn from_blockdev(back: T) -> Self {
        MbrReader { store: back }
    }
}
