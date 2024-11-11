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

    init_value: bool, // Whether we've recieved an initial value. This may not be necessary

    changed: watch::Sender<T>,
}

impl<T> DataParameter<T>
where
    T: Copy + Clone + Default + PartialEq + PartialOrd + ToString,
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

            init_value: false,

            changed: channel_sender,
        }
    }

    pub fn set_value(&mut self, value: T) {
        if self.value != value {
            self.value = value;

            self.update_observed_values();
            self.send_changed();
        }
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

    /// Checks if the observed minimum and maximum values have been exceeded, then updates them.
    /// If `self.init_value` is `false`, we first set the observed values to the current value.
    /// `self.init` value serves to stop the `Default::default()` value from taking precedence.
    fn update_observed_values(&mut self) {
        let value = self.value;

        if !self.init_value {
            self.observed_min = value;
            self.observed_max = value;

            self.init_value = true;
        } else {
            if value > self.observed_max {
                self.observed_max = value
            } else if value < self.observed_min {
                self.observed_min = value
            }
        }
    }

    fn send_changed(&self) {
        match self.changed.send(self.value) {
            _ => {}
        }
    }

    pub fn watch(&self) -> watch::Receiver<T> {
        self.changed.subscribe()
    }
}

impl<T> Default for DataParameter<T>
where
    T: Copy + Clone + Default + PartialEq + PartialOrd + ToString,
{
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}
