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

#[macro_export]
macro_rules! impl_write_from_update {
    ($t: ty) => {
        impl std::io::Write for $t {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                self.update(buf);
                Ok(buf.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
    };
}
