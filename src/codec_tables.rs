#![allow(unused, clippy::identity_op)]

pub(crate) const ALT_FLAG: u8 = 1 << 5;
pub(crate) const ERR_FLAG: u8 = 1 << 6;

pub(crate) const ALT_MASK: u64 = (ALT_FLAG as u64)
    | ((ALT_FLAG as u64) << 8)
    | ((ALT_FLAG as u64) << 16)
    | ((ALT_FLAG as u64) << 24)
    | ((ALT_FLAG as u64) << 32)
    | ((ALT_FLAG as u64) << 40)
    | ((ALT_FLAG as u64) << 48)
    | ((ALT_FLAG as u64) << 56);

pub(crate) const ERR_MASK: u64 = (ERR_FLAG as u64)
    | ((ERR_FLAG as u64) << 8)
    | ((ERR_FLAG as u64) << 16)
    | ((ERR_FLAG as u64) << 24)
    | ((ERR_FLAG as u64) << 32)
    | ((ERR_FLAG as u64) << 40)
    | ((ERR_FLAG as u64) << 48)
    | ((ERR_FLAG as u64) << 56);

include!(concat!(env!("OUT_DIR"), "/codec_tables.rs"));
