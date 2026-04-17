use std::{
    collections::BTreeMap,
    sync::{Arc, LazyLock},
};

pub static STATIC_OBD_DATA: LazyLock<Arc<StaticObddata>> = LazyLock::new(|| Default::default());

crate::parameter_struct!(StaticOBDData {
    [hidden] initialized: bool = false,

    s1_current_data {
        [ro] supported_pids: BTreeMap<u8, bool>,
    },

    s9_vehicle_information {
        [ro] vin: String,
        [ro] supported_pids: BTreeMap<u8, bool>,
    },
});
