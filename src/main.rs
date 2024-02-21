use std::fs::File;
use evdev_rs::enums::{EventCode, EV_KEY, EV_ABS};
use evdev_rs::{AbsInfo, EnableCodeData, Device, DeviceWrapper, InputEvent, ReadFlag, UInputDevice};
use uinput::event::Event;
use uinput::event::Event::{Controller, Absolute};
use uinput::event::Absolute::{Position, Wheel};
use uinput::event::controller::Controller::{GamePad, DPad, JoyStick};
use uinput::event::controller::GamePad as GamePad_Events;
use uinput::event::controller::{DPad as D_Events, JoyStick as J_Events};
use std::sync::Arc;
use std::sync::Mutex;

pub mod profiles;
pub mod devices;

use profiles::*;
use devices::*;

pub struct VirtualJoystick {
    _events: Vec<Event>,
    _device: uinput::Device
}

impl VirtualJoystick {
    pub fn new() -> Self {
        let _events = vec![
            Controller(GamePad(GamePad_Events::A)),
            Controller(GamePad(GamePad_Events::B)),
            Controller(GamePad(GamePad_Events::C)),
            Controller(GamePad(GamePad_Events::X)),
            Controller(GamePad(GamePad_Events::Y)),
            Controller(GamePad(GamePad_Events::Z)),
            Controller(DPad(D_Events::Down)),
            Controller(DPad(D_Events::Left)),
            Controller(DPad(D_Events::Right)),
            Controller(DPad(D_Events::Up)),
            Controller(JoyStick(J_Events::Thumb)),
            Controller(GamePad(GamePad_Events::TL)),
            Controller(GamePad(GamePad_Events::TR)),
            Controller(GamePad(GamePad_Events::TL2)),
            Controller(GamePad(GamePad_Events::TR2)),
            Controller(GamePad(GamePad_Events::Select)),
            Controller(GamePad(GamePad_Events::Start)),
            Controller(GamePad(GamePad_Events::Mode)),
            Controller(GamePad(GamePad_Events::ThumbL)),
            Controller(GamePad(GamePad_Events::ThumbR)),
            Absolute(Position(uinput::event::absolute::Position::X)),
            Absolute(Position(uinput::event::absolute::Position::Y)),
            Absolute(Position(uinput::event::absolute::Position::RX)),
            Absolute(Position(uinput::event::absolute::Position::RY)),
            Absolute(Wheel(uinput::event::absolute::Wheel::Throttle)),
            Absolute(Wheel(uinput::event::absolute::Wheel::Brake))
        ];

        let mut dev_builder = match uinput::default(){
            Ok(build) => build.name("Virtual Joystick").unwrap(),
            Err(_e) => {panic!("Unable to create virual joystick with UInput")}
        };

        for i in _events.iter() {
            dev_builder = dev_builder.event(*i).unwrap();
        }

        Self {
            _events,
            _device: dev_builder.create().unwrap()
        }
    }
}


fn main() {
    let mut _interface = VirtualJoystick::new();
    let events = build();
    let manager = DeviceManager::new(events.clone());
    let keyboard = Device::new_from_file(
        File::open(events.0.1).unwrap()
    ).unwrap();
    let mouse = Device::new_from_file(
        File::open(events.0.0).unwrap()
    ).unwrap();

    manager.handle_events(keyboard, mouse);
    loop {

    }
}
