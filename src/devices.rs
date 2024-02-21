use evdev_rs::enums::EV_SYN;

use crate::*;

#[derive(Clone)]
pub struct DeviceManager {
    joystick: Arc<Mutex<UInputDevice>>,
    pub events: Vec<CustomEvent>,
}

impl DeviceManager {
    pub fn new(events: ((String, String), Vec<CustomEvent>)) -> Self {
        let interface = Device::new_from_file(
            File::open("/dev/input/event15").unwrap()
        ).unwrap();

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
            flat: 0,
            resolution: 0
        });

        interface.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_PRESSURE), Some(aux)).unwrap();
        interface.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_WHEEL), Some(aux)).unwrap();
        interface.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_RX), Some(aux)).unwrap();
        interface.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_RY), Some(aux)).unwrap();
        interface.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_X), Some(aux1)).unwrap();
        interface.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_Y), Some(aux1)).unwrap();

        let joystick = Arc::new(Mutex::new(UInputDevice::create_from_device(&interface).unwrap()));

        Self {
            joystick,
            events: events.1,
        }
    }

    pub fn handle_events(&self, keyboard: Device, mouse: Device) {
        let data_clone = self.clone();
        std::thread::spawn(move || {
            read_input(keyboard, data_clone);
        });
        let data_clone = self.clone();
        std::thread::spawn(move || {
            read_input(mouse, data_clone);
        });
    }

    fn make_action(&self, event: CustomEvent, trigger: InputEvent, _data: DeviceManager) {
        let code = match event.event {
            Ok(ok) => EventCode::EV_ABS(ok),
            Err(e) => EventCode::EV_KEY(e)
        };
        self.joystick.lock().unwrap().write_event(&InputEvent {
                time: trigger.time,
                event_code: code,
                value: trigger.value * event.multi * event.sign
            }
        ).unwrap();
        self.joystick.lock().unwrap().write_event(&InputEvent {
            time: trigger.time,
            event_code: EventCode::EV_SYN(EV_SYN::SYN_REPORT),
            value: 0,
        }).unwrap();

    }
}


fn read_input(device: Device, data_clone: DeviceManager) {
    loop {
        let (_, event) = device.next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING).unwrap();
        for i in data_clone.events.iter() {
            if let Ok(_code) = i.trigger {
                match event.event_code {
                    EventCode::EV_REL(code) if code == _code => {
                        data_clone.make_action(i.clone(), event.clone(), data_clone.clone());
                        break;
                    },
                    _ => ()
                }
            }else if let Err(_code) = i.trigger {
                match event.event_code {
                    EventCode::EV_KEY(code) if code == _code => {
                        println!("{:?}", _code);
                        data_clone.make_action(i.clone(), event.clone(), data_clone.clone());
                        break;
                    }
                    _ => ()
                }
            }
        }
    }
}
