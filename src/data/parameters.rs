use std::fmt::Debug;

use crate::data::units::Unit;
use crossbeam::atomic::AtomicCell;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tokio::sync::watch;

pub struct DataParameter<T> {
    min: AtomicCell<T>,
    max: AtomicCell<T>,

    value: AtomicCell<T>,
    units: Unit,

    changed: watch::Sender<T>,
}

#[derive(Clone)]
pub struct FieldParameter<T> {
    value: T,

    changed: watch::Sender<T>,
}

impl<T> DataParameter<T>
where
    T: Copy + Clone + Default + PartialEq + PartialOrd + Debug,
{
    pub fn new(min: T, max: T, value: Option<T>, units: Option<Unit>) -> Self {
        let (changed, _) = watch::channel(Default::default());

        Self {
            min: AtomicCell::new(min),
            max: AtomicCell::new(max),

            value: AtomicCell::new(value.unwrap_or_default()),
            units: units.unwrap_or_default(),

            changed,
        }
    }

    pub fn set_max(&self, value: T) {
        self.max.store(value);
    }

    pub fn set_min(&self, value: T) {
        self.min.store(value);
    }

    pub fn set_value(&self, value: T) {
        if self.value.swap(value) != value {
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

    pub fn value(&self) -> T {
        self.value.load()
    }

    #[allow(unused)]
    pub fn min(&self) -> T {
        self.min.load()
    }

    #[allow(unused)]
    pub fn max(&self) -> T {
        self.max.load()
    }

    fn send_changed(&self) {
        self.changed.send_replace(self.value());
    }

    pub fn watch(&self) -> watch::Receiver<T> {
        self.changed.subscribe()
    }
}

impl<T> Default for DataParameter<T>
where
    T: Copy + Clone + Default + PartialEq + PartialOrd + Debug,
{
    fn default() -> Self {
        Self::new(Default::default(), Default::default(), None, None)
    }
}

impl<T> From<T> for DataParameter<T>
where
    T: Copy + Clone + Default + PartialEq + PartialOrd + Debug,
{
    fn from(value: T) -> Self {
        Self::new(Default::default(), Default::default(), Some(value), None)
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
