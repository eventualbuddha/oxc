//! Niched non-max types, similar to `NonZero*`, but with the illegal values at top of the range,
//! rather than 0.
//!
//! `nonmax` crate provides similar types, but they work by storing the value internally as a `NonZero*`,
//! and inverting the bits on every read/write (`native_u32 = nonmax_u32 as u32 ^ u32::MAX`).
//! This reserves only a single illegal value, but has (small) runtime cost on every read/write.
//!
//! The `NonMax*` types provided here make a different trade-off.
//!
//! `NonMaxU8` is an enum with a single illegal value (255).
//! All the other types store the value as a number of `u8`s + a single `NonMaxU8`.
//! i.e. they reserve all values which have 255 as the highest byte.
//!
//! * `NonMaxU8` can represent the values `0` to `254`.
//! * `NonMaxU16` can represent the values `0` to `255 << 8 - 1`.
//! * `NonMaxU32` can represent the values `0` to `255 << 24 - 1`.
//! * `NonMaxU64` can represent the values `0` to `255 << 56 - 1`.
//! * `NonMaxU128` can represent the values `0` to `255 << 120 - 1`.
//!
//! We trade approx 0.4% of the legal range being unavailable, in return for zero runtime cost
//! when reading or writing the values.
//!
//! All `NonMax*` types have a single niche (even though they have multiple illegal values).
//! `Option<NonMax*>` is same size as its native equivalent for all `NonMax*` types
//! (`size_of::<Option<NonMaxU32>>() == size_of::<u32>()`).

use std::{
    cmp,
    fmt::{self, Debug, Display},
    mem::{align_of, size_of},
};

macro_rules! impl_nonmax {
    ($nonmax:ident, $native:ident) => {
        const _: () = {
            assert!(size_of::<$nonmax>() == size_of::<$native>());
            assert!(align_of::<$nonmax>() == align_of::<$native>());
        };

        impl $nonmax {
            /// Maximum value representable by this type
            pub const MAX: $native = ((255 as $native) << ($native::BITS - 8)) - 1;

            #[doc = concat!("Create new `", stringify!($nonmax), "` from `", stringify!($native), "`.")]
            #[doc = ""]
            #[doc = "# Panics"]
            #[doc = concat!("Panics if `n` is greater than `", stringify!($nonmax), "::MAX`.")]
            #[inline]
            pub const fn new(n: $native) -> Self {
                assert!(n <= Self::MAX);
                // SAFETY: We just checked `n` does not exceed `Self::MAX`
                unsafe { Self::new_unchecked(n) }
            }

            #[doc = concat!("Create new `", stringify!($nonmax), "` from `", stringify!($native), "`, without checking validity of the input.")]
            #[doc = ""]
            #[doc = "# SAFETY"]
            #[doc = concat!("Caller must ensure `n` does not exceed `", stringify!($nonmax), "::MAX`.")]
            #[inline]
            #[allow(clippy::missing_safety_doc)]
            pub const unsafe fn new_unchecked(n: $native) -> Self {
                // SAFETY: Caller guarantees `n` does not exceed `Self::MAX`.
                // Size and align of nonmax type and native type are the same.
                unsafe { std::mem::transmute::<$native, Self>(n) }
            }

            #[doc = concat!("Convert `", stringify!($nonmax), "` to `", stringify!($native), "`.")]
            #[inline]
            pub const fn to_native(self) -> $native {
                // SAFETY: All valid bit patterns of nonmax type are valid bit patterns for native type.
                // Size and align of nonmax type and native type are the same.
                unsafe { std::mem::transmute::<Self, $native>(self) }
            }
        }

        impl From<$nonmax> for $native {
            #[doc = concat!("Convert `", stringify!($nonmax), "` to `", stringify!($native), "`.")]
            #[inline]
            fn from(n: $nonmax) -> $native {
                n.to_native()
            }
        }

        impl TryFrom<$native> for $nonmax {
            type Error = ();

            #[doc = concat!("Try to convert `", stringify!($native), "` to `", stringify!($nonmax), "`.")]
            #[doc = ""]
            #[doc = concat!("Returns `Err` if `n >= ", stringify!($nonmax), "::MAX`.")]
            #[inline]
            fn try_from(n: $native) -> Result<Self, ()> {
                if n <= Self::MAX {
                    // SAFETY: We just checked `n` does not exceed `Self::MAX`
                    Ok(unsafe { Self::new_unchecked(n) })
                } else {
                    Err(())
                }
            }
        }

        impl PartialEq<Self> for $nonmax {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.to_native() == other.to_native()
            }
        }

        impl Eq for $nonmax {}

        impl PartialOrd for $nonmax {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $nonmax {
            #[inline]
            fn cmp(&self, other: &Self) -> cmp::Ordering {
                self.to_native().cmp(&other.to_native())
            }
        }

        impl Display for $nonmax {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                Display::fmt(&self.to_native(), f)
            }
        }

        impl Debug for $nonmax {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                Debug::fmt(&self.to_native(), f)
            }
        }
    }
}

/// `u8` with a niche for maximum value.
///
/// Equivalent of `NonZeroU8`, but with illegal value of `u8::MAX`, instead of 0.
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum NonMaxU8 {
    _0 = 0,
    _1 = 1,
    _2 = 2,
    _3 = 3,
    _4 = 4,
    _5 = 5,
    _6 = 6,
    _7 = 7,
    _8 = 8,
    _9 = 9,
    _10 = 10,
    _11 = 11,
    _12 = 12,
    _13 = 13,
    _14 = 14,
    _15 = 15,
    _16 = 16,
    _17 = 17,
    _18 = 18,
    _19 = 19,
    _20 = 20,
    _21 = 21,
    _22 = 22,
    _23 = 23,
    _24 = 24,
    _25 = 25,
    _26 = 26,
    _27 = 27,
    _28 = 28,
    _29 = 29,
    _30 = 30,
    _31 = 31,
    _32 = 32,
    _33 = 33,
    _34 = 34,
    _35 = 35,
    _36 = 36,
    _37 = 37,
    _38 = 38,
    _39 = 39,
    _40 = 40,
    _41 = 41,
    _42 = 42,
    _43 = 43,
    _44 = 44,
    _45 = 45,
    _46 = 46,
    _47 = 47,
    _48 = 48,
    _49 = 49,
    _50 = 50,
    _51 = 51,
    _52 = 52,
    _53 = 53,
    _54 = 54,
    _55 = 55,
    _56 = 56,
    _57 = 57,
    _58 = 58,
    _59 = 59,
    _60 = 60,
    _61 = 61,
    _62 = 62,
    _63 = 63,
    _64 = 64,
    _65 = 65,
    _66 = 66,
    _67 = 67,
    _68 = 68,
    _69 = 69,
    _70 = 70,
    _71 = 71,
    _72 = 72,
    _73 = 73,
    _74 = 74,
    _75 = 75,
    _76 = 76,
    _77 = 77,
    _78 = 78,
    _79 = 79,
    _80 = 80,
    _81 = 81,
    _82 = 82,
    _83 = 83,
    _84 = 84,
    _85 = 85,
    _86 = 86,
    _87 = 87,
    _88 = 88,
    _89 = 89,
    _90 = 90,
    _91 = 91,
    _92 = 92,
    _93 = 93,
    _94 = 94,
    _95 = 95,
    _96 = 96,
    _97 = 97,
    _98 = 98,
    _99 = 99,
    _100 = 100,
    _101 = 101,
    _102 = 102,
    _103 = 103,
    _104 = 104,
    _105 = 105,
    _106 = 106,
    _107 = 107,
    _108 = 108,
    _109 = 109,
    _110 = 110,
    _111 = 111,
    _112 = 112,
    _113 = 113,
    _114 = 114,
    _115 = 115,
    _116 = 116,
    _117 = 117,
    _118 = 118,
    _119 = 119,
    _120 = 120,
    _121 = 121,
    _122 = 122,
    _123 = 123,
    _124 = 124,
    _125 = 125,
    _126 = 126,
    _127 = 127,
    _128 = 128,
    _129 = 129,
    _130 = 130,
    _131 = 131,
    _132 = 132,
    _133 = 133,
    _134 = 134,
    _135 = 135,
    _136 = 136,
    _137 = 137,
    _138 = 138,
    _139 = 139,
    _140 = 140,
    _141 = 141,
    _142 = 142,
    _143 = 143,
    _144 = 144,
    _145 = 145,
    _146 = 146,
    _147 = 147,
    _148 = 148,
    _149 = 149,
    _150 = 150,
    _151 = 151,
    _152 = 152,
    _153 = 153,
    _154 = 154,
    _155 = 155,
    _156 = 156,
    _157 = 157,
    _158 = 158,
    _159 = 159,
    _160 = 160,
    _161 = 161,
    _162 = 162,
    _163 = 163,
    _164 = 164,
    _165 = 165,
    _166 = 166,
    _167 = 167,
    _168 = 168,
    _169 = 169,
    _170 = 170,
    _171 = 171,
    _172 = 172,
    _173 = 173,
    _174 = 174,
    _175 = 175,
    _176 = 176,
    _177 = 177,
    _178 = 178,
    _179 = 179,
    _180 = 180,
    _181 = 181,
    _182 = 182,
    _183 = 183,
    _184 = 184,
    _185 = 185,
    _186 = 186,
    _187 = 187,
    _188 = 188,
    _189 = 189,
    _190 = 190,
    _191 = 191,
    _192 = 192,
    _193 = 193,
    _194 = 194,
    _195 = 195,
    _196 = 196,
    _197 = 197,
    _198 = 198,
    _199 = 199,
    _200 = 200,
    _201 = 201,
    _202 = 202,
    _203 = 203,
    _204 = 204,
    _205 = 205,
    _206 = 206,
    _207 = 207,
    _208 = 208,
    _209 = 209,
    _210 = 210,
    _211 = 211,
    _212 = 212,
    _213 = 213,
    _214 = 214,
    _215 = 215,
    _216 = 216,
    _217 = 217,
    _218 = 218,
    _219 = 219,
    _220 = 220,
    _221 = 221,
    _222 = 222,
    _223 = 223,
    _224 = 224,
    _225 = 225,
    _226 = 226,
    _227 = 227,
    _228 = 228,
    _229 = 229,
    _230 = 230,
    _231 = 231,
    _232 = 232,
    _233 = 233,
    _234 = 234,
    _235 = 235,
    _236 = 236,
    _237 = 237,
    _238 = 238,
    _239 = 239,
    _240 = 240,
    _241 = 241,
    _242 = 242,
    _243 = 243,
    _244 = 244,
    _245 = 245,
    _246 = 246,
    _247 = 247,
    _248 = 248,
    _249 = 249,
    _250 = 250,
    _251 = 251,
    _252 = 252,
    _253 = 253,
    _254 = 254,
    // No variant for 255
}

impl_nonmax!(NonMaxU8, u8);

/// `u16` with niche for maximum values.
///
/// Equivalent of `NonZeroU16`, but with illegal values of `>= (255 << 8)`, instead of 0.
///
/// Although has 256 illegal values, this type has only a single niche.
#[derive(Clone, Copy)]
#[repr(C, align(2))]
pub struct NonMaxU16 {
    #[cfg(target_endian = "big")]
    top: NonMaxU8,
    bottom: u8,
    #[cfg(target_endian = "little")]
    top: NonMaxU8,
}

impl_nonmax!(NonMaxU16, u16);

/// `u32` with niche for maximum values.
///
/// Equivalent of `NonZeroU32`, but with illegal values of `>= (255 << 24)`, instead of 0.
///
/// Although has `1 << 24` illegal values, this type has only a single niche.
#[derive(Clone, Copy)]
#[repr(C, align(4))]
pub struct NonMaxU32 {
    #[cfg(target_endian = "big")]
    top: NonMaxU8,
    bottom: [u8; 3],
    #[cfg(target_endian = "little")]
    top: NonMaxU8,
}

impl_nonmax!(NonMaxU32, u32);

/// `u64` with niche for maximum values.
///
/// Equivalent of `NonZeroU64`, but with illegal values of `>= (255 << 56)`, instead of 0.
///
/// Although has `1 << 56` illegal values, this type has only a single niche.
#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct NonMaxU64 {
    #[cfg(target_endian = "big")]
    top: NonMaxU8,
    bottom: [u8; 7],
    #[cfg(target_endian = "little")]
    top: NonMaxU8,
}

impl_nonmax!(NonMaxU64, u64);

/// `u128` with niche for maximum values.
///
/// Equivalent of `NonZeroU128`, but with illegal values of `>= (255 << 120)`, instead of 0.
///
/// Although has `1 << 120` illegal values, this type has only a single niche.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct NonMaxU128 {
    #[cfg(target_endian = "big")]
    top: NonMaxU8,
    bottom: [u8; 15],
    #[cfg(target_endian = "little")]
    top: NonMaxU8,
    // Align same as `u128` which is 16 on new versions of rustc, 8 on older versions
    _align: [u128; 0],
}

impl_nonmax!(NonMaxU128, u128);

/// `usize` with niche for maximum values.
///
/// Equivalent of `NonZeroUsize`, but with illegal values of
/// `>= (255 << ((std::mem::size_of::<usize>() - 1) * 8))`
/// (`255 << 56` on 64-bit systems, `255 << 24` on 32-bit systems), instead of 0.
///
/// Although has `1 << ((std::mem::size_of::<usize>() - 1) * 8)` illegal values,
/// this type has only a single niche.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct NonMaxUsize {
    #[cfg(target_endian = "big")]
    top: NonMaxU8,
    bottom: [u8; size_of::<usize>() - 1],
    #[cfg(target_endian = "little")]
    top: NonMaxU8,
    // Align same as `usize`
    _align: [usize; 0],
}

impl_nonmax!(NonMaxUsize, usize);
