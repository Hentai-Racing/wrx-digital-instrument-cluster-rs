use crate::data::units;
use tokio::sync::watch;

#[derive(Clone)]
pub struct DataParameter<T> {
    min: T,
    max: T,

    observed_min: T,
    observed_max: T,

    value: T,
    units: units::Unit,

    init_value: bool,

    changed: watch::Sender<T>,
}

impl<T> DataParameter<T>
where
    T: Copy + Clone + Default + PartialEq + PartialOrd,
{
    pub fn new(min: T, max: T) -> Self {
        let (channel_sender, _) = watch::channel(Default::default());

        Self {
            min,
            max,

            observed_max: Default::default(),
            observed_min: Default::default(),

            value: Default::default(),
            units: Default::default(),

            init_value: true,

            changed: channel_sender,
        }
    }

    pub fn get_unit_short_str(&self) -> &str {
        self.units.get_short_str()
    }

    pub fn set_max(&mut self, value: T) {
        self.max = value;
    }

    pub fn set_min(&mut self, value: T) {
        self.min = value;
    }

    pub fn set_value(&mut self, value: T) {
        if self.init_value || (self.value != value) {
            if self.init_value {
                self.init_value = false;
            }

            self.value = value;

            self.update_observed_values();
            self.send_changed();
        }
    }

    pub fn set_units(&mut self, unit: units::Unit) {
        self.units = unit;
    }

    #[allow(unused)]
    pub fn units(&self) -> units::Unit {
        self.units
    }

    #[allow(unused)]
    pub fn value(&self) -> T {
        self.value
    }

    #[allow(unused)]
    pub fn min(&self) -> T {
        self.min
    }

    #[allow(unused)]
    pub fn max(&self) -> T {
        self.max
    }

    #[allow(unused)]
    pub fn observed_min(&self) -> T {
        self.observed_min
    }

    #[allow(unused)]
    pub fn observed_max(&self) -> T {
        self.observed_max
    }

    fn update_observed_values(&mut self) {
        let value = self.value;

        if self.init_value {
            self.observed_min = value;
            self.observed_max = value;
        } else {
            if value > self.observed_max {
                self.observed_max = value
            } else if value < self.observed_min {
                self.observed_min = value
            }
        }
    }

    fn send_changed(&self) {
        self.changed.send_replace(self.value);
    }

    pub fn watch(&self) -> watch::Receiver<T> {
        self.changed.subscribe()
    }
}

impl<T> Default for DataParameter<T>
where
    T: Copy + Clone + Default + PartialEq + PartialOrd,
{
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}
