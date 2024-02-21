CEMU Virtual Controller for Linux

Este programa cria um controle virtual que o emulador CEMU é capaz de entender.

Uso:

Clone o repositório e execute o programa usando cargo.

comando - cargo run --release

Configuração:

O programa utiliza uma váriavel de ambiente chamada "GAMEPAD_PROFILES". Seu arquivo de configuração precisa se chamar "virtualconf.json". Abaixo está um exemplo de configuração:

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

O arquivo precisa declarar mouse, que é o caminho para seu mouse em /dev/input. O mesmo vale para o teclado.
