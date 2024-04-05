use crate::fields::fp::Fp;
use crate::fields::FieldElement;
use crate::u256::U256;

#[derive(Copy, Debug, Clone)]
pub struct Point {
    x: Fp,
    y: Fp,
    z: Fp,
}

#[derive(Copy, Debug, Clone)]
pub struct TwistPoint {
    x: [U256; 2],
    y: [U256; 2],
    z: [U256; 2],
}

// 群 G1的生成元 P1 = (xP1 , yP1);
// P1.X 0x93DE051D62BF718FF5ED0704487D01D6E1E4086909DC3280E8C4E4817C66DDDD
// P1.Y 0x21FE8DDA4F21E607631065125C395BBC1C1C00CBFA6024350C464CD70A3EA616
const G1: Point = Point {
    x: [
        0xe8c4e4817c66dddd,
        0xe1e4086909dc3280,
        0xf5ed0704487d01d6,
        0x93de051d62bf718f,
    ],
    y: [
        0x0c464cd70a3ea616,
        0x1c1c00cbfa602435,
        0x631065125c395bbc,
        0x21fe8dda4f21e607,
    ],
    z: [1, 0, 0, 0],
};

/*
    X : [0x3722755292130b08d2aab97fd34ec120ee265948d19c17abf9b7213baf82d65b,
         0x85aef3d078640c98597b6027b441a01ff1dd2c190f5e93c454806c11d8806141],
    Y : [0xa7cf28d519be3da65f3170153d278ff247efba98a71a08116215bba5c999a7c7,
         0x17509b092e845c1266ba0d262cbee6ed0736a96fa347c8bd856dc76b84ebeb96],
    Z : [1n, 0n],
*/
// 群 G2的生成元 P2 = (xP2, yP2)：
const G2: TwistPoint = TwistPoint {
    x: [
        [
            0xF9B7213BAF82D65B,
            0xEE265948D19C17AB,
            0xD2AAB97FD34EC120,
            0x3722755292130B08,
        ],
        [
            0x54806C11D8806141,
            0xF1DD2C190F5E93C4,
            0x597B6027B441A01F,
            0x85AEF3D078640C98,
        ],
    ],
    y: [
        [
            0x6215BBA5C999A7C7,
            0x47EFBA98A71A0811,
            0x5F3170153D278FF2,
            0xA7CF28D519BE3DA6,
        ],
        [
            0x856DC76B84EBEB96,
            0x0736A96FA347C8BD,
            0x66BA0D262CBEE6ED,
            0x17509B092E845C12,
        ],
    ],
    z: [[1, 0, 0, 0], [0, 0, 0, 0]],
};

impl Point {
    pub fn zero() -> Self {
        Self {
            x: Fp::one(),
            y: Fp::one(),
            z: Fp::zero(),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.z.is_zero()
    }

    pub fn point_double(&self) -> Self {
        if self.is_zero() {
            return self.clone();
        }
        let mut x1 = self.x;
        let mut y1 = self.y;
        let z1 = self.z;

        let mut t1 = Fp::zero();
        let mut t2 = Fp::zero();
        let mut t3 = Fp::zero();

        let mut x3 = Fp::zero();
        let mut y3 = Fp::zero();
        let mut z3 = Fp::zero();

        t2 = x1.fp_sqr();
        t2 = t2.fp_triple();
        y3 = y1.fp_double();
        z3 = y3.fp_mul(&z1);
        y3 = y3.fp_sqr();
        t3 = y3.fp_mul(&x1);
        y3 = y3.fp_sqr();
        y3 = y3.fp_div2();
        x3 = t2.fp_sqr();

        t1 = t3.fp_double();
        x3 = x3.fp_sub(&t1);
        t1 = t3.fp_sub(&x3);
        t1 = t1.fp_mul(&t2);
        y3 = t1.fp_sub(&y3);

        Self {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    pub fn point_add(&self, rhs: &Self) -> Self {
        if rhs.is_zero() {
            return self.clone();
        }

        if self.is_zero() {
            return rhs.clone();
        }

        todo!()
    }

    pub fn point_sub(&self, rhs: &Self) -> Self {
        let t = rhs.point_neg();
        self.point_add(&t)
    }

    pub fn point_neg(&self) -> Self {
        Point {
            x: self.x.clone(),
            y: self.y.fp_neg().clone(),
            z: self.z.clone(),
        }
    }

    pub fn point_double_x5(&self) -> Self {
        let mut r = self.point_double();
        r = r.point_double();
        r = r.point_double();
        r = r.point_double();
        r = r.point_double();
        r
    }

    pub fn point_mul(&self, k: &U256) -> Self {
        todo!()
    }
}

impl TwistPoint {
    pub fn point_double(&self) -> Self {
        todo!()
    }

    pub fn point_add(&self, rhs: &Self) -> Self {
        todo!()
    }

    pub fn point_sub(&self, rhs: &Self) -> Self {
        todo!()
    }

    pub fn point_neg(&self) -> Self {
        todo!()
    }

    pub fn point_mul(&self, k: &U256) -> Self {
        todo!()
    }
}