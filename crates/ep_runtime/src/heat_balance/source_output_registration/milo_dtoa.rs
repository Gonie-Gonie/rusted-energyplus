//! Literal finite Milo Grisu2/default324 dtoa, not Rust shortest-format replacement.
// Tencent is pleased to support the open source community by making RapidJSON available.
//
// Copyright (C) 2015 THL A29 Limited, a Tencent company, and Milo Yip. All rights reserved.
//
// Licensed under the MIT License (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
// http://opensource.org/licenses/MIT
//
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

// This is a C++ header-only implementation of Grisu2 algorithm from the publication:
// Loitsch, Florian. "Printing floating-point numbers quickly and accurately with
// integers." ACM Sigplan Notices 45.6 (2010): 233-243.

use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
#[derive(Clone, Copy)]
struct Diy {
    f: u64,
    e: i32,
}
impl Diy {
    fn from_double(value: f64) -> Self {
        let bits = value.to_bits();
        let biased = ((bits & 0x7ff0000000000000) >> 52) as i32;
        let significand = bits & 0x000fffffffffffff;
        if biased != 0 {
            Self {
                f: significand + (1u64 << 52),
                e: biased - 1075,
            }
        } else {
            Self {
                f: significand,
                e: -1074,
            }
        }
    }
    fn minus(self, rhs: Self) -> Self {
        Self {
            f: self.f.wrapping_sub(rhs.f),
            e: self.e,
        }
    }
    fn multiply(self, rhs: Self) -> Self {
        // Native x64 _umul128/__int128 branch, including its one high-bit rounding.
        let product = u128::from(self.f) * u128::from(rhs.f);
        let mut high = (product >> 64) as u64;
        if (product as u64) & (1u64 << 63) != 0 {
            high = high.wrapping_add(1);
        }
        Self {
            f: high,
            e: self.e + rhs.e + 64,
        }
    }
    fn normalize(self) -> Self {
        let s = self.f.leading_zeros();
        Self {
            f: self.f << s,
            e: self.e - s as i32,
        }
    }
    fn normalize_boundary(self) -> Self {
        let mut res = self;
        while res.f & ((1u64 << 52) << 1) == 0 {
            res.f <<= 1;
            res.e -= 1;
        }
        res.f <<= 10;
        res.e -= 10;
        res
    }
    fn normalized_boundaries(self) -> (Self, Self) {
        let plus = Self {
            f: (self.f << 1) + 1,
            e: self.e - 1,
        }
        .normalize_boundary();
        let mut minus = if self.f == (1u64 << 52) {
            Self {
                f: (self.f << 2) - 1,
                e: self.e - 2,
            }
        } else {
            Self {
                f: (self.f << 1) - 1,
                e: self.e - 1,
            }
        };
        minus.f <<= (minus.e - plus.e) as u32;
        minus.e = plus.e;
        (minus, plus)
    }
}
const POW10: [u64; 20] = [
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    10000000000000,
    100000000000000,
    1000000000000000,
    10000000000000000,
    100000000000000000,
    1000000000000000000,
    10000000000000000000,
];
const CACHED_F: [u64; 87] = [
    0xfa8fd5a0081c0288,
    0xbaaee17fa23ebf76,
    0x8b16fb203055ac76,
    0xcf42894a5dce35ea,
    0x9a6bb0aa55653b2d,
    0xe61acf033d1a45df,
    0xab70fe17c79ac6ca,
    0xff77b1fcbebcdc4f,
    0xbe5691ef416bd60c,
    0x8dd01fad907ffc3c,
    0xd3515c2831559a83,
    0x9d71ac8fada6c9b5,
    0xea9c227723ee8bcb,
    0xaecc49914078536d,
    0x823c12795db6ce57,
    0xc21094364dfb5637,
    0x9096ea6f3848984f,
    0xd77485cb25823ac7,
    0xa086cfcd97bf97f4,
    0xef340a98172aace5,
    0xb23867fb2a35b28e,
    0x84c8d4dfd2c63f3b,
    0xc5dd44271ad3cdba,
    0x936b9fcebb25c996,
    0xdbac6c247d62a584,
    0xa3ab66580d5fdaf6,
    0xf3e2f893dec3f126,
    0xb5b5ada8aaff80b8,
    0x87625f056c7c4a8b,
    0xc9bcff6034c13053,
    0x964e858c91ba2655,
    0xdff9772470297ebd,
    0xa6dfbd9fb8e5b88f,
    0xf8a95fcf88747d94,
    0xb94470938fa89bcf,
    0x8a08f0f8bf0f156b,
    0xcdb02555653131b6,
    0x993fe2c6d07b7fac,
    0xe45c10c42a2b3b06,
    0xaa242499697392d3,
    0xfd87b5f28300ca0e,
    0xbce5086492111aeb,
    0x8cbccc096f5088cc,
    0xd1b71758e219652c,
    0x9c40000000000000,
    0xe8d4a51000000000,
    0xad78ebc5ac620000,
    0x813f3978f8940984,
    0xc097ce7bc90715b3,
    0x8f7e32ce7bea5c70,
    0xd5d238a4abe98068,
    0x9f4f2726179a2245,
    0xed63a231d4c4fb27,
    0xb0de65388cc8ada8,
    0x83c7088e1aab65db,
    0xc45d1df942711d9a,
    0x924d692ca61be758,
    0xda01ee641a708dea,
    0xa26da3999aef774a,
    0xf209787bb47d6b85,
    0xb454e4a179dd1877,
    0x865b86925b9bc5c2,
    0xc83553c5c8965d3d,
    0x952ab45cfa97a0b3,
    0xde469fbd99a05fe3,
    0xa59bc234db398c25,
    0xf6c69a72a3989f5c,
    0xb7dcbf5354e9bece,
    0x88fcf317f22241e2,
    0xcc20ce9bd35c78a5,
    0x98165af37b2153df,
    0xe2a0b5dc971f303a,
    0xa8d9d1535ce3b396,
    0xfb9b7cd9a4a7443c,
    0xbb764c4ca7a44410,
    0x8bab8eefb6409c1a,
    0xd01fef10a657842c,
    0x9b10a4e5e9913129,
    0xe7109bfba19c0c9d,
    0xac2820d9623bf429,
    0x80444b5e7aa7cf85,
    0xbf21e44003acdd2d,
    0x8e679c2f5e44ff8f,
    0xd433179d9c8cb841,
    0x9e19db92b4e31ba9,
    0xeb96bf6ebadf77d9,
    0xaf87023b9bf0ee6b,
];
const CACHED_E: [i32; 87] = [
    -1220, -1193, -1166, -1140, -1113, -1087, -1060, -1034, -1007, -980, -954, -927, -901, -874,
    -847, -821, -794, -768, -741, -715, -688, -661, -635, -608, -582, -555, -529, -502, -475, -449,
    -422, -396, -369, -343, -316, -289, -263, -236, -210, -183, -157, -130, -103, -77, -50, -24, 3,
    30, 56, 83, 109, 136, 162, 189, 216, 242, 269, 295, 322, 348, 375, 402, 428, 455, 481, 508,
    534, 561, 588, 614, 641, 667, 694, 720, 747, 774, 800, 827, 853, 880, 907, 933, 960, 986, 1013,
    1039, 1066,
];
fn cached_power(e: i32) -> (Diy, i32) {
    let dk = f64::from(-61 - e) * 0.30102999566398114 + 347.0;
    let mut k = dk as i32;
    if dk - f64::from(k) > 0.0 {
        k += 1;
    }
    let index = ((k >> 3) + 1) as usize;
    (
        Diy {
            f: CACHED_F[index],
            e: CACHED_E[index],
        },
        -(-348 + ((index as i32) << 3)),
    )
}
fn grisu_round(
    buffer: &mut [u8],
    delta: u64,
    mut rest: u64,
    ten_kappa: u64,
    wp_w: u64,
) -> Result<(), Error> {
    while rest < wp_w
        && delta.wrapping_sub(rest) >= ten_kappa
        && (rest.wrapping_add(ten_kappa) < wp_w
            || wp_w.wrapping_sub(rest) > rest.wrapping_add(ten_kappa).wrapping_sub(wp_w))
    {
        let digit = buffer
            .last_mut()
            .ok_or(Error::Unavailable("actual Milo GrisuRound nonempty buffer"))?;
        *digit = digit.wrapping_sub(1);
        rest = rest.wrapping_add(ten_kappa);
    }
    Ok(())
}
fn digit_gen(
    w: Diy,
    mp: Diy,
    mut delta: u64,
    buffer: &mut Vec<u8>,
    k: &mut i32,
) -> Result<(), Error> {
    let shift = (-mp.e) as u32;
    let one = Diy {
        f: 1u64 << shift,
        e: mp.e,
    };
    let wp_w = mp.minus(w);
    let mut p1 = (mp.f >> shift) as u32;
    let mut p2 = mp.f & (one.f - 1);
    let mut kappa = 1;
    while kappa < 9 && u64::from(p1) >= POW10[kappa as usize] {
        kappa += 1;
    }
    buffer.clear();
    while kappa > 0 {
        let divisor = POW10[(kappa - 1) as usize] as u32;
        let d = p1 / divisor;
        p1 %= divisor;
        if d != 0 || !buffer.is_empty() {
            buffer.push(b'0' + d as u8);
        }
        kappa -= 1;
        let tmp = (u64::from(p1) << shift).wrapping_add(p2);
        if tmp <= delta {
            *k += kappa;
            grisu_round(buffer, delta, tmp, POW10[kappa as usize] << shift, wp_w.f)?;
            return Ok(());
        }
    }
    loop {
        p2 = p2.wrapping_mul(10);
        delta = delta.wrapping_mul(10);
        let d = (p2 >> shift) as u8;
        if d != 0 || !buffer.is_empty() {
            buffer.push(b'0'.wrapping_add(d));
        }
        p2 &= one.f - 1;
        kappa -= 1;
        if p2 < delta {
            *k += kappa;
            let index = (-kappa) as usize;
            grisu_round(
                buffer,
                delta,
                p2,
                one.f,
                wp_w.f
                    .wrapping_mul(if index < 20 { POW10[index] } else { 0 }),
            )?;
            return Ok(());
        }
    }
}
pub(super) fn source_dtoa(value: f64) -> Result<String, Error> {
    if !value.is_finite() {
        return Err(Error::Unavailable("actual Milo finite double domain"));
    }
    if value == 0.0 {
        return Ok(if value.is_sign_negative() {
            "-0.0"
        } else {
            "0.0"
        }
        .into());
    }
    let negative = value < 0.0;
    let v = Diy::from_double(if negative { -value } else { value });
    let (mut wm, mut wp) = v.normalized_boundaries();
    let (cached, mut k) = cached_power(wp.e);
    let w = v.normalize().multiply(cached);
    wp = wp.multiply(cached);
    wm = wm.multiply(cached);
    wm.f = wm.f.wrapping_add(1);
    wp.f = wp.f.wrapping_sub(1);
    let mut digits = Vec::new();
    digit_gen(w, wp, wp.f.wrapping_sub(wm.f), &mut digits, &mut k)?;
    let length =
        i32::try_from(digits.len()).map_err(|_| Error::Unavailable("actual Milo digit length"))?;
    let kk = length + k;
    let mut result = if negative {
        String::from("-")
    } else {
        String::new()
    };
    let text =
        std::str::from_utf8(&digits).map_err(|_| Error::Unavailable("actual Milo digit domain"))?;
    // Native Prettify's maxDecimalPlaces is its default324. Finite f64 digits
    // cannot reach its truncation branches; subnormals use the exponent branch.
    if k >= 0 && kk <= 21 {
        result.push_str(text);
        for _ in length..kk {
            result.push('0');
        }
        result.push_str(".0");
    } else if kk > 0 && kk <= 21 {
        let at = kk as usize;
        result.push_str(&text[..at]);
        result.push('.');
        result.push_str(&text[at..]);
    } else if kk > -6 && kk <= 0 {
        result.push_str("0.");
        for _ in 0..-kk {
            result.push('0');
        }
        result.push_str(text);
    } else if length == 1 {
        result.push_str(text);
        result.push('e');
        result.push_str(&(kk - 1).to_string());
    } else {
        result.push_str(&text[..1]);
        result.push('.');
        result.push_str(&text[1..]);
        result.push('e');
        result.push_str(&(kk - 1).to_string());
    }
    Ok(result)
}
