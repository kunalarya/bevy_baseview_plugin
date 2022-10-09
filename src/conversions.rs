use bevy::input::mouse::MouseButton;

pub fn baseview_mousebutton_to_bevy(button: baseview::MouseButton) -> MouseButton {
    match button {
        baseview::MouseButton::Left => MouseButton::Left,
        baseview::MouseButton::Middle => MouseButton::Middle,
        baseview::MouseButton::Right => MouseButton::Right,
        baseview::MouseButton::Other(val) => MouseButton::Other(val as u16),
        _ => {
            MouseButton::Other(0) // TODO: Figure out back/forward mapping
        }
    }
}
