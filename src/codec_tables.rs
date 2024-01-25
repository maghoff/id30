pub(crate) const ALT_FLAG: u8 = 1 << 5;
pub(crate) const ERR_FLAG: u8 = 1 << 6;

include!(concat!(env!("OUT_DIR"), "/codec_tables.rs"));
