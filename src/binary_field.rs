//! GF(2^256) = GF(2^128)[w]/(w²+w+β), with packed polynomial-basis limbs.
//! GF(2^128) uses x^128+x^7+x^2+x+1. β is selected by its absolute trace.
use ark_ff::{
    AdditiveGroup, BigInt, Field, Fp, FpConfig, LegendreSymbol, PrimeField, SqrtPrecomputation,
};
use ark_serialize::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::{
    fmt,
    iter::{Product, Sum},
    marker::PhantomData,
    ops::*,
};
use zeroize::Zeroize;

#[derive(Debug)]
pub struct BitConfig;
pub type Bit = Fp<BitConfig, 1>;
impl FpConfig<1> for BitConfig {
    const MODULUS: BigInt<1> = BigInt([2]);
    const GENERATOR: Bit = Self::ONE;
    const ZERO: Bit = Fp(BigInt([0]), PhantomData);
    const ONE: Bit = Fp(BigInt([1]), PhantomData);
    const TWO_ADICITY: u32 = 0;
    const TWO_ADIC_ROOT_OF_UNITY: Bit = Self::ONE;
    const SQRT_PRECOMP: Option<SqrtPrecomputation<Bit>> = None;
    fn add_assign(a: &mut Bit, b: &Bit) {
        a.0 .0[0] ^= b.0 .0[0];
    }
    fn sub_assign(a: &mut Bit, b: &Bit) {
        Self::add_assign(a, b);
    }
    fn double_in_place(a: &mut Bit) {
        *a = Self::ZERO;
    }
    fn neg_in_place(_: &mut Bit) {}
    fn mul_assign(a: &mut Bit, b: &Bit) {
        a.0 .0[0] &= b.0 .0[0];
    }
    fn sum_of_products<const T: usize>(a: &[Bit; T], b: &[Bit; T]) -> Bit {
        a.iter().zip(b).map(|(a, b)| *a * b).sum()
    }
    fn square_in_place(_: &mut Bit) {}
    fn inverse(a: &Bit) -> Option<Bit> {
        if a.0 .0[0] == 0 {
            None
        } else {
            Some(*a)
        }
    }
    fn from_bigint(a: BigInt<1>) -> Option<Bit> {
        if a.0[0] < 2 {
            Some(Fp(a, PhantomData))
        } else {
            None
        }
    }
    fn into_bigint(a: Bit) -> BigInt<1> {
        a.0
    }
}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Binary256(pub [u128; 2]);
impl Binary256 {
    pub const ZERO: Self = Self([0, 0]);
    pub const ONE: Self = Self([1, 0]);
    // Trace of x^121 in this GF(2^128) representation is one (tested below).
    pub const BETA: u128 = 1u128 << 121;
    pub fn from_bits(words: [u64; 4]) -> Self {
        Self([
            words[0] as u128 | ((words[1] as u128) << 64),
            words[2] as u128 | ((words[3] as u128) << 64),
        ])
    }
    pub fn from_bigint(words: BigInt<4>) -> Option<Self> {
        Some(Self::from_bits(words.0))
    }
    pub fn into_bigint(self) -> BigInt<4> {
        BigInt([
            self.0[0] as u64,
            (self.0[0] >> 64) as u64,
            self.0[1] as u64,
            (self.0[1] >> 64) as u64,
        ])
    }
    pub fn bytes(self) -> [u8; 32] {
        let mut out = [0; 32];
        out[..16].copy_from_slice(&self.0[0].to_le_bytes());
        out[16..].copy_from_slice(&self.0[1].to_le_bytes());
        out
    }
    pub fn from_bytes(b: [u8; 32]) -> Self {
        Self([
            u128::from_le_bytes(b[..16].try_into().unwrap()),
            u128::from_le_bytes(b[16..].try_into().unwrap()),
        ])
    }
    pub fn basis(bit: usize) -> Self {
        let mut a = Self::ZERO;
        a.0[bit / 128] = 1u128 << (bit % 128);
        a
    }
    pub fn bit(self, bit: usize) -> bool {
        (self.0[bit / 128] >> (bit % 128)) & 1 == 1
    }
}

/// Portable reference used to independently test the accelerated kernel.
pub fn mul128_reference(mut a: u128, mut b: u128) -> u128 {
    let mut c = 0;
    for _ in 0..128 {
        c ^= a & 0u128.wrapping_sub(b & 1);
        b >>= 1;
        let top = a >> 127;
        a = (a << 1) ^ (0x87 & 0u128.wrapping_sub(top));
    }
    c
}
#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn clmul(a: u64, b: u64) -> u128 {
    // Apple Silicon has ARMv8 cryptographic PMULL. Other ARM targets must enable +aes.
    unsafe { core::arch::aarch64::vmull_p64(a, b) }
}
// AWS Intel builds use RUSTFLAGS="-C target-cpu=native". Compile-time feature
// selection keeps the hot loop inline and leaves portable builds runnable on
// older x86 CPUs. The benchmark preflight rejects the portable fallback.
#[cfg(all(target_arch = "x86_64", target_feature = "pclmulqdq"))]
#[inline(always)]
fn clmul(a: u64, b: u64) -> u128 {
    use core::arch::x86_64::{_mm_clmulepi64_si128, _mm_set_epi64x};
    unsafe {
        let product =
            _mm_clmulepi64_si128::<0>(_mm_set_epi64x(0, a as i64), _mm_set_epi64x(0, b as i64));
        core::mem::transmute::<core::arch::x86_64::__m128i, u128>(product)
    }
}

pub const fn multiplication_backend() -> &'static str {
    if cfg!(target_arch = "aarch64") {
        "aarch64-pmull"
    } else if cfg!(all(target_arch = "x86_64", target_feature = "pclmulqdq")) {
        "x86_64-pclmulqdq"
    } else {
        "portable-reference"
    }
}
#[inline(always)]
pub fn mul128(a: u128, b: u128) -> u128 {
    #[cfg(any(
        target_arch = "aarch64",
        all(target_arch = "x86_64", target_feature = "pclmulqdq")
    ))]
    {
        let p0 = clmul(a as u64, b as u64);
        let p2 = clmul((a >> 64) as u64, (b >> 64) as u64);
        let p1 = clmul(
            (a as u64) ^ ((a >> 64) as u64),
            (b as u64) ^ ((b >> 64) as u64),
        ) ^ p0
            ^ p2;
        let lo = p0 ^ (p1 << 64);
        let hi = p2 ^ (p1 >> 64);
        // Reduce hi*x^128 using x^128 = x^7+x^2+x+1, including the overflow.
        let overflow = (hi >> 127) ^ (hi >> 126) ^ (hi >> 121);
        lo ^ hi
            ^ (hi << 1)
            ^ (hi << 2)
            ^ (hi << 7)
            ^ overflow
            ^ (overflow << 1)
            ^ (overflow << 2)
            ^ (overflow << 7)
    }
    #[cfg(not(any(
        target_arch = "aarch64",
        all(target_arch = "x86_64", target_feature = "pclmulqdq")
    )))]
    {
        mul128_reference(a, b)
    }
}
#[inline(always)]
fn mul(a: Binary256, b: Binary256) -> Binary256 {
    let ac = mul128(a.0[0], b.0[0]);
    let bd = mul128(a.0[1], b.0[1]);
    let cross = mul128(a.0[0] ^ a.0[1], b.0[0] ^ b.0[1]);
    Binary256([ac ^ mul128(bd, Binary256::BETA), cross ^ ac])
}
impl fmt::Display for Binary256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:032x}{:032x}", self.0[1], self.0[0])
    }
}
impl Zeroize for Binary256 {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}
impl ark_ff::Zero for Binary256 {
    fn zero() -> Self {
        Self::ZERO
    }
    fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }
}
impl ark_ff::One for Binary256 {
    fn one() -> Self {
        Self::ONE
    }
    fn is_one(&self) -> bool {
        *self == Self::ONE
    }
}
impl Neg for Binary256 {
    type Output = Self;
    fn neg(self) -> Self {
        self
    }
}
macro_rules! operation {
    ($trait:ident,$method:ident,$assign:ident,$assign_method:ident,$body:expr) => {
        impl $trait for Binary256 {
            type Output = Self;
            #[inline(always)]
            fn $method(self, rhs: Self) -> Self {
                ($body)(self, rhs)
            }
        }
        impl $assign for Binary256 {
            #[inline(always)]
            fn $assign_method(&mut self, rhs: Self) {
                *self = (*self).$method(rhs);
            }
        }
        impl<'a> $trait<&'a Self> for Binary256 {
            type Output = Self;
            #[inline(always)]
            fn $method(self, rhs: &Self) -> Self {
                self.$method(*rhs)
            }
        }
        impl<'a> $trait<&'a mut Self> for Binary256 {
            type Output = Self;
            #[inline(always)]
            fn $method(self, rhs: &mut Self) -> Self {
                self.$method(*rhs)
            }
        }
        impl<'a> $assign<&'a Self> for Binary256 {
            #[inline(always)]
            fn $assign_method(&mut self, rhs: &Self) {
                self.$assign_method(*rhs);
            }
        }
        impl<'a> $assign<&'a mut Self> for Binary256 {
            #[inline(always)]
            fn $assign_method(&mut self, rhs: &mut Self) {
                self.$assign_method(*rhs);
            }
        }
    };
}
operation!(
    Add,
    add,
    AddAssign,
    add_assign,
    |a: Binary256, b: Binary256| Binary256([a.0[0] ^ b.0[0], a.0[1] ^ b.0[1]])
);
operation!(
    Sub,
    sub,
    SubAssign,
    sub_assign,
    |a: Binary256, b: Binary256| a + b
);
operation!(Mul, mul, MulAssign, mul_assign, mul);
operation!(
    Div,
    div,
    DivAssign,
    div_assign,
    |a: Binary256, b: Binary256| a * b.inverse().expect("division by zero")
);
macro_rules! integer {($($t:ty),*)=>{$(impl From<$t> for Binary256 {fn from(x:$t)->Self {Self([(x as u128)&1,0])}})*}}
integer!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, bool);
impl Sum for Binary256 {
    fn sum<I: Iterator<Item = Self>>(i: I) -> Self {
        i.fold(Self::ZERO, |a, b| a + b)
    }
}
impl<'a> Sum<&'a Self> for Binary256 {
    fn sum<I: Iterator<Item = &'a Self>>(i: I) -> Self {
        i.copied().sum()
    }
}
impl Product for Binary256 {
    fn product<I: Iterator<Item = Self>>(i: I) -> Self {
        i.fold(Self::ONE, |a, b| a * b)
    }
}
impl<'a> Product<&'a Self> for Binary256 {
    fn product<I: Iterator<Item = &'a Self>>(i: I) -> Self {
        i.copied().product()
    }
}
impl Distribution<Binary256> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Binary256 {
        Binary256(rng.gen())
    }
}
impl AdditiveGroup for Binary256 {
    type Scalar = Self;
    const ZERO: Self = Self::ZERO;
    fn double(&self) -> Self {
        Self::ZERO
    }
    fn double_in_place(&mut self) -> &mut Self {
        *self = Self::ZERO;
        self
    }
}
impl Field for Binary256 {
    type BasePrimeField = Bit;
    const SQRT_PRECOMP: Option<SqrtPrecomputation<Self>> = None;
    const ONE: Self = Self::ONE;
    fn extension_degree() -> u64 {
        256
    }
    fn to_base_prime_field_elements(&self) -> impl Iterator<Item = Bit> {
        let value = *self;
        (0..256).map(move |i| Bit::from(value.bit(i)))
    }
    fn from_base_prime_field_elems(e: impl IntoIterator<Item = Bit>) -> Option<Self> {
        let mut out = Self::ZERO;
        let mut count = 0;
        for b in e {
            if count >= 256 {
                return None;
            }
            if b.into_bigint().0[0] != 0 {
                out += Self::basis(count)
            }
            count += 1;
        }
        if count == 256 {
            Some(out)
        } else {
            None
        }
    }
    fn mul_by_base_prime_field(&self, b: &Bit) -> Self {
        if b.into_bigint().0[0] == 0 {
            Self::ZERO
        } else {
            *self
        }
    }
    fn from_base_prime_field(b: Bit) -> Self {
        Self::from(b.into_bigint().0[0])
    }
    fn from_random_bytes_with_flags<F: Flags>(b: &[u8]) -> Option<(Self, F)> {
        if F::BIT_SIZE > 8 || b.len() < 32 + usize::from(F::BIT_SIZE > 0) {
            return None;
        }
        let flags = F::from_u8(if F::BIT_SIZE > 0 { b[32] } else { 0 })?;
        Some((Self::from_bytes(b[..32].try_into().ok()?), flags))
    }
    fn legendre(&self) -> LegendreSymbol {
        if *self == Self::ZERO {
            LegendreSymbol::Zero
        } else {
            LegendreSymbol::QuadraticResidue
        }
    }
    #[inline(always)]
    fn square(&self) -> Self {
        Self([
            mul128(self.0[0], self.0[0]) ^ mul128(mul128(self.0[1], self.0[1]), Self::BETA),
            mul128(self.0[1], self.0[1]),
        ])
    }
    fn square_in_place(&mut self) -> &mut Self {
        *self = self.square();
        self
    }
    fn inverse(&self) -> Option<Self> {
        if *self == Self::ZERO {
            return None;
        }
        // Conjugate w -> w+1. Invert the norm in GF(2^128).
        let a = self.0[0];
        let b = self.0[1];
        let norm = mul128(a, a) ^ mul128(a, b) ^ mul128(mul128(b, b), Self::BETA);
        let mut inv = norm;
        for _ in 1..127 {
            inv = mul128(mul128(inv, inv), norm);
        }
        inv = mul128(inv, inv);
        Some(Self([mul128(a ^ b, inv), mul128(b, inv)]))
    }
    fn inverse_in_place(&mut self) -> Option<&mut Self> {
        *self = self.inverse()?;
        Some(self)
    }
    fn sqrt(&self) -> Option<Self> {
        let mut a = *self;
        for _ in 0..255 {
            a = a.square()
        }
        Some(a)
    }
    fn frobenius_map_in_place(&mut self, p: usize) {
        for _ in 0..p % 256 {
            *self = self.square()
        }
    }
}
impl Valid for Binary256 {
    fn check(&self) -> Result<(), SerializationError> {
        Ok(())
    }
}
impl CanonicalSerialize for Binary256 {
    fn serialize_with_mode<W: Write>(
        &self,
        mut w: W,
        _: Compress,
    ) -> Result<(), SerializationError> {
        w.write_all(&self.bytes())?;
        Ok(())
    }
    fn serialized_size(&self, _: Compress) -> usize {
        32
    }
}
impl CanonicalDeserialize for Binary256 {
    fn deserialize_with_mode<R: Read>(
        mut r: R,
        _: Compress,
        _: Validate,
    ) -> Result<Self, SerializationError> {
        let mut b = [0; 32];
        r.read_exact(&mut b)?;
        Ok(Self::from_bytes(b))
    }
}
impl CanonicalSerializeWithFlags for Binary256 {
    fn serialize_with_flags<W: Write, F: Flags>(
        &self,
        mut w: W,
        f: F,
    ) -> Result<(), SerializationError> {
        if F::BIT_SIZE > 8 {
            return Err(SerializationError::NotEnoughSpace);
        }
        self.serialize_compressed(&mut w)?;
        if F::BIT_SIZE > 0 {
            w.write_all(&[f.u8_bitmask()])?
        }
        Ok(())
    }
    fn serialized_size_with_flags<F: Flags>(&self) -> usize {
        32 + usize::from(F::BIT_SIZE > 0)
    }
}
impl CanonicalDeserializeWithFlags for Binary256 {
    fn deserialize_with_flags<R: Read, F: Flags>(
        mut r: R,
    ) -> Result<(Self, F), SerializationError> {
        if F::BIT_SIZE > 8 {
            return Err(SerializationError::NotEnoughSpace);
        }
        let a = Self::deserialize_compressed(&mut r)?;
        let mut b = [0];
        if F::BIT_SIZE > 0 {
            r.read_exact(&mut b)?
        }
        let f = F::from_u8(b[0]).ok_or(SerializationError::UnexpectedFlags)?;
        Ok((a, f))
    }
}
impl spongefish::Encoding<[u8]> for Binary256 {
    fn encode(&self) -> impl AsRef<[u8]> {
        self.bytes()
    }
}
impl spongefish::Decoding<[u8]> for Binary256 {
    type Repr = [u8; 32];
    fn decode(b: Self::Repr) -> Self {
        Self::from_bytes(b)
    }
}
impl spongefish::NargDeserialize for Binary256 {
    fn deserialize_from_narg(b: &mut &[u8]) -> spongefish::VerificationResult<Self> {
        if b.len() < 32 {
            return Err(spongefish::VerificationError);
        }
        let a = Self::from_bytes(b[..32].try_into().unwrap());
        *b = &b[32..];
        Ok(a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn carryless_kernel_all_basis_pairs() {
        // Covers every coefficient of this bilinear multiplication/reduction
        // map, including the high-bit overflow in polynomial reduction.
        for i in 0..128 {
            for j in 0..128 {
                assert_eq!(mul128(1 << i, 1 << j), mul128_reference(1 << i, 1 << j));
            }
        }
        for a in [0, 1, u128::MAX, 1 << 127, 0x87] {
            for b in [0, 1, u128::MAX, 1 << 127, 0x87] {
                assert_eq!(mul128(a, b), mul128_reference(a, b));
            }
        }
    }
    #[test]
    fn field_axioms_and_kernel() {
        let mut rng = rand::thread_rng();
        let mut trace = 0;
        let mut power = Binary256::BETA;
        for _ in 0..128 {
            trace ^= power;
            power = mul128_reference(power, power);
        }
        assert_eq!(trace, 1, "quadratic extension must be irreducible");
        for _ in 0..128 {
            let a: Binary256 = rng.gen();
            let b: Binary256 = rng.gen();
            let c: Binary256 = rng.gen();
            assert_eq!(mul128(a.0[0], b.0[0]), mul128_reference(a.0[0], b.0[0]));
            assert_eq!(a * (b + c), a * b + a * c);
            assert_eq!((a * b) * c, a * (b * c));
            assert_eq!(a + a, Binary256::ZERO);
            assert_eq!(a.square(), a * a);
            assert_eq!(a * a.inverse().unwrap(), Binary256::ONE);
            let mut f = a;
            f.frobenius_map_in_place(255);
            assert_eq!(f.square(), a);
            assert_eq!(
                Binary256::from_base_prime_field_elems(a.to_base_prime_field_elements()),
                Some(a)
            );
            let mut bytes = Vec::new();
            a.serialize_compressed(&mut bytes).unwrap();
            assert_eq!(bytes.len(), 32);
            assert_eq!(
                Binary256::deserialize_compressed(bytes.as_slice()).unwrap(),
                a
            );
        }
        assert_eq!(Binary256::from(2), Binary256::ZERO);
    }
}
