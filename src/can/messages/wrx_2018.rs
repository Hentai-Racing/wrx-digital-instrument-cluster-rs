// Generated code!
#![allow(unused_comparisons, unreachable_patterns, unused_imports)]
#![allow(dead_code)]
#![allow(clippy::let_and_return, clippy::eq_op)]
#![allow(clippy::useless_conversion, clippy::unnecessary_cast)]
#![allow(clippy::excessive_precision, clippy::manual_range_contains, clippy::absurd_extreme_comparisons, clippy::too_many_arguments)]
#![deny(clippy::arithmetic_side_effects)]

//! Message definitions from file `"WRX_2018.dbc"`
//!
//! - Version: `Version("0.4.3")`

use core::ops::BitOr;
use bitvec::prelude::*;
use embedded_can::{Id, StandardId, ExtendedId};

/// All messages
#[derive(Clone)]
pub enum Messages {
    /// g_sensor
    GSensor(GSensor),
    /// XXXMsg209
    XxxMsg209(XxxMsg209),
    /// driver_road_assists
    DriverRoadAssists(DriverRoadAssists),
    /// wheel_speeds
    WheelSpeeds(WheelSpeeds),
    /// steering
    Steering(Steering),
    /// motor_control
    MotorControl(MotorControl),
    /// engine_status
    EngineStatus(EngineStatus),
    /// transmission
    Transmission(Transmission),
    /// status_switches
    StatusSwitches(StatusSwitches),
    /// XXXMsg340
    XxxMsg340(XxxMsg340),
    /// bsd_rcta
    BsdRcta(BsdRcta),
    /// XXXMsg640
    XxxMsg640(XxxMsg640),
    /// ignition
    Ignition(Ignition),
    /// engine_status_2
    EngineStatus2(EngineStatus2),
    /// engine_warning_lights
    EngineWarningLights(EngineWarningLights),
    /// srs_status
    SrsStatus(SrsStatus),
    /// XXXMsg884
    XxxMsg884(XxxMsg884),
    /// XXXMsg885
    XxxMsg885(XxxMsg885),
    /// DimmerAndHood
    DimmerAndHood(DimmerAndHood),
    /// Dash_State2_VERIFY
    DashState2Verify(DashState2Verify),
    /// odometer
    Odometer(Odometer),
    /// tpms
    Tpms(Tpms),
}

impl Messages {
    /// Read message from CAN frame
    #[inline(never)]
    pub fn from_can_message(id: Id, payload: &[u8]) -> Result<Self, CanError> {
        
        let res = match id {
            GSensor::MESSAGE_ID => Messages::GSensor(GSensor::try_from(payload)?),
            XxxMsg209::MESSAGE_ID => Messages::XxxMsg209(XxxMsg209::try_from(payload)?),
            DriverRoadAssists::MESSAGE_ID => Messages::DriverRoadAssists(DriverRoadAssists::try_from(payload)?),
            WheelSpeeds::MESSAGE_ID => Messages::WheelSpeeds(WheelSpeeds::try_from(payload)?),
            Steering::MESSAGE_ID => Messages::Steering(Steering::try_from(payload)?),
            MotorControl::MESSAGE_ID => Messages::MotorControl(MotorControl::try_from(payload)?),
            EngineStatus::MESSAGE_ID => Messages::EngineStatus(EngineStatus::try_from(payload)?),
            Transmission::MESSAGE_ID => Messages::Transmission(Transmission::try_from(payload)?),
            StatusSwitches::MESSAGE_ID => Messages::StatusSwitches(StatusSwitches::try_from(payload)?),
            XxxMsg340::MESSAGE_ID => Messages::XxxMsg340(XxxMsg340::try_from(payload)?),
            BsdRcta::MESSAGE_ID => Messages::BsdRcta(BsdRcta::try_from(payload)?),
            XxxMsg640::MESSAGE_ID => Messages::XxxMsg640(XxxMsg640::try_from(payload)?),
            Ignition::MESSAGE_ID => Messages::Ignition(Ignition::try_from(payload)?),
            EngineStatus2::MESSAGE_ID => Messages::EngineStatus2(EngineStatus2::try_from(payload)?),
            EngineWarningLights::MESSAGE_ID => Messages::EngineWarningLights(EngineWarningLights::try_from(payload)?),
            SrsStatus::MESSAGE_ID => Messages::SrsStatus(SrsStatus::try_from(payload)?),
            XxxMsg884::MESSAGE_ID => Messages::XxxMsg884(XxxMsg884::try_from(payload)?),
            XxxMsg885::MESSAGE_ID => Messages::XxxMsg885(XxxMsg885::try_from(payload)?),
            DimmerAndHood::MESSAGE_ID => Messages::DimmerAndHood(DimmerAndHood::try_from(payload)?),
            DashState2Verify::MESSAGE_ID => Messages::DashState2Verify(DashState2Verify::try_from(payload)?),
            Odometer::MESSAGE_ID => Messages::Odometer(Odometer::try_from(payload)?),
            Tpms::MESSAGE_ID => Messages::Tpms(Tpms::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// g_sensor
///
/// - Standard ID: 208 (0xd0)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct GSensor {
    raw: [u8; 8],
}

impl GSensor {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0xd0)});
    
    pub const G_SENSOR_LATERAL_MIN: f32 = -255_f32;
    pub const G_SENSOR_LATERAL_MAX: f32 = 255_f32;
    pub const G_SENSOR_LONGITUDINAL_MIN: f32 = -255_f32;
    pub const G_SENSOR_LONGITUDINAL_MAX: f32 = 255_f32;
    pub const STEERING_ANGLE_MIN: f32 = 0_f32;
    pub const STEERING_ANGLE_MAX: f32 = 1_f32;
    
    /// Construct new g_sensor from values
    pub fn new(g_sensor_lateral: f32, g_sensor_longitudinal: f32, steering_angle: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_g_sensor_lateral(g_sensor_lateral)?;
        res.set_g_sensor_longitudinal(g_sensor_longitudinal)?;
        res.set_steering_angle(steering_angle)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// g_sensor_lateral
    ///
    /// - Min: -255
    /// - Max: 255
    /// - Unit: "deg"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn g_sensor_lateral(&self) -> f32 {
        self.g_sensor_lateral_raw()
    }
    
    /// Get raw value of g_sensor_lateral
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: -0.0035
    /// - Offset: 1
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn g_sensor_lateral_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<i16>();
        
        let factor = -0.0035_f32;
        let offset = 1_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of g_sensor_lateral
    #[inline(always)]
    pub fn set_g_sensor_lateral(&mut self, value: f32) -> Result<(), CanError> {
        if value < -255_f32 || 255_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: GSensor::MESSAGE_ID });
        }
        let factor = -0.0035_f32;
        let offset = 1_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// g_sensor_longitudinal
    ///
    /// - Min: -255
    /// - Max: 255
    /// - Unit: "deg"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn g_sensor_longitudinal(&self) -> f32 {
        self.g_sensor_longitudinal_raw()
    }
    
    /// Get raw value of g_sensor_longitudinal
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: -0.0035
    /// - Offset: 1
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn g_sensor_longitudinal_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<i16>();
        
        let factor = -0.0035_f32;
        let offset = 1_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of g_sensor_longitudinal
    #[inline(always)]
    pub fn set_g_sensor_longitudinal(&mut self, value: f32) -> Result<(), CanError> {
        if value < -255_f32 || 255_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: GSensor::MESSAGE_ID });
        }
        let factor = -0.0035_f32;
        let offset = 1_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
    /// steering_angle
    ///
    /// might_be_actual_wheel_angle_not_steering_wheel
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "deg"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn steering_angle(&self) -> f32 {
        self.steering_angle_raw()
    }
    
    /// Get raw value of steering_angle
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn steering_angle_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<i16>();
        
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of steering_angle
    #[inline(always)]
    pub fn set_steering_angle(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 1_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: GSensor::MESSAGE_ID });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for GSensor {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for GSensor {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// XXXMsg209
///
/// - Standard ID: 209 (0xd1)
/// - Size: 4 bytes
#[derive(Clone, Copy)]
pub struct XxxMsg209 {
    raw: [u8; 4],
}

impl XxxMsg209 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0xd1)});
    
    pub const BRAKE_PEDAL_PRESSURE_MIN: f32 = 0_f32;
    pub const BRAKE_PEDAL_PRESSURE_MAX: f32 = 100_f32;
    pub const VEHICLE_SPEED_MIN: f32 = 0_f32;
    pub const VEHICLE_SPEED_MAX: f32 = 290_f32;
    
    /// Construct new XXXMsg209 from values
    pub fn new(brake_pedal_pressure: f32, vehicle_speed: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 4] };
        res.set_brake_pedal_pressure(brake_pedal_pressure)?;
        res.set_vehicle_speed(vehicle_speed)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 4] {
        &self.raw
    }
    
    /// brake_pedal_pressure
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "%"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brake_pedal_pressure(&self) -> f32 {
        self.brake_pedal_pressure_raw()
    }
    
    /// Get raw value of brake_pedal_pressure
    ///
    /// - Start bit: 16
    /// - Signal size: 8 bits
    /// - Factor: 0.78125
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_pedal_pressure_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<u8>();
        
        let factor = 0.78125_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of brake_pedal_pressure
    #[inline(always)]
    pub fn set_brake_pedal_pressure(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 100_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: XxxMsg209::MESSAGE_ID });
        }
        let factor = 0.78125_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
        Ok(())
    }
    
    /// vehicle_speed
    ///
    /// - Min: 0
    /// - Max: 290
    /// - Unit: "KPH"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn vehicle_speed(&self) -> f32 {
        self.vehicle_speed_raw()
    }
    
    /// Get raw value of vehicle_speed
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.05625
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn vehicle_speed_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.05625_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of vehicle_speed
    #[inline(always)]
    pub fn set_vehicle_speed(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 290_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: XxxMsg209::MESSAGE_ID });
        }
        let factor = 0.05625_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for XxxMsg209 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 4 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 4];
        raw.copy_from_slice(&payload[..4]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for XxxMsg209 {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// driver_road_assists
///
/// - Standard ID: 211 (0xd3)
/// - Size: 7 bytes
#[derive(Clone, Copy)]
pub struct DriverRoadAssists {
    raw: [u8; 7],
}

impl DriverRoadAssists {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0xd3)});
    
    
    /// Construct new driver_road_assists from values
    pub fn new(hill_assist_enabled: bool, active_tq_vectoring_enabled: bool, traction_control_disabled: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 7] };
        res.set_hill_assist_enabled(hill_assist_enabled)?;
        res.set_active_tq_vectoring_enabled(active_tq_vectoring_enabled)?;
        res.set_traction_control_disabled(traction_control_disabled)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 7] {
        &self.raw
    }
    
    /// hill_assist_enabled
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn hill_assist_enabled(&self) -> bool {
        self.hill_assist_enabled_raw()
    }
    
    /// Get raw value of hill_assist_enabled
    ///
    /// - Start bit: 15
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn hill_assist_enabled_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[8..9].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of hill_assist_enabled
    #[inline(always)]
    pub fn set_hill_assist_enabled(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[8..9].store_be(value);
        Ok(())
    }
    
    /// active_tq_vectoring_enabled
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn active_tq_vectoring_enabled(&self) -> bool {
        self.active_tq_vectoring_enabled_raw()
    }
    
    /// Get raw value of active_tq_vectoring_enabled
    ///
    /// - Start bit: 3
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn active_tq_vectoring_enabled_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[4..5].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of active_tq_vectoring_enabled
    #[inline(always)]
    pub fn set_active_tq_vectoring_enabled(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[4..5].store_be(value);
        Ok(())
    }
    
    /// traction_control_disabled
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn traction_control_disabled(&self) -> bool {
        self.traction_control_disabled_raw()
    }
    
    /// Get raw value of traction_control_disabled
    ///
    /// - Start bit: 11
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn traction_control_disabled_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[12..13].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of traction_control_disabled
    #[inline(always)]
    pub fn set_traction_control_disabled(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[12..13].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for DriverRoadAssists {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 7 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 7];
        raw.copy_from_slice(&payload[..7]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for DriverRoadAssists {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// wheel_speeds
///
/// - Standard ID: 212 (0xd4)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct WheelSpeeds {
    raw: [u8; 8],
}

impl WheelSpeeds {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0xd4)});
    
    pub const LEFT_FRONT_WHEEL_SPEED_MIN: f32 = 0_f32;
    pub const LEFT_FRONT_WHEEL_SPEED_MAX: f32 = 255_f32;
    pub const LEFT_REAR_WHEEL_SPEED_MIN: f32 = 0_f32;
    pub const LEFT_REAR_WHEEL_SPEED_MAX: f32 = 255_f32;
    pub const RIGHT_FRONT_WHEEL_SPEED_MIN: f32 = 0_f32;
    pub const RIGHT_FRONT_WHEEL_SPEED_MAX: f32 = 255_f32;
    pub const RIGHT_REAR_WHEEL_SPEED_MIN: f32 = 0_f32;
    pub const RIGHT_REAR_WHEEL_SPEED_MAX: f32 = 255_f32;
    
    /// Construct new wheel_speeds from values
    pub fn new(left_front_wheel_speed: f32, left_rear_wheel_speed: f32, right_front_wheel_speed: f32, right_rear_wheel_speed: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_left_front_wheel_speed(left_front_wheel_speed)?;
        res.set_left_rear_wheel_speed(left_rear_wheel_speed)?;
        res.set_right_front_wheel_speed(right_front_wheel_speed)?;
        res.set_right_rear_wheel_speed(right_rear_wheel_speed)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// left_front_wheel_speed
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: "KPH"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn left_front_wheel_speed(&self) -> f32 {
        self.left_front_wheel_speed_raw()
    }
    
    /// Get raw value of left_front_wheel_speed
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.0592
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn left_front_wheel_speed_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.0592_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of left_front_wheel_speed
    #[inline(always)]
    pub fn set_left_front_wheel_speed(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 255_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: WheelSpeeds::MESSAGE_ID });
        }
        let factor = 0.0592_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// left_rear_wheel_speed
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: "KPH"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn left_rear_wheel_speed(&self) -> f32 {
        self.left_rear_wheel_speed_raw()
    }
    
    /// Get raw value of left_rear_wheel_speed
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.0592
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn left_rear_wheel_speed_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 0.0592_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of left_rear_wheel_speed
    #[inline(always)]
    pub fn set_left_rear_wheel_speed(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 255_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: WheelSpeeds::MESSAGE_ID });
        }
        let factor = 0.0592_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// right_front_wheel_speed
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: "KPH"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn right_front_wheel_speed(&self) -> f32 {
        self.right_front_wheel_speed_raw()
    }
    
    /// Get raw value of right_front_wheel_speed
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.0592
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn right_front_wheel_speed_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.0592_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of right_front_wheel_speed
    #[inline(always)]
    pub fn set_right_front_wheel_speed(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 255_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: WheelSpeeds::MESSAGE_ID });
        }
        let factor = 0.0592_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// right_rear_wheel_speed
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: "KPH"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn right_rear_wheel_speed(&self) -> f32 {
        self.right_rear_wheel_speed_raw()
    }
    
    /// Get raw value of right_rear_wheel_speed
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.0592
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn right_rear_wheel_speed_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let factor = 0.0592_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of right_rear_wheel_speed
    #[inline(always)]
    pub fn set_right_rear_wheel_speed(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 255_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: WheelSpeeds::MESSAGE_ID });
        }
        let factor = 0.0592_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for WheelSpeeds {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for WheelSpeeds {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// steering
///
/// - Standard ID: 282 (0x11a)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Steering {
    raw: [u8; 8],
}

impl Steering {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x11a)});
    
    pub const STEERING_WHEEL_ANGLE_MIN: f32 = 0_f32;
    pub const STEERING_WHEEL_ANGLE_MAX: f32 = 0_f32;
    
    /// Construct new steering from values
    pub fn new(steering_wheel_angle: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_steering_wheel_angle(steering_wheel_angle)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// steering_wheel_angle
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "deg"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn steering_wheel_angle(&self) -> f32 {
        self.steering_wheel_angle_raw()
    }
    
    /// Get raw value of steering_wheel_angle
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.02
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn steering_wheel_angle_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<i16>();
        
        let factor = 0.02_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of steering_wheel_angle
    #[inline(always)]
    pub fn set_steering_wheel_angle(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Steering::MESSAGE_ID });
        }
        let factor = 0.02_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Steering {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Steering {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// motor_control
///
/// - Standard ID: 320 (0x140)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct MotorControl {
    raw: [u8; 8],
}

impl MotorControl {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x140)});
    
    pub const ACCELERATOR_PEDAL_POSITION_MIN: f32 = 0_f32;
    pub const ACCELERATOR_PEDAL_POSITION_MAX: f32 = 1_f32;
    pub const COMBINED_ACCELERATOR_MIN: f32 = 0_f32;
    pub const COMBINED_ACCELERATOR_MAX: f32 = 100_f32;
    pub const THROTTLE_PLATE_POSITION_MIN: f32 = 0_f32;
    pub const THROTTLE_PLATE_POSITION_MAX: f32 = 100_f32;
    
    /// Construct new motor_control from values
    pub fn new(accelerator_pedal_position: f32, clutch_sw: bool, combined_accelerator: f32, n_accelerator_pedal_max_sw: bool, throttle_plate_position: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_accelerator_pedal_position(accelerator_pedal_position)?;
        res.set_clutch_sw(clutch_sw)?;
        res.set_combined_accelerator(combined_accelerator)?;
        res.set_n_accelerator_pedal_max_sw(n_accelerator_pedal_max_sw)?;
        res.set_throttle_plate_position(throttle_plate_position)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// accelerator_pedal_position
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "%"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn accelerator_pedal_position(&self) -> f32 {
        self.accelerator_pedal_position_raw()
    }
    
    /// Get raw value of accelerator_pedal_position
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 0.392157
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn accelerator_pedal_position_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 0.392157_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of accelerator_pedal_position
    #[inline(always)]
    pub fn set_accelerator_pedal_position(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 1_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: MotorControl::MESSAGE_ID });
        }
        let factor = 0.392157_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// clutch_sw
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn clutch_sw(&self) -> bool {
        self.clutch_sw_raw()
    }
    
    /// Get raw value of clutch_sw
    ///
    /// - Start bit: 15
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn clutch_sw_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[8..9].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of clutch_sw
    #[inline(always)]
    pub fn set_clutch_sw(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[8..9].store_be(value);
        Ok(())
    }
    
    /// combined_accelerator
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "%"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn combined_accelerator(&self) -> f32 {
        self.combined_accelerator_raw()
    }
    
    /// Get raw value of combined_accelerator
    ///
    /// - Start bit: 40
    /// - Signal size: 8 bits
    /// - Factor: 0.392157
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn combined_accelerator_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[40..48].load_le::<u8>();
        
        let factor = 0.392157_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of combined_accelerator
    #[inline(always)]
    pub fn set_combined_accelerator(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 100_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: MotorControl::MESSAGE_ID });
        }
        let factor = 0.392157_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[40..48].store_le(value);
        Ok(())
    }
    
    /// n_accelerator_pedal_max_sw
    ///
    /// is 0 when pressed NOT VERIFIED
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn n_accelerator_pedal_max_sw(&self) -> bool {
        self.n_accelerator_pedal_max_sw_raw()
    }
    
    /// Get raw value of n_accelerator_pedal_max_sw
    ///
    /// - Start bit: 14
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn n_accelerator_pedal_max_sw_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[14..15].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of n_accelerator_pedal_max_sw
    #[inline(always)]
    pub fn set_n_accelerator_pedal_max_sw(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[14..15].store_le(value);
        Ok(())
    }
    
    /// throttle_plate_position
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "%"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn throttle_plate_position(&self) -> f32 {
        self.throttle_plate_position_raw()
    }
    
    /// Get raw value of throttle_plate_position
    ///
    /// - Start bit: 48
    /// - Signal size: 8 bits
    /// - Factor: 0.392157
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn throttle_plate_position_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..56].load_le::<u8>();
        
        let factor = 0.392157_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of throttle_plate_position
    #[inline(always)]
    pub fn set_throttle_plate_position(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 100_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: MotorControl::MESSAGE_ID });
        }
        let factor = 0.392157_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[48..56].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for MotorControl {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for MotorControl {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// engine_status
///
/// - Standard ID: 321 (0x141)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct EngineStatus {
    raw: [u8; 8],
}

impl EngineStatus {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x141)});
    
    pub const ENGINE_RPM_MIN: u16 = 0_u16;
    pub const ENGINE_RPM_MAX: u16 = 8000_u16;
    pub const ENGINE_TORQUE_MIN: u16 = 0_u16;
    pub const ENGINE_TORQUE_MAX: u16 = 255_u16;
    pub const MT_GEAR_MIN: u8 = 0_u8;
    pub const MT_GEAR_MAX: u8 = 7_u8;
    
    /// Construct new engine_status from values
    pub fn new(engine_rpm: u16, engine_running: bool, engine_stop: bool, engine_torque: u16, mt_gear: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_engine_rpm(engine_rpm)?;
        res.set_engine_running(engine_running)?;
        res.set_engine_stop(engine_stop)?;
        res.set_engine_torque(engine_torque)?;
        res.set_mt_gear(mt_gear)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// engine_rpm
    ///
    /// - Min: 0
    /// - Max: 8000
    /// - Unit: "RPM"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn engine_rpm(&self) -> u16 {
        self.engine_rpm_raw()
    }
    
    /// Get raw value of engine_rpm
    ///
    /// - Start bit: 32
    /// - Signal size: 14 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn engine_rpm_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[32..46].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of engine_rpm
    #[inline(always)]
    pub fn set_engine_rpm(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 8000_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: EngineStatus::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: EngineStatus::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..46].store_le(value);
        Ok(())
    }
    
    /// engine_running
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn engine_running(&self) -> bool {
        self.engine_running_raw()
    }
    
    /// Get raw value of engine_running
    ///
    /// - Start bit: 51
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn engine_running_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[51..52].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of engine_running
    #[inline(always)]
    pub fn set_engine_running(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[51..52].store_le(value);
        Ok(())
    }
    
    /// engine_stop
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn engine_stop(&self) -> bool {
        self.engine_stop_raw()
    }
    
    /// Get raw value of engine_stop
    ///
    /// - Start bit: 15
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn engine_stop_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[15..16].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of engine_stop
    #[inline(always)]
    pub fn set_engine_stop(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[15..16].store_le(value);
        Ok(())
    }
    
    /// engine_torque
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn engine_torque(&self) -> u16 {
        self.engine_torque_raw()
    }
    
    /// Get raw value of engine_torque
    ///
    /// - Start bit: 0
    /// - Signal size: 15 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn engine_torque_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..15].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of engine_torque
    #[inline(always)]
    pub fn set_engine_torque(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 255_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: EngineStatus::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: EngineStatus::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..15].store_le(value);
        Ok(())
    }
    
    /// mt_gear
    ///
    /// - Min: 0
    /// - Max: 7
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn mt_gear(&self) -> EngineStatusMtGear {
        let signal = self.raw.view_bits::<Lsb0>()[48..51].load_le::<u8>();
        
        match signal {
            0 => EngineStatusMtGear::Floating,
            1 => EngineStatusMtGear::X1,
            2 => EngineStatusMtGear::X2,
            3 => EngineStatusMtGear::X3,
            4 => EngineStatusMtGear::X4,
            5 => EngineStatusMtGear::X5,
            6 => EngineStatusMtGear::X6,
            7 => EngineStatusMtGear::Neutral,
            _ => EngineStatusMtGear::_Other(self.mt_gear_raw()),
        }
    }
    
    /// Get raw value of mt_gear
    ///
    /// - Start bit: 48
    /// - Signal size: 3 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn mt_gear_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[48..51].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of mt_gear
    #[inline(always)]
    pub fn set_mt_gear(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 7_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: EngineStatus::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: EngineStatus::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[48..51].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for EngineStatus {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for EngineStatus {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}
/// Defined values for mt_gear
#[derive(Clone, Copy, PartialEq)]
pub enum EngineStatusMtGear {
    Floating,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    Neutral,
    _Other(u8),
}

impl From<EngineStatusMtGear> for u8 {
    fn from(val: EngineStatusMtGear) -> u8 {
        match val {
            EngineStatusMtGear::Floating => 0,
            EngineStatusMtGear::X1 => 1,
            EngineStatusMtGear::X2 => 2,
            EngineStatusMtGear::X3 => 3,
            EngineStatusMtGear::X4 => 4,
            EngineStatusMtGear::X5 => 5,
            EngineStatusMtGear::X6 => 6,
            EngineStatusMtGear::Neutral => 7,
            EngineStatusMtGear::_Other(x) => x,
        }
    }
}


/// transmission
///
/// - Standard ID: 328 (0x148)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Transmission {
    raw: [u8; 8],
}

impl Transmission {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x148)});
    
    pub const MT_GEAR_VERIFY_MIN: u8 = 0_u8;
    pub const MT_GEAR_VERIFY_MAX: u8 = 7_u8;
    
    /// Construct new transmission from values
    pub fn new(mt_gear_verify: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_mt_gear_verify(mt_gear_verify)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// mt_gear_VERIFY
    ///
    /// - Min: 0
    /// - Max: 7
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn mt_gear_verify(&self) -> u8 {
        self.mt_gear_verify_raw()
    }
    
    /// Get raw value of mt_gear_VERIFY
    ///
    /// - Start bit: 4
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn mt_gear_verify_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[4..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of mt_gear_VERIFY
    #[inline(always)]
    pub fn set_mt_gear_verify(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 7_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Transmission::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Transmission::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[4..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Transmission {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Transmission {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// status_switches
///
/// - Standard ID: 338 (0x152)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct StatusSwitches {
    raw: [u8; 8],
}

impl StatusSwitches {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x152)});
    
    
    /// Construct new status_switches from values
    pub fn new(acc_power: bool, brake_sw: bool, handbrake_sw: bool, highbeams_enabled: bool, key_on: bool, lowbeams_enabled: bool, parking_lights_enabled: bool, reverse_sw: bool, running_lights_enabled: bool, wiper_moving_sw: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_acc_power(acc_power)?;
        res.set_brake_sw(brake_sw)?;
        res.set_handbrake_sw(handbrake_sw)?;
        res.set_highbeams_enabled(highbeams_enabled)?;
        res.set_key_on(key_on)?;
        res.set_lowbeams_enabled(lowbeams_enabled)?;
        res.set_parking_lights_enabled(parking_lights_enabled)?;
        res.set_reverse_sw(reverse_sw)?;
        res.set_running_lights_enabled(running_lights_enabled)?;
        res.set_wiper_moving_sw(wiper_moving_sw)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// acc_power
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn acc_power(&self) -> bool {
        self.acc_power_raw()
    }
    
    /// Get raw value of acc_power
    ///
    /// - Start bit: 5
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn acc_power_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[2..3].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of acc_power
    #[inline(always)]
    pub fn set_acc_power(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[2..3].store_be(value);
        Ok(())
    }
    
    /// brake_sw
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brake_sw(&self) -> bool {
        self.brake_sw_raw()
    }
    
    /// Get raw value of brake_sw
    ///
    /// - Start bit: 52
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_sw_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[51..52].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of brake_sw
    #[inline(always)]
    pub fn set_brake_sw(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[51..52].store_be(value);
        Ok(())
    }
    
    /// handbrake_sw
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn handbrake_sw(&self) -> bool {
        self.handbrake_sw_raw()
    }
    
    /// Get raw value of handbrake_sw
    ///
    /// - Start bit: 51
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn handbrake_sw_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[52..53].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of handbrake_sw
    #[inline(always)]
    pub fn set_handbrake_sw(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[52..53].store_be(value);
        Ok(())
    }
    
    /// highbeams_enabled
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn highbeams_enabled(&self) -> bool {
        self.highbeams_enabled_raw()
    }
    
    /// Get raw value of highbeams_enabled
    ///
    /// - Start bit: 60
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn highbeams_enabled_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[59..60].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of highbeams_enabled
    #[inline(always)]
    pub fn set_highbeams_enabled(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[59..60].store_be(value);
        Ok(())
    }
    
    /// key_on
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn key_on(&self) -> bool {
        self.key_on_raw()
    }
    
    /// Get raw value of key_on
    ///
    /// - Start bit: 6
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn key_on_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[1..2].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of key_on
    #[inline(always)]
    pub fn set_key_on(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[1..2].store_be(value);
        Ok(())
    }
    
    /// lowbeams_enabled
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn lowbeams_enabled(&self) -> bool {
        self.lowbeams_enabled_raw()
    }
    
    /// Get raw value of lowbeams_enabled
    ///
    /// - Start bit: 59
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn lowbeams_enabled_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[60..61].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of lowbeams_enabled
    #[inline(always)]
    pub fn set_lowbeams_enabled(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[60..61].store_be(value);
        Ok(())
    }
    
    /// parking_lights_enabled
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn parking_lights_enabled(&self) -> bool {
        self.parking_lights_enabled_raw()
    }
    
    /// Get raw value of parking_lights_enabled
    ///
    /// - Start bit: 58
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn parking_lights_enabled_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[61..62].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of parking_lights_enabled
    #[inline(always)]
    pub fn set_parking_lights_enabled(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[61..62].store_be(value);
        Ok(())
    }
    
    /// reverse_sw
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn reverse_sw(&self) -> bool {
        self.reverse_sw_raw()
    }
    
    /// Get raw value of reverse_sw
    ///
    /// - Start bit: 50
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn reverse_sw_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[53..54].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of reverse_sw
    #[inline(always)]
    pub fn set_reverse_sw(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[53..54].store_be(value);
        Ok(())
    }
    
    /// running_lights_enabled
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn running_lights_enabled(&self) -> bool {
        self.running_lights_enabled_raw()
    }
    
    /// Get raw value of running_lights_enabled
    ///
    /// - Start bit: 57
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn running_lights_enabled_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[62..63].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of running_lights_enabled
    #[inline(always)]
    pub fn set_running_lights_enabled(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[62..63].store_be(value);
        Ok(())
    }
    
    /// wiper_moving_sw
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn wiper_moving_sw(&self) -> bool {
        self.wiper_moving_sw_raw()
    }
    
    /// Get raw value of wiper_moving_sw
    ///
    /// - Start bit: 62
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn wiper_moving_sw_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[62..63].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of wiper_moving_sw
    #[inline(always)]
    pub fn set_wiper_moving_sw(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[62..63].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for StatusSwitches {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for StatusSwitches {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// XXXMsg340
///
/// - Standard ID: 340 (0x154)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct XxxMsg340 {
    raw: [u8; 8],
}

impl XxxMsg340 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x154)});
    
    
    /// Construct new XXXMsg340 from values
    pub fn new(any_door_open: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_any_door_open(any_door_open)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// any_door_open
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn any_door_open(&self) -> bool {
        self.any_door_open_raw()
    }
    
    /// Get raw value of any_door_open
    ///
    /// - Start bit: 48
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn any_door_open_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[55..56].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of any_door_open
    #[inline(always)]
    pub fn set_any_door_open(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[55..56].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for XxxMsg340 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for XxxMsg340 {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// bsd_rcta
///
/// - Standard ID: 604 (0x25c)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct BsdRcta {
    raw: [u8; 8],
}

impl BsdRcta {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x25c)});
    
    
    /// Construct new bsd_rcta from values
    pub fn new(rcta_enabled: bool, bsd_left: bool, bsd_right: bool, rcta_left_adjacent: bool, rcta_left_approaching: bool, rcta_right_adjacent: bool, rcta_right_approaching: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_rcta_enabled(rcta_enabled)?;
        res.set_bsd_left(bsd_left)?;
        res.set_bsd_right(bsd_right)?;
        res.set_rcta_left_adjacent(rcta_left_adjacent)?;
        res.set_rcta_left_approaching(rcta_left_approaching)?;
        res.set_rcta_right_adjacent(rcta_right_adjacent)?;
        res.set_rcta_right_approaching(rcta_right_approaching)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// rcta_enabled
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn rcta_enabled(&self) -> bool {
        self.rcta_enabled_raw()
    }
    
    /// Get raw value of rcta_enabled
    ///
    /// - Start bit: 5
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn rcta_enabled_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[5..6].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of rcta_enabled
    #[inline(always)]
    pub fn set_rcta_enabled(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[5..6].store_le(value);
        Ok(())
    }
    
    /// bsd_left
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn bsd_left(&self) -> bool {
        self.bsd_left_raw()
    }
    
    /// Get raw value of bsd_left
    ///
    /// - Start bit: 47
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn bsd_left_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[47..48].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of bsd_left
    #[inline(always)]
    pub fn set_bsd_left(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[47..48].store_le(value);
        Ok(())
    }
    
    /// bsd_right
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn bsd_right(&self) -> bool {
        self.bsd_right_raw()
    }
    
    /// Get raw value of bsd_right
    ///
    /// - Start bit: 46
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn bsd_right_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[46..47].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of bsd_right
    #[inline(always)]
    pub fn set_bsd_right(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[46..47].store_le(value);
        Ok(())
    }
    
    /// rcta_left_adjacent
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn rcta_left_adjacent(&self) -> bool {
        self.rcta_left_adjacent_raw()
    }
    
    /// Get raw value of rcta_left_adjacent
    ///
    /// - Start bit: 33
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn rcta_left_adjacent_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[33..34].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of rcta_left_adjacent
    #[inline(always)]
    pub fn set_rcta_left_adjacent(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[33..34].store_le(value);
        Ok(())
    }
    
    /// rcta_left_approaching
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn rcta_left_approaching(&self) -> bool {
        self.rcta_left_approaching_raw()
    }
    
    /// Get raw value of rcta_left_approaching
    ///
    /// - Start bit: 43
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn rcta_left_approaching_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[43..44].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of rcta_left_approaching
    #[inline(always)]
    pub fn set_rcta_left_approaching(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[43..44].store_le(value);
        Ok(())
    }
    
    /// rcta_right_adjacent
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn rcta_right_adjacent(&self) -> bool {
        self.rcta_right_adjacent_raw()
    }
    
    /// Get raw value of rcta_right_adjacent
    ///
    /// - Start bit: 32
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn rcta_right_adjacent_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[32..33].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of rcta_right_adjacent
    #[inline(always)]
    pub fn set_rcta_right_adjacent(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[32..33].store_le(value);
        Ok(())
    }
    
    /// rcta_right_approaching
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn rcta_right_approaching(&self) -> bool {
        self.rcta_right_approaching_raw()
    }
    
    /// Get raw value of rcta_right_approaching
    ///
    /// - Start bit: 42
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn rcta_right_approaching_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[42..43].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of rcta_right_approaching
    #[inline(always)]
    pub fn set_rcta_right_approaching(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[42..43].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for BsdRcta {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for BsdRcta {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// XXXMsg640
///
/// - Standard ID: 642 (0x282)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct XxxMsg640 {
    raw: [u8; 8],
}

impl XxxMsg640 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x282)});
    
    pub const FUEL_LEVEL_MIN: f32 = 0_f32;
    pub const FUEL_LEVEL_MAX: f32 = 100_f32;
    pub const RAW_FUEL_MIN: i16 = 0_i16;
    pub const RAW_FUEL_MAX: i16 = 4096_i16;
    
    /// Construct new XXXMsg640 from values
    pub fn new(driver_seatbelt_warning_enabled: bool, fuel_level: f32, left_turn_signal_enabled: bool, passenger_seatbelt_warning_enabled: bool, raw_fuel: i16, right_turn_signal_enabled: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_driver_seatbelt_warning_enabled(driver_seatbelt_warning_enabled)?;
        res.set_fuel_level(fuel_level)?;
        res.set_left_turn_signal_enabled(left_turn_signal_enabled)?;
        res.set_passenger_seatbelt_warning_enabled(passenger_seatbelt_warning_enabled)?;
        res.set_raw_fuel(raw_fuel)?;
        res.set_right_turn_signal_enabled(right_turn_signal_enabled)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// driver_seatbelt_warning_enabled
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn driver_seatbelt_warning_enabled(&self) -> bool {
        self.driver_seatbelt_warning_enabled_raw()
    }
    
    /// Get raw value of driver_seatbelt_warning_enabled
    ///
    /// - Start bit: 40
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn driver_seatbelt_warning_enabled_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[47..48].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of driver_seatbelt_warning_enabled
    #[inline(always)]
    pub fn set_driver_seatbelt_warning_enabled(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[47..48].store_be(value);
        Ok(())
    }
    
    /// fuel_level
    ///
    /// need_to_verify_min/max/scale
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "%"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn fuel_level(&self) -> f32 {
        self.fuel_level_raw()
    }
    
    /// Get raw value of fuel_level
    ///
    /// - Start bit: 0
    /// - Signal size: 12 bits
    /// - Factor: 0.033
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fuel_level_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..12].load_le::<u16>();
        
        let factor = 0.033_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of fuel_level
    #[inline(always)]
    pub fn set_fuel_level(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 100_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: XxxMsg640::MESSAGE_ID });
        }
        let factor = 0.033_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..12].store_le(value);
        Ok(())
    }
    
    /// left_turn_signal_enabled
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn left_turn_signal_enabled(&self) -> bool {
        self.left_turn_signal_enabled_raw()
    }
    
    /// Get raw value of left_turn_signal_enabled
    ///
    /// - Start bit: 44
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn left_turn_signal_enabled_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[43..44].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of left_turn_signal_enabled
    #[inline(always)]
    pub fn set_left_turn_signal_enabled(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[43..44].store_be(value);
        Ok(())
    }
    
    /// passenger_seatbelt_warning_enabled
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn passenger_seatbelt_warning_enabled(&self) -> bool {
        self.passenger_seatbelt_warning_enabled_raw()
    }
    
    /// Get raw value of passenger_seatbelt_warning_enabled
    ///
    /// - Start bit: 41
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn passenger_seatbelt_warning_enabled_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[46..47].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of passenger_seatbelt_warning_enabled
    #[inline(always)]
    pub fn set_passenger_seatbelt_warning_enabled(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[46..47].store_be(value);
        Ok(())
    }
    
    /// raw_fuel
    ///
    /// - Min: 0
    /// - Max: 4096
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn raw_fuel(&self) -> i16 {
        self.raw_fuel_raw()
    }
    
    /// Get raw value of raw_fuel
    ///
    /// - Start bit: 0
    /// - Signal size: 12 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn raw_fuel_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..12].load_le::<i16>();
        
        let factor = 1;
        let signal = signal as i16;
        i16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of raw_fuel
    #[inline(always)]
    pub fn set_raw_fuel(&mut self, value: i16) -> Result<(), CanError> {
        if value < 0_i16 || 4096_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: XxxMsg640::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: XxxMsg640::MESSAGE_ID })?;
        let value = (value / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..12].store_le(value);
        Ok(())
    }
    
    /// right_turn_signal_enabled
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn right_turn_signal_enabled(&self) -> bool {
        self.right_turn_signal_enabled_raw()
    }
    
    /// Get raw value of right_turn_signal_enabled
    ///
    /// - Start bit: 45
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn right_turn_signal_enabled_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[42..43].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of right_turn_signal_enabled
    #[inline(always)]
    pub fn set_right_turn_signal_enabled(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[42..43].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for XxxMsg640 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for XxxMsg640 {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// ignition
///
/// - Standard ID: 644 (0x284)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Ignition {
    raw: [u8; 8],
}

impl Ignition {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x284)});
    
    
    /// Construct new ignition from values
    pub fn new(access_key_detected: bool, ignition_acc: bool, ignition_on: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_access_key_detected(access_key_detected)?;
        res.set_ignition_acc(ignition_acc)?;
        res.set_ignition_on(ignition_on)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// access_key_detected
    ///
    /// 4:6_bits_are_set_but_only_check_6
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn access_key_detected(&self) -> bool {
        self.access_key_detected_raw()
    }
    
    /// Get raw value of access_key_detected
    ///
    /// - Start bit: 46
    /// - Signal size: 1 bits
    /// - Factor: -1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn access_key_detected_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[41..42].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of access_key_detected
    #[inline(always)]
    pub fn set_access_key_detected(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[41..42].store_be(value);
        Ok(())
    }
    
    /// ignition_acc
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn ignition_acc(&self) -> bool {
        self.ignition_acc_raw()
    }
    
    /// Get raw value of ignition_acc
    ///
    /// - Start bit: 25
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn ignition_acc_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[30..31].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of ignition_acc
    #[inline(always)]
    pub fn set_ignition_acc(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[30..31].store_be(value);
        Ok(())
    }
    
    /// ignition_on
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn ignition_on(&self) -> bool {
        self.ignition_on_raw()
    }
    
    /// Get raw value of ignition_on
    ///
    /// - Start bit: 6
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn ignition_on_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[1..2].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of ignition_on
    #[inline(always)]
    pub fn set_ignition_on(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[1..2].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Ignition {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Ignition {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// engine_status_2
///
/// - Standard ID: 864 (0x360)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct EngineStatus2 {
    raw: [u8; 8],
}

impl EngineStatus2 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x360)});
    
    pub const CRUISE_CONTROL_SPEED_MIN: u8 = 0_u8;
    pub const CRUISE_CONTROL_SPEED_MAX: u8 = 255_u8;
    pub const ENGINE_BOOST_PRESSURE_MIN: f32 = 0_f32;
    pub const ENGINE_BOOST_PRESSURE_MAX: f32 = 240.9_f32;
    pub const ENGINE_COOLANT_TEMP_MIN: i16 = 0_i16;
    pub const ENGINE_COOLANT_TEMP_MAX: i16 = 216_i16;
    pub const ENGINE_FUEL_FLOW_MIN: u8 = 0_u8;
    pub const ENGINE_FUEL_FLOW_MAX: u8 = 255_u8;
    pub const ENGINE_OIL_TEMP_MIN: i16 = 0_i16;
    pub const ENGINE_OIL_TEMP_MAX: i16 = 216_i16;
    
    /// Construct new engine_status_2 from values
    pub fn new(cruise_control_enabled: bool, cruise_control_set_enabled: bool, cruise_control_speed: u8, engine_boost_pressure: f32, engine_coolant_temp: i16, engine_fuel_flow: u8, engine_oil_temp: i16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_cruise_control_enabled(cruise_control_enabled)?;
        res.set_cruise_control_set_enabled(cruise_control_set_enabled)?;
        res.set_cruise_control_speed(cruise_control_speed)?;
        res.set_engine_boost_pressure(engine_boost_pressure)?;
        res.set_engine_coolant_temp(engine_coolant_temp)?;
        res.set_engine_fuel_flow(engine_fuel_flow)?;
        res.set_engine_oil_temp(engine_oil_temp)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// cruise_control_enabled
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn cruise_control_enabled(&self) -> bool {
        self.cruise_control_enabled_raw()
    }
    
    /// Get raw value of cruise_control_enabled
    ///
    /// - Start bit: 44
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn cruise_control_enabled_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[43..44].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of cruise_control_enabled
    #[inline(always)]
    pub fn set_cruise_control_enabled(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[43..44].store_be(value);
        Ok(())
    }
    
    /// cruise_control_set_enabled
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn cruise_control_set_enabled(&self) -> bool {
        self.cruise_control_set_enabled_raw()
    }
    
    /// Get raw value of cruise_control_set_enabled
    ///
    /// - Start bit: 45
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn cruise_control_set_enabled_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[42..43].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of cruise_control_set_enabled
    #[inline(always)]
    pub fn set_cruise_control_set_enabled(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[42..43].store_be(value);
        Ok(())
    }
    
    /// cruise_control_speed
    ///
    /// unit_is_determined_by_vehicle_region_or_bit_flag
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn cruise_control_speed(&self) -> u8 {
        self.cruise_control_speed_raw()
    }
    
    /// Get raw value of cruise_control_speed
    ///
    /// - Start bit: 56
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn cruise_control_speed_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[56..64].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of cruise_control_speed
    #[inline(always)]
    pub fn set_cruise_control_speed(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: EngineStatus2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: EngineStatus2::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[56..64].store_le(value);
        Ok(())
    }
    
    /// engine_boost_pressure
    ///
    /// need_to_verify
    ///
    /// - Min: 0
    /// - Max: 240.9
    /// - Unit: "psi"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn engine_boost_pressure(&self) -> f32 {
        self.engine_boost_pressure_raw()
    }
    
    /// Get raw value of engine_boost_pressure
    ///
    /// - Start bit: 32
    /// - Signal size: 8 bits
    /// - Factor: 0.3
    /// - Offset: -15.1
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn engine_boost_pressure_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..40].load_le::<i8>();
        
        let factor = 0.3_f32;
        let offset = -15.1_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of engine_boost_pressure
    #[inline(always)]
    pub fn set_engine_boost_pressure(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 240.9_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: EngineStatus2::MESSAGE_ID });
        }
        let factor = 0.3_f32;
        let offset = -15.1_f32;
        let value = ((value - offset) / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..40].store_le(value);
        Ok(())
    }
    
    /// engine_coolant_temp
    ///
    /// - Min: 0
    /// - Max: 216
    /// - Unit: "degC"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn engine_coolant_temp(&self) -> i16 {
        self.engine_coolant_temp_raw()
    }
    
    /// Get raw value of engine_coolant_temp
    ///
    /// - Start bit: 24
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: -40
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn engine_coolant_temp_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[24..32].load_le::<u8>();
        
        let factor = 1;
        i16::from(signal).saturating_mul(factor).saturating_sub(40)
    }
    
    /// Set value of engine_coolant_temp
    #[inline(always)]
    pub fn set_engine_coolant_temp(&mut self, value: i16) -> Result<(), CanError> {
        if value < 0_i16 || 216_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: EngineStatus2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_add(40)
            .ok_or(CanError::ParameterOutOfRange { message_id: EngineStatus2::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[24..32].store_le(value);
        Ok(())
    }
    
    /// engine_fuel_flow
    ///
    /// scale/unit_undetermined
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn engine_fuel_flow(&self) -> u8 {
        self.engine_fuel_flow_raw()
    }
    
    /// Get raw value of engine_fuel_flow
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn engine_fuel_flow_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of engine_fuel_flow
    #[inline(always)]
    pub fn set_engine_fuel_flow(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: EngineStatus2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: EngineStatus2::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// engine_oil_temp
    ///
    /// - Min: 0
    /// - Max: 216
    /// - Unit: "degC"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn engine_oil_temp(&self) -> i16 {
        self.engine_oil_temp_raw()
    }
    
    /// Get raw value of engine_oil_temp
    ///
    /// - Start bit: 16
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: -40
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn engine_oil_temp_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<u8>();
        
        let factor = 1;
        i16::from(signal).saturating_mul(factor).saturating_sub(40)
    }
    
    /// Set value of engine_oil_temp
    #[inline(always)]
    pub fn set_engine_oil_temp(&mut self, value: i16) -> Result<(), CanError> {
        if value < 0_i16 || 216_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: EngineStatus2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_add(40)
            .ok_or(CanError::ParameterOutOfRange { message_id: EngineStatus2::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for EngineStatus2 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for EngineStatus2 {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// engine_warning_lights
///
/// - Standard ID: 865 (0x361)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct EngineWarningLights {
    raw: [u8; 8],
}

impl EngineWarningLights {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x361)});
    
    
    /// Construct new engine_warning_lights from values
    pub fn new(check_engine_light_enabled: bool, oil_pressure_warning_light_enabled: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_check_engine_light_enabled(check_engine_light_enabled)?;
        res.set_oil_pressure_warning_light_enabled(oil_pressure_warning_light_enabled)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// check_engine_light_enabled
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn check_engine_light_enabled(&self) -> bool {
        self.check_engine_light_enabled_raw()
    }
    
    /// Get raw value of check_engine_light_enabled
    ///
    /// - Start bit: 39
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn check_engine_light_enabled_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[32..33].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of check_engine_light_enabled
    #[inline(always)]
    pub fn set_check_engine_light_enabled(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[32..33].store_be(value);
        Ok(())
    }
    
    /// oil_pressure_warning_light_enabled
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn oil_pressure_warning_light_enabled(&self) -> bool {
        self.oil_pressure_warning_light_enabled_raw()
    }
    
    /// Get raw value of oil_pressure_warning_light_enabled
    ///
    /// - Start bit: 12
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn oil_pressure_warning_light_enabled_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[11..12].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of oil_pressure_warning_light_enabled
    #[inline(always)]
    pub fn set_oil_pressure_warning_light_enabled(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[11..12].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for EngineWarningLights {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for EngineWarningLights {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// srs_status
///
/// - Standard ID: 882 (0x372)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct SrsStatus {
    raw: [u8; 8],
}

impl SrsStatus {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x372)});
    
    
    /// Construct new srs_status from values
    pub fn new(srs_system_warning_light_enabled: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_srs_system_warning_light_enabled(srs_system_warning_light_enabled)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// srs_system_warning_light_enabled
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn srs_system_warning_light_enabled(&self) -> bool {
        self.srs_system_warning_light_enabled_raw()
    }
    
    /// Get raw value of srs_system_warning_light_enabled
    ///
    /// - Start bit: 16
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn srs_system_warning_light_enabled_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[23..24].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of srs_system_warning_light_enabled
    #[inline(always)]
    pub fn set_srs_system_warning_light_enabled(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[23..24].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for SrsStatus {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for SrsStatus {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// XXXMsg884
///
/// - Standard ID: 884 (0x374)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct XxxMsg884 {
    raw: [u8; 8],
}

impl XxxMsg884 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x374)});
    
    
    /// Construct new XXXMsg884 from values
    pub fn new(fog_lights_enabled: bool, tpms_warning_light_enabled: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_fog_lights_enabled(fog_lights_enabled)?;
        res.set_tpms_warning_light_enabled(tpms_warning_light_enabled)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// fog_lights_enabled
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn fog_lights_enabled(&self) -> bool {
        self.fog_lights_enabled_raw()
    }
    
    /// Get raw value of fog_lights_enabled
    ///
    /// - Start bit: 14
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fog_lights_enabled_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[9..10].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of fog_lights_enabled
    #[inline(always)]
    pub fn set_fog_lights_enabled(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[9..10].store_be(value);
        Ok(())
    }
    
    /// tpms_warning_light_enabled
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn tpms_warning_light_enabled(&self) -> bool {
        self.tpms_warning_light_enabled_raw()
    }
    
    /// Get raw value of tpms_warning_light_enabled
    ///
    /// - Start bit: 36
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn tpms_warning_light_enabled_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[35..36].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of tpms_warning_light_enabled
    #[inline(always)]
    pub fn set_tpms_warning_light_enabled(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[35..36].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for XxxMsg884 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for XxxMsg884 {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// XXXMsg885
///
/// - Standard ID: 885 (0x375)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct XxxMsg885 {
    raw: [u8; 8],
}

impl XxxMsg885 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x375)});
    
    
    /// Construct new XXXMsg885 from values
    pub fn new(dimmer_max_brightness_enable: bool, headlight_dimmer_enabled: bool, left_front_door_open: bool, left_rear_door_open: bool, right_front_door_open: bool, right_rear_door_open: bool, trunk_open: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_dimmer_max_brightness_enable(dimmer_max_brightness_enable)?;
        res.set_headlight_dimmer_enabled(headlight_dimmer_enabled)?;
        res.set_left_front_door_open(left_front_door_open)?;
        res.set_left_rear_door_open(left_rear_door_open)?;
        res.set_right_front_door_open(right_front_door_open)?;
        res.set_right_rear_door_open(right_rear_door_open)?;
        res.set_trunk_open(trunk_open)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// dimmer_max_brightness_enable
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn dimmer_max_brightness_enable(&self) -> bool {
        self.dimmer_max_brightness_enable_raw()
    }
    
    /// Get raw value of dimmer_max_brightness_enable
    ///
    /// - Start bit: 31
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn dimmer_max_brightness_enable_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[24..25].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of dimmer_max_brightness_enable
    #[inline(always)]
    pub fn set_dimmer_max_brightness_enable(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[24..25].store_be(value);
        Ok(())
    }
    
    /// headlight_dimmer_enabled
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn headlight_dimmer_enabled(&self) -> bool {
        self.headlight_dimmer_enabled_raw()
    }
    
    /// Get raw value of headlight_dimmer_enabled
    ///
    /// - Start bit: 27
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn headlight_dimmer_enabled_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[28..29].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of headlight_dimmer_enabled
    #[inline(always)]
    pub fn set_headlight_dimmer_enabled(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[28..29].store_be(value);
        Ok(())
    }
    
    /// left_front_door_open
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn left_front_door_open(&self) -> bool {
        self.left_front_door_open_raw()
    }
    
    /// Get raw value of left_front_door_open
    ///
    /// - Start bit: 8
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn left_front_door_open_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[15..16].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of left_front_door_open
    #[inline(always)]
    pub fn set_left_front_door_open(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[15..16].store_be(value);
        Ok(())
    }
    
    /// left_rear_door_open
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn left_rear_door_open(&self) -> bool {
        self.left_rear_door_open_raw()
    }
    
    /// Get raw value of left_rear_door_open
    ///
    /// - Start bit: 11
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn left_rear_door_open_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[12..13].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of left_rear_door_open
    #[inline(always)]
    pub fn set_left_rear_door_open(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[12..13].store_be(value);
        Ok(())
    }
    
    /// right_front_door_open
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn right_front_door_open(&self) -> bool {
        self.right_front_door_open_raw()
    }
    
    /// Get raw value of right_front_door_open
    ///
    /// - Start bit: 9
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn right_front_door_open_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[14..15].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of right_front_door_open
    #[inline(always)]
    pub fn set_right_front_door_open(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[14..15].store_be(value);
        Ok(())
    }
    
    /// right_rear_door_open
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn right_rear_door_open(&self) -> bool {
        self.right_rear_door_open_raw()
    }
    
    /// Get raw value of right_rear_door_open
    ///
    /// - Start bit: 10
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn right_rear_door_open_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[13..14].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of right_rear_door_open
    #[inline(always)]
    pub fn set_right_rear_door_open(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[13..14].store_be(value);
        Ok(())
    }
    
    /// trunk_open
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn trunk_open(&self) -> bool {
        self.trunk_open_raw()
    }
    
    /// Get raw value of trunk_open
    ///
    /// - Start bit: 13
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn trunk_open_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[10..11].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of trunk_open
    #[inline(always)]
    pub fn set_trunk_open(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[10..11].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for XxxMsg885 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for XxxMsg885 {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// DimmerAndHood
///
/// - Standard ID: 886 (0x376)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct DimmerAndHood {
    raw: [u8; 8],
}

impl DimmerAndHood {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x376)});
    
    pub const DIMMER_DIAL_VALUE_MIN: u8 = 0_u8;
    pub const DIMMER_DIAL_VALUE_MAX: u8 = 250_u8;
    
    /// Construct new DimmerAndHood from values
    pub fn new(dimmer_dial_value: u8, hood_closed: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_dimmer_dial_value(dimmer_dial_value)?;
        res.set_hood_closed(hood_closed)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// dimmer_dial_value
    ///
    /// - Min: 0
    /// - Max: 250
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn dimmer_dial_value(&self) -> DimmerAndHoodDimmerDialValue {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        match signal {
            0 => DimmerAndHoodDimmerDialValue::X0,
            33 => DimmerAndHoodDimmerDialValue::X1,
            82 => DimmerAndHoodDimmerDialValue::X2,
            125 => DimmerAndHoodDimmerDialValue::X3,
            173 => DimmerAndHoodDimmerDialValue::X4,
            250 => DimmerAndHoodDimmerDialValue::X5,
            _ => DimmerAndHoodDimmerDialValue::_Other(self.dimmer_dial_value_raw()),
        }
    }
    
    /// Get raw value of dimmer_dial_value
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn dimmer_dial_value_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of dimmer_dial_value
    #[inline(always)]
    pub fn set_dimmer_dial_value(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 250_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: DimmerAndHood::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: DimmerAndHood::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// hood_closed
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn hood_closed(&self) -> bool {
        self.hood_closed_raw()
    }
    
    /// Get raw value of hood_closed
    ///
    /// - Start bit: 8
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn hood_closed_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[8..9].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of hood_closed
    #[inline(always)]
    pub fn set_hood_closed(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[8..9].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for DimmerAndHood {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for DimmerAndHood {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}
/// Defined values for dimmer_dial_value
#[derive(Clone, Copy, PartialEq)]
pub enum DimmerAndHoodDimmerDialValue {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    _Other(u8),
}

impl From<DimmerAndHoodDimmerDialValue> for u8 {
    fn from(val: DimmerAndHoodDimmerDialValue) -> u8 {
        match val {
            DimmerAndHoodDimmerDialValue::X0 => 0,
            DimmerAndHoodDimmerDialValue::X1 => 33,
            DimmerAndHoodDimmerDialValue::X2 => 82,
            DimmerAndHoodDimmerDialValue::X3 => 125,
            DimmerAndHoodDimmerDialValue::X4 => 173,
            DimmerAndHoodDimmerDialValue::X5 => 250,
            DimmerAndHoodDimmerDialValue::_Other(x) => x,
        }
    }
}


/// Dash_State2_VERIFY
///
/// - Standard ID: 977 (0x3d1)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct DashState2Verify {
    raw: [u8; 8],
}

impl DashState2Verify {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x3d1)});
    
    
    /// Construct new Dash_State2_VERIFY from values
    pub fn new(units: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_units(units)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// UNITS
    ///
    /// 0 = Metric, 1 = Imperial NOT VERIFIED
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn units(&self) -> bool {
        self.units_raw()
    }
    
    /// Get raw value of UNITS
    ///
    /// - Start bit: 15
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn units_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[15..16].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of UNITS
    #[inline(always)]
    pub fn set_units(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[15..16].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for DashState2Verify {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for DashState2Verify {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// odometer
///
/// - Standard ID: 1745 (0x6d1)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Odometer {
    raw: [u8; 8],
}

impl Odometer {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x6d1)});
    
    pub const ODOMETER_MIN: f32 = 0_f32;
    pub const ODOMETER_MAX: f32 = 4294970000_f32;
    
    /// Construct new odometer from values
    pub fn new(odometer: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_odometer(odometer)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// odometer
    ///
    /// USCS_converted
    ///
    /// - Min: 0
    /// - Max: 4294970000
    /// - Unit: "KM"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn odometer(&self) -> f32 {
        self.odometer_raw()
    }
    
    /// Get raw value of odometer
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 0.160934
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn odometer_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<u32>();
        
        let factor = 0.160934_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of odometer
    #[inline(always)]
    pub fn set_odometer(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 4294970000_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Odometer::MESSAGE_ID });
        }
        let factor = 0.160934_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u32;
        
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Odometer {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Odometer {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// tpms
///
/// - Standard ID: 1883 (0x75b)
/// - Size: 8 bytes
///
/// no successful communication with bcm yet
#[derive(Clone, Copy)]
pub struct Tpms {
    raw: [u8; 8],
}

impl Tpms {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x75b)});
    
    pub const LEFT_FRONT_TIRE_PRESSURE_MIN: u8 = 0_u8;
    pub const LEFT_FRONT_TIRE_PRESSURE_MAX: u8 = 1_u8;
    pub const LEFT_REAR_TIRE_PRESSURE_MIN: u8 = 0_u8;
    pub const LEFT_REAR_TIRE_PRESSURE_MAX: u8 = 1_u8;
    pub const RIGHT_FRONT_TIRE_PRESSURE_MIN: u8 = 0_u8;
    pub const RIGHT_FRONT_TIRE_PRESSURE_MAX: u8 = 1_u8;
    pub const RIGHT_REAR_TIRE_PRESSURE_MIN: u8 = 0_u8;
    pub const RIGHT_REAR_TIRE_PRESSURE_MAX: u8 = 1_u8;
    
    /// Construct new tpms from values
    pub fn new(left_front_tire_pressure: u8, left_rear_tire_pressure: u8, right_front_tire_pressure: u8, right_rear_tire_pressure: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_left_front_tire_pressure(left_front_tire_pressure)?;
        res.set_left_rear_tire_pressure(left_rear_tire_pressure)?;
        res.set_right_front_tire_pressure(right_front_tire_pressure)?;
        res.set_right_rear_tire_pressure(right_rear_tire_pressure)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// left_front_tire_pressure
    ///
    /// scale/unit_undetermined
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn left_front_tire_pressure(&self) -> u8 {
        self.left_front_tire_pressure_raw()
    }
    
    /// Get raw value of left_front_tire_pressure
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn left_front_tire_pressure_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of left_front_tire_pressure
    #[inline(always)]
    pub fn set_left_front_tire_pressure(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 1_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Tpms::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Tpms::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// left_rear_tire_pressure
    ///
    /// scale/unit_undetermined
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn left_rear_tire_pressure(&self) -> u8 {
        self.left_rear_tire_pressure_raw()
    }
    
    /// Get raw value of left_rear_tire_pressure
    ///
    /// - Start bit: 24
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn left_rear_tire_pressure_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[24..32].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of left_rear_tire_pressure
    #[inline(always)]
    pub fn set_left_rear_tire_pressure(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 1_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Tpms::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Tpms::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[24..32].store_le(value);
        Ok(())
    }
    
    /// right_front_tire_pressure
    ///
    /// scale/unit_undetermined
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn right_front_tire_pressure(&self) -> u8 {
        self.right_front_tire_pressure_raw()
    }
    
    /// Get raw value of right_front_tire_pressure
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn right_front_tire_pressure_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of right_front_tire_pressure
    #[inline(always)]
    pub fn set_right_front_tire_pressure(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 1_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Tpms::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Tpms::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// right_rear_tire_pressure
    ///
    /// scale/unit_undetermined
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn right_rear_tire_pressure(&self) -> u8 {
        self.right_rear_tire_pressure_raw()
    }
    
    /// Get raw value of right_rear_tire_pressure
    ///
    /// - Start bit: 16
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn right_rear_tire_pressure_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of right_rear_tire_pressure
    #[inline(always)]
    pub fn set_right_rear_tire_pressure(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 1_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Tpms::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Tpms::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Tpms {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Tpms {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}


/// This is just to make testing easier
#[allow(dead_code)]
fn main() {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanError {
    UnknownMessageId(embedded_can::Id),
    /// Signal parameter is not within the range
    /// defined in the dbc
    ParameterOutOfRange {
        /// dbc message id
        message_id: embedded_can::Id,
    },
    InvalidPayloadSize,
    /// Multiplexor value not defined in the dbc
    InvalidMultiplexor {
        /// dbc message id
        message_id: embedded_can::Id,
        /// Multiplexor value not defined in the dbc
        multiplexor: u16,
    },
}

impl core::fmt::Display for CanError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

