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

        println!("Total number of character: {:?}", content.len());
        println!("{:?}", &content[..99]);

        Ok(())
    }
}
