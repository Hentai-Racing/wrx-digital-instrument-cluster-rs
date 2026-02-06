use crate::data::units::UnitSystem;
use crate::slint_generatedApp::{App, ClusterTheme, RSTypeResolver, SBoundInt};
use crate::slint_ui::backend::lang::{Lang, StrLang};

use slint::{ComponentHandle, SharedString, Weak};
use strum::VariantArray;

macro_rules! variants_match_as_model {
    ($val:ident => {$($ty:tt),*}) => {
        match $val.as_str() {
            $(
                stringify!($ty) => $ty::VARIANTS
                    .iter()
                    .map(|var| format!("{var}").into())
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

        resolver.on_rs_string_bool(|value| value.as_str().trim().parse().unwrap_or(false));
        resolver.on_sl_bool_string(|value| value.to_string().into());
        resolver.on_sboundint_to_string(|value| value.to_string().into());

        resolver.on_get_enum_variants(|ty| match ty.as_str().trim() {
            stringify!(StrLang) => Lang::iter()
                .map(|var| SharedString::from(var.as_ref()))
                .collect::<Vec<SharedString>>()
                .as_slice()
                .into(),
            _ => variants_match_as_model!(ty => {
                UnitSystem,
                ClusterTheme
            }),
        });
    }
}
