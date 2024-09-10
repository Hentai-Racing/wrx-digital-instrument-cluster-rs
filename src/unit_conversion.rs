#![allow(dead_code, unused)]

#[derive(Debug)]
pub enum Units {
    USCS, // US Customary System
    SI,   // International System of Units
}

#[derive(Debug)]

pub enum PressureUnits {
    PSI,
    KPA,
    BAR,
}

pub struct UnitHandler {
    pub output_units: Units,
    pub input_units: Units,
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

impl UnitHandler {
    pub fn new(input_units: Units, output_units: Units) -> UnitHandler {
        UnitHandler {
            input_units,
            output_units,
        }
    }

    pub fn distance(&self, input: f32) -> f32 {
        match self.input_units {
            Units::USCS => match self.output_units {
                Units::USCS => input,
                Units::SI => mi_to_km(input),
            },
            Units::SI => match self.output_units {
                Units::USCS => km_to_mi(input),
                Units::SI => input,
            },
        }
    }

    pub fn distance_special(&self, input: f32, input_units: Units) -> f32 {
        match input_units {
            Units::USCS => match self.output_units {
                Units::USCS => input,
                Units::SI => mi_to_km(input),
            },
            Units::SI => match self.output_units {
                Units::USCS => km_to_mi(input),
                Units::SI => input,
            },
        }
    }

    pub fn speed(&self, input: f32) -> f32 {
        match self.input_units {
            Units::USCS => match self.output_units {
                Units::USCS => input,
                Units::SI => mph_to_kph(input),
            },
            Units::SI => match self.output_units {
                Units::USCS => kph_to_mph(input),
                Units::SI => input,
            },
        }
    }

    pub fn speed_special(&self, input: f32, input_units: Units) -> f32 {
        match input_units {
            Units::USCS => match self.output_units {
                Units::USCS => input,
                Units::SI => mph_to_kph(input),
            },
            Units::SI => match self.output_units {
                Units::USCS => kph_to_mph(input),
                Units::SI => input,
            },
        }
    }

    pub fn temperature(&self, input: f32) -> f32 {
        match self.input_units {
            Units::USCS => match self.output_units {
                Units::USCS => input,
                Units::SI => degf_to_degc(input),
            },
            Units::SI => match self.output_units {
                Units::USCS => degc_to_degf(input),
                Units::SI => input,
            },
        }
    }

    pub fn temperature_special(&self, input: f32, input_units: Units) -> f32 {
        match input_units {
            Units::USCS => match self.output_units {
                Units::USCS => input,
                Units::SI => degf_to_degc(input),
            },
            Units::SI => match self.output_units {
                Units::USCS => degc_to_degf(input),
                Units::SI => input,
            },
        }
    }

    pub fn mass(&self, input: f32) -> f32 {
        match self.input_units {
            Units::USCS => match self.output_units {
                Units::USCS => input,
                Units::SI => lb_to_kg(input),
            },
            Units::SI => match self.output_units {
                Units::USCS => kg_to_lb(input),
                Units::SI => input,
            },
        }
    }

    pub fn mass_special(&self, input: f32, input_units: Units) -> f32 {
        match input_units {
            Units::USCS => match self.output_units {
                Units::USCS => input,
                Units::SI => lb_to_kg(input),
            },
            Units::SI => match self.output_units {
                Units::USCS => kg_to_lb(input),
                Units::SI => input,
            },
        }
    }

    pub fn volume(&self, input: f32) -> f32 {
        match self.input_units {
            Units::USCS => match self.output_units {
                Units::USCS => input,
                Units::SI => gal_to_l(input),
            },
            Units::SI => match self.output_units {
                Units::USCS => l_to_gal(input),
                Units::SI => input,
            },
        }
    }

    pub fn volume_special(&self, input: f32, input_units: Units) -> f32 {
        match input_units {
            Units::USCS => match self.output_units {
                Units::USCS => input,
                Units::SI => gal_to_l(input),
            },
            Units::SI => match self.output_units {
                Units::USCS => l_to_gal(input),
                Units::SI => input,
            },
        }
    }

    pub fn pressure(&self, input: f32, input_units: PressureUnits) -> f32 {
        match input_units {
            PressureUnits::PSI => match self.output_units {
                Units::USCS => psi_to_kpa(input),
                Units::SI => psi_to_bar(input),
            },
            PressureUnits::KPA => match self.output_units {
                Units::USCS => kpa_to_psi(input),
                Units::SI => kpa_to_bar(input),
            },
            PressureUnits::BAR => match self.output_units {
                Units::USCS => bar_to_psi(input),
                Units::SI => bar_to_kpa(input),
            },
        }
    }

    pub fn mass_flow(&self, input: f32) -> f32 {
        match self.input_units {
            Units::USCS => match self.output_units {
                Units::USCS => input,
                Units::SI => lbmin_to_galmin(input),
            },
            Units::SI => match self.output_units {
                Units::USCS => galmin_to_lbmin(input),
                Units::SI => input,
            },
        }
    }

    pub fn mass_flow_special(&self, input: f32, input_units: Units) -> f32 {
        match input_units {
            Units::USCS => match self.output_units {
                Units::USCS => input,
                Units::SI => lbmin_to_galmin(input),
            },
            Units::SI => match self.output_units {
                Units::USCS => galmin_to_lbmin(input),
                Units::SI => input,
            },
        }
    }

    pub fn distance_unit_string(&self) -> String {
        match self.output_units {
            Units::USCS => "mi".to_string(),
            Units::SI => "km".to_string(),
        }
    }

    pub fn speed_unit_string(&self) -> String {
        match self.output_units {
            Units::USCS => "mph".to_string(),
            Units::SI => "kph".to_string(),
        }
    }

    pub fn temperature_unit_string(&self) -> String {
        match self.output_units {
            Units::USCS => "f".to_string(),
            Units::SI => "c".to_string(),
        }
    }

    pub fn mass_unit_string(&self) -> String {
        match self.output_units {
            Units::USCS => "lb".to_string(),
            Units::SI => "kg".to_string(),
        }
    }

    pub fn volume_unit_string(&self) -> String {
        match self.output_units {
            Units::USCS => "gal".to_string(),
            Units::SI => "l".to_string(),
        }
    }

    pub fn pressure_unit_string(&self, output_units: PressureUnits) -> String {
        match output_units {
            PressureUnits::PSI => "psi".to_string(),
            PressureUnits::KPA => "kpa".to_string(),
            PressureUnits::BAR => "bar".to_string(),
        }
    }

    pub fn mass_flow_unit_string(&self) -> String {
        match self.output_units {
            Units::USCS => "lb/min".to_string(),
            Units::SI => "gal/min".to_string(),
        }
    }
}
