// Copyright 2007 The RE2 Authors. All Rights Reserved.
// Pinned util/rune.cc runetochar. BSD license: ../LICENSE.RE2.
//! Exact Rune bytes, including Native surrogate and Runeerror handling.
pub(crate) fn encode_rune(rune: u32) -> ([u8; 4], usize) {
    let mut output = [0_u8; 4];
    if rune <= 0x7f {
        output[0] = rune as u8;
        return (output, 1);
    }
    if rune <= 0x7ff {
        output[0] = 0xc0 | (rune >> 6) as u8;
        output[1] = 0x80 | (rune & 0x3f) as u8;
        return (output, 2);
    }
    let rune = if rune > 0x10ffff { 0xfffd } else { rune };
    if rune <= 0xffff {
        output[0] = 0xe0 | (rune >> 12) as u8;
        output[1] = 0x80 | ((rune >> 6) & 0x3f) as u8;
        output[2] = 0x80 | (rune & 0x3f) as u8;
        return (output, 3);
    }
    output[0] = 0xf0 | (rune >> 18) as u8;
    output[1] = 0x80 | ((rune >> 12) & 0x3f) as u8;
    output[2] = 0x80 | ((rune >> 6) & 0x3f) as u8;
    output[3] = 0x80 | (rune & 0x3f) as u8;
    (output, 4)
}
