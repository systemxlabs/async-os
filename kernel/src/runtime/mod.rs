mod executor;
mod waker;

pub use executor::*;
pub use waker::*;

pub fn init() {
    init_executor();
}
