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
        let ret = tmp.clone();
        self.value.store(tmp);
        ret
    }

    pub fn set_value(&self, value: T) {
        let current = self.value.take();

        if current != value {
            self.value.store(value.clone());
            self.send_changed(value);
        } else {
            self.value.store(current);
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

#[macro_export]
#[doc(hidden)]
macro_rules! __default_value {
    ($type:ty| $param_default:expr) => {
        $param_default.into()
    };
    (String| ) => {
        String::from("Unknown").into()
    };
    ($type:ty| ) => {
        Default::default()
    };
}

/// Executes a match on the value of the specified parameter.
/// Re-executes upon parameter updates.
#[macro_export]
macro_rules! parameter_match {
    ($param:expr => { $($val:pat => $blk:block),* $(,)? }) => {{
        let __param = $param;
        tokio::spawn(async move {
            let mut watch = $param.watch();
            let mut value = $param.value();

            loop {
                 match value {
                    $($val => $blk),*
                    _ => {}
                }

                tokio::select! {
                    Ok(_) = watch.changed() => {
                        value = *watch.borrow_and_update();
                    },
                    else => {break;}
                }
            }
        });
    }}
}

/// Defines a serdes capable structure with parameter fields and optional default values.
/// Capable of having multiple pages
/// TODO: add example (src/application/user.rs:@ConfigManager)
#[macro_export]
macro_rules! parameter_struct {
    ($page:ident { $($items:tt)* }) => {
        crate::parameter_struct!(@munch_page
            $page
            { }
            { }
            { }
            { }
            $($items)*
        );
    };

    (@munch_page $page:ident
        { $($params:tt)* }
        { $($inits:tt)* }
        { $($defs:tt)* }
        { $($entries:tt)* }
    ) => {
        $($defs)*

        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(non_camel_case_types)]
        pub struct $page {
            $($params)*
        }

        impl Default for $page {
            fn default() -> Self {
                Self {
                    $($inits)*
                }
            }
        }

        #[allow(unused)]
        impl $page {
            pub fn apply(&self, from: Self) {
                crate::parameter_struct!(@apply self, from; $($entries)*);
            }

            pub fn set_by_path(&self, param_path: &str, value: &str) {
                let param_path: Vec<&str> = (*param_path).split(".").collect();
                for i in param_path {
                    println!("{i}");
                }
            }
        }
    };

    (@munch_page
        $page:ident
        { $($params:tt)* }
        { $($inits:tt)* }
        { $($defs:tt)* }
        { $($entries:tt)* }
        $vis:vis $param:ident : $param_type:ty $(= $val:expr)? , $($rest:tt)*
    ) => {
        crate::parameter_struct!(@munch_page
            $page
            { $($params)* $vis $param: crate::data::parameters::Parameter<$param_type>, }
            { $($inits)* $param: crate::__default_value!($param_type| $($val)?), }
            { $($defs)* }
            { $($entries)* (param $param) }
            $($rest)*
        );
    };

    (@munch_page
        $page:ident
        { $($params:tt)* }
        { $($inits:tt)* }
        { $($defs:tt)* }
        { $($entries:tt)* }
        $sub:ident { $($inner:tt)* } , $($rest:tt)*
    ) => {
        crate::parameter_struct!(@munch_page
            $page
            { $($params)* pub $sub: $sub, }
            { $($inits)* $sub: Default::default(), }
            { $($defs)* crate::parameter_struct!($sub { $($inner)* }); }
            { $($entries)* (sub $sub) }
            $($rest)*
        );
    };

    (@apply $self:ident, $from:ident;) => {};
    (@apply $self:ident, $from:ident; (param $param:ident) $($rest:tt)*) => {
        $self.$param.set_value($from.$param.value());
        crate::parameter_struct!(@apply $self, $from; $($rest)*)
    };
    (@apply $self:ident, $from:ident; (sub $sub:ident) $($rest:tt)*) => {
        $self.$sub.apply($from.$sub);
        crate::parameter_struct!(@apply $self, $from; $($rest)*)
    };
}
