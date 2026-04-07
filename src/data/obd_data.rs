use std::collections::BTreeMap;

crate::parameter_struct!(StaticOBDData {
    [hidden] loaded: bool,

    s1_current_data {
        // pub [ro] supported_pids: BTreeMap<u32, bool>,
    },

    s9_vehicle_information {
        pub [ro] vin: bool,
    },
});
