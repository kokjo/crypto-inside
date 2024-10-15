pub const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
];

pub fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ ((!x) & z)
}

pub fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ ( x & z) ^ (y & z)
}

pub fn ep0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

pub fn ep1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

pub fn sig0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

pub fn sig1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

pub struct SHA256Context {
    data: [u8; 64],
    datalen: usize,
    bitlen: u64,
    state: [u32; 8],
}

impl SHA256Context {
    pub fn new() -> Self {
        Self {
            data: [0u8; 0x40],
            datalen: 0,
            bitlen: 0,
            state: [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19],
        }
    }

    pub fn transform(&mut self) {
        log::info!("SHA256 Transform: state={:08x?} data={:02x?}", self.state, self.data);

        let mut m = [0u32; 64];
        for i in 0..16 {
            m[i] = u32::from_be_bytes(self.data[4*i..4*(i+1)].try_into().unwrap());
            log::trace!("m[{:2}]={:08x}", i, m[i]);
        }
        for i in 16..64 {
            m[i] = sig1(m[i-2]).wrapping_add(m[i-7]).wrapping_add(sig0(m[i-15])).wrapping_add(m[i-16]);
            log::trace!("m[{:2}]={:08x} sig1({:08x})={:08x} m[{:2}]={:08x} sig0({:08x})={:08x} m[{:2}]={:08x}", i, m[i], m[i-2], sig1(m[i-2]), i-7, m[i-7], m[i-15], sig0(m[i-15]), i-16, m[i-16]);
        }

        log::debug!("m = {:08x?}", m);
        
        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];
        let mut e = self.state[4];
        let mut f = self.state[5];
        let mut g = self.state[6];
        let mut h = self.state[7];

        for i in 0..64 {
            let t1 = h.wrapping_add(ep1(e)).wrapping_add(ch(e, f, g)).wrapping_add(K[i]).wrapping_add(m[i]);
            let t2 = ep0(a).wrapping_add(maj(a, b, c));
            log::trace!("Round {:2}: t1={:08x} t2={:08x} k[{:2}]={:08x} m[{:2}]={:08x}", i, t1, t2, i, K[i], i, m[i]);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
            log::debug!("Round {:2}: a={:08x} b={:08x} c={:08x} d={:08x} e={:08x} f={:08x} g={:08x} h={:08x}", i, a, b, c, d, e, f, g, h);
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
        self.state[5] = self.state[5].wrapping_add(f);
        self.state[6] = self.state[6].wrapping_add(g);
        self.state[7] = self.state[7].wrapping_add(h);

        log::info!("SHA256 Transform: state={:08x?}", self.state);
    }

    pub fn update(&mut self, data: &[u8]) {
        log::info!("SHA256 Update: datalen={:2} bitlen={:016x} data={:02x?}", self.datalen, self.bitlen, data);

        for x in data {
            self.data[self.datalen] = *x;
            self.datalen += 1;
            if self.datalen == 64 {
                self.transform();
                self.datalen = 0;
                self.bitlen += 512;
            }
        }

        log::info!("SHA256 Update: datalen={:2} bitlen={:016x}", self.datalen, self.bitlen);
    }

    pub fn finalize(&mut self) -> [u8; 32] {
        log::info!("SHA256 Finalize: datalen={:2} bitlen={:016x}", self.datalen, self.bitlen);

        self.bitlen += 8 * self.datalen as u64;

        log::debug!("data[{:2}]={:02x}", self.datalen, 0x80);

        self.data[self.datalen] = 0x80;
        self.datalen += 1;

        log::debug!("Padding datalen={:2}", self.datalen);
        while self.datalen != 56 {
            self.data[self.datalen] = 0x00;
            self.datalen += 1;
            if self.datalen == 64 {
                self.transform();
                self.datalen = 0;
            }
        }

        log::debug!("bitlen={:016x}", self.bitlen);
        self.data[56..64].copy_from_slice(&self.bitlen.to_be_bytes());

        self.transform();

        let mut hash = [0u8; 32];

        hash[0..4].copy_from_slice(&self.state[0].to_be_bytes());
        hash[4..8].copy_from_slice(&self.state[1].to_be_bytes());
        hash[8..12].copy_from_slice(&self.state[2].to_be_bytes());
        hash[12..16].copy_from_slice(&self.state[3].to_be_bytes());
        hash[16..20].copy_from_slice(&self.state[4].to_be_bytes());
        hash[20..24].copy_from_slice(&self.state[5].to_be_bytes());
        hash[24..28].copy_from_slice(&self.state[6].to_be_bytes());
        hash[28..32].copy_from_slice(&self.state[7].to_be_bytes());

        hash
    }
}
