use crate::data::units::UnitSystem;
use crate::slint_generatedApp::{App, RSTypeResolver};

use slint::{ComponentHandle, Weak};

pub fn bridge(handle_weak: Weak<App>) {
    if let Some(handle) = handle_weak.upgrade() {
        let global_resolver = handle.global::<RSTypeResolver>();

        global_resolver.on_rs_string_bool(|value| value.as_str() == format!("{}", true));
        global_resolver.on_sl_bool_string(|value| format!("{}", bool::from(value)).into());

        // TODO: make proc_macro for user facing enums to get variants
        global_resolver.on_is_enum_type(|ty| matches!(ty.as_str(), stringify!(UnitSystem)));
        global_resolver.on_get_enum_values(|ty| match ty.as_str() {
            stringify!(UnitSystem) => ["USCS".into(), "SI".into()].into(),
            _ => ["ERROR".into()].into(),
        });
    }
}
