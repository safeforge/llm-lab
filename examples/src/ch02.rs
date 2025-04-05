use macros::metadata;

use crate::Example;

#[metadata(
    id = "02.01",
    description = "listings::ch02::load_sample_text",
    page = "31"
)]
pub struct EG01;

impl Example for EG01 {
    fn main(&self) -> anyhow::Result<()> {
        let content = listings::ch02::load_sample_text()?;

        assert_eq!(content.len(), 20479);

        println!("Total number of character: {:?}", content.len());
        println!("{:?}", &content[..99]);

        Ok(())
    }
}

#[metadata(
    id = "02.02",
    description = "usage of `listings::ch02::create_sample_vocab`",
    page = "35"
)]
pub struct EG02;

impl Example for EG02 {
    fn main(&self) -> anyhow::Result<()> {
        let vocab = listings::ch02::create_sample_vocab()?;

        assert_eq!(vocab.len(), 1130);

        vocab
            .iter()
            .take(50)
            .for_each(|(k, v)| println!("{}: {}", k, v));

        Ok(())
    }
}
