use ::window::configuration::WindowConfiguration;
use config::configuration;

pub struct ConfigBridge;

impl WindowConfiguration for ConfigBridge {
    fn use_ime(&self) -> bool {
        configuration().use_ime
    }

    fn use_dead_keys(&self) -> bool {
        configuration().use_dead_keys
    }

    fn send_composed_key_when_left_alt_is_pressed(&self) -> bool {
        configuration().send_composed_key_when_left_alt_is_pressed
    }

    fn send_composed_key_when_right_alt_is_pressed(&self) -> bool {
        configuration().send_composed_key_when_right_alt_is_pressed
    }

    fn enable_wayland(&self) -> bool {
        configuration().enable_wayland
    }

    fn prefer_egl(&self) -> bool {
        configuration().prefer_egl
    }

    fn native_macos_fullscreen_mode(&self) -> bool {
        configuration().native_macos_fullscreen_mode
    }

    fn window_background_opacity(&self) -> f32 {
        configuration().window_background_opacity
    }
}
