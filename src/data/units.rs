#![allow(dead_code, unused)]

use std::default;

use rand::distributions::DistIter;

#[derive(Copy, Clone, Default, Debug)]
pub enum UnitSystem {
    #[default]
    SI, // International System of Units
    USCS, // US Customary System
}

#[derive(Clone, Copy, Default, Debug)]
pub enum PressureUnit {
    #[default]
    BAR,
    KPA,
    PSI,
}

#[derive(Clone, Copy, Default, Debug)]
pub enum Unit {
    #[default]
    None,
    Distance(UnitSystem), // default
    Pressure(UnitSystem),
    Speed(UnitSystem),
    Temperature(UnitSystem),
    Flow(UnitSystem),
    Volume(UnitSystem),
}

impl Unit {
    pub fn convert_value_to(&self, value: impl Into<f64>, to: UnitSystem) -> f64 {
        use Unit::*;
        use UnitSystem::*;

        match to {
            USCS => match *self {
                None => value.into(),
                Distance(from) => match from {
                    SI => km_to_mi(value),
                    USCS => value.into(),
                },
                Pressure(from) => match from {
                    SI => bar_to_psi(value),
                    USCS => value.into(),
                },
                Speed(from) => match from {
                    SI => kph_to_mph(value),
                    USCS => value.into(),
                },
                Temperature(from) => match from {
                    SI => degc_to_degf(value),
                    USCS => value.into(),
                },
                Flow(from) => match from {
                    SI => lmin_to_galmin(value),
                    USCS => value.into(),
                },
                Volume(from) => match from {
                    SI => l_to_gal(value),
                    USCS => value.into(),
                },
            },
            SI => match *self {
                None => value.into(),
                Distance(from) => match from {
                    USCS => mi_to_km(value),
                    SI => value.into(),
                },
                Pressure(from) => match from {
                    USCS => psi_to_bar(value),
                    SI => value.into(),
                },
                Speed(from) => match from {
                    USCS => mph_to_kph(value),
                    SI => value.into(),
                },
                Temperature(from) => match from {
                    USCS => degf_to_degc(value),
                    SI => value.into(),
                },
                Flow(from) => match from {
                    USCS => galmin_to_lmin(value),
                    SI => value.into(),
                },
                Volume(from) => match from {
                    USCS => gal_to_l(value),
                    SI => value.into(),
                },
            },
        }
    }

    pub fn convert_system_to(&self, to: UnitSystem) -> Unit {
        use Unit::*;

        match *self {
            None => Self::None,
            Distance(_) => Self::Distance(to),
            Pressure(_) => Self::Pressure(to),
            Speed(_) => Self::Speed(to),
            Temperature(_) => Self::Temperature(to),
            Flow(_) => Self::Flow(to),
            Volume(_) => Self::Volume(to),
        }
    }

    pub fn get_short_str(&self) -> &str {
        use Unit::*;
        use UnitSystem::*;

        match *self {
            None => "NONE",
            Distance(unit_system) => match unit_system {
                USCS => "MI",
                SI => "KM",
            },
            Pressure(unit_system) => match unit_system {
                USCS => "PSI",
                SI => "BAR",
            },
            Speed(unit_system) => match unit_system {
                USCS => "MPH",
                SI => "KPH",
            },
            Temperature(unit_system) => match unit_system {
                USCS => "°F",
                SI => "°C",
            },
            Flow(unit_system) => match unit_system {
                USCS => "GAL/MIN",
                SI => "L/MIN",
            },
            Volume(unit_system) => match unit_system {
                USCS => "GAL",
                SI => "L",
            },
        }
    }
}

impl ToString for UnitSystem {
    fn to_string(&self) -> String {
        match self {
            UnitSystem::SI => String::from("SI"),
            UnitSystem::USCS => String::from("USCS"),
        }
    }
}

impl ToString for PressureUnit {
    fn to_string(&self) -> String {
        match self {
            Self::PSI => String::from("PSI"),
            Self::KPA => String::from("KPA"),
            Self::BAR => String::from("BAR"),
        }
    }
}

#[inline]
pub fn km_to_mi(km: impl Into<f64>) -> f64 {
    km.into() * 0.621371
}
#[inline]
pub fn mi_to_km(mi: impl Into<f64>) -> f64 {
    mi.into() / 0.621371
}
#[inline]
pub fn degc_to_degf(c: impl Into<f64>) -> f64 {
    c.into() * 9.0 / 5.0 + 32.0
}
#[inline]
pub fn degf_to_degc(f: impl Into<f64>) -> f64 {
    (f.into() - 32.0) * 5.0 / 9.0
}
#[inline]
pub fn kg_to_lb(kg: impl Into<f64>) -> f64 {
    kg.into() * 2.20462
}
#[inline]
pub fn lb_to_kg(lb: impl Into<f64>) -> f64 {
    lb.into() / 2.20462
}
#[inline]
pub fn l_to_gal(l: impl Into<f64>) -> f64 {
    l.into() * 0.264172
}
#[inline]
pub fn gal_to_l(gal: impl Into<f64>) -> f64 {
    gal.into() / 0.264172
}
#[inline]
pub fn mph_to_kph(mph: impl Into<f64>) -> f64 {
    mph.into() * 1.60934
}
#[inline]
pub fn kph_to_mph(kph: impl Into<f64>) -> f64 {
    kph.into() / 1.60934
}
#[inline]
pub fn psi_to_kpa(psi: impl Into<f64>) -> f64 {
    psi.into() * 6.89476
}
#[inline]
pub fn kpa_to_psi(kpa: impl Into<f64>) -> f64 {
    kpa.into() / 6.89476
}
#[inline]
pub fn psi_to_bar(psi: impl Into<f64>) -> f64 {
    psi.into() * 0.0689476
}
#[inline]
pub fn bar_to_psi(bar: impl Into<f64>) -> f64 {
    bar.into() / 0.0689476
}
#[inline]
pub fn kpa_to_bar(kpa: impl Into<f64>) -> f64 {
    kpa.into() * 0.01
}
#[inline]
pub fn bar_to_kpa(bar: impl Into<f64>) -> f64 {
    bar.into() / 0.01
}
#[inline]
pub fn lmin_to_galmin(lmin: impl Into<f64>) -> f64 {
    lmin.into() * 0.264172
}
#[inline]
pub fn galmin_to_lmin(galmin: impl Into<f64>) -> f64 {
    galmin.into() / 0.264172
}
