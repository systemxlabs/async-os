use core::panic::PanicInfo;

use log::error;
use sbi_rt::system_reset;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        error!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message()
        );
    } else {
        error!("Panicked: {}", info.message());
    }
    system_reset(sbi_rt::Shutdown, sbi_rt::SystemFailure);
    unreachable!()
}
