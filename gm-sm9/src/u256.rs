pub type U256 = [u64; 4];
pub type U512 = [u64; 8];

pub(crate) const SM9_ZERO: U256 = [0, 0, 0, 0];
pub(crate) const SM9_ONE: U256 = [1, 0, 0, 0];
pub(crate) const SM9_TWO: U256 = [2, 0, 0, 0];
pub(crate) const SM9_FIVE: U256 = [5, 0, 0, 0];

#[inline(always)]
pub const fn u256_add(a: &U256, b: &U256) -> (U256, bool) {
    let mut sum = [0; 4];
    let mut carry = false;
    let mut i = 0;
    loop {
        let (t_sum, c) = {
            let (m, c1) = a[i].overflowing_add(b[i]);
            let (r, c2) = m.overflowing_add(carry as u64);
            (r, c1 || c2)
        };
        sum[i] = t_sum;
        carry = c;
        if i == 3 {
            break;
        }
        i += 1;
    }
    (sum, carry)
}

#[inline(always)]
pub const fn u512_add(a: &U512, b: &U512) -> (U512, bool) {
    let mut sum = [0; 8];
    let mut carry = false;
    let mut i = 0;
    loop {
        let (t_sum, c) = {
            let (m, c1) = a[i].overflowing_add(b[i]);
            let (r, c2) = m.overflowing_add(carry as u64);
            (r, c1 || c2)
        };
        sum[i] = t_sum;
        carry = c;
        if i == 7 {
            break;
        }
        i += 1;
    }
    (sum, carry)
}

#[inline(always)]
pub const fn u256_sub(a: &U256, b: &U256) -> (U256, bool) {
    let mut r = [0; 4];
    let mut borrow = false;
    let mut j = 3;
    loop {
        let i = 3 - j;
        let (diff, bor) = {
            let (a, b1) = a[i].overflowing_sub(borrow as u64);
            let (res, b2) = a.overflowing_sub(b[i]);
            (res, b1 || b2)
        };
        r[i] = diff;
        borrow = bor;
        if j == 0 {
            break;
        }
        j -= 1;
    }
    (r, borrow)
}

#[inline(always)]
pub const fn u512_sub(a: &U512, b: &U512) -> (U512, bool) {
    let mut r = [0; 8];
    let mut borrow = false;
    let mut j = 7;
    loop {
        let i = 7 - j;
        let (diff, bor) = {
            let (a, b1) = a[i].overflowing_sub(borrow as u64);
            let (res, b2) = a.overflowing_sub(b[i]);
            (res, b1 || b2)
        };
        r[i] = diff;
        borrow = bor;
        if j == 0 {
            break;
        }
        j -= 1;
    }
    (r, borrow)
}

#[inline(always)]
pub const fn u256_cmp(a: &U256, b: &U256) -> i32 {
    if a[3] > b[3] {
        return 1;
    }
    if a[3] < b[3] {
        return -1;
    }
    if a[2] > b[2] {
        return 1;
    }
    if (a[2] < b[2]) {
        return -1;
    }
    if (a[1] > b[1]) {
        return 1;
    }
    if a[1] < b[1] {
        return -1;
    }
    if a[0] > b[0] {
        return 1;
    }
    if a[0] < b[0] {
        return -1;
    }
    return 0;
}

#[inline(always)]
pub fn u256_mul(a: &U256, b: &U256) -> U512 {
    let mut a_: [u64; 8] = [0; 8];
    let mut b_: [u64; 8] = [0; 8];
    let mut ret: [u64; 8] = [0; 8];
    let mut s: [u64; 16] = [0; 16];

    for i in 0..4 {
        a_[2 * i] = a[i] & 0xffffffff;
        b_[2 * i] = b[i] & 0xffffffff;
        a_[2 * i + 1] = a[i] >> 32;
        b_[2 * i + 1] = b[i] >> 32;
    }

    let mut u = 0;
    for i in 0..8 {
        u = 0;
        for j in 0..8 {
            u = s[i + j] + a_[i] * b_[j] + u;
            s[i + j] = u & 0xffffffff;
            u >>= 32;
        }
        s[i + 8] = u;
    }

    for i in 0..8 {
        ret[i] = (s[2 * i + 1] << 32) | s[2 * i];
    }
    ret
}

#[inline(always)]
pub fn sm9_u256_to_bytes(a: &U256) -> [u8; 32] {
    let mut out = [0; 32];
    out[0..8].copy_from_slice(&a[3].to_le_bytes());
    out[8..16].copy_from_slice(&a[2].to_le_bytes());
    out[16..24].copy_from_slice(&a[1].to_le_bytes());
    out[24..32].copy_from_slice(&a[0].to_le_bytes());
    out
}

#[inline(always)]
pub fn sm9_u256_from_bytes(input: &[u8; 32]) -> U256 {
    let mut r: U256 = [0_u64; 4];
    r[3] = u64::from_le_bytes(input[0..8].try_into().unwrap());
    r[2] = u64::from_le_bytes(input[8..16].try_into().unwrap());
    r[1] = u64::from_le_bytes(input[16..24].try_into().unwrap());
    r[0] = u64::from_le_bytes(input[24..32].try_into().unwrap());
    r
}

#[cfg(test)]
mod test_operation {
    use num_bigint::BigUint;

    use crate::u256::{u256_add, u256_mul, u256_sub};

    #[test]
    fn test_raw_add_u64() {
        let a: [u64; 4] = [
            0x54806C11D8806141,
            0xF1DD2C190F5E93C4,
            0x597B6027B441A01F,
            0x85AEF3D078640C98,
        ];

        let b: [u64; 4] = [
            0x0E75C05FB4E3216D,
            0x1006E85F5CDFF073,
            0x1A7CE027B7A46F74,
            0x41E00A53DDA532DA,
        ];

        let a1 = BigUint::from_bytes_be(
            &hex::decode("85AEF3D078640C98597B6027B441A01FF1DD2C190F5E93C454806C11D8806141")
                .unwrap(),
        );

        let b1 = BigUint::from_bytes_be(
            &hex::decode("41E00A53DDA532DA1A7CE027B7A46F741006E85F5CDFF0730E75C05FB4E3216D")
                .unwrap(),
        );

        let (mut r, c) = u256_add(&a, &b);
        r.reverse();
        let mut sum = (&a1 + &b1).to_u64_digits();
        sum.reverse();
        assert_eq!(r, *sum);

        let (mut r, c) = u256_sub(&a, &b);
        r.reverse();
        let mut sub = (&a1 - &b1).to_u64_digits();
        sub.reverse();
        assert_eq!(r, *sub);

        let mut r = u256_mul(&a, &b);
        r.reverse();
        let mut mul = (&a1 * &b1).to_u64_digits();
        mul.reverse();
        assert_eq!(r, *mul);
    }
}