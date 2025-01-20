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
    pub fn new(min: T, max: T, value: Option<T>, units: Option<units::Unit>) -> Self {
        let (channel_sender, _) = watch::channel(Default::default());

        Self {
            min,
            max,

            value: value.unwrap_or_default(),
            units: units.unwrap_or_default(),

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

    /// Unlikely to be used. Preferred method is the following:
    /// ```
    /// let value = *parameter.watch().borrow();
    /// ```
    /// Most of the time this data is being accessed from a seperate thread
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

    /// Sends the current value to the tokio::watch for all receivers to update
    /// Only sends the value, because the other contents of the struct should not change after instantiation
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
        Self::new(Default::default(), Default::default(), None, None)
    }
}
