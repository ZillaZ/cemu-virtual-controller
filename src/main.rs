use std::fs::File;
use evdev_rs::enums::{BusType, EventCode, EventType, EV_KEY, EV_REL, EV_SYN, EV_ABS};
use evdev_rs::{Device, DeviceWrapper, InputEvent, ReadFlag, UInputDevice, UninitDevice, EnableCodeData, AbsInfo};
use uinput::event::Event;
use uinput::event::Event::{Controller, Absolute};
use uinput::event::Absolute::{Position, Wheel};
use uinput::event::controller::Controller::{GamePad, DPad, Mouse};
use uinput::event::controller::GamePad as GamePad_Events;
use uinput::event::controller::{DPad as D_Events, Mouse as M_Events};
use std::sync::Arc;
use std::sync::Mutex;

pub struct ControllerInterface{
    button_events: [Event; 19],
    axes_events: [Event; 6],
    device: uinput::Device
}

impl ControllerInterface{
    pub fn new() -> ControllerInterface {
        //All the button Events
        let button_events = [
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
            Controller(GamePad(GamePad_Events::TL)),
            Controller(GamePad(GamePad_Events::TR)),
            Controller(GamePad(GamePad_Events::TL2)),
            Controller(GamePad(GamePad_Events::TR2)),
            Controller(GamePad(GamePad_Events::Select)),
            Controller(GamePad(GamePad_Events::Start)),
            Controller(GamePad(GamePad_Events::Mode)),
            Controller(GamePad(GamePad_Events::ThumbL)),
            Controller(GamePad(GamePad_Events::ThumbR)),

        ];
        //All the slider events
        let axes_events = [
            Absolute(Position(uinput::event::absolute::Position::X)),
            Absolute(Position(uinput::event::absolute::Position::Y)),
            Absolute(Position(uinput::event::absolute::Position::RX)),
            Absolute(Position(uinput::event::absolute::Position::RY)),
            Absolute(Wheel(uinput::event::absolute::Wheel::Throttle)),
            Absolute(Wheel(uinput::event::absolute::Wheel::Brake))
        ];
        //Make a new device, and error out cleanly.
        let mut dev_builder = match uinput::default(){
            Ok(build) => build.name("Joystick MUITO FODA").unwrap(),
            Err(_e) => {panic!("
Uinput file not found, you may need to enable the uinput kernel module with:
    modprobe uinput
If you are still getting this error, make sure that your user has rw access to /dev/uinput.\n")}
        };
        //Add the sliders
        for i in 0..6{
            dev_builder = dev_builder.event(axes_events[i]).unwrap();
        }
        //Add the buttons
        for i in 0..19{
            dev_builder = dev_builder.event(button_events[i]).unwrap();
        }
        //Make a new interface
        ControllerInterface {
            button_events,
            axes_events,
            device: dev_builder.create().unwrap()
        }
    }

    pub fn send_event(&mut self, num_button: usize, new_state: bool){
        //Changes the state of a controller button
        self.device.send(self.button_events[num_button], new_state as i32).unwrap();
        self.device.synchronize().unwrap();
    }

    pub fn axes_change(&mut self, num_axes: usize, new_value: f64){
        //Changes the state of a controller axes
        //Axes go to i16 limits, so scale from float
        let axis_value: i32 = (new_value*(i16::MAX as f64)) as i32;
        self.device.send(self.axes_events[num_axes], axis_value).unwrap();
        self.device.synchronize().unwrap()
    }
}


fn main() -> Result<(), std::io::Error> {
    let mut _interface = ControllerInterface::new();

    let ifile = File::open("/dev/input/event15")?;
    let interface = Device::new_from_file(ifile)?;

    let aux = EnableCodeData::AbsInfo(AbsInfo {
        value: 1,
        minimum: -100000,
        maximum: 100000,
        fuzz: 0,
        flat: 0,
        resolution: 0
    });
   let aux1 = EnableCodeData::AbsInfo(AbsInfo {
        value: 1,
        minimum: -300000,
        maximum: 300000,
        fuzz: 0,
        flat: 300000,
        resolution: 0
    });

    interface.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_PRESSURE), Some(aux))?;
    interface.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_WHEEL), Some(aux))?;
    interface.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_RX), Some(aux))?;
    interface.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_RY), Some(aux))?;
    interface.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_X), Some(aux1))?;
    interface.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_Y), Some(aux1))?;
    interface.enable(EventCode::EV_KEY(EV_KEY::BTN_TL2))?;
    interface.enable(EventCode::EV_KEY(EV_KEY::BTN_0))?;
    let v : Arc<Mutex<UInputDevice>> = Arc::new(Mutex::new(UInputDevice::create_from_device(&interface)?));

    let v_clone = Arc::clone(&v);
    let v_c2 = Arc::clone(&v);
    let v_c3 = Arc::clone(&v);
    let kbf = File::open("/dev/input/event6")?;
    let kb = Device::new_from_file(kbf)?;

    let _ = std::thread::spawn(move || {
        let kf = File::open("/dev/input/event6").unwrap();
        let kb = Device::new_from_file(kf).unwrap();
        kb.enable(EventCode::EV_KEY(EV_KEY::BTN_LEFT)).unwrap();

        loop {
            let (_status, event) = kb.next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING).unwrap();
            // Map these to mouse events
            println!("Event: {:?}", event);

            // Map direction keys to mouse events
            let e = match event.event_code {
                EventCode::EV_KEY(EV_KEY::KEY_W) => Some((EV_ABS::ABS_RY, event.value * 100000)),
                EventCode::EV_KEY(EV_KEY::KEY_A) => Some((EV_ABS::ABS_RX, event.value * -100000)),
                EventCode::EV_KEY(EV_KEY::KEY_S) => Some((EV_ABS::ABS_RY, event.value * -100000)),
                EventCode::EV_KEY(EV_KEY::KEY_D) => Some((EV_ABS::ABS_RX, event.value * 100000)),
                EventCode::EV_KEY(EV_KEY::KEY_2) => Some((EV_ABS::ABS_BRAKE, event.value * 100000)),
                EventCode::EV_KEY(EV_KEY::KEY_LEFTCTRL) => Some((EV_ABS::ABS_PRESSURE, event.value * 100000)),
                _ => None
            };


            if let Some((e, n)) = e {
                v_c3.lock().unwrap().write_event(&InputEvent {
                    time: event.time,
                    event_code: EventCode::EV_ABS(e),
                    value: n,
                });

                v_c3.lock().unwrap().write_event(&InputEvent {
                    time: event.time,
                    event_code: EventCode::EV_SYN(EV_SYN::SYN_REPORT),
                    value: 0,
                });
            }}
    });


    let _ = std::thread::spawn(move || {
        let mf = File::open("/dev/input/event7").unwrap();
        let m = Device::new_from_file(mf).unwrap();
        m.enable(EventCode::EV_KEY(EV_KEY::BTN_LEFT)).unwrap();

        loop {
            let (_status, event) = m.next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING).unwrap();
            // Map these to mouse events
            println!("Event: {:?}", event);

            // Map direction keys to mouse events
            let e = match event.event_code {
                EventCode::EV_KEY(EV_KEY::BTN_LEFT) => Some((EV_KEY::BTN_WEST, event.value)),
                EventCode::EV_KEY(EV_KEY::BTN_RIGHT) => Some((EV_KEY::BTN_TR, event.value)),
                _ => None
            };


            if let Some((e, n)) = e {
                v_clone.lock().unwrap().write_event(&InputEvent {
                    time: event.time,
                    event_code: EventCode::EV_KEY(e),
                    value: n,
                });

                v_clone.lock().unwrap().write_event(&InputEvent {
                    time: event.time,
                    event_code: EventCode::EV_SYN(EV_SYN::SYN_REPORT),
                    value: 0,
                });
            }}
    });


    let _ = std::thread::spawn(move || {
        let mf = File::open("/dev/input/event7").unwrap();
        let m = Device::new_from_file(mf).unwrap();
        m.enable(EventCode::EV_KEY(EV_KEY::BTN_LEFT)).unwrap();
//        let virt = UInputDevice::create_from_device(&m).unwrap();

        loop {
            let (_status, event) = m.next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING).unwrap();
            // Map these to mouse events
            println!("Event: {:?}", event);

            // Map direction keys to mouse events
            let e = match event.event_code {
                EventCode::EV_REL(EV_REL::REL_X) => Some((EV_ABS::ABS_X, event.value * 10000)),
                EventCode::EV_REL(EV_REL::REL_Y) => Some((EV_ABS::ABS_Y, event.value * 10000)),
                _ => None
            };

            /*let e1 = match event.event_code {
                EventCode::EV_REL(EV_REL::REL_X) => Some((EV_REL::REL_X, -event.value)),
                EventCode::EV_REL(EV_REL::REL_Y) => Some((EV_REL::REL_Y, -event.value)),
                _ => None
            };*/


            if let Some((e, n)) = e {
                v_c2.lock().unwrap().write_event(&InputEvent {
                    time: event.time,
                    event_code: EventCode::EV_ABS(e),
                    value: n,
                });

                v_c2.lock().unwrap().write_event(&InputEvent {
                    time: event.time,
                    event_code: EventCode::EV_SYN(EV_SYN::SYN_REPORT),
                    value: 0,
                });
            }
/*            if let Some((e, n)) = e1 {
                virt.write_event(&InputEvent {
                    time: event.time,
                    event_code: EventCode::EV_REL(e),
                    value: n,
                });

                virt.write_event(&InputEvent {
                    time: event.time,
                    event_code: EventCode::EV_SYN(EV_SYN::SYN_REPORT),
                    value: 0,
                });
            }*/
        }
    });



    loop {
        let (_status, event) = kb.next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING)?;
        println!("Event: {:?}", event);

        let e = match event.event_code {
            EventCode::EV_KEY(EV_KEY::KEY_SPACE) => Some((EV_KEY::BTN_NORTH, event.value)),
            EventCode::EV_KEY(EV_KEY::KEY_F) => Some((EV_KEY::BTN_SOUTH, event.value)),
            EventCode::EV_KEY(EV_KEY::KEY_Q) => Some((EV_KEY::BTN_MODE, event.value)),
            EventCode::EV_KEY(EV_KEY::KEY_E) => Some((EV_KEY::BTN_TL, event.value)),
            EventCode::EV_KEY(EV_KEY::KEY_LEFTSHIFT) => Some((EV_KEY::BTN_EAST, event.value)),
            EventCode::EV_KEY(EV_KEY::KEY_R) => Some((EV_KEY::BTN_SELECT, event.value)),
            EventCode::EV_KEY(EV_KEY::KEY_1) => Some((EV_KEY::BTN_START, event.value)),
            EventCode::EV_KEY(EV_KEY::KEY_LEFT) => Some((EV_KEY::BTN_DPAD_LEFT, event.value)),
            EventCode::EV_KEY(EV_KEY::KEY_RIGHT) => Some((EV_KEY::BTN_DPAD_RIGHT, event.value)),
            EventCode::EV_KEY(EV_KEY::KEY_UP) => Some((EV_KEY::BTN_DPAD_UP, event.value)),
            EventCode::EV_KEY(EV_KEY::KEY_DOWN) => Some((EV_KEY::BTN_DPAD_DOWN, event.value)),
            _ => None
        };


       if let Some((e, n)) = e {
            v.lock().unwrap().write_event(&InputEvent {
                time: event.time,
                event_code: EventCode::EV_KEY(e),
                value: n,
            })?;

            v.lock().unwrap().write_event(&InputEvent {
                time: event.time,
                event_code: EventCode::EV_SYN(EV_SYN::SYN_REPORT),
                value: 0,
            })?;
        }
    }
}
