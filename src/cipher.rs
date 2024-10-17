pub trait BlockCipher {
    const KEY_SIZE: usize;
    const BLOCK_SIZE: usize;

    fn new(key: [u8; Self::KEY_SIZE]) -> Self;

    fn encrypt(&self, block: [u8; Self::BLOCK_SIZE]) -> [u8; Self::BLOCK_SIZE];

    fn decrypt(&self, block: [u8; Self::BLOCK_SIZE]) -> [u8; Self::BLOCK_SIZE];
}