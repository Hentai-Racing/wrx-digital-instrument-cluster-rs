#![allow(dead_code, unused)]

#[derive(Debug, Clone, Copy)]
pub enum UnitSystem {
    USCS, // US Customary System
    SI,   // International System of Units
}

impl Default for UnitSystem {
    fn default() -> Self {
        UnitSystem::SI
    }
}

impl ToString for UnitSystem {
    fn to_string(&self) -> String {
        match self {
            UnitSystem::SI => "SI".to_string(),
            UnitSystem::USCS => "USCS".to_string(),
        }
    }
}

#[derive(Debug)]

pub enum Pressure {
    PSI,
    KPA,
    BAR,
}

impl Default for Pressure {
    fn default() -> Self {
        Pressure::KPA
    }
}

impl ToString for Pressure {
    fn to_string(&self) -> String {
        match self {
            Pressure::PSI => "PSI".to_string(),
            Pressure::KPA => "KPA".to_string(),
            Pressure::BAR => "BAR".to_string(),
        }
    }
}

pub fn km_to_mi(km: f32) -> f32 {
    km * 0.621371
}
pub fn mi_to_km(mi: f32) -> f32 {
    mi / 0.621371
}
pub fn degc_to_degf(c: f32) -> f32 {
    c * 9.0 / 5.0 + 32.0
}
pub fn degf_to_degc(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}
pub fn kg_to_lb(kg: f32) -> f32 {
    kg * 2.20462
}
pub fn lb_to_kg(lb: f32) -> f32 {
    lb / 2.20462
}
pub fn l_to_gal(l: f32) -> f32 {
    l * 0.264172
}
pub fn gal_to_l(gal: f32) -> f32 {
    gal / 0.264172
}
pub fn mph_to_kph(mph: f32) -> f32 {
    mph * 1.60934
}
pub fn kph_to_mph(kph: f32) -> f32 {
    kph / 1.60934
}
pub fn psi_to_kpa(psi: f32) -> f32 {
    psi * 6.89476
}
pub fn kpa_to_psi(kpa: f32) -> f32 {
    kpa / 6.89476
}
pub fn psi_to_bar(psi: f32) -> f32 {
    psi * 0.0689476
}
pub fn bar_to_psi(bar: f32) -> f32 {
    bar / 0.0689476
}
pub fn kpa_to_bar(kpa: f32) -> f32 {
    kpa * 0.01
}
pub fn bar_to_kpa(bar: f32) -> f32 {
    bar / 0.01
}
pub fn lbmin_to_galmin(lbmin: f32) -> f32 {
    lbmin * 0.119826
}
pub fn galmin_to_lbmin(galmin: f32) -> f32 {
    galmin / 0.119826
}

pub trait UnitConversion<T>
where
    T: Into<f32>,
{
    fn input_units(&self) -> UnitSystem;
    fn output_units(&self) -> UnitSystem;
    fn input_value(&self) -> T;

    fn distance(&self) -> f32 {
        let input = self.input_value().into();

        match self.input_units() {
            UnitSystem::USCS => match self.output_units() {
                UnitSystem::USCS => input,
                UnitSystem::SI => mi_to_km(input),
            },
            UnitSystem::SI => match self.output_units() {
                UnitSystem::USCS => km_to_mi(input),
                UnitSystem::SI => input,
            },
        }
    }

    fn distance_special(&self, input_units: UnitSystem) -> f32 {
        let input = self.input_value().into();

        match input_units {
            UnitSystem::USCS => match self.output_units() {
                UnitSystem::USCS => input,
                UnitSystem::SI => mi_to_km(input),
            },
            UnitSystem::SI => match self.output_units() {
                UnitSystem::USCS => km_to_mi(input),
                UnitSystem::SI => input,
            },
        }
    }

    fn speed(&self) -> f32 {
        let input = self.input_value().into();

        match self.input_units() {
            UnitSystem::USCS => match self.output_units() {
                UnitSystem::USCS => input,
                UnitSystem::SI => mph_to_kph(input),
            },
            UnitSystem::SI => match self.output_units() {
                UnitSystem::USCS => kph_to_mph(input),
                UnitSystem::SI => input,
            },
        }
    }

    fn speed_special(&self, input_units: UnitSystem) -> f32 {
        let input = self.input_value().into();

        match input_units {
            UnitSystem::USCS => match self.output_units() {
                UnitSystem::USCS => input,
                UnitSystem::SI => mph_to_kph(input),
            },
            UnitSystem::SI => match self.output_units() {
                UnitSystem::USCS => kph_to_mph(input),
                UnitSystem::SI => input,
            },
        }
    }

    fn temperature(&self) -> f32 {
        let input = self.input_value().into();

        match self.input_units() {
            UnitSystem::USCS => match self.output_units() {
                UnitSystem::USCS => input,
                UnitSystem::SI => degf_to_degc(input),
            },
            UnitSystem::SI => match self.output_units() {
                UnitSystem::USCS => degc_to_degf(input),
                UnitSystem::SI => input,
            },
        }
    }

    fn temperature_special(&self, input_units: UnitSystem) -> f32 {
        let input = self.input_value().into();

        match input_units {
            UnitSystem::USCS => match self.output_units() {
                UnitSystem::USCS => input,
                UnitSystem::SI => degf_to_degc(input),
            },
            UnitSystem::SI => match self.output_units() {
                UnitSystem::USCS => degc_to_degf(input),
                UnitSystem::SI => input,
            },
        }
    }

    fn mass(&self) -> f32 {
        let input = self.input_value().into();

        match self.input_units() {
            UnitSystem::USCS => match self.output_units() {
                UnitSystem::USCS => input,
                UnitSystem::SI => lb_to_kg(input),
            },
            UnitSystem::SI => match self.output_units() {
                UnitSystem::USCS => kg_to_lb(input),
                UnitSystem::SI => input,
            },
        }
    }

    fn mass_special(&self, input_units: UnitSystem) -> f32 {
        let input = self.input_value().into();

        match input_units {
            UnitSystem::USCS => match self.output_units() {
                UnitSystem::USCS => input,
                UnitSystem::SI => lb_to_kg(input),
            },
            UnitSystem::SI => match self.output_units() {
                UnitSystem::USCS => kg_to_lb(input),
                UnitSystem::SI => input,
            },
        }
    }

    fn volume(&self) -> f32 {
        let input = self.input_value().into();

        match self.input_units() {
            UnitSystem::USCS => match self.output_units() {
                UnitSystem::USCS => input,
                UnitSystem::SI => gal_to_l(input),
            },
            UnitSystem::SI => match self.output_units() {
                UnitSystem::USCS => l_to_gal(input),
                UnitSystem::SI => input,
            },
        }
    }

    fn volume_special(&self, input_units: UnitSystem) -> f32 {
        let input = self.input_value().into();

        match input_units {
            UnitSystem::USCS => match self.output_units() {
                UnitSystem::USCS => input,
                UnitSystem::SI => gal_to_l(input),
            },
            UnitSystem::SI => match self.output_units() {
                UnitSystem::USCS => l_to_gal(input),
                UnitSystem::SI => input,
            },
        }
    }

    fn pressure(&self, input_units: Pressure) -> f32 {
        let input = self.input_value().into();

        match input_units {
            Pressure::PSI => match self.output_units() {
                UnitSystem::USCS => psi_to_kpa(input),
                UnitSystem::SI => psi_to_bar(input),
            },
            Pressure::KPA => match self.output_units() {
                UnitSystem::USCS => kpa_to_psi(input),
                UnitSystem::SI => kpa_to_bar(input),
            },
            Pressure::BAR => match self.output_units() {
                UnitSystem::USCS => bar_to_psi(input),
                UnitSystem::SI => bar_to_kpa(input),
            },
        }
    }

    fn mass_flow(&self) -> f32 {
        let input = self.input_value().into();

        match self.input_units() {
            UnitSystem::USCS => match self.output_units() {
                UnitSystem::USCS => input,
                UnitSystem::SI => lbmin_to_galmin(input),
            },
            UnitSystem::SI => match self.output_units() {
                UnitSystem::USCS => galmin_to_lbmin(input),
                UnitSystem::SI => input,
            },
        }
    }

    fn mass_flow_special(&self, input_units: UnitSystem) -> f32 {
        let input = self.input_value().into();

        match input_units {
            UnitSystem::USCS => match self.output_units() {
                UnitSystem::USCS => input,
                UnitSystem::SI => lbmin_to_galmin(input),
            },
            UnitSystem::SI => match self.output_units() {
                UnitSystem::USCS => galmin_to_lbmin(input),
                UnitSystem::SI => input,
            },
        }
    }

    fn distance_unit_string(&self) -> String {
        match self.output_units() {
            UnitSystem::USCS => "mi".to_string(),
            UnitSystem::SI => "km".to_string(),
        }
    }

    fn speed_unit_string(&self) -> String {
        match self.output_units() {
            UnitSystem::USCS => "mph".to_string(),
            UnitSystem::SI => "kph".to_string(),
        }
    }

    fn temperature_unit_string(&self) -> String {
        match self.output_units() {
            UnitSystem::USCS => "f".to_string(),
            UnitSystem::SI => "c".to_string(),
        }
    }

    fn mass_unit_string(&self) -> String {
        match self.output_units() {
            UnitSystem::USCS => "lb".to_string(),
            UnitSystem::SI => "kg".to_string(),
        }
    }

    fn volume_unit_string(&self) -> String {
        match self.output_units() {
            UnitSystem::USCS => "gal".to_string(),
            UnitSystem::SI => "l".to_string(),
        }
    }

    fn pressure_unit_string(&self, output_units: Pressure) -> String {
        output_units.to_string()
    }

    fn mass_flow_unit_string(&self) -> String {
        match self.output_units() {
            UnitSystem::USCS => "lb/min".to_string(),
            UnitSystem::SI => "gal/min".to_string(),
        }
    }
}
