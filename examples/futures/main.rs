use log::{debug, info};

use std::time::Duration;
use std::env;
use std::thread;
use std::fmt;
use std::task::Poll;

use futures_util::future;

/*
* Reading _bytes_ from the serial port.
*/
struct EZSP_Device {
    count: u8
}

impl EZSP_Device {
    fn new() -> EZSP_Device {
        EZSP_Device{ count: 0 }
    }
}
impl AsyncReader for EZSP_Device {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize>> {
        let count=self.count;
        self.count += 1;

        if (count % 2 ==1) {
            Poll::Pending
        } else {
            buf[0] = count;
            Poll::Ready(Ok(1))
        }
    }
}

async fn a_main() -> Result<()> {
    let dev = EZSP_Device::new();

    // Stream stuff from 'AsyncReader'


}

fn main() -> Result<(),String> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug")
    }
    env_logger::init();     // tbd. there's another way to mark that 'debug' is the default

    info!("main");

    Ok(())
}
