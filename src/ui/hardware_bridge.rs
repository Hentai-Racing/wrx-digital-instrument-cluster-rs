use crate::hardware::hardware_backend::{self, HardwareBackend};

struct HardwareBridge {
    backend: HardwareBackend,
}

impl HardwareBridge {
    pub fn new(backend: HardwareBackend) -> Self {
        Self { backend }
    }
}
