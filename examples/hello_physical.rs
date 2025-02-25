use uinput::event::keyboard;

use std::thread;
use std::time::Duration;

fn main() {
    let mut device = uinput::open("/dev/input/event10")
        .unwrap()
        .create_wrapper()
        .unwrap();

    thread::sleep(Duration::from_secs(1));

    device.click(&keyboard::Key::H).unwrap();
    device.click(&keyboard::Key::E).unwrap();
    device.click(&keyboard::Key::L).unwrap();
    device.click(&keyboard::Key::L).unwrap();
    device.click(&keyboard::Key::O).unwrap();
    device.click(&keyboard::Key::Space).unwrap();
    device.click(&keyboard::Key::W).unwrap();
    device.click(&keyboard::Key::O).unwrap();
    device.click(&keyboard::Key::R).unwrap();
    device.click(&keyboard::Key::L).unwrap();
    device.click(&keyboard::Key::D).unwrap();
    device.click(&keyboard::Key::Enter).unwrap();

    device.synchronize().unwrap();
}
