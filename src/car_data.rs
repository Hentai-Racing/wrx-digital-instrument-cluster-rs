use crate::unit_conversion;

pub struct CarData {
    pub engine_rpm: u16,
}

/*
todo: find best way to store data as parameters
parameter should include:
    max possible value
    min possible value

    max observed value
    min observed value

    average value
    current value

    delta value - change over dt
    delta time - some period of time to measure dv

    units - original units from canbus data
    display units - this should probably be part of the unit handler. This should be the string displayed to the user
*/
pub struct DataParameter<T> {
    max_value: T,
    min_value: T,

    max_observed: T,
    min_observed: T,

    average_value: T,
    current_value: T,

    units: unit_conversion::Units,
    display_units: String,
}
