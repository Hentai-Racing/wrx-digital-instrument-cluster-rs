//! Generated code from build.rs::generate_can_data_emulator()!

use crate::can::messages::wrx_2018;
use crate::can::can_backend::CanFrame;
use rand::Rng;

pub fn generate_frames() -> Vec<CanFrame> {
	let mut ret_frames = vec![];	let g_sensor_lateral = rand::rng().random_range(wrx_2018::GSensor::G_SENSOR_LATERAL_MIN..=wrx_2018::GSensor::G_SENSOR_LATERAL_MAX);
	let g_sensor_longitudinal = rand::rng().random_range(wrx_2018::GSensor::G_SENSOR_LONGITUDINAL_MIN..=wrx_2018::GSensor::G_SENSOR_LONGITUDINAL_MAX);
	let steering_angle = rand::rng().random_range(wrx_2018::GSensor::STEERING_ANGLE_MIN..=wrx_2018::GSensor::STEERING_ANGLE_MAX);
	let gsensor_frame = wrx_2018::GSensor::new(g_sensor_lateral, g_sensor_longitudinal, steering_angle).expect("Failed to create frame");
	ret_frames.push(CanFrame::from_frame(gsensor_frame));

	let brake_pedal_pressure = rand::rng().random_range(wrx_2018::BrakePedal::BRAKE_PEDAL_PRESSURE_MIN..=wrx_2018::BrakePedal::BRAKE_PEDAL_PRESSURE_MAX);
	let vehicle_speed = rand::rng().random_range(wrx_2018::BrakePedal::VEHICLE_SPEED_MIN..=wrx_2018::BrakePedal::VEHICLE_SPEED_MAX);
	let brakepedal_frame = wrx_2018::BrakePedal::new(brake_pedal_pressure, vehicle_speed).expect("Failed to create frame");
	ret_frames.push(CanFrame::from_frame(brakepedal_frame));

	let active_tq_vectoring_enabled = rand::rng().random_bool(0.5);
	let brake_cruise_on = rand::rng().random_bool(0.5);
	let hill_assist_enabled = rand::rng().random_bool(0.5);
	let traction_control_disabled = rand::rng().random_bool(0.5);
	let driverroadassists_frame = wrx_2018::DriverRoadAssists::new(active_tq_vectoring_enabled, brake_cruise_on, hill_assist_enabled, traction_control_disabled).expect("Failed to create frame");
	ret_frames.push(CanFrame::from_frame(driverroadassists_frame));

	let left_front_wheel_speed = rand::rng().random_range(wrx_2018::WheelSpeeds::LEFT_FRONT_WHEEL_SPEED_MIN..=wrx_2018::WheelSpeeds::LEFT_FRONT_WHEEL_SPEED_MAX);
	let left_rear_wheel_speed = rand::rng().random_range(wrx_2018::WheelSpeeds::LEFT_REAR_WHEEL_SPEED_MIN..=wrx_2018::WheelSpeeds::LEFT_REAR_WHEEL_SPEED_MAX);
	let right_front_wheel_speed = rand::rng().random_range(wrx_2018::WheelSpeeds::RIGHT_FRONT_WHEEL_SPEED_MIN..=wrx_2018::WheelSpeeds::RIGHT_FRONT_WHEEL_SPEED_MAX);
	let right_rear_wheel_speed = rand::rng().random_range(wrx_2018::WheelSpeeds::RIGHT_REAR_WHEEL_SPEED_MIN..=wrx_2018::WheelSpeeds::RIGHT_REAR_WHEEL_SPEED_MAX);
	let wheelspeeds_frame = wrx_2018::WheelSpeeds::new(left_front_wheel_speed, left_rear_wheel_speed, right_front_wheel_speed, right_rear_wheel_speed).expect("Failed to create frame");
	ret_frames.push(CanFrame::from_frame(wheelspeeds_frame));

	let steering_wheel_angle = rand::rng().random_range(wrx_2018::Steering::STEERING_WHEEL_ANGLE_MIN..=wrx_2018::Steering::STEERING_WHEEL_ANGLE_MAX);
	let steering_frame = wrx_2018::Steering::new(steering_wheel_angle).expect("Failed to create frame");
	ret_frames.push(CanFrame::from_frame(steering_frame));

	let accelerator_combined = rand::rng().random_range(wrx_2018::MotorControl::ACCELERATOR_COMBINED_MIN..=wrx_2018::MotorControl::ACCELERATOR_COMBINED_MAX);
	let accelerator_cruise_position = rand::rng().random_range(wrx_2018::MotorControl::ACCELERATOR_CRUISE_POSITION_MIN..=wrx_2018::MotorControl::ACCELERATOR_CRUISE_POSITION_MAX);
	let accelerator_pedal_position = rand::rng().random_range(wrx_2018::MotorControl::ACCELERATOR_PEDAL_POSITION_MIN..=wrx_2018::MotorControl::ACCELERATOR_PEDAL_POSITION_MAX);
	let mt_clutch_sw = rand::rng().random_bool(0.5);
	let n_accelerator_pedal_max_sw = rand::rng().random_bool(0.5);
	let throttle_plate_position = rand::rng().random_range(wrx_2018::MotorControl::THROTTLE_PLATE_POSITION_MIN..=wrx_2018::MotorControl::THROTTLE_PLATE_POSITION_MAX);
	let motorcontrol_frame = wrx_2018::MotorControl::new(accelerator_combined, accelerator_cruise_position, accelerator_pedal_position, mt_clutch_sw, n_accelerator_pedal_max_sw, throttle_plate_position).expect("Failed to create frame");
	ret_frames.push(CanFrame::from_frame(motorcontrol_frame));

	let engine_rpm = rand::rng().random_range(wrx_2018::Engine::ENGINE_RPM_MIN..=wrx_2018::Engine::ENGINE_RPM_MAX);
	let engine_running = rand::rng().random_bool(0.5);
	let engine_stop = rand::rng().random_bool(0.5);
	let mt_gear = rand::rng().random_range(wrx_2018::Engine::MT_GEAR_MIN..=wrx_2018::Engine::MT_GEAR_MAX);
	let engine_frame = wrx_2018::Engine::new(engine_rpm, engine_running, engine_stop, mt_gear).expect("Failed to create frame");
	ret_frames.push(CanFrame::from_frame(engine_frame));

	let mt_gear_verify = rand::rng().random_range(wrx_2018::Transmission::MT_GEAR_VERIFY_MIN..=wrx_2018::Transmission::MT_GEAR_VERIFY_MAX);
	let transmission_temp_verify = rand::rng().random_range(wrx_2018::Transmission::TRANSMISSION_TEMP_VERIFY_MIN..=wrx_2018::Transmission::TRANSMISSION_TEMP_VERIFY_MAX);
	let transmission_frame = wrx_2018::Transmission::new(mt_gear_verify, transmission_temp_verify).expect("Failed to create frame");
	ret_frames.push(CanFrame::from_frame(transmission_frame));

	let acc_power = rand::rng().random_bool(0.5);
	let brake_sw = rand::rng().random_bool(0.5);
	let handbrake_sw = rand::rng().random_bool(0.5);
	let highbeams_enabled = rand::rng().random_bool(0.5);
	let key_on = rand::rng().random_bool(0.5);
	let lowbeams_enabled = rand::rng().random_bool(0.5);
	let parking_lights_enabled = rand::rng().random_bool(0.5);
	let reverse_sw = rand::rng().random_bool(0.5);
	let running_lights_enabled = rand::rng().random_bool(0.5);
	let wiper_moving_sw = rand::rng().random_bool(0.5);
	let statusswitches_frame = wrx_2018::StatusSwitches::new(acc_power, brake_sw, handbrake_sw, highbeams_enabled, key_on, lowbeams_enabled, parking_lights_enabled, reverse_sw, running_lights_enabled, wiper_moving_sw).expect("Failed to create frame");
	ret_frames.push(CanFrame::from_frame(statusswitches_frame));

	let any_door_open = rand::rng().random_bool(0.5);
	let xxxmsg340_frame = wrx_2018::XxxMsg340::new(any_door_open).expect("Failed to create frame");
	ret_frames.push(CanFrame::from_frame(xxxmsg340_frame));

	let bsd_left_adjacent = rand::rng().random_bool(0.5);
	let bsd_left_approaching = rand::rng().random_bool(0.5);
	let bsd_right_adjacent = rand::rng().random_bool(0.5);
	let bsd_right_approaching = rand::rng().random_bool(0.5);
	let rcta_disabled = rand::rng().random_bool(0.5);
	let rcta_left = rand::rng().random_bool(0.5);
	let rcta_right = rand::rng().random_bool(0.5);
	let bsdrcta_frame = wrx_2018::BsdRcta::new(bsd_left_adjacent, bsd_left_approaching, bsd_right_adjacent, bsd_right_approaching, rcta_disabled, rcta_left, rcta_right).expect("Failed to create frame");
	ret_frames.push(CanFrame::from_frame(bsdrcta_frame));

	let airflow_distribution_mode = rand::rng().random_range(wrx_2018::ClimateControl::AIRFLOW_DISTRIBUTION_MODE_MIN..=wrx_2018::ClimateControl::AIRFLOW_DISTRIBUTION_MODE_MAX);
	let blower_fan_level = rand::rng().random_range(wrx_2018::ClimateControl::BLOWER_FAN_LEVEL_MIN..=wrx_2018::ClimateControl::BLOWER_FAN_LEVEL_MAX);
	let blend_door = rand::rng().random_range(wrx_2018::ClimateControl::BLEND_DOOR_MIN..=wrx_2018::ClimateControl::BLEND_DOOR_MAX);
	let rear_defrost_enabled = rand::rng().random_bool(0.5);
	let air_vent_mode_enabled = rand::rng().random_bool(0.5);
	let air_recirculation_mode_enabled = rand::rng().random_bool(0.5);
	let climatecontrol_frame = wrx_2018::ClimateControl::new(airflow_distribution_mode, blower_fan_level, blend_door, rear_defrost_enabled, air_vent_mode_enabled, air_recirculation_mode_enabled).expect("Failed to create frame");
	ret_frames.push(CanFrame::from_frame(climatecontrol_frame));

	let driver_seatbelt_warning_enabled = rand::rng().random_bool(0.5);
	let fuel_level = rand::rng().random_range(wrx_2018::Cluster::FUEL_LEVEL_MIN..=wrx_2018::Cluster::FUEL_LEVEL_MAX);
	let left_turn_signal_enabled = rand::rng().random_bool(0.5);
	let passenger_seatbelt_warning_enabled = rand::rng().random_bool(0.5);
	let right_turn_signal_enabled = rand::rng().random_bool(0.5);
	let cluster_frame = wrx_2018::Cluster::new(driver_seatbelt_warning_enabled, fuel_level, left_turn_signal_enabled, passenger_seatbelt_warning_enabled, right_turn_signal_enabled).expect("Failed to create frame");
	ret_frames.push(CanFrame::from_frame(cluster_frame));

	let access_key_detected = rand::rng().random_bool(0.5);
	let ignition_acc = rand::rng().random_bool(0.5);
	let ignition_on = rand::rng().random_bool(0.5);
	let ignition_frame = wrx_2018::Ignition::new(access_key_detected, ignition_acc, ignition_on).expect("Failed to create frame");
	ret_frames.push(CanFrame::from_frame(ignition_frame));

	let cruise_control_enabled = rand::rng().random_bool(0.5);
	let cruise_control_set_enabled = rand::rng().random_bool(0.5);
	let cruise_control_speed = rand::rng().random_range(wrx_2018::EngineStatus2::CRUISE_CONTROL_SPEED_MIN..=wrx_2018::EngineStatus2::CRUISE_CONTROL_SPEED_MAX);
	let engine_boost_pressure = rand::rng().random_range(wrx_2018::EngineStatus2::ENGINE_BOOST_PRESSURE_MIN..=wrx_2018::EngineStatus2::ENGINE_BOOST_PRESSURE_MAX);
	let engine_coolant_temp = rand::rng().random_range(wrx_2018::EngineStatus2::ENGINE_COOLANT_TEMP_MIN..=wrx_2018::EngineStatus2::ENGINE_COOLANT_TEMP_MAX);
	let engine_fuel_flow = rand::rng().random_range(wrx_2018::EngineStatus2::ENGINE_FUEL_FLOW_MIN..=wrx_2018::EngineStatus2::ENGINE_FUEL_FLOW_MAX);
	let engine_oil_temp = rand::rng().random_range(wrx_2018::EngineStatus2::ENGINE_OIL_TEMP_MIN..=wrx_2018::EngineStatus2::ENGINE_OIL_TEMP_MAX);
	let enginestatus2_frame = wrx_2018::EngineStatus2::new(cruise_control_enabled, cruise_control_set_enabled, cruise_control_speed, engine_boost_pressure, engine_coolant_temp, engine_fuel_flow, engine_oil_temp).expect("Failed to create frame");
	ret_frames.push(CanFrame::from_frame(enginestatus2_frame));

	let check_engine_light_enabled = rand::rng().random_bool(0.5);
	let oil_pressure_warning_light_enabled = rand::rng().random_bool(0.5);
	let enginewarninglights_frame = wrx_2018::EngineWarningLights::new(check_engine_light_enabled, oil_pressure_warning_light_enabled).expect("Failed to create frame");
	ret_frames.push(CanFrame::from_frame(enginewarninglights_frame));

	let srs_warning_light_enabled = rand::rng().random_bool(0.5);
	let srsstatus_frame = wrx_2018::SrsStatus::new(srs_warning_light_enabled).expect("Failed to create frame");
	ret_frames.push(CanFrame::from_frame(srsstatus_frame));

	let fog_lights_enabled = rand::rng().random_bool(0.5);
	let tpms_warning_light_enabled = rand::rng().random_bool(0.5);
	let rear_fog_lights_enabled = rand::rng().random_bool(0.5);
	let cluster2_frame = wrx_2018::Cluster2::new(fog_lights_enabled, tpms_warning_light_enabled, rear_fog_lights_enabled).expect("Failed to create frame");
	ret_frames.push(CanFrame::from_frame(cluster2_frame));

	let dimmer_max_brightness_enabled = rand::rng().random_bool(0.5);
	let headlight_dimmer_enabled = rand::rng().random_bool(0.5);
	let left_front_door_open = rand::rng().random_bool(0.5);
	let left_rear_door_open = rand::rng().random_bool(0.5);
	let right_front_door_open = rand::rng().random_bool(0.5);
	let right_rear_door_open = rand::rng().random_bool(0.5);
	let trunk_open = rand::rng().random_bool(0.5);
	let left_front_door_locked = rand::rng().random_bool(0.5);
	let right_front_door_locked = rand::rng().random_bool(0.5);
	let cabin_frame = wrx_2018::Cabin::new(dimmer_max_brightness_enabled, headlight_dimmer_enabled, left_front_door_open, left_rear_door_open, right_front_door_open, right_rear_door_open, trunk_open, left_front_door_locked, right_front_door_locked).expect("Failed to create frame");
	ret_frames.push(CanFrame::from_frame(cabin_frame));

	let dimmer_dial_value = rand::rng().random_range(wrx_2018::DimmerAndHood::DIMMER_DIAL_VALUE_MIN..=wrx_2018::DimmerAndHood::DIMMER_DIAL_VALUE_MAX);
	let hood_open = rand::rng().random_bool(0.5);
	let dimmerandhood_frame = wrx_2018::DimmerAndHood::new(dimmer_dial_value, hood_open).expect("Failed to create frame");
	ret_frames.push(CanFrame::from_frame(dimmerandhood_frame));

	let units = rand::rng().random_bool(0.5);
	let dashstateverify_frame = wrx_2018::DashStateVerify::new(units).expect("Failed to create frame");
	ret_frames.push(CanFrame::from_frame(dashstateverify_frame));

	let odometer = rand::rng().random_range(wrx_2018::Odometer::ODOMETER_MIN..=wrx_2018::Odometer::ODOMETER_MAX);
	let odometer_frame = wrx_2018::Odometer::new(odometer).expect("Failed to create frame");
	ret_frames.push(CanFrame::from_frame(odometer_frame));

	let left_front_tire_pressure = rand::rng().random_range(wrx_2018::Tpms::LEFT_FRONT_TIRE_PRESSURE_MIN..=wrx_2018::Tpms::LEFT_FRONT_TIRE_PRESSURE_MAX);
	let left_rear_tire_pressure = rand::rng().random_range(wrx_2018::Tpms::LEFT_REAR_TIRE_PRESSURE_MIN..=wrx_2018::Tpms::LEFT_REAR_TIRE_PRESSURE_MAX);
	let right_front_tire_pressure = rand::rng().random_range(wrx_2018::Tpms::RIGHT_FRONT_TIRE_PRESSURE_MIN..=wrx_2018::Tpms::RIGHT_FRONT_TIRE_PRESSURE_MAX);
	let right_rear_tire_pressure = rand::rng().random_range(wrx_2018::Tpms::RIGHT_REAR_TIRE_PRESSURE_MIN..=wrx_2018::Tpms::RIGHT_REAR_TIRE_PRESSURE_MAX);
	let tpms_frame = wrx_2018::Tpms::new(left_front_tire_pressure, left_rear_tire_pressure, right_front_tire_pressure, right_rear_tire_pressure).expect("Failed to create frame");
	ret_frames.push(CanFrame::from_frame(tpms_frame));

	ret_frames
}