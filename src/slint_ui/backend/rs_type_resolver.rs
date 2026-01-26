use crate::data::units::UnitSystem;
use crate::slint_generatedApp::{App, ClusterThemes, RSTypeResolver};
use crate::slint_ui::backend::lang::{Lang, StrLang};

use slint::{ComponentHandle, SharedString, Weak};
use strum::VariantArray;

macro_rules! variants_match_as_model {
    ($val:ident => {$($ty:tt),*}) => {
        match $val.as_str() {
            $(
                stringify!($ty) => $ty::VARIANTS
                    .iter()
                    .map(|var| SharedString::from(var.as_ref()))
                    .collect::<Vec<SharedString>>()
                    .as_slice()
                    .into(),
            )*
            _ => ["error_variant".into()].into()
        }
    };
}

pub fn bridge(handle_weak: Weak<App>) {
    if let Some(handle) = handle_weak.upgrade() {
        let resolver = handle.global::<RSTypeResolver>();

        resolver.on_rs_string_bool(|value| value.as_str() == format!("{}", true));
        resolver.on_sl_bool_string(|value| format!("{}", bool::from(value)).into());

        resolver.on_get_enum_variants(|ty| match ty.as_str() {
            stringify!(StrLang) => Lang::iter()
                .map(|var| SharedString::from(var.as_ref()))
                .collect::<Vec<SharedString>>()
                .as_slice()
                .into(),
            _ => variants_match_as_model!(ty => {
                UnitSystem,
                ClusterThemes
            }),
        });
    }
}
