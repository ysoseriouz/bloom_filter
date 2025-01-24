pub trait Encodable {
    fn byte_size(&self) -> usize; // Base on 64-bit (8 bytes)
    fn encode(&self) -> Vec<u8>;
}
