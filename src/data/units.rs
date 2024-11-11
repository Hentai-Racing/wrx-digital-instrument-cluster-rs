#![allow(dead_code, unused)]

use std::default;

#[derive(Debug, Clone, Copy)]
pub enum UnitsSystem {
    USCS, // US Customary System
    SI,   // International System of Units
}

impl Default for UnitsSystem {
    fn default() -> Self {
        UnitsSystem::SI
    }
}

impl ToString for UnitsSystem {
    fn to_string(&self) -> String {
        match self {
            UnitsSystem::SI => "SI".to_string(),
            UnitsSystem::USCS => "USCS".to_string(),
        }
    }
}

#[derive(Debug, Default)]
pub enum PressureUnit {
    PSI,
    #[default]
    KPA,
    BAR,
}

#[derive(Clone, Copy, Default)]
pub enum Unit {
    #[default]
    Distance,
    Pressure,
    Speed,
    Temperature,
    Flow,
    Volume,
}

impl ToString for PressureUnit {
    fn to_string(&self) -> String {
        match self {
            Self::PSI => "PSI".to_string(),
            Self::KPA => "KPA".to_string(),
            Self::BAR => "BAR".to_string(),
        }
    }
}

#[inline]
pub fn km_to_mi(km: f32) -> f32 {
    km * 0.621371
}
#[inline]
pub fn mi_to_km(mi: f32) -> f32 {
    mi / 0.621371
}
#[inline]
pub fn degc_to_degf(c: f32) -> f32 {
    c * 9.0 / 5.0 + 32.0
}
#[inline]
pub fn degf_to_degc(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}
#[inline]
pub fn kg_to_lb(kg: f32) -> f32 {
    kg * 2.20462
}
#[inline]
pub fn lb_to_kg(lb: f32) -> f32 {
    lb / 2.20462
}
#[inline]
pub fn l_to_gal(l: f32) -> f32 {
    l * 0.264172
}
#[inline]
pub fn gal_to_l(gal: f32) -> f32 {
    gal / 0.264172
}
#[inline]
pub fn mph_to_kph(mph: f32) -> f32 {
    mph * 1.60934
}
#[inline]
pub fn kph_to_mph(kph: f32) -> f32 {
    kph / 1.60934
}
#[inline]
pub fn psi_to_kpa(psi: f32) -> f32 {
    psi * 6.89476
}
#[inline]
pub fn kpa_to_psi(kpa: f32) -> f32 {
    kpa / 6.89476
}
#[inline]
pub fn psi_to_bar(psi: f32) -> f32 {
    psi * 0.0689476
}
#[inline]
pub fn bar_to_psi(bar: f32) -> f32 {
    bar / 0.0689476
}
#[inline]
pub fn kpa_to_bar(kpa: f32) -> f32 {
    kpa * 0.01
}
#[inline]
pub fn bar_to_kpa(bar: f32) -> f32 {
    bar / 0.01
}
#[inline]
pub fn lbmin_to_galmin(lbmin: f32) -> f32 {
    lbmin * 0.119826
}
#[inline]
pub fn galmin_to_lbmin(galmin: f32) -> f32 {
    galmin / 0.119826
}

pub trait UnitConversion<T>
where
    T: Into<f32>,
{
    fn input_units(&self) -> UnitsSystem;
    fn output_units(&self) -> UnitsSystem;
    fn input_value(&self) -> T;

    fn distance(&self) -> f32 {
        let input = self.input_value().into();

        match self.input_units() {
            UnitsSystem::USCS => match self.output_units() {
                UnitsSystem::USCS => input,
                UnitsSystem::SI => mi_to_km(input),
            },
            UnitsSystem::SI => match self.output_units() {
                UnitsSystem::USCS => km_to_mi(input),
                UnitsSystem::SI => input,
            },
        }
    }

    fn distance_special(&self, input_units: UnitsSystem) -> f32 {
        let input = self.input_value().into();

        match input_units {
            UnitsSystem::USCS => match self.output_units() {
                UnitsSystem::USCS => input,
                UnitsSystem::SI => mi_to_km(input),
            },
            UnitsSystem::SI => match self.output_units() {
                UnitsSystem::USCS => km_to_mi(input),
                UnitsSystem::SI => input,
            },
        }
    }

    fn speed(&self) -> f32 {
        let input = self.input_value().into();

        match self.input_units() {
            UnitsSystem::USCS => match self.output_units() {
                UnitsSystem::USCS => input,
                UnitsSystem::SI => mph_to_kph(input),
            },
            UnitsSystem::SI => match self.output_units() {
                UnitsSystem::USCS => kph_to_mph(input),
                UnitsSystem::SI => input,
            },
        }
    }

    fn speed_special(&self, input_units: UnitsSystem) -> f32 {
        let input = self.input_value().into();

        match input_units {
            UnitsSystem::USCS => match self.output_units() {
                UnitsSystem::USCS => input,
                UnitsSystem::SI => mph_to_kph(input),
            },
            UnitsSystem::SI => match self.output_units() {
                UnitsSystem::USCS => kph_to_mph(input),
                UnitsSystem::SI => input,
            },
        }
    }

    fn temperature(&self) -> f32 {
        let input = self.input_value().into();

        match self.input_units() {
            UnitsSystem::USCS => match self.output_units() {
                UnitsSystem::USCS => input,
                UnitsSystem::SI => degf_to_degc(input),
            },
            UnitsSystem::SI => match self.output_units() {
                UnitsSystem::USCS => degc_to_degf(input),
                UnitsSystem::SI => input,
            },
        }
    }

    fn temperature_special(&self, input_units: UnitsSystem) -> f32 {
        let input = self.input_value().into();

        match input_units {
            UnitsSystem::USCS => match self.output_units() {
                UnitsSystem::USCS => input,
                UnitsSystem::SI => degf_to_degc(input),
            },
            UnitsSystem::SI => match self.output_units() {
                UnitsSystem::USCS => degc_to_degf(input),
                UnitsSystem::SI => input,
            },
        }
    }

    fn mass(&self) -> f32 {
        let input = self.input_value().into();

        match self.input_units() {
            UnitsSystem::USCS => match self.output_units() {
                UnitsSystem::USCS => input,
                UnitsSystem::SI => lb_to_kg(input),
            },
            UnitsSystem::SI => match self.output_units() {
                UnitsSystem::USCS => kg_to_lb(input),
                UnitsSystem::SI => input,
            },
        }
    }

    fn mass_special(&self, input_units: UnitsSystem) -> f32 {
        let input = self.input_value().into();

        match input_units {
            UnitsSystem::USCS => match self.output_units() {
                UnitsSystem::USCS => input,
                UnitsSystem::SI => lb_to_kg(input),
            },
            UnitsSystem::SI => match self.output_units() {
                UnitsSystem::USCS => kg_to_lb(input),
                UnitsSystem::SI => input,
            },
        }
    }

    fn volume(&self) -> f32 {
        let input = self.input_value().into();

        match self.input_units() {
            UnitsSystem::USCS => match self.output_units() {
                UnitsSystem::USCS => input,
                UnitsSystem::SI => gal_to_l(input),
            },
            UnitsSystem::SI => match self.output_units() {
                UnitsSystem::USCS => l_to_gal(input),
                UnitsSystem::SI => input,
            },
        }
    }

    fn volume_special(&self, input_units: UnitsSystem) -> f32 {
        let input = self.input_value().into();

        match input_units {
            UnitsSystem::USCS => match self.output_units() {
                UnitsSystem::USCS => input,
                UnitsSystem::SI => gal_to_l(input),
            },
            UnitsSystem::SI => match self.output_units() {
                UnitsSystem::USCS => l_to_gal(input),
                UnitsSystem::SI => input,
            },
        }
    }

    fn pressure(&self, input_units: PressureUnit) -> f32 {
        let input = self.input_value().into();

        match input_units {
            PressureUnit::PSI => match self.output_units() {
                UnitsSystem::USCS => psi_to_kpa(input),
                UnitsSystem::SI => psi_to_bar(input),
            },
            PressureUnit::KPA => match self.output_units() {
                UnitsSystem::USCS => kpa_to_psi(input),
                UnitsSystem::SI => kpa_to_bar(input),
            },
            PressureUnit::BAR => match self.output_units() {
                UnitsSystem::USCS => bar_to_psi(input),
                UnitsSystem::SI => bar_to_kpa(input),
            },
        }
    }

    fn mass_flow(&self) -> f32 {
        let input = self.input_value().into();

        match self.input_units() {
            UnitsSystem::USCS => match self.output_units() {
                UnitsSystem::USCS => input,
                UnitsSystem::SI => lbmin_to_galmin(input),
            },
            UnitsSystem::SI => match self.output_units() {
                UnitsSystem::USCS => galmin_to_lbmin(input),
                UnitsSystem::SI => input,
            },
        }
    }

    fn mass_flow_special(&self, input_units: UnitsSystem) -> f32 {
        let input = self.input_value().into();

        match input_units {
            UnitsSystem::USCS => match self.output_units() {
                UnitsSystem::USCS => input,
                UnitsSystem::SI => lbmin_to_galmin(input),
            },
            UnitsSystem::SI => match self.output_units() {
                UnitsSystem::USCS => galmin_to_lbmin(input),
                UnitsSystem::SI => input,
            },
        }
    }

    fn distance_unit_string(&self) -> String {
        match self.output_units() {
            UnitsSystem::USCS => "mi".to_string(),
            UnitsSystem::SI => "km".to_string(),
        }
    }

    fn speed_unit_string(&self) -> String {
        match self.output_units() {
            UnitsSystem::USCS => "mph".to_string(),
            UnitsSystem::SI => "kph".to_string(),
        }
    }

    fn temperature_unit_string(&self) -> String {
        match self.output_units() {
            UnitsSystem::USCS => "f".to_string(),
            UnitsSystem::SI => "c".to_string(),
        }
    }

    fn mass_unit_string(&self) -> String {
        match self.output_units() {
            UnitsSystem::USCS => "lb".to_string(),
            UnitsSystem::SI => "kg".to_string(),
        }
    }

    fn volume_unit_string(&self) -> String {
        match self.output_units() {
            UnitsSystem::USCS => "gal".to_string(),
            UnitsSystem::SI => "l".to_string(),
        }
    }

    fn pressure_unit_string(&self, output_units: PressureUnit) -> String {
        output_units.to_string()
    }

    fn mass_flow_unit_string(&self) -> String {
        match self.output_units() {
            UnitsSystem::USCS => "lb/min".to_string(),
            UnitsSystem::SI => "gal/min".to_string(),
        }
    }
}
