use bevy::input::keyboard::KeyCode as BevyKeyCode;
use keyboard_types::Code;

#[cfg(target_os = "linux")]
pub fn code_to_scancode(code: Code) -> u32 {
    match code {
        Code::Escape => 0x0009,
        Code::Digit1 => 0x000A,
        Code::Digit2 => 0x000B,
        Code::Digit3 => 0x000C,
        Code::Digit4 => 0x000D,
        Code::Digit5 => 0x000E,
        Code::Digit6 => 0x000F,
        Code::Digit7 => 0x0010,
        Code::Digit8 => 0x0011,
        Code::Digit9 => 0x0012,
        Code::Digit0 => 0x0013,
        Code::Minus => 0x0014,
        Code::Equal => 0x0015,
        Code::Backspace => 0x0016,
        Code::Tab => 0x0017,
        Code::KeyQ => 0x0018,
        Code::KeyW => 0x0019,
        Code::KeyE => 0x001A,
        Code::KeyR => 0x001B,
        Code::KeyT => 0x001C,
        Code::KeyY => 0x001D,
        Code::KeyU => 0x001E,
        Code::KeyI => 0x001F,
        Code::KeyO => 0x0020,
        Code::KeyP => 0x0021,
        Code::BracketLeft => 0x0022,
        Code::BracketRight => 0x0023,
        Code::Enter => 0x0024,
        Code::ControlLeft => 0x0025,
        Code::KeyA => 0x0026,
        Code::KeyS => 0x0027,
        Code::KeyD => 0x0028,
        Code::KeyF => 0x0029,
        Code::KeyG => 0x002A,
        Code::KeyH => 0x002B,
        Code::KeyJ => 0x002C,
        Code::KeyK => 0x002D,
        Code::KeyL => 0x002E,
        Code::Semicolon => 0x002F,
        Code::Quote => 0x0030,
        Code::Backquote => 0x0031,
        Code::ShiftLeft => 0x0032,
        Code::Backslash => 0x0033,
        Code::KeyZ => 0x0034,
        Code::KeyX => 0x0035,
        Code::KeyC => 0x0036,
        Code::KeyV => 0x0037,
        Code::KeyB => 0x0038,
        Code::KeyN => 0x0039,
        Code::KeyM => 0x003A,
        Code::Comma => 0x003B,
        Code::Period => 0x003C,
        Code::Slash => 0x003D,
        Code::ShiftRight => 0x003E,
        Code::NumpadMultiply => 0x003F,
        Code::AltLeft => 0x0040,
        Code::Space => 0x0041,
        Code::CapsLock => 0x0042,
        Code::F1 => 0x0043,
        Code::F2 => 0x0044,
        Code::F3 => 0x0045,
        Code::F4 => 0x0046,
        Code::F5 => 0x0047,
        Code::F6 => 0x0048,
        Code::F7 => 0x0049,
        Code::F8 => 0x004A,
        Code::F9 => 0x004B,
        Code::F10 => 0x004C,
        Code::NumLock => 0x004D,
        Code::ScrollLock => 0x004E,
        Code::Numpad7 => 0x004F,
        Code::Numpad8 => 0x0050,
        Code::Numpad9 => 0x0051,
        Code::NumpadSubtract => 0x0052,
        Code::Numpad4 => 0x0053,
        Code::Numpad5 => 0x0054,
        Code::Numpad6 => 0x0055,
        Code::NumpadAdd => 0x0056,
        Code::Numpad1 => 0x0057,
        Code::Numpad2 => 0x0058,
        Code::Numpad3 => 0x0059,
        Code::Numpad0 => 0x005A,
        Code::NumpadDecimal => 0x005B,
        Code::IntlBackslash => 0x005E,
        Code::F11 => 0x005F,
        Code::F12 => 0x0060,
        Code::IntlRo => 0x0061,
        Code::Convert => 0x0064,
        Code::KanaMode => 0x0065,
        Code::NonConvert => 0x0066,
        Code::NumpadEnter => 0x0068,
        Code::ControlRight => 0x0069,
        Code::NumpadDivide => 0x006A,
        Code::PrintScreen => 0x006B,
        Code::AltRight => 0x006C,
        Code::Home => 0x006E,
        Code::ArrowUp => 0x006F,
        Code::PageUp => 0x0070,
        Code::ArrowLeft => 0x0071,
        Code::ArrowRight => 0x0072,
        Code::End => 0x0073,
        Code::ArrowDown => 0x0074,
        Code::PageDown => 0x0075,
        Code::Insert => 0x0076,
        Code::Delete => 0x0077,
        Code::AudioVolumeMute => 0x0079,
        Code::AudioVolumeDown => 0x007A,
        Code::AudioVolumeUp => 0x007B,
        Code::NumpadEqual => 0x007D,
        Code::Pause => 0x007F,
        Code::NumpadComma => 0x0081,
        Code::Lang1 => 0x0082,
        Code::Lang2 => 0x0083,
        Code::IntlYen => 0x0084,
        Code::MetaLeft => 0x0085,
        Code::MetaRight => 0x0086,
        Code::ContextMenu => 0x0087,
        Code::BrowserStop => 0x0088,
        Code::Again => 0x0089,
        Code::Props => 0x008A,
        Code::Undo => 0x008B,
        Code::Select => 0x008C,
        Code::Copy => 0x008D,
        Code::Open => 0x008E,
        Code::Paste => 0x008F,
        Code::Find => 0x0090,
        Code::Cut => 0x0091,
        Code::Help => 0x0092,
        Code::LaunchApp2 => 0x0094,
        Code::WakeUp => 0x0097,
        Code::LaunchApp1 => 0x0098,
        Code::LaunchMail => 0x00A3,
        Code::BrowserFavorites => 0x00A4,
        Code::BrowserBack => 0x00A6,
        Code::BrowserForward => 0x00A7,
        Code::Eject => 0x00A9,
        Code::MediaTrackNext => 0x00AB,
        Code::MediaPlayPause => 0x00AC,
        Code::MediaTrackPrevious => 0x00AD,
        Code::MediaStop => 0x00AE,
        Code::MediaSelect => 0x00B3,
        Code::BrowserHome => 0x00B4,
        Code::BrowserRefresh => 0x00B5,
        Code::BrowserSearch => 0x00E1,
        Code::Unidentified => 0x0000,
        _ => 0x0000,
    }
}

#[cfg(target_os = "windows")]
pub fn code_to_scancode(code: Code) -> u32 {
    match scan_code {
        Code::Escape => 0x1,
        Code::Digit1 => 0x2,
        Code::Digit2 => 0x3,
        Code::Digit3 => 0x4,
        Code::Digit4 => 0x5,
        Code::Digit5 => 0x6,
        Code::Digit6 => 0x7,
        Code::Digit7 => 0x8,
        Code::Digit8 => 0x9,
        Code::Digit9 => 0xA,
        Code::Digit0 => 0xB,
        Code::Minus => 0xC,
        Code::Equal => 0xD,
        Code::Backspace => 0xE,
        Code::Tab => 0xF,
        Code::KeyQ => 0x10,
        Code::KeyW => 0x11,
        Code::KeyE => 0x12,
        Code::KeyR => 0x13,
        Code::KeyT => 0x14,
        Code::KeyY => 0x15,
        Code::KeyU => 0x16,
        Code::KeyI => 0x17,
        Code::KeyO => 0x18,
        Code::KeyP => 0x19,
        Code::BracketLeft => 0x1A,
        Code::BracketRight => 0x1B,
        Code::Enter => 0x1C,
        Code::ControlLeft => 0x1D,
        Code::KeyA => 0x1E,
        Code::KeyS => 0x1F,
        Code::KeyD => 0x20,
        Code::KeyF => 0x21,
        Code::KeyG => 0x22,
        Code::KeyH => 0x23,
        Code::KeyJ => 0x24,
        Code::KeyK => 0x25,
        Code::KeyL => 0x26,
        Code::Semicolon => 0x27,
        Code::Quote => 0x28,
        Code::Backquote => 0x29,
        Code::ShiftLeft => 0x2A,
        Code::Backslash => 0x2B,
        Code::KeyZ => 0x2C,
        Code::KeyX => 0x2D,
        Code::KeyC => 0x2E,
        Code::KeyV => 0x2F,
        Code::KeyB => 0x30,
        Code::KeyN => 0x31,
        Code::KeyM => 0x32,
        Code::Comma => 0x33,
        Code::Period => 0x34,
        Code::Slash => 0x35,
        Code::ShiftRight => 0x36,
        Code::NumpadMultiply => 0x37,
        Code::AltLeft => 0x38,
        Code::Space => 0x39,
        Code::CapsLock => 0x3A,
        Code::F1 => 0x3B,
        Code::F2 => 0x3C,
        Code::F3 => 0x3D,
        Code::F4 => 0x3E,
        Code::F5 => 0x3F,
        Code::F6 => 0x40,
        Code::F7 => 0x41,
        Code::F8 => 0x42,
        Code::F9 => 0x43,
        Code::F10 => 0x44,
        Code::Pause => 0x45,
        Code::ScrollLock => 0x46,
        Code::Numpad7 => 0x47,
        Code::Numpad8 => 0x48,
        Code::Numpad9 => 0x49,
        Code::NumpadSubtract => 0x4A,
        Code::Numpad4 => 0x4B,
        Code::Numpad5 => 0x4C,
        Code::Numpad6 => 0x4D,
        Code::NumpadAdd => 0x4E,
        Code::Numpad1 => 0x4F,
        Code::Numpad2 => 0x50,
        Code::Numpad3 => 0x51,
        Code::Numpad0 => 0x52,
        Code::NumpadDecimal => 0x53,
        Code::PrintScreen => 0x54,
        Code::IntlBackslash => 0x56,
        Code::F11 => 0x57,
        Code::F12 => 0x58,
        Code::NumpadEqual => 0x59,
        Code::KanaMode => 0x70,
        Code::Lang2 => 0x71,
        Code::Lang1 => 0x72,
        Code::IntlRo => 0x73,
        Code::Convert => 0x79,
        Code::NonConvert => 0x7B,
        Code::IntlYen => 0x7D,
        Code::NumpadComma => 0x7E,
        Code::MediaTrackPrevious => 0x110,
        Code::MediaTrackNext => 0x119,
        Code::NumpadEnter => 0x11C,
        Code::ControlRight => 0x11D,
        Code::AudioVolumeMute => 0x120,
        Code::LaunchApp2 => 0x121,
        Code::MediaPlayPause => 0x122,
        Code::MediaStop => 0x124,
        Code::AudioVolumeDown => 0x12E,
        Code::AudioVolumeUp => 0x130,
        Code::BrowserHome => 0x132,
        Code::NumpadDivide => 0x135,
        Code::PrintScreen => 0x137,
        Code::AltRight => 0x138,
        Code::NumLock => 0x145,
        Code::Home => 0x147,
        Code::ArrowUp => 0x148,
        Code::PageUp => 0x149,
        Code::ArrowLeft => 0x14B,
        Code::ArrowRight => 0x14D,
        Code::End => 0x14F,
        Code::ArrowDown => 0x150,
        Code::PageDown => 0x151,
        Code::Insert => 0x152,
        Code::Delete => 0x153,
        Code::MetaLeft => 0x15B,
        Code::MetaRight => 0x15C,
        Code::ContextMenu => 0x15D,
        Code::Power => 0x15E,
        Code::BrowserSearch => 0x165,
        Code::BrowserFavorites => 0x166,
        Code::BrowserRefresh => 0x167,
        Code::BrowserStop => 0x168,
        Code::BrowserForward => 0x169,
        Code::BrowserBack => 0x16A,
        Code::LaunchApp1 => 0x16B,
        Code::LaunchMail => 0x16C,
        Code::MediaSelect => 0x16D,
        Code::Lang2 => 0x1F1,
        Code::Lang1 => 0x1F2,
        Code::Unidentified => 0x000,
        _ => 0x000,
    }
}
#[cfg(target_os = "macos")]
pub fn code_to_scancode(code: Code) -> u32 {
    match code {
        Code::KeyA => 0x00,
        Code::KeyS => 0x01,
        Code::KeyD => 0x02,
        Code::KeyF => 0x03,
        Code::KeyH => 0x04,
        Code::KeyG => 0x05,
        Code::KeyZ => 0x06,
        Code::KeyX => 0x07,
        Code::KeyC => 0x08,
        Code::KeyV => 0x09,
        Code::IntlBackslash => 0x0a,
        Code::KeyB => 0x0b,
        Code::KeyQ => 0x0c,
        Code::KeyW => 0x0d,
        Code::KeyE => 0x0e,
        Code::KeyR => 0x0f,
        Code::KeyY => 0x10,
        Code::KeyT => 0x11,
        Code::Digit1 => 0x12,
        Code::Digit2 => 0x13,
        Code::Digit3 => 0x14,
        Code::Digit4 => 0x15,
        Code::Digit6 => 0x16,
        Code::Digit5 => 0x17,
        Code::Equal => 0x18,
        Code::Digit9 => 0x19,
        Code::Digit7 => 0x1a,
        Code::Minus => 0x1b,
        Code::Digit8 => 0x1c,
        Code::Digit0 => 0x1d,
        Code::BracketRight => 0x1e,
        Code::KeyO => 0x1f,
        Code::KeyU => 0x20,
        Code::BracketLeft => 0x21,
        Code::KeyI => 0x22,
        Code::KeyP => 0x23,
        Code::Enter => 0x24,
        Code::KeyL => 0x25,
        Code::KeyJ => 0x26,
        Code::Quote => 0x27,
        Code::KeyK => 0x28,
        Code::Semicolon => 0x29,
        Code::Backslash => 0x2a,
        Code::Comma => 0x2b,
        Code::Slash => 0x2c,
        Code::KeyN => 0x2d,
        Code::KeyM => 0x2e,
        Code::Period => 0x2f,
        Code::Tab => 0x30,
        Code::Space => 0x31,
        Code::Backquote => 0x32,
        Code::Backspace => 0x33,
        Code::NumpadEnter => 0x34,
        Code::Escape => 0x35,
        Code::MetaRight => 0x36,
        Code::MetaLeft => 0x37,
        Code::ShiftLeft => 0x38,
        Code::CapsLock => 0x39,
        // Note: in the linked source doc=> this is "OSLeft"
        Code::AltLeft => 0x3a,
        Code::ControlLeft => 0x3b,
        Code::ShiftRight => 0x3c,
        // Note: in the linked source doc=> this is "OSRight"
        Code::AltRight => 0x3d,
        Code::ControlRight => 0x3e,
        //  Code::Fn=>  No events fired        0x3f                                                           ,
        //  Code::F17=>                          0x40                                                         ,
        Code::NumpadDecimal => 0x41,
        Code::NumpadMultiply => 0x43,
        Code::NumpadAdd => 0x45,
        Code::NumLock => 0x47,
        Code::AudioVolumeUp => 0x48,
        Code::AudioVolumeDown => 0x49,
        Code::AudioVolumeMute => 0x4a,
        Code::NumpadDivide => 0x4b,
        // Code::NumpadEnter => 0x4c,
        Code::NumpadSubtract => 0x4e,
        //  Code::F18=>                          0x4f                                                         ,
        //  Code::F19=>                          0x50                                                         ,
        Code::NumpadEqual => 0x51,
        Code::Numpad0 => 0x52,
        Code::Numpad1 => 0x53,
        Code::Numpad2 => 0x54,
        Code::Numpad3 => 0x55,
        Code::Numpad4 => 0x56,
        Code::Numpad5 => 0x57,
        Code::Numpad6 => 0x58,
        Code::Numpad7 => 0x59,
        //  Code::F20=>                          0x5a                                                         ,
        Code::Numpad8 => 0x5b,
        Code::Numpad9 => 0x5c,
        Code::IntlYen => 0x5d,
        Code::IntlRo => 0x5e,
        Code::NumpadComma => 0x5f,
        Code::F5 => 0x60,
        Code::F6 => 0x61,
        Code::F7 => 0x62,
        Code::F3 => 0x63,
        Code::F8 => 0x64,
        Code::F9 => 0x65,
        Code::Lang2 => 0x66,
        Code::F11 => 0x67,
        Code::Lang1 => 0x68,
        // Note: this is listed as F13, but in testing with a standard
        // USB kb, this the code produced by PrtSc.
        Code::PrintScreen => 0x69,
        //  Code::F16=>                          0x6a                                                         ,
        //  Code::F14=>                          0x6b                                                         ,
        Code::F10 => 0x6d,
        Code::ContextMenu => 0x6e,
        Code::F12 => 0x6f,
        //  Code::F15=>                          0x71                                                         ,
        Code::Help => 0x72,
        Code::Home => 0x73,
        Code::PageUp => 0x74,
        Code::Delete => 0x75,
        Code::F4 => 0x76,
        Code::End => 0x77,
        Code::F2 => 0x78,
        Code::PageDown => 0x79,
        Code::F1 => 0x7a,
        Code::ArrowLeft => 0x7b,
        Code::ArrowRight => 0x7c,
        Code::ArrowDown => 0x7d,
        Code::ArrowUp => 0x7e,
        Code::Unidentified => 0x0,
        _ => 0x0,
    }
}

pub fn key_to_keycode(key: keyboard_types::Key) -> Option<BevyKeyCode> {
    match key {
        // keyboard_types::Key::AVRInput => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::AVRPower => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Accept => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Again => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::AllCandidates => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Alphanumeric => Some(BevyKeyCode::TODO),
        keyboard_types::Key::Alt => Some(BevyKeyCode::RAlt), // TODO: Less accurate.
        // keyboard_types::Key::AltGraph => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::AppSwitch => Some(BevyKeyCode::TODO),
        keyboard_types::Key::ArrowDown => Some(BevyKeyCode::Down),
        keyboard_types::Key::ArrowLeft => Some(BevyKeyCode::Left),
        keyboard_types::Key::ArrowRight => Some(BevyKeyCode::Right),
        keyboard_types::Key::ArrowUp => Some(BevyKeyCode::Up),
        // keyboard_types::Key::Attn => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::AudioBalanceLeft => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::AudioBalanceRight => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::AudioBassBoostDown => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::AudioBassBoostToggle => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::AudioBassBoostUp => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::AudioFaderFront => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::AudioFaderRear => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::AudioSurroundModeNext => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::AudioTrebleDown => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::AudioTrebleUp => Some(BevyKeyCode::TODO),
        keyboard_types::Key::AudioVolumeDown => Some(BevyKeyCode::VolumeDown),
        keyboard_types::Key::AudioVolumeMute => Some(BevyKeyCode::Mute),
        keyboard_types::Key::AudioVolumeUp => Some(BevyKeyCode::VolumeUp),
        keyboard_types::Key::Backspace => Some(BevyKeyCode::Back),
        // keyboard_types::Key::BrightnessDown => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::BrightnessUp => Some(BevyKeyCode::TODO),
        keyboard_types::Key::BrowserBack => Some(BevyKeyCode::WebBack),
        keyboard_types::Key::BrowserFavorites => Some(BevyKeyCode::WebFavorites),
        keyboard_types::Key::BrowserForward => Some(BevyKeyCode::WebForward),
        keyboard_types::Key::BrowserHome => Some(BevyKeyCode::WebHome),
        keyboard_types::Key::BrowserRefresh => Some(BevyKeyCode::WebRefresh),
        keyboard_types::Key::BrowserSearch => Some(BevyKeyCode::WebSearch),
        keyboard_types::Key::BrowserStop => Some(BevyKeyCode::WebStop),
        // keyboard_types::Key::Call => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Camera => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::CameraFocus => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Cancel => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::CapsLock => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::ChannelDown => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::ChannelUp => Some(BevyKeyCode::TODO),
        keyboard_types::Key::Character(s) => match s.as_str() {
            "A" | "a" => Some(BevyKeyCode::A),
            "B" | "b" => Some(BevyKeyCode::B),
            "C" | "c" => Some(BevyKeyCode::C),
            "D" | "d" => Some(BevyKeyCode::D),
            "E" | "e" => Some(BevyKeyCode::E),
            "F" | "f" => Some(BevyKeyCode::F),
            "G" | "g" => Some(BevyKeyCode::G),
            "H" | "h" => Some(BevyKeyCode::H),
            "I" | "i" => Some(BevyKeyCode::I),
            "J" | "j" => Some(BevyKeyCode::J),
            "K" | "k" => Some(BevyKeyCode::K),
            "L" | "l" => Some(BevyKeyCode::L),
            "M" | "m" => Some(BevyKeyCode::M),
            "N" | "n" => Some(BevyKeyCode::N),
            "O" | "o" => Some(BevyKeyCode::O),
            "P" | "p" => Some(BevyKeyCode::P),
            "Q" | "q" => Some(BevyKeyCode::Q),
            "R" | "r" => Some(BevyKeyCode::R),
            "S" | "s" => Some(BevyKeyCode::S),
            "T" | "t" => Some(BevyKeyCode::T),
            "U" | "u" => Some(BevyKeyCode::U),
            "V" | "v" => Some(BevyKeyCode::V),
            "W" | "w" => Some(BevyKeyCode::W),
            "X" | "x" => Some(BevyKeyCode::X),
            "Y" | "y" => Some(BevyKeyCode::Y),
            "Z" | "z" => Some(BevyKeyCode::Z),
            " " => Some(BevyKeyCode::Space),
            "-" => Some(BevyKeyCode::Minus),
            "0" => Some(BevyKeyCode::Key0),
            "1" => Some(BevyKeyCode::Key1),
            "2" => Some(BevyKeyCode::Key2),
            "3" => Some(BevyKeyCode::Key3),
            "4" => Some(BevyKeyCode::Key4),
            "5" => Some(BevyKeyCode::Key5),
            "6" => Some(BevyKeyCode::Key6),
            "7" => Some(BevyKeyCode::Key7),
            "8" => Some(BevyKeyCode::Key8),
            "9" => Some(BevyKeyCode::Key9),
            ";" => Some(BevyKeyCode::Semicolon),
            "/" => Some(BevyKeyCode::Slash),
            "," => Some(BevyKeyCode::Comma),
            "." => Some(BevyKeyCode::Period),
            "=" => Some(BevyKeyCode::Equals),
            "+" => Some(BevyKeyCode::Plus),
            "@" => Some(BevyKeyCode::At),
            "*" => Some(BevyKeyCode::Asterisk),
            "'" => Some(BevyKeyCode::Apostrophe),
            "^" => Some(BevyKeyCode::Caret),
            _ => None,
        },
        // keyboard_types::Key::Clear => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Close => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::ClosedCaptionToggle => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::CodeInput => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::ColorF0Red => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::ColorF1Green => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::ColorF2Yellow => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::ColorF3Blue => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::ColorF4Grey => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::ColorF5Brown => Some(BevyKeyCode::TODO),
        keyboard_types::Key::Compose => Some(BevyKeyCode::Compose),
        // keyboard_types::Key::ContextMenu => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Control => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Convert => Some(BevyKeyCode::TODO),
        keyboard_types::Key::Copy => Some(BevyKeyCode::Copy),
        // keyboard_types::Key::CrSel => Some(BevyKeyCode::TODO),
        keyboard_types::Key::Cut => Some(BevyKeyCode::Cut),
        // keyboard_types::Key::DVR => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Dead => Some(BevyKeyCode::TODO),
        keyboard_types::Key::Delete => Some(BevyKeyCode::Delete),
        // keyboard_types::Key::Dimmer => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::DisplaySwap => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Eisu => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Eject => Some(BevyKeyCode::TODO),
        keyboard_types::Key::End => Some(BevyKeyCode::End),
        // keyboard_types::Key::EndCall => Some(BevyKeyCode::TODO),
        keyboard_types::Key::Enter => Some(BevyKeyCode::Return),
        // keyboard_types::Key::EraseEof => Some(BevyKeyCode::TODO),
        keyboard_types::Key::Escape => Some(BevyKeyCode::Escape),
        // keyboard_types::Key::ExSel => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Execute => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Exit => Some(BevyKeyCode::TODO),
        keyboard_types::Key::F1 => Some(BevyKeyCode::F1),
        keyboard_types::Key::F10 => Some(BevyKeyCode::F10),
        keyboard_types::Key::F11 => Some(BevyKeyCode::F11),
        keyboard_types::Key::F12 => Some(BevyKeyCode::F12),
        keyboard_types::Key::F13 => Some(BevyKeyCode::F13),
        keyboard_types::Key::F14 => Some(BevyKeyCode::F14),
        keyboard_types::Key::F15 => Some(BevyKeyCode::F15),
        keyboard_types::Key::F16 => Some(BevyKeyCode::F16),
        keyboard_types::Key::F17 => Some(BevyKeyCode::F17),
        keyboard_types::Key::F18 => Some(BevyKeyCode::F18),
        keyboard_types::Key::F19 => Some(BevyKeyCode::F19),
        keyboard_types::Key::F2 => Some(BevyKeyCode::F2),
        keyboard_types::Key::F20 => Some(BevyKeyCode::F20),
        keyboard_types::Key::F21 => Some(BevyKeyCode::F21),
        keyboard_types::Key::F22 => Some(BevyKeyCode::F22),
        keyboard_types::Key::F23 => Some(BevyKeyCode::F23),
        keyboard_types::Key::F24 => Some(BevyKeyCode::F24),
        keyboard_types::Key::F3 => Some(BevyKeyCode::F3),
        keyboard_types::Key::F4 => Some(BevyKeyCode::F4),
        keyboard_types::Key::F5 => Some(BevyKeyCode::F5),
        keyboard_types::Key::F6 => Some(BevyKeyCode::F6),
        keyboard_types::Key::F7 => Some(BevyKeyCode::F7),
        keyboard_types::Key::F8 => Some(BevyKeyCode::F8),
        keyboard_types::Key::F9 => Some(BevyKeyCode::F9),
        // keyboard_types::Key::FavoriteClear0 => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::FavoriteClear1 => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::FavoriteClear2 => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::FavoriteClear3 => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::FavoriteRecall0 => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::FavoriteRecall1 => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::FavoriteRecall2 => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::FavoriteRecall3 => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::FavoriteStore0 => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::FavoriteStore1 => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::FavoriteStore2 => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::FavoriteStore3 => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::FinalMode => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Find => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Fn => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::FnLock => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::GoBack => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::GoHome => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::GroupFirst => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::GroupLast => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::GroupNext => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::GroupPrevious => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Guide => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::GuideNextDay => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::GuidePreviousDay => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::HangulMode => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::HanjaMode => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Hankaku => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::HeadsetHook => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Help => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Hibernate => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Hiragana => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::HiraganaKatakana => Some(BevyKeyCode::TODO),
        keyboard_types::Key::Home => Some(BevyKeyCode::Home),
        // keyboard_types::Key::Hyper => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Info => Some(BevyKeyCode::TODO),
        keyboard_types::Key::Insert => Some(BevyKeyCode::Insert),
        // keyboard_types::Key::InstantReplay => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::JunjaMode => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::KanaMode => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::KanjiMode => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Katakana => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Key11 => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Key12 => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::LastNumberRedial => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::LaunchApplication1 => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::LaunchApplication2 => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::LaunchCalendar => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::LaunchContacts => Some(BevyKeyCode::TODO),
        keyboard_types::Key::LaunchMail => Some(BevyKeyCode::Mail),
        // keyboard_types::Key::LaunchMediaPlayer => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::LaunchMusicPlayer => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::LaunchPhone => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::LaunchScreenSaver => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::LaunchSpreadsheet => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::LaunchWebBrowser => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::LaunchWebCam => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::LaunchWordProcessor => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Link => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::ListProgram => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::LiveContent => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Lock => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::LogOff => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::MailForward => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::MailReply => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::MailSend => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::MannerMode => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::MediaApps => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::MediaAudioTrack => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::MediaClose => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::MediaFastForward => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::MediaLast => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::MediaPause => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::MediaPlay => Some(BevyKeyCode::TODO),
        keyboard_types::Key::MediaPlayPause => Some(BevyKeyCode::PlayPause),
        // keyboard_types::Key::MediaRecord => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::MediaRewind => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::MediaSkipBackward => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::MediaSkipForward => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::MediaStepBackward => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::MediaStepForward => Some(BevyKeyCode::TODO),
        keyboard_types::Key::MediaStop => Some(BevyKeyCode::MediaStop),
        // keyboard_types::Key::MediaTopMenu => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::MediaTrackNext => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::MediaTrackPrevious => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Meta => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::MicrophoneToggle => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::MicrophoneVolumeDown => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::MicrophoneVolumeMute => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::MicrophoneVolumeUp => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::ModeChange => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::NavigateIn => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::NavigateNext => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::NavigateOut => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::NavigatePrevious => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::New => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::NextCandidate => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::NextFavoriteChannel => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::NextUserProfile => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::NonConvert => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Notification => Some(BevyKeyCode::TODO),
        keyboard_types::Key::NumLock => Some(BevyKeyCode::Numlock),
        // keyboard_types::Key::OnDemand => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Open => Some(BevyKeyCode::TODO),
        keyboard_types::Key::PageDown => Some(BevyKeyCode::PageDown),
        keyboard_types::Key::PageUp => Some(BevyKeyCode::PageUp),
        // keyboard_types::Key::Pairing => Some(BevyKeyCode::TODO),
        keyboard_types::Key::Paste => Some(BevyKeyCode::Paste),
        keyboard_types::Key::Pause => Some(BevyKeyCode::Pause),
        // keyboard_types::Key::PinPDown => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::PinPMove => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::PinPToggle => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::PinPUp => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Play => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::PlaySpeedDown => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::PlaySpeedReset => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::PlaySpeedUp => Some(BevyKeyCode::TODO),
        keyboard_types::Key::Power => Some(BevyKeyCode::Power),
        // keyboard_types::Key::PowerOff => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::PreviousCandidate => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Print => Some(BevyKeyCode::TODO),
        keyboard_types::Key::PrintScreen => Some(BevyKeyCode::Snapshot),
        // keyboard_types::Key::Process => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Props => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::RandomToggle => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::RcLowBattery => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::RecordSpeedNext => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Redo => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::RfBypass => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Romaji => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::STBInput => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::STBPower => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Save => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::ScanChannelsToggle => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::ScreenModeNext => Some(BevyKeyCode::TODO),
        keyboard_types::Key::ScrollLock => Some(BevyKeyCode::Scroll),
        // keyboard_types::Key::Select => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::Settings => Some(BevyKeyCode::TODO),
        keyboard_types::Key::Shift => Some(BevyKeyCode::LShift),
        // keyboard_types::Key::SingleCandidate => Some(BevyKeyCode::TODO),
        keyboard_types::Key::Super => Some(BevyKeyCode::LWin), // TODO:
        // keyboard_types::Key::Symbol => Some(BevyKeyCode::TODO),
        // keyboard_types::Key::SymbolLock => Some(BevyKeyCode::TODO),
        keyboard_types::Key::Tab => Some(BevyKeyCode::Tab),
        _ => None,
    }
}
