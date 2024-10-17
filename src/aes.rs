use std::array::from_fn;

use crate::cipher::BlockCipher;

pub const SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

pub const SBOX_INV: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

pub fn xtime(x: u8) -> u8 {
    x.wrapping_shl(1) ^ if x & 0x80 == 0x80 { 0x1b } else { 0x00 }
}

pub fn gfmul(x: u8, mut y: u8) -> u8 {
    let mut z = 0;
    for i in 0..8 {
        if (x >> i) & 1 == 1 {
            z ^= y;
        }
        y = xtime(y);
    }
    z
}

pub fn mix_column(x: u32) -> u32 {
    let x = x.to_be_bytes();
    u32::from_be_bytes([
        gfmul(2, x[0]) ^ gfmul(3, x[1]) ^ gfmul(1, x[2]) ^ gfmul(1, x[3]),
        gfmul(2, x[1]) ^ gfmul(3, x[2]) ^ gfmul(1, x[3]) ^ gfmul(1, x[0]),
        gfmul(2, x[2]) ^ gfmul(3, x[3]) ^ gfmul(1, x[0]) ^ gfmul(1, x[1]),
        gfmul(2, x[3]) ^ gfmul(3, x[0]) ^ gfmul(1, x[1]) ^ gfmul(1, x[2]),
    ])
}

pub fn inv_mix_column(x: u32) -> u32 {
    let x = x.to_be_bytes();
    u32::from_be_bytes([
        gfmul(14, x[0]) ^ gfmul(11, x[1]) ^ gfmul(13, x[2]) ^ gfmul(9, x[3]),
        gfmul(14, x[1]) ^ gfmul(11, x[2]) ^ gfmul(13, x[3]) ^ gfmul(9, x[0]),
        gfmul(14, x[2]) ^ gfmul(11, x[3]) ^ gfmul(13, x[0]) ^ gfmul(9, x[1]),
        gfmul(14, x[3]) ^ gfmul(11, x[0]) ^ gfmul(13, x[1]) ^ gfmul(9, x[2]),
    ])
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AESState(pub [[u8; 4]; 4]);

impl AESState {
    pub fn transpose(self) -> Self {
        Self(from_fn(|i| from_fn(|j| self.0[j][i])))
    }

    pub fn to_bytes(self) -> [u8; 16] {
        from_fn(|i| self.0[i & 3][i >> 2])
    }

    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(from_fn(|i| from_fn(|j| bytes[(j << 2) | i])))
    }

    pub fn to_words(self) -> [u32; 4] {
        self.0.map(u32::from_be_bytes)
    }

    pub fn from_words(words: [u32; 4]) -> Self {
        Self(words.map(u32::to_be_bytes))
    }

    pub fn map_bytes(self, f: impl Fn(u8) -> u8) -> Self {
        Self(self.0.map(|arr| arr.map(&f)))
    }

    pub fn map_words(self, f: impl Fn(u32) -> u32) -> Self {
        Self::from_words(self.to_words().map(f))
    }

    pub fn sub_bytes(self) -> Self {
        self.map_bytes(|byte| SBOX[byte as usize])
    }

    pub fn inv_sub_bytes(self) -> Self {
        self.map_bytes(|byte| SBOX_INV[byte as usize])
    }

    pub fn mix_columns(self) -> Self {
        self.transpose().map_words(mix_column).transpose()
    }

    pub fn inv_mix_columns(self) -> Self {
        self.transpose().map_words(inv_mix_column).transpose()
    }

    pub fn shift_rows(self) -> Self {
        let rows = self.to_words();
        Self::from_words([
            rows[0].rotate_left(0),
            rows[1].rotate_left(8),
            rows[2].rotate_left(16),
            rows[3].rotate_left(24),
        ])
    }

    pub fn inv_shift_rows(self) -> Self {
        let rows = self.to_words();
        Self::from_words([
            rows[0].rotate_left(0),
            rows[1].rotate_left(24),
            rows[2].rotate_left(16),
            rows[3].rotate_left(8),
        ])
    }

    pub fn add_round_key(self, key: [u8; 16]) -> Self{
        let bytes = self.to_bytes();
        Self::from_bytes(from_fn(|i| bytes[i] ^ key[i]))
    }

}

pub fn sub_word(word: u32) -> u32 {
    u32::from_be_bytes(word.to_be_bytes().map(|byte| SBOX[byte as usize]))
}

pub fn rot_word(word: u32) -> u32 {
    word.rotate_left(8)
}

const RCON: [u32; 20] = [
    0x8d000000, 0x01000000, 0x02000000, 0x04000000,
    0x08000000, 0x10000000, 0x20000000, 0x40000000,
    0x80000000, 0x1b000000, 0x36000000, 0x6c000000,
    0xd8000000, 0xab000000, 0x4d000000, 0x9a000000,
    0x2f000000, 0x5e000000, 0xbc000000, 0x63000000,
];

pub struct AES<const NK: usize, const NR: usize>(pub [[u8; 16]; NR+1]) where [(); 4*(NR+1)]:;

impl<const NK: usize, const NR: usize> AES<NK, NR> where [(); 4*(NR+1)]: {
    pub fn key_schedule(key: [u8; 4*NK]) -> Self {
        let mut w = [0u32; 4*(NR+1)];

        let mut i = 0;
        while i < NK {
            w[i] = u32::from_be_bytes(from_fn(|j| key[4*i + j]));
            i += 1;
        }

        while i < 4*(NR+1) {
            let mut word = w[i-1];
            if i % NK == 0 {
                word = sub_word(rot_word(word)) ^ RCON[i / NK];
            } else if NK > 6 && i % NK == 4 {
                word = sub_word(word)
            }
            w[i] = w[i-NK] ^ word;
            i += 1;
        }

        Self(from_fn(|i| from_fn(|j| w[4*i +(j >> 2)].to_be_bytes()[j & 3])))
    }

    pub fn encrypt(&self, block: [u8; 16]) -> [u8; 16] {
        let mut st = AESState::from_bytes(block);
        st = st.add_round_key(self.0[0]);
        for i in 1..NR {
            st = st.sub_bytes().shift_rows().mix_columns().add_round_key(self.0[i])
        }
        st = st.sub_bytes().shift_rows().add_round_key(self.0[NR]);
        st.to_bytes()
    }

    pub fn decrypt(&self, block: [u8; 16]) -> [u8; 16] {
        let mut st = AESState::from_bytes(block);
        st = st.add_round_key(self.0[NR]).inv_shift_rows().inv_sub_bytes();
        for i in (1..NR).rev() {
            st = st.add_round_key(self.0[i]).inv_mix_columns().inv_shift_rows().inv_sub_bytes();
        }
        st = st.add_round_key(self.0[0]);
        st.to_bytes()
    }
}


macro_rules! make_aes_impl {
    ($st: ident, $nk: expr, $nr: expr) => {
        pub struct $st(AES<$nk, $nr>);

        impl BlockCipher for $st {
            const KEY_SIZE: usize = 4*$nk;

            const BLOCK_SIZE: usize = 16;

            fn new(key: [u8; Self::KEY_SIZE]) -> Self {
                Self(AES::<$nk, $nr>::key_schedule(key))
            }

            fn encrypt(&self, block: [u8; Self::BLOCK_SIZE]) -> [u8; Self::BLOCK_SIZE] {
                self.0.encrypt(block)
            }

            fn decrypt(&self, block: [u8; Self::BLOCK_SIZE]) -> [u8; Self::BLOCK_SIZE] {
                self.0.decrypt(block)
            }
        }
    };
}

make_aes_impl!(AES128, 4, 10);
make_aes_impl!(AES192, 6, 12);
make_aes_impl!(AES256, 8, 14);

#[cfg(test)]
mod tests {
    use rand::random;

    use super::*;

    #[test]
    pub fn test_subbytes() {
        let st0 = AESState([
            [0x00, 0x01, 0x02, 0x03],
            [0x04, 0x05, 0x06, 0x07],
            [0x08, 0x09, 0x0a, 0x0b],
            [0x0c, 0x0d, 0x0e, 0x0f],
        ]);
        let st1 = st0.sub_bytes();

        assert_eq!(st1, AESState([
            [0x63, 0x7c, 0x77, 0x7b],
            [0xf2, 0x6b, 0x6f, 0xc5],
            [0x30, 0x01, 0x67, 0x2b],
            [0xfe, 0xd7, 0xab, 0x76],
        ]));
    }

    #[test]
    pub fn test_subbytes_bijective() {
        for _ in 0..256 {
            let st0 = AESState(random());
            let st1 = st0.sub_bytes();
            let st2 = st1.inv_sub_bytes();
            let st3 = st2.sub_bytes();

            assert_eq!(st0, st2);
            assert_eq!(st1, st3);
        }
    }

    #[test]
    pub fn test_add_round_key() {
        let st0 = AESState::from_bytes([0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff]);
        let st1 = st0.add_round_key([0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f]);
        let st2 = AESState::from_bytes([0x00, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xa0, 0xb0, 0xc0, 0xd0, 0xe0, 0xf0]);
        assert_eq!(st1, st2);
    }

    #[test]
    pub fn test_shift_rows_state() {
        let st0 = AESState([
            [0x00, 0x01, 0x02, 0x03],
            [0x04, 0x05, 0x06, 0x07],
            [0x08, 0x09, 0x0a, 0x0b],
            [0x0c, 0x0d, 0x0e, 0x0f],
        ]);

        let st1 = st0.shift_rows();

        assert_eq!(st1, AESState([
            [0x00, 0x01, 0x02, 0x03],
            [0x05, 0x06, 0x07, 0x04],
            [0x0a, 0x0b, 0x08, 0x09],
            [0x0f, 0x0c, 0x0d, 0x0e],
        ]));
    }

    #[test]
    pub fn test_shift_rows_bytes() {
        let st0 = AESState::from_bytes([0x63, 0xca, 0xb7, 0x04, 0x09, 0x53, 0xd0, 0x51, 0xcd, 0x60, 0xe0, 0xe7, 0xba, 0x70, 0xe1, 0x8c]);
        let st1 = st0.shift_rows();
        let st2 = AESState::from_bytes([0x63, 0x53, 0xe0, 0x8c, 0x09, 0x60, 0xe1, 0x04, 0xcd, 0x70, 0xb7, 0x51, 0xba, 0xca, 0xd0, 0xe7]);
        assert_eq!(st1, st2);
    }

    #[test]
    pub fn test_shift_rows_bijective() {
        for _ in 0..256 {
            let st0 = AESState(random());
            let st1 = st0.shift_rows();
            let st2 = st1.inv_shift_rows();
            let st3 = st2.shift_rows();

            assert_eq!(st0, st2);
            assert_eq!(st1, st3);
        }
    }

    #[test]
    pub fn test_mix_columns_state() {
        let st0 = AESState([
            [0x00, 0x01, 0x02, 0x03],
            [0x04, 0x05, 0x06, 0x07],
            [0x08, 0x09, 0x0a, 0x0b],
            [0x0c, 0x0d, 0x0e, 0x0f],
        ]);

        let st1 = st0.mix_columns();
        
        assert_eq!(st1, AESState([
            [0x08, 0x09, 0x0a, 0x0b],
            [0x1c, 0x1d, 0x1e, 0x1f],
            [0x00, 0x01, 0x02, 0x03],
            [0x14, 0x15, 0x16, 0x17],
        ]));
    }

    #[test]
    pub fn test_mix_columns_bytes() {
        let st0 = AESState::from_bytes([0x63, 0x53, 0xe0, 0x8c, 0x09, 0x60, 0xe1, 0x04, 0xcd, 0x70, 0xb7, 0x51, 0xba, 0xca, 0xd0, 0xe7]);
        let st1 = st0.mix_columns();
        let st2 = AESState::from_bytes([0x5f, 0x72, 0x64, 0x15, 0x57, 0xf5, 0xbc, 0x92, 0xf7, 0xbe, 0x3b, 0x29, 0x1d, 0xb9, 0xf9, 0x1a]);
        assert_eq!(st1, st2);
    }

    #[test]
    pub fn test_mix_columns_bijective() {
        for _ in 0..256 {
            let st0 = AESState(random());
            let st1 = st0.mix_columns();
            let st2 = st1.inv_mix_columns();
            let st3 = st2.mix_columns();

            assert_eq!(st0, st2);
            assert_eq!(st1, st3);
        }
    }

    #[test]
    pub fn test_aes128_key_schedule() {
        let aes = AES128::new([0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c]);
        assert_eq!(aes.0.0[0], [0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c]);
        assert_eq!(aes.0.0[1], [0xa0, 0xfa, 0xfe, 0x17, 0x88, 0x54, 0x2c, 0xb1, 0x23, 0xa3, 0x39, 0x39, 0x2a, 0x6c, 0x76, 0x05]);
        assert_eq!(aes.0.0[10], [0xd0, 0x14, 0xf9, 0xa8, 0xc9, 0xee, 0x25, 0x89, 0xe1, 0x3f, 0x0c, 0xc8, 0xb6, 0x63, 0x0c, 0xa6])
    }

    #[test]
    pub fn test_aes128_encrypt() {
        let aes = AES128::new([0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c]);
        let pt = [0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34];
        let ct = aes.encrypt(pt);
        assert_eq!(ct, [0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb, 0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a, 0x0b, 0x32]);
    }

    #[test]
    pub fn test_aes128_decrypt() {
        let aes = AES128::new([0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c]);
        let ct = [0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb, 0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a, 0x0b, 0x32];
        let pt = aes.decrypt(ct);
        assert_eq!(pt, [0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34]);
    }

    #[test]
    pub fn test_aes128_bijective() {
        for _ in 0..256 {
            let aes = AES128::new(random());
            let pt: [u8; 16]= random();
            let ct = aes.encrypt(pt);
            assert_eq!(aes.decrypt(ct), pt);
        }
    }

    #[test]
    pub fn test_aes192_key_schedule() {
        let aes = AES192::new([
            0x8e, 0x73, 0xb0, 0xf7, 0xda, 0x0e, 0x64, 0x52, 0xc8, 0x10, 0xf3, 0x2b, 0x80, 0x90, 0x79, 0xe5,
            0x62, 0xf8, 0xea, 0xd2, 0x52, 0x2c, 0x6b, 0x7b,
        ]);
        assert_eq!(aes.0.0[0], [0x8e, 0x73, 0xb0, 0xf7, 0xda, 0x0e, 0x64, 0x52, 0xc8, 0x10, 0xf3, 0x2b, 0x80, 0x90, 0x79, 0xe5]);
        assert_eq!(aes.0.0[1], [0x62, 0xf8, 0xea, 0xd2, 0x52, 0x2c, 0x6b, 0x7b, 0xfe, 0x0c, 0x91, 0xf7, 0x24, 0x02, 0xf5, 0xa5]);
        assert_eq!(aes.0.0[12], [0xe9, 0x8b, 0xa0, 0x6f, 0x44, 0x8c, 0x77, 0x3c, 0x8e, 0xcc, 0x72, 0x04, 0x01, 0x00, 0x22, 0x02]);
    }

    #[test]
    pub fn test_aes192_encrypt() {
        let aes = AES192::new([
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17
        ]);
        let pt = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff];
        let ct = aes.encrypt(pt);
        assert_eq!(ct, [0xdd, 0xa9, 0x7c, 0xa4, 0x86, 0x4c, 0xdf, 0xe0, 0x6e, 0xaf, 0x70, 0xa0, 0xec, 0x0d, 0x71, 0x91]);
    }

    #[test]
    pub fn test_aes192_decrypt() {
        let aes = AES192::new([
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17
        ]);
        let ct = [0xdd, 0xa9, 0x7c, 0xa4, 0x86, 0x4c, 0xdf, 0xe0, 0x6e, 0xaf, 0x70, 0xa0, 0xec, 0x0d, 0x71, 0x91];
        let pt = aes.decrypt(ct);
        assert_eq!(pt, [0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff]);
    }

    #[test]
    pub fn test_aes192_bijective() {
        for _ in 0..256 {
            let aes = AES192::new(random());
            let pt: [u8; 16]= random();
            let ct = aes.encrypt(pt);
            assert_eq!(aes.decrypt(ct), pt);
        }
    }

    #[test]
    pub fn test_aes256_key_schedule() {
        let aes = AES256::new([
            0x60, 0x3d, 0xeb, 0x10, 0x15, 0xca, 0x71, 0xbe, 0x2b, 0x73, 0xae, 0xf0, 0x85, 0x7d, 0x77, 0x81,
            0x1f, 0x35, 0x2c, 0x07, 0x3b, 0x61, 0x08, 0xd7, 0x2d, 0x98, 0x10, 0xa3, 0x09, 0x14, 0xdf, 0xf4,
        ]);
        assert_eq!(aes.0.0[0], [0x60, 0x3d, 0xeb, 0x10, 0x15, 0xca, 0x71, 0xbe, 0x2b, 0x73, 0xae, 0xf0, 0x85, 0x7d, 0x77, 0x81]);
        assert_eq!(aes.0.0[1], [0x1f, 0x35, 0x2c, 0x07, 0x3b, 0x61, 0x08, 0xd7, 0x2d, 0x98, 0x10, 0xa3, 0x09, 0x14, 0xdf, 0xf4]);
        assert_eq!(aes.0.0[14], [0xfe, 0x48, 0x90, 0xd1, 0xe6, 0x18, 0x8d, 0x0b, 0x04, 0x6d, 0xf3, 0x44, 0x70, 0x6c, 0x63, 0x1e]);
    }

    #[test]
    pub fn test_aes256_encrypt() {
        let aes = AES256::new([
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        ]);
        let pt = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff];
        let ct = aes.encrypt(pt);
        assert_eq!(ct, [0x8e, 0xa2, 0xb7, 0xca, 0x51, 0x67, 0x45, 0xbf, 0xea, 0xfc, 0x49, 0x90, 0x4b, 0x49, 0x60, 0x89]);
    }

    #[test]
    pub fn test_aes256_decrypt() {
        let aes = AES256::new([
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        ]);
        let ct = [0x8e, 0xa2, 0xb7, 0xca, 0x51, 0x67, 0x45, 0xbf, 0xea, 0xfc, 0x49, 0x90, 0x4b, 0x49, 0x60, 0x89];
        let pt = aes.decrypt(ct);
        assert_eq!(pt, [0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff]);
    }

    #[test]
    pub fn test_aes256_bijective() {
        for _ in 0..256 {
            let aes = AES256::new(random());
            let pt: [u8; 16]= random();
            let ct = aes.encrypt(pt);
            assert_eq!(aes.decrypt(ct), pt);
        }
    }
}