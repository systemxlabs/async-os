#[derive(Debug)]
pub enum KError {
    MemNotMapped,
}

pub type KResult<T> = Result<T, KError>;
