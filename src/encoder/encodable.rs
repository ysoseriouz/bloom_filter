pub trait Encodable {
    fn byte_size(&self) -> usize; // Base on 64-bit (8 bytes)
    fn encode(&self) -> Vec<u8>;
    // TODO: implement validate or reader mechanism
    fn decode(bytes: &[u8]) -> Self;
}
