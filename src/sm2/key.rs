use num_bigint::BigUint;

use crate::sm2::error::{Sm2Error, Sm2Result};
use crate::sm2::p256_ecc::{Point, P256C_PARAMS};
use crate::sm2::{kdf, p256_ecc, random_uint};
use crate::sm3::sm3_hash;

#[derive(Debug, Clone)]
pub struct Sm2PublicKey {
    p: Point,
    compress_modle: CompressModle,
}

impl Sm2PublicKey {
    pub fn encrypt(&self, msg: &[u8]) -> Sm2Result<Vec<u8>> {
        loop {
            let klen = msg.len();
            let k = random_uint();
            let c1_p = p256_ecc::base_mul_point(&k, &P256C_PARAMS.g_point);
            let c1_p = c1_p.to_affine(); // 根据加密算法，z坐标会被丢弃，为保证解密还原回来的坐标在曲线上，则必须转换坐标系到 affine 坐标系

            let s_p = p256_ecc::base_mul_point(P256C_PARAMS.h.inner(), &self.p);
            if s_p.is_zero() {
                return Err(Sm2Error::ZeroPoint);
            }

            let c2_p = p256_ecc::base_mul_point(&k, &self.p).to_affine();
            let x2_bytes = c2_p.x.inner().to_bytes_be();
            let y2_bytes = c2_p.y.inner().to_bytes_be();
            let mut c2_append = vec![];
            c2_append.extend_from_slice(&x2_bytes);
            c2_append.extend_from_slice(&y2_bytes);

            let t = kdf(&c2_append[..], klen);
            let mut flag = true;
            for elem in &t {
                if elem != &0 {
                    flag = false;
                    break;
                }
            }
            if !flag {
                let c2 = BigUint::from_bytes_be(msg) ^ BigUint::from_bytes_be(&t[..]);
                let mut c3_append: Vec<u8> = vec![];
                c3_append.extend_from_slice(&x2_bytes);
                c3_append.extend_from_slice(msg);
                c3_append.extend_from_slice(&y2_bytes);
                let c3 = sm3_hash(&c3_append);

                let mut c: Vec<u8> = vec![];
                c.extend_from_slice(&c1_p.to_byte(self.compress_modle));
                c.extend_from_slice(&c2.to_bytes_be());
                c.extend_from_slice(&c3);
                return Ok(c);
            }
        }
    }

    pub fn to_str_hex(&self) -> String {
        format!("{}{}", self.p.x.to_str_radix(16), self.p.y.to_str_radix(16))
    }
}

#[derive(Debug, Clone)]
pub struct Sm2PrivateKey {
    d: BigUint,
    compress_modle: CompressModle,
}

impl Sm2PrivateKey {
    pub fn decrypt(&self, ciphertext: &[u8]) -> Sm2Result<Vec<u8>> {
        let c1_end_index = match self.compress_modle {
            CompressModle::Compressed => {33}
            CompressModle::Uncompressed  | CompressModle::Mixed=> {65}
        };

        let c1_bytes = &ciphertext[0..c1_end_index];
        let c2_bytes = &ciphertext[c1_end_index..(ciphertext.len() - 32)];
        let c3_bytes = &ciphertext[(ciphertext.len() - 32)..];

        let kelen = c2_bytes.len();
        let c1_point = Point::from_byte(c1_bytes, self.compress_modle)?;
        if !c1_point.to_affine().is_valid_affine() {
            return Err(Sm2Error::CheckPointErr);
        }

        let s_point = p256_ecc::base_mul_point(P256C_PARAMS.h.inner(), &c1_point);
        if s_point.is_zero() {
            return Err(Sm2Error::ZeroPoint);
        }

        let c2_point = p256_ecc::base_mul_point(&self.d, &c1_point).to_affine();
        let x2_bytes = c2_point.x.inner().to_bytes_be();
        let y2_bytes = c2_point.y.inner().to_bytes_be();
        let mut prepend: Vec<u8> = vec![];
        prepend.extend_from_slice(&x2_bytes);
        prepend.extend_from_slice(&y2_bytes);
        let t = kdf(&prepend, kelen);
        let mut flag = true;
        for elem in &t {
            if elem != &0 {
                flag = false;
                break;
            }
        }
        if flag {
            return Err(Sm2Error::ZeroData);
        }

        let m = BigUint::from_bytes_be(c2_bytes) ^ BigUint::from_bytes_be(&t);
        let mut prepend: Vec<u8> = vec![];
        prepend.extend_from_slice(&x2_bytes);
        prepend.extend_from_slice(&m.to_bytes_be());
        prepend.extend_from_slice(&y2_bytes);

        let u = sm3_hash(&prepend);
        if u != c3_bytes {
            return Err(Sm2Error::HashNotEqual);
        }
        Ok(m.to_bytes_be())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CompressModle {
    Compressed,
    Uncompressed,
    Mixed,
}

/// generate key pair
pub fn gen_keypair(compress_modle: CompressModle) -> (Sm2PublicKey, Sm2PrivateKey) {
    let sk = Sm2PrivateKey {
        d: random_uint(),
        compress_modle,
    };
    (public_from_private(&sk, compress_modle), sk)
}

fn public_from_private(sk: &Sm2PrivateKey, compress_modle: CompressModle) -> Sm2PublicKey {
    let p = p256_ecc::base_mul_point(&sk.d, &P256C_PARAMS.g_point);
    println!("Check public_key point = {}", p.is_valid());
    Sm2PublicKey { p, compress_modle }
}

#[cfg(test)]
mod test {
    use crate::sm2::key::{CompressModle, gen_keypair};

    #[test]
    fn test_gen_keypair() {
        let (pk, sk) = gen_keypair(CompressModle::Compressed);
        println!("sk={}", format!("{:x}", &sk.d));

        let msg = "你好 world,asjdkajhdjadahkubbhj12893718927391873891,@@！！ world,1231 wo12321321313asdadadahello world，hello world".as_bytes();
        let encrypt = pk.encrypt(msg).unwrap();
        let plain = sk.decrypt(&encrypt).unwrap();
        let s = String::from_utf8_lossy(&plain);
        println!("plain = {}", s);
        assert_eq!(msg, plain)
    }
}