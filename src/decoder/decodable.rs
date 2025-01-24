pub trait Decodable {
    // TODO: implement validate or reader mechanism
    fn decode(bytes: &[u8]) -> Self;
}
