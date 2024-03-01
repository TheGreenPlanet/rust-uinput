use std::thread;
use std::time::Duration;
use uinput::event::relative::Position::{X, Y};

fn main() {
    let mut device = uinput::open("/dev/input/event14")
        .unwrap()
        .create_wrapper()
        .unwrap();

    for _ in 1..10 {
        thread::sleep(Duration::from_secs(1));

        device.send(X, 50).unwrap();
        device.send(Y, 50).unwrap();
        device.synchronize().unwrap();
    }
}