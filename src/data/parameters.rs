use crate::data::units::Unit;

use crossbeam::atomic::AtomicCell;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tokio::sync::watch;

use std::fmt::Debug;

pub struct Parameter<T> {
    value: AtomicCell<T>,

    changed: watch::Sender<T>,
}

impl<T> Parameter<T>
where
    T: Clone + Default + Serialize + PartialEq,
{
    pub fn new(value: T) -> Self {
        let (changed, _) = watch::channel(Default::default());

        Self {
            value: AtomicCell::new(value),
            changed,
        }
    }

    pub fn value(&self) -> T {
        let tmp = self.value.take();
        let clone = tmp.clone();
        self.value.store(tmp);
        clone
    }

    pub fn set_value(&self, value: T) {
        if self.value.swap(value.clone()) != value {
            self.send_changed(value);
        }
    }

    fn send_changed(&self, value: T) {
        self.changed.send_replace(value);
    }

    pub fn watch(&self) -> watch::Receiver<T> {
        self.changed.subscribe()
    }
}

impl<T> Default for Parameter<T>
where
    T: Clone + Default + Serialize + PartialEq,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T> Serialize for Parameter<T>
where
    T: Serialize + Default,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = self.value.take();
        let ret = value.serialize(serializer);

        self.value.store(value);

        ret
    }
}

impl<'de, T> Deserialize<'de> for Parameter<T>
where
    T: Deserialize<'de> + Clone + Default + Serialize + PartialEq,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Parameter::new)
    }
}

impl<T> From<T> for Parameter<T>
where
    T: Clone + Default + Serialize + PartialEq,
{
    fn from(value: T) -> Self {
        Parameter::new(value)
    }
}

pub struct DataParameter<T> {
    min: T,
    max: T,

    value: AtomicCell<T>,
    units: Unit,

    changed: watch::Sender<T>,
}

impl<T> DataParameter<T>
where
    T: Copy + Clone + Default + PartialEq + PartialOrd + Debug,
{
    pub fn new(min: T, max: T, value: Option<T>, units: Option<Unit>) -> Self {
        let (changed, _) = watch::channel(Default::default());

        Self {
            min: min,
            max: max,

            value: AtomicCell::new(value.unwrap_or_default()),
            units: units.unwrap_or_default(),

            changed,
        }
    }

    pub fn set_value(&self, value: T) {
        if self.value.swap(value) != value {
            self.send_changed();
        }
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
        self.min
    }

    #[allow(unused)]
    pub fn max(&self) -> T {
        self.max
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
