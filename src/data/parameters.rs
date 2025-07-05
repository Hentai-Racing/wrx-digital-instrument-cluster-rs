use crate::data::units::Unit;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tokio::sync::watch;

#[derive(Clone, Serialize, Deserialize)]
pub struct DataParameter<T> {
    min: T,
    max: T,

    value: T,
    units: Unit,

    #[serde(skip)]
    changed: watch::Sender<T>,
}

#[derive(Clone)]
pub struct FieldParameter<T> {
    value: T,

    changed: watch::Sender<T>,
}

impl<T> DataParameter<T>
where
    T: Copy + Clone + Default + PartialEq + PartialOrd + Serialize,
{
    pub fn new(min: T, max: T, value: Option<T>, units: Option<Unit>) -> Self {
        let (changed, _) = watch::channel(Default::default());

        Self {
            min,
            max,

            value: value.unwrap_or_default(),
            units: units.unwrap_or_default(),

            changed,
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

    pub fn set_units(&mut self, unit: Unit) {
        self.units = unit;
    }

    #[allow(unused)]
    pub fn units(&self) -> Unit {
        self.units
    }

    /// Unlikely to be used. Preferred method is the following:
    /// ```
    /// let value = *parameter.watch().borrow();
    /// ```
    /// Most of the time, this data is being accessed from a seperate thread
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
        self.changed.send_replace(self.value());
    }

    pub fn watch(&self) -> watch::Receiver<T> {
        self.changed.subscribe()
    }
}

impl<T> Default for DataParameter<T>
where
    T: Copy + Clone + Default + PartialEq + PartialOrd + Serialize,
{
    fn default() -> Self {
        Self::new(Default::default(), Default::default(), None, None)
    }
}

impl<T> FieldParameter<T>
where
    T: Clone + Default + Serialize + PartialEq,
{
    pub fn new(value: T) -> Self {
        let (changed, _) = watch::channel(value.clone());

        Self { value, changed }
    }

    pub fn value(&self) -> T {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: impl Into<T>) {
        let value: T = value.into();

        if self.value != value {
            self.value = value;

            self.send_changed();
        }
    }

    fn send_changed(&self) {
        self.changed.send_replace(self.value());
    }

    pub fn watch(&self) -> watch::Receiver<T> {
        self.changed.subscribe()
    }
}

impl<T> Default for FieldParameter<T>
where
    T: Clone + Default + Serialize + PartialEq,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T> Serialize for FieldParameter<T>
where
    T: Serialize + Clone + Default,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.value.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for FieldParameter<T>
where
    T: Deserialize<'de> + Clone + Default + Serialize + PartialEq,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(FieldParameter::new)
    }
}

impl<T> From<T> for FieldParameter<T>
where
    T: Clone + Default + Serialize + PartialEq,
{
    fn from(value: T) -> Self {
        FieldParameter::new(value)
    }
}
