use anyhow::Result;

pub trait Decodable: Sized {
    // TODO: implement validate or reader mechanism
    fn decode(bytes: &[u8]) -> Result<Self>;
}
