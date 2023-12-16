use std::ffi::CStr;
use std::str;

use crate::{Sf2Error, Sf2Result};

pub(crate) fn str_from_zstr(data: &[u8]) -> Sf2Result<&str> {
    CStr::from_bytes_until_nul(data)
        .map_err(|_| Sf2Error::MalformedZstr)?
        .to_str()
        .map_err(|_| Sf2Error::MalformedZstr)
}

pub(crate) fn str_from_fixedstr(data: &[u8]) -> Sf2Result<&str> {
    // Fixed-length strings may contain garbage after the zero-terminator that may
    // cause issues with the string conversion. (GeneralUser GS)
    let terminator_pos = data.iter().position(|&b| b == b'\0').unwrap_or(data.len());

    str::from_utf8(&data[..terminator_pos]).map_err(|_| Sf2Error::MalformedFixedstr)
}
