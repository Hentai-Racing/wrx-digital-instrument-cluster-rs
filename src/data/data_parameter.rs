use crate::data::units;
use tokio::sync::watch;

#[derive(Clone)]
pub struct DataParameter<T> {
    min: T,
    max: T,

    value: T,
    units: units::Unit,

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

            value: Default::default(),
            units: Default::default(),

            changed: channel_sender,
        }
    }

    pub fn set_max(&mut self, value: T) {
        self.max = value;
    }

    pub fn set_min(&mut self, value: T) {
        self.min = value;
    }

    pub fn set_value(&mut self, value: T) {
        if self.value != value {
            self.value = value;

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
