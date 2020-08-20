pub(crate) trait Show {
    fn show() -> Result;
}

pub(crate) type Result = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;
