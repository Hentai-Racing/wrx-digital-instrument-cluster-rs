use crate::data::units::Unit;

use crossbeam::atomic::AtomicCell;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tokio::sync::watch;

use std::fmt::{self, Debug};

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

#[allow(unused)]
#[derive(Debug)]
pub enum Node {
    ReadOnlyParameter(&'static str),
    Parameter(&'static str),
    Page {
        name: &'static str,
        items: Box<[Node]>,
    },
}

impl Node {
    fn fmt_with_indent(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        let pad = "  ".repeat(indent);

        match self {
            Node::ReadOnlyParameter(name) => {
                writeln!(f, "{pad}[RO] {name}")
            }
            Node::Parameter(name) => {
                writeln!(f, "{pad}[RW] {name}")
            }
            Node::Page { name, items } => {
                writeln!(f, "{pad}{name} {{")?;
                for item in items.iter() {
                    item.fmt_with_indent(f, indent + 1)?;
                }
                writeln!(f, "{pad}}}")
            }
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

/// Defines a serdes capable structure with parameter fields and optional default values.
/// Capable of having multiple pages
/// TODO: add example (src/application/user.rs:@UserConfig)
#[macro_export]
macro_rules! parameter_struct {
    ($page:ident { $($items:tt)* }) => {
        $crate::parameter_struct!(@page $page
            { }
            { }
            { }
            { }
            $($items)*
        );
    };

    (@page $page:ident
        { $($params:tt)* }
        { $($inits:tt)* }
        { $($defs:tt)* }
        { $($entries:tt)* }
    ) => {pastey::paste!{
        $($defs)*

        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(non_camel_case_types)]
        pub struct [<$page:camel_edge>] {
            $($params)*
        }

        #[allow(unused)]
        impl [<$page:camel_edge>] {
            pub fn apply(&self, from: Self) {
                $crate::parameter_struct!(@apply self, from| $($entries)*);
            }

            pub fn set_by_path(&self, param_path: &str, value: &str) {
                let (target, path) = match param_path.split_once('.') {
                    Some((root, path)) => {(root, path)}
                    _ => {(param_path, "")}
                };

                $crate::parameter_struct!(@path self target; path; value| $($entries)*)
            }

            pub fn get_page_layout(&self) -> $crate::data::parameters::Node {
                return $crate::parameter_struct!(@node $page self| $($entries)*);
            }
        }

        impl Default for [<$page:camel_edge>] {
            fn default() -> Self {
                Self {
                    $($inits)*
                }
            }
        }
    }};

    (@page $page:ident
        { $($params:tt)* }
        { $($inits:tt)* }
        { $($defs:tt)* }
        { $($entries:tt)* }
        $vis:vis $([$permissions:tt])? $param:ident: $ty:ty $(= $val:expr)?, $($rest:tt)*
    ) => {
        $crate::parameter_struct!(@page $page
            { $($params)* $vis $param: $crate::data::parameters::Parameter<$ty>, }
            { $($inits)* $param: $crate::__default_value!($ty| $($val)?), }
            { $($defs)* }
            { $($entries)* (param $([$permissions])? $param: $ty) }
            $($rest)*
        );
    };

    (@page $page:ident
        { $($params:tt)* }
        { $($inits:tt)* }
        { $($defs:tt)* }
        { $($entries:tt)* }
        $sub:ident { $($inner:tt)* }, $($rest:tt)*
    ) => {pastey::paste!{
        $crate::parameter_struct!(@page $page
            { $($params)* pub $sub: [<$sub:camel_edge>], }
            { $($inits)* $sub: Default::default(), }
            { $($defs)* $crate::parameter_struct!($sub { $($inner)* }); }
            { $($entries)* (page $sub) }
            $($rest)*
        );
    }};

    (@apply $self:ident, $from:ident|) => {};
    (@apply $self:ident, $from:ident| (param $([$permissions:tt])? $param:ident: $ty:ty) $($rest:tt)*) => {
        $self.$param.set_value($from.$param.value());
        $crate::parameter_struct!(@apply $self, $from| $($rest)*)
    };
    (@apply $self:ident, $from:ident| (page $sub:ident) $($rest:tt)*) => {
        $self.$sub.apply($from.$sub);
        $crate::parameter_struct!(@apply $self, $from| $($rest)*)
    };

    (@node-internal $self:ident| param [ro] $param:ident: $ty:ty) => {
        $crate::data::parameters::Node::ReadOnlyParameter(stringify!($param))
    };
    (@node-internal $self:ident| param $param:ident: $ty:ty) => {
        $crate::data::parameters::Node::Parameter(stringify!($param))
    };
    (@node-internal $self:ident| page $sub:ident) => {
        $self.$sub.get_page_layout()
    };
    (@node $page:ident $self:ident| $(($node:tt $([$permissions:tt])? $thing:ident $(: $ty:ty)?))*) => {
        $crate::data::parameters::Node::Page{
            name: stringify!($page),
            items: Box::new([$(
                $crate::parameter_struct!(@node-internal $self| $node $([$permissions])? $thing $(: $ty)?)
            ),*])
        }
    };

    (@path-internal $self:ident $path:expr; $value:ident| param [ro] $param:ident: $ty:ty) => {
        eprintln!("Failed to set {} to {:?}: Parameter is read-only", stringify!($param), $value)
    };
    (@path-internal $self:ident $path:expr; $value:ident| param $param:ident: $ty:ty) => {
        match $value.parse::<$ty>() {
            Ok(value) => $self.$param.set_value(value),
            Err(e) => eprintln!("Failed to set {} to {:?}: {e:?}", stringify!($param), $value)
        }
    };
    (@path-internal $self:ident $path:expr; $value:ident| page $sub:ident) => {
        $self.$sub.set_by_path($path, $value);
    };
    (@path $self:ident $target:expr; $path:expr; $val:ident| $(($node:tt $([$permissions:tt])? $thing: ident $(: $ty:ty)?))*) => {
        match $target {
            $(stringify!($thing) => {
                $crate::parameter_struct!(@path-internal $self $path; $val| $node $([$permissions])? $thing $(: $ty)?);
            }),*
            _ => {
                println!("Path [{}] does not exist!", $path);
            }
        }
    };
}
