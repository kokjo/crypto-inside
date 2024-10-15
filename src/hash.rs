pub trait Update {
    fn update(&mut self, data: &[u8]);
}

pub trait HashAlgorithm: Default + Update {
    const DIGEST_SIZE: usize;

    fn finalize(self) -> [u8; Self::DIGEST_SIZE];
}

pub trait DynHashAlgorithm: Default + Update {
    fn digest_size(&self) -> usize;

    fn finalize(self) -> Vec<u8>;
}

#[macro_export]
macro_rules! impl_dynhash_from_hash {
    ($t: ty) => {
        impl DynHashAlgorithm for $t {
            fn digest_size(&self) -> usize {
                <Self as HashAlgorithm>::DIGEST_SIZE
            }

            fn finalize(self) -> Vec<u8> {
                HashAlgorithm::finalize(self).to_vec()
            }
        }
    };
}