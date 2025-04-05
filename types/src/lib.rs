use anyhow::Result;

pub trait Example: Send + Sync + Metadata {
    fn main(&self) -> Result<()>;
}

pub trait Metadata {
    fn id(&self) -> String;
    fn description(&self) -> String;
    fn page(&self) -> String;

    fn to_desc(&self) -> String {
        format!(
            "{} - {} - page {}",
            self.id(),
            self.description(),
            self.page()
        )
    }
}
