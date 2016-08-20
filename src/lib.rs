//! Drives, Disks, Memory Sticks, Flash Cards, and other storage devices typically have a basic
//! data structure present which splits these devices into multiple parts known as "partitions".
//!
//! This crate provides mechanisms for working with certain types of partitions
//!
//!
//! Different partition types allow specifying different extra information for each partition
//!
//! The 2 things in common between all partitions are start and end.
//!
//! As a result, each partition type provides it's own `Partition` implimentor.
//!
//! Each partition-type also supports an ordering of a limited number of partitions. The maximum
//! number of partitions varies with partition-type and other items.

extern crate io_at;
extern crate io_block;

pub mod mbr;
pub mod gpt;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
