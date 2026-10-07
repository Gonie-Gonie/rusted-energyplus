//! Precision of the pinned original ObjexxFCL three-point centroid product.
//!
//! `Vector3.hh:1689-1693` initializes a long-double third from binary64
//! `1.0 / 3.0`. The coordinate sum is binary64; the product is rounded to
//! the actual original x87 precision and then converted to binary64.
//! GEO-02 observed PC64, nearest-even, with unchanged controls in every
//! original helper and physical initialization observation. Integer arithmetic
//! implements those two roundings without changing thread floating controls.

pub(crate) const SOURCE_PRODUCT_PRECISION_BITS: u32 = 64;
pub(crate) const SOURCE_THIRD_BITS: u64 = (1.0_f64 / 3.0_f64).to_bits();

const SIGN: u64 = 1 << 63;
const FRACTION: u64 = (1 << 52) - 1;

/// Multiply an already evaluated, ordered binary64 sum by the source third.
pub(crate) fn multiply_by_source_third(sum: f64) -> f64 {
    if !sum.is_finite() {
        return sum * f64::from_bits(SOURCE_THIRD_BITS);
    }
    // Preserve the actual sum's sign, including an all-negative-zero sum.
    if sum == 0.0 {
        return sum;
    }
    let bits = sum.to_bits();
    let sign = bits & SIGN;
    let exponent_bits = ((bits >> 52) & 0x7ff) as i32;
    let (significand, exponent) = if exponent_bits == 0 {
        (bits & FRACTION, -1074)
    } else {
        ((bits & FRACTION) | (1 << 52), exponent_bits - 1023 - 52)
    };
    let third_significand = (SOURCE_THIRD_BITS & FRACTION) | (1 << 52);
    let third_exponent = ((SOURCE_THIRD_BITS >> 52) & 0x7ff) as i32 - 1023 - 52;
    // At most 106 significant bits. The original extended exponent range
    // contains every finite binary64-by-third product, including subnormals.
    let exact = u128::from(significand) * u128::from(third_significand);
    let discarded = (128 - exact.leading_zeros()).saturating_sub(SOURCE_PRODUCT_PRECISION_BITS);
    let product = round_right(exact, discarded);
    let product_exponent = exponent + third_exponent + discarded as i32;
    f64::from_bits(sign | binary64_magnitude(product, product_exponent))
}

/// Round a positive integer divided by a power of two, with ties to even.
fn round_right(value: u128, discarded: u32) -> u128 {
    if discarded == 0 {
        return value;
    }
    if discarded > 128 {
        return 0;
    }
    if discarded == 128 {
        return u128::from(value > (1_u128 << 127));
    }
    let kept = value >> discarded;
    let remainder = value & ((1_u128 << discarded) - 1);
    let half = 1_u128 << (discarded - 1);
    kept + u128::from(remainder > half || (remainder == half && kept & 1 != 0))
}

/// Convert the rounded extended product to binary64, including gradual underflow.
fn binary64_magnitude(product: u128, exponent: i32) -> u64 {
    let width = 128 - product.leading_zeros();
    let mut highest_exponent = width as i32 - 1 + exponent;
    if highest_exponent < -1022 {
        let discarded = -1074 - exponent;
        let subnormal = if discarded >= 0 {
            round_right(product, discarded as u32)
        } else {
            product << ((-discarded) as u32)
        };
        // Rounding to 2^52 directly encodes the smallest normal number.
        return subnormal as u64;
    }
    let mut significand = if width >= 53 {
        round_right(product, width - 53)
    } else {
        product << (53 - width)
    };
    if significand == 1_u128 << 53 {
        significand >>= 1;
        highest_exponent += 1;
    }
    if highest_exponent > 1023 {
        return 0x7ff0_0000_0000_0000;
    }
    (((highest_exponent + 1023) as u64) << 52) | (significand as u64 & FRACTION)
}
