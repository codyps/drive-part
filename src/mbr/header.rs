fn r32(x: &[u8]) -> u32 {
    x[0] as u32 | (x[1] as u32) << 8 | (x[2] as u32) << 16 | (x[3] as u32) << 24
}

pub struct MbrHeader<'a> {
    data: &'a [u8;512]
}

impl<'a> MbrHeader<'a> {
    pub fn from_bytes(data: &'a [u8;512]) -> Self {
        MbrHeader { data: data }
    }

    pub fn bootsig(&self) -> [u8;2] {
        *index_fixed!(&self.data; 510, .. 512)
    }

    pub fn bootsig_is_valid(&self) -> bool {
        self.bootsig() == [0x55, 0xAA]
    }

    pub fn disk_sig(&self) -> u32 {
        r32(&self.data[440..444])
    }

    pub fn copy_protect(&self) -> u16 {
        self.data[444] as u16 | (self.data[445] as u16) << 8
    }

    pub fn primary_partitions(&self) -> [PartitionEntry;4] {
        [
            PartitionEntry { data: index_fixed!(&self.data; 446, ..462) },
            PartitionEntry { data: index_fixed!(&self.data; 462, ..478) },
            PartitionEntry { data: index_fixed!(&self.data; 478, ..494) },
            PartitionEntry { data: index_fixed!(&self.data; 494, ..510) },
        ]
    }
}

pub struct PartitionEntry<'a> {
    data: &'a [u8;16]
}

pub enum PartitionStatus {
    /// "Bootable"
    Active,
    /// "Not bootable"
    Inactive,
    /// 0x01 to 0x7f are invalid
    Invalid(u8),
}

impl<'a> PartitionEntry<'a> {
    pub fn status(&self) -> PartitionStatus {
        match self.data[0] {
            0  => PartitionStatus::Inactive,
            0x80  => PartitionStatus::Active,
            x => PartitionStatus::Invalid(x),
        }
    }

    /// Logical Block Address (ie: block number) of the first block in the partition
    pub fn lba_first(&self) -> u32 {
        r32(&self.data[8..0xC])
    }

    /// Size of the partition in logical blocks
    pub fn lba_size(&self) -> u32 {
        r32(&self.data[0xC..(0xC+4)])
    }

    /// Partition type
    pub fn part_type(&self) -> u8 {
        self.data[4]
    }

    pub fn chs_first(&self) -> Chs {
        Chs { data: *index_fixed!(&self.data; 1,..4) }
    }

    pub fn chs_last(&self) -> Chs {
        Chs { data: *index_fixed!(&self.data; 5,..8) }
    }
}

/// Cylinder, Head, Sector address
///
/// This is legacy stuff. Look at the LBA instead. If CHS can't represent the value, this will be
/// 0xFFFFFF, (1023, 255, 63) on UEFI systems. Others may use (1023, 254, 63) for the same meaning.
pub struct Chs {
    data: [u8;3]
}

impl Chs {
    /// 10-bit cylinder
    pub fn c(&self) -> u16 {
        self.data[2] as u16 | (((self.data[1] >> 6) as u16) << 8)
    }

    /// 8-bit head
    pub fn h(&self) -> u8 {
        self.data[0]
    }

    /// 6-bit sector
    pub fn s(&self) -> u8 {
        self.data[1] & ((1 << 6) - 1)
    }
}
