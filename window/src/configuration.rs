use std::sync::{Arc, Mutex};

pub trait WindowConfiguration {
    fn use_ime(&self) -> bool {
        false
    }

    fn send_composed_key_when_left_alt_is_pressed(&self) -> bool {
        false
    }

    fn send_composed_key_when_right_alt_is_pressed(&self) -> bool {
        true
    }

    // Applies to Windows only.
    // For macos, see send_composed_key_when_XXX_alt_is_pressed
    fn use_dead_keys(&self) -> bool {
        true
    }

    fn enable_wayland(&self) -> bool {
        true
    }

    fn prefer_egl(&self) -> bool {
        true
    }

    fn native_macos_fullscreen_mode(&self) -> bool {
        false
    }

    /// Retrieves the opacity configuration from the host application.
    /// Note that this value doesn't directly control the opacity of
    /// the window from the perspective of this window crate; the application
    /// must set the alpha level of the pixels when it renders the window.
    /// This method is used by the macOS impl to adjust other window settings
    /// when the window is transparent.
    fn window_background_opacity(&self) -> f32 {
        1.0
    }
}

lazy_static::lazy_static! {
    static ref CONFIG: Mutex<Arc<dyn WindowConfiguration + Send + Sync>> = default_config();
}

pub(crate) fn config() -> Arc<dyn WindowConfiguration + Send + Sync> {
    Arc::clone(&CONFIG.lock().unwrap())
}

fn default_config() -> Mutex<Arc<dyn WindowConfiguration + Send + Sync>> {
    struct DefConfig;
    impl WindowConfiguration for DefConfig {}
    Mutex::new(Arc::new(DefConfig))
}

pub fn set_configuration<C: WindowConfiguration + Send + Sync + 'static>(c: C) {
    let mut global_config = CONFIG.lock().unwrap();
    *global_config = Arc::new(c);
}
