# Keyboard to Joystick

This program translates keyboard events to joystick events! It was made with the main purpose of emulating joystick movement with the mouse.

## Usage:

Clone the repository and execute the program using [cargo](https://www.rust-lang.org/learn/get-started). The user needs to be in the "input" group; otherwise, it will be necessary to run the program as root. It is also necessary for the uinput module to be activated.

```bash
sudo modprobe uinput
cargo run --release
```

## Configuration:

The program uses an environment variable called "GAMEPAD_PROFILES". Its configuration file must be called "virtualconf.json". Below is an example of configuration:

### virtualconf.json

```json
{
    "mouse" : "/dev/input/event7",
    "keyboard" : "/dev/input/event6",
    "events" : {
        "REL_X": "10000|+|ABS_X",
        "REL_Y": "10000|+|ABS_Y",
        "BTN_LEFT": "1|+|BTN_WEST",
        "BTN_RIGHT": "1|+|BTN_TR",
        "KEY_W": "100000|-|ABS_RY",
        "KEY_A": "100000|-|ABS_RX",
        "KEY_S": "100000|+|ABS_RY",
        "KEY_D": "100000|+|ABS_RX",
        "KEY_F": "1|+|BTN_SOUTH",
        "KEY_Q": "1|+|BTN_MODE",
        "KEY_SPACE": "1|+|BTN_NORTH",
        "KEY_E": "1|+|BTN_TL",
        "KEY_X": "1|+|BTN_THUMBL",
        "KEY_LEFTCTRL": "1|+|BTN_THUMBR",
        "KEY_LEFTSHIFT": "1|+|BTN_EAST",
        "KEY_3": "1|+|BTN_THUMB",
        "KEY_R": "1|+|BTN_SELECT",
        "KEY_1": "1|+|BTN_START",
        "KEY_LEFT": "1|+|BTN_DPAD_LEFT",
        "KEY_RIGHT": "1|+|BTN_DPAD_RIGHT",
        "KEY_UP": "1|+|BTN_DPAD_UP",
        "KEY_DOWN": "1|+|BTN_DPAD_DOWN"
    }
}

```
The file needs to declare mouse, which is the path to your mouse in /dev/input. The same goes for the keyboard. In events, the first value corresponds to the trigger, that is, the mouse/keyboard event. The value associated with the trigger is the Virtual Joystick event, in the following format:

```
{Value multiplier}|{Multiplication sign}|{Event}
```

To find out the valid events, visit the documentation of the [evdev](https://docs.rs/evdev-rs/latest/evdev_rs/enums/enum.EventCode.html) library for Rust.
