use crate::data::units::UnitSystem;
use crate::slint_generatedApp::{App, RSTypeResolver};

use slint::{ComponentHandle, SharedString, Weak};
use strum::VariantArray;

pub fn bridge(handle_weak: Weak<App>) {
    if let Some(handle) = handle_weak.upgrade() {
        let global_resolver = handle.global::<RSTypeResolver>();

        global_resolver.on_rs_string_bool(|value| value.as_str() == format!("{}", true));
        global_resolver.on_sl_bool_string(|value| format!("{}", bool::from(value)).into());

        // TODO: make a macro to generate this for all user-facing enums
        // TODO: find a better way
        global_resolver.on_is_enum_type(|ty| matches!(ty.as_str(), stringify!(UnitSystem)));
        global_resolver.on_get_enum_variants(|ty| match ty.as_str() {
            stringify!(UnitSystem) => {
                let list: Vec<SharedString> = UnitSystem::VARIANTS
                    .iter()
                    .map(|var| SharedString::from(var.as_ref()))
                    .collect();
                list.as_slice().into()
            }
            _ => ["ERROR".into()].into(),
        });
    }
}
