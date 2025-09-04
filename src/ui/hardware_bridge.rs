use crate::hardware::hardware_backend::HardwareBackend;

struct HardwareBridge {
    backend: HardwareBackend,
}

impl HardwareBridge {
    pub fn new(backend: HardwareBackend) -> Self {
        Self { backend }
    }
}
