/// Generated code from build.rs::generate_vcan_handler()!
use embedded_can::Frame;
use rand::Rng;
use socketcan::tokio::CanSocket;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::thread::sleep;
use crate::can::messages::wrx_2018;
pub async fn run_vcan_generator(
    socket: &mut CanSocket,
    running: Arc<AtomicBool>,
    simulating: Arc<AtomicBool>,
    delay: Duration,
) {
    while running.load(Ordering::SeqCst) {
        while simulating.load(Ordering::SeqCst) {
            let steering_angle = rand::thread_rng()
                .gen_range(
                    wrx_2018::GSensor::STEERING_ANGLE_MIN..=wrx_2018::GSensor::STEERING_ANGLE_MAX,
                );
            let g_sensor_lateral = rand::thread_rng()
                .gen_range(
                    wrx_2018::GSensor::G_SENSOR_LATERAL_MIN..=wrx_2018::GSensor::G_SENSOR_LATERAL_MAX,
                );
            let g_sensor_longitudinal = rand::thread_rng()
                .gen_range(
                    wrx_2018::GSensor::G_SENSOR_LONGITUDINAL_MIN..=wrx_2018::GSensor::G_SENSOR_LONGITUDINAL_MAX,
                );
            let gsensor_frame = wrx_2018::GSensor::new(
                    steering_angle,
                    g_sensor_lateral,
                    g_sensor_longitudinal,
                )
                .expect("Failed to create frame");
            if let Some(frame) = Frame::new(gsensor_frame.id(), gsensor_frame.data()) {
                socket.write_frame(frame).unwrap().await.unwrap();
            }
            let vehicle_speed = rand::thread_rng()
                .gen_range(
                    wrx_2018::BrakePedal::VEHICLE_SPEED_MIN..=wrx_2018::BrakePedal::VEHICLE_SPEED_MAX,
                );
            let brake_pedal_pressure = rand::thread_rng()
                .gen_range(
                    wrx_2018::BrakePedal::BRAKE_PEDAL_PRESSURE_MIN..=wrx_2018::BrakePedal::BRAKE_PEDAL_PRESSURE_MAX,
                );
            let brakepedal_frame = wrx_2018::BrakePedal::new(
                    vehicle_speed,
                    brake_pedal_pressure,
                )
                .expect("Failed to create frame");
            if let Some(frame) = Frame::new(
                brakepedal_frame.id(),
                brakepedal_frame.data(),
            ) {
                socket.write_frame(frame).unwrap().await.unwrap();
            }
            let active_tq_vectoring_enabled = rand::thread_rng().gen_bool(0.5);
            let traction_control_disabled = rand::thread_rng().gen_bool(0.5);
            let hill_assist_enabled = rand::thread_rng().gen_bool(0.5);
            let brake_cruise_on = rand::thread_rng().gen_bool(0.5);
            let driverroadassists_frame = wrx_2018::DriverRoadAssists::new(
                    active_tq_vectoring_enabled,
                    traction_control_disabled,
                    hill_assist_enabled,
                    brake_cruise_on,
                )
                .expect("Failed to create frame");
            if let Some(frame) = Frame::new(
                driverroadassists_frame.id(),
                driverroadassists_frame.data(),
            ) {
                socket.write_frame(frame).unwrap().await.unwrap();
            }
            let left_front_wheel_speed = rand::thread_rng()
                .gen_range(
                    wrx_2018::WheelSpeeds::LEFT_FRONT_WHEEL_SPEED_MIN..=wrx_2018::WheelSpeeds::LEFT_FRONT_WHEEL_SPEED_MAX,
                );
            let right_front_wheel_speed = rand::thread_rng()
                .gen_range(
                    wrx_2018::WheelSpeeds::RIGHT_FRONT_WHEEL_SPEED_MIN..=wrx_2018::WheelSpeeds::RIGHT_FRONT_WHEEL_SPEED_MAX,
                );
            let left_rear_wheel_speed = rand::thread_rng()
                .gen_range(
                    wrx_2018::WheelSpeeds::LEFT_REAR_WHEEL_SPEED_MIN..=wrx_2018::WheelSpeeds::LEFT_REAR_WHEEL_SPEED_MAX,
                );
            let right_rear_wheel_speed = rand::thread_rng()
                .gen_range(
                    wrx_2018::WheelSpeeds::RIGHT_REAR_WHEEL_SPEED_MIN..=wrx_2018::WheelSpeeds::RIGHT_REAR_WHEEL_SPEED_MAX,
                );
            let wheelspeeds_frame = wrx_2018::WheelSpeeds::new(
                    left_front_wheel_speed,
                    right_front_wheel_speed,
                    left_rear_wheel_speed,
                    right_rear_wheel_speed,
                )
                .expect("Failed to create frame");
            if let Some(frame) = Frame::new(
                wheelspeeds_frame.id(),
                wheelspeeds_frame.data(),
            ) {
                socket.write_frame(frame).unwrap().await.unwrap();
            }
            let steering_wheel_angle = rand::thread_rng()
                .gen_range(
                    wrx_2018::Steering::STEERING_WHEEL_ANGLE_MIN..=wrx_2018::Steering::STEERING_WHEEL_ANGLE_MAX,
                );
            let steering_frame = wrx_2018::Steering::new(steering_wheel_angle)
                .expect("Failed to create frame");
            if let Some(frame) = Frame::new(steering_frame.id(), steering_frame.data()) {
                socket.write_frame(frame).unwrap().await.unwrap();
            }
            let accelerator_pedal_position = rand::thread_rng()
                .gen_range(
                    wrx_2018::MotorControl::ACCELERATOR_PEDAL_POSITION_MIN..=wrx_2018::MotorControl::ACCELERATOR_PEDAL_POSITION_MAX,
                );
            let n_accelerator_pedal_max_sw = rand::thread_rng().gen_bool(0.5);
            let mt_clutch_sw = rand::thread_rng().gen_bool(0.5);
            let accelerator_cruise_position = rand::thread_rng()
                .gen_range(
                    wrx_2018::MotorControl::ACCELERATOR_CRUISE_POSITION_MIN..=wrx_2018::MotorControl::ACCELERATOR_CRUISE_POSITION_MAX,
                );
            let accelerator_combined = rand::thread_rng()
                .gen_range(
                    wrx_2018::MotorControl::ACCELERATOR_COMBINED_MIN..=wrx_2018::MotorControl::ACCELERATOR_COMBINED_MAX,
                );
            let throttle_plate_position = rand::thread_rng()
                .gen_range(
                    wrx_2018::MotorControl::THROTTLE_PLATE_POSITION_MIN..=wrx_2018::MotorControl::THROTTLE_PLATE_POSITION_MAX,
                );
            let motorcontrol_frame = wrx_2018::MotorControl::new(
                    accelerator_pedal_position,
                    n_accelerator_pedal_max_sw,
                    mt_clutch_sw,
                    accelerator_cruise_position,
                    accelerator_combined,
                    throttle_plate_position,
                )
                .expect("Failed to create frame");
            if let Some(frame) = Frame::new(
                motorcontrol_frame.id(),
                motorcontrol_frame.data(),
            ) {
                socket.write_frame(frame).unwrap().await.unwrap();
            }
            let engine_torque = rand::thread_rng()
                .gen_range(
                    wrx_2018::Engine::ENGINE_TORQUE_MIN..=wrx_2018::Engine::ENGINE_TORQUE_MAX,
                );
            let engine_stop = rand::thread_rng().gen_bool(0.5);
            let engine_rpm = rand::thread_rng()
                .gen_range(
                    wrx_2018::Engine::ENGINE_RPM_MIN..=wrx_2018::Engine::ENGINE_RPM_MAX,
                );
            let mt_gear = rand::thread_rng()
                .gen_range(
                    wrx_2018::Engine::MT_GEAR_MIN..=wrx_2018::Engine::MT_GEAR_MAX,
                );
            let engine_running = rand::thread_rng().gen_bool(0.5);
            let engine_frame = wrx_2018::Engine::new(
                    engine_torque,
                    engine_stop,
                    engine_rpm,
                    mt_gear,
                    engine_running,
                )
                .expect("Failed to create frame");
            if let Some(frame) = Frame::new(engine_frame.id(), engine_frame.data()) {
                socket.write_frame(frame).unwrap().await.unwrap();
            }
            let mt_gear_verify = rand::thread_rng()
                .gen_range(
                    wrx_2018::Transmission::MT_GEAR_VERIFY_MIN..=wrx_2018::Transmission::MT_GEAR_VERIFY_MAX,
                );
            let transmission_frame = wrx_2018::Transmission::new(mt_gear_verify)
                .expect("Failed to create frame");
            if let Some(frame) = Frame::new(
                transmission_frame.id(),
                transmission_frame.data(),
            ) {
                socket.write_frame(frame).unwrap().await.unwrap();
            }
            let acc_power = rand::thread_rng().gen_bool(0.5);
            let key_on = rand::thread_rng().gen_bool(0.5);
            let reverse_sw = rand::thread_rng().gen_bool(0.5);
            let handbrake_sw = rand::thread_rng().gen_bool(0.5);
            let brake_sw = rand::thread_rng().gen_bool(0.5);
            let running_lights_enabled = rand::thread_rng().gen_bool(0.5);
            let parking_lights_enabled = rand::thread_rng().gen_bool(0.5);
            let lowbeams_enabled = rand::thread_rng().gen_bool(0.5);
            let highbeams_enabled = rand::thread_rng().gen_bool(0.5);
            let wiper_moving_sw = rand::thread_rng().gen_bool(0.5);
            let statusswitches_frame = wrx_2018::StatusSwitches::new(
                    acc_power,
                    key_on,
                    reverse_sw,
                    handbrake_sw,
                    brake_sw,
                    running_lights_enabled,
                    parking_lights_enabled,
                    lowbeams_enabled,
                    highbeams_enabled,
                    wiper_moving_sw,
                )
                .expect("Failed to create frame");
            if let Some(frame) = Frame::new(
                statusswitches_frame.id(),
                statusswitches_frame.data(),
            ) {
                socket.write_frame(frame).unwrap().await.unwrap();
            }
            let any_door_open = rand::thread_rng().gen_bool(0.5);
            let xxxmsg340_frame = wrx_2018::XxxMsg340::new(any_door_open)
                .expect("Failed to create frame");
            if let Some(frame) = Frame::new(
                xxxmsg340_frame.id(),
                xxxmsg340_frame.data(),
            ) {
                socket.write_frame(frame).unwrap().await.unwrap();
            }
            let rcta_enabled = rand::thread_rng().gen_bool(0.5);
            let rcta_right_adjacent = rand::thread_rng().gen_bool(0.5);
            let rcta_left_adjacent = rand::thread_rng().gen_bool(0.5);
            let rcta_right_approaching = rand::thread_rng().gen_bool(0.5);
            let rcta_left_approaching = rand::thread_rng().gen_bool(0.5);
            let bsd_right = rand::thread_rng().gen_bool(0.5);
            let bsd_left = rand::thread_rng().gen_bool(0.5);
            let bsdrcta_frame = wrx_2018::BsdRcta::new(
                    rcta_enabled,
                    rcta_right_adjacent,
                    rcta_left_adjacent,
                    rcta_right_approaching,
                    rcta_left_approaching,
                    bsd_right,
                    bsd_left,
                )
                .expect("Failed to create frame");
            if let Some(frame) = Frame::new(bsdrcta_frame.id(), bsdrcta_frame.data()) {
                socket.write_frame(frame).unwrap().await.unwrap();
            }
            let fuel_level = rand::thread_rng()
                .gen_range(
                    wrx_2018::Cluster::FUEL_LEVEL_MIN..=wrx_2018::Cluster::FUEL_LEVEL_MAX,
                );
            let raw_fuel_testing = rand::thread_rng()
                .gen_range(
                    wrx_2018::Cluster::RAW_FUEL_TESTING_MIN..=wrx_2018::Cluster::RAW_FUEL_TESTING_MAX,
                );
            let driver_seatbelt_warning_enabled = rand::thread_rng().gen_bool(0.5);
            let passenger_seatbelt_warning_enabled = rand::thread_rng().gen_bool(0.5);
            let left_turn_signal_enabled = rand::thread_rng().gen_bool(0.5);
            let right_turn_signal_enabled = rand::thread_rng().gen_bool(0.5);
            let cluster_frame = wrx_2018::Cluster::new(
                    fuel_level,
                    raw_fuel_testing,
                    driver_seatbelt_warning_enabled,
                    passenger_seatbelt_warning_enabled,
                    left_turn_signal_enabled,
                    right_turn_signal_enabled,
                )
                .expect("Failed to create frame");
            if let Some(frame) = Frame::new(cluster_frame.id(), cluster_frame.data()) {
                socket.write_frame(frame).unwrap().await.unwrap();
            }
            let ignition_on = rand::thread_rng().gen_bool(0.5);
            let ignition_acc = rand::thread_rng().gen_bool(0.5);
            let access_key_detected = rand::thread_rng().gen_bool(0.5);
            let ignition_frame = wrx_2018::Ignition::new(
                    ignition_on,
                    ignition_acc,
                    access_key_detected,
                )
                .expect("Failed to create frame");
            if let Some(frame) = Frame::new(ignition_frame.id(), ignition_frame.data()) {
                socket.write_frame(frame).unwrap().await.unwrap();
            }
            let engine_fuel_flow = rand::thread_rng()
                .gen_range(
                    wrx_2018::EngineStatus2::ENGINE_FUEL_FLOW_MIN..=wrx_2018::EngineStatus2::ENGINE_FUEL_FLOW_MAX,
                );
            let engine_oil_temp = rand::thread_rng()
                .gen_range(
                    wrx_2018::EngineStatus2::ENGINE_OIL_TEMP_MIN..=wrx_2018::EngineStatus2::ENGINE_OIL_TEMP_MAX,
                );
            let engine_coolant_temp = rand::thread_rng()
                .gen_range(
                    wrx_2018::EngineStatus2::ENGINE_COOLANT_TEMP_MIN..=wrx_2018::EngineStatus2::ENGINE_COOLANT_TEMP_MAX,
                );
            let engine_boost_pressure = rand::thread_rng()
                .gen_range(
                    wrx_2018::EngineStatus2::ENGINE_BOOST_PRESSURE_MIN..=wrx_2018::EngineStatus2::ENGINE_BOOST_PRESSURE_MAX,
                );
            let cruise_control_enabled = rand::thread_rng().gen_bool(0.5);
            let cruise_control_set_enabled = rand::thread_rng().gen_bool(0.5);
            let cruise_control_speed = rand::thread_rng()
                .gen_range(
                    wrx_2018::EngineStatus2::CRUISE_CONTROL_SPEED_MIN..=wrx_2018::EngineStatus2::CRUISE_CONTROL_SPEED_MAX,
                );
            let enginestatus2_frame = wrx_2018::EngineStatus2::new(
                    engine_fuel_flow,
                    engine_oil_temp,
                    engine_coolant_temp,
                    engine_boost_pressure,
                    cruise_control_enabled,
                    cruise_control_set_enabled,
                    cruise_control_speed,
                )
                .expect("Failed to create frame");
            if let Some(frame) = Frame::new(
                enginestatus2_frame.id(),
                enginestatus2_frame.data(),
            ) {
                socket.write_frame(frame).unwrap().await.unwrap();
            }
            let oil_pressure_warning_light_enabled = rand::thread_rng().gen_bool(0.5);
            let check_engine_light_enabled = rand::thread_rng().gen_bool(0.5);
            let enginewarninglights_frame = wrx_2018::EngineWarningLights::new(
                    oil_pressure_warning_light_enabled,
                    check_engine_light_enabled,
                )
                .expect("Failed to create frame");
            if let Some(frame) = Frame::new(
                enginewarninglights_frame.id(),
                enginewarninglights_frame.data(),
            ) {
                socket.write_frame(frame).unwrap().await.unwrap();
            }
            let srs_warning_light_enabled = rand::thread_rng().gen_bool(0.5);
            let srsstatus_frame = wrx_2018::SrsStatus::new(srs_warning_light_enabled)
                .expect("Failed to create frame");
            if let Some(frame) = Frame::new(
                srsstatus_frame.id(),
                srsstatus_frame.data(),
            ) {
                socket.write_frame(frame).unwrap().await.unwrap();
            }
            let fog_lights_enabled = rand::thread_rng().gen_bool(0.5);
            let tpms_warning_light_enabled = rand::thread_rng().gen_bool(0.5);
            let cluster2_frame = wrx_2018::Cluster2::new(
                    fog_lights_enabled,
                    tpms_warning_light_enabled,
                )
                .expect("Failed to create frame");
            if let Some(frame) = Frame::new(cluster2_frame.id(), cluster2_frame.data()) {
                socket.write_frame(frame).unwrap().await.unwrap();
            }
            let left_front_door_open = rand::thread_rng().gen_bool(0.5);
            let right_front_door_open = rand::thread_rng().gen_bool(0.5);
            let right_rear_door_open = rand::thread_rng().gen_bool(0.5);
            let left_rear_door_open = rand::thread_rng().gen_bool(0.5);
            let trunk_open = rand::thread_rng().gen_bool(0.5);
            let headlight_dimmer_enabled = rand::thread_rng().gen_bool(0.5);
            let dimmer_max_brightness_enabled = rand::thread_rng().gen_bool(0.5);
            let cabin_frame = wrx_2018::Cabin::new(
                    left_front_door_open,
                    right_front_door_open,
                    right_rear_door_open,
                    left_rear_door_open,
                    trunk_open,
                    headlight_dimmer_enabled,
                    dimmer_max_brightness_enabled,
                )
                .expect("Failed to create frame");
            if let Some(frame) = Frame::new(cabin_frame.id(), cabin_frame.data()) {
                socket.write_frame(frame).unwrap().await.unwrap();
            }
            let dimmer_dial_value = rand::thread_rng()
                .gen_range(
                    wrx_2018::DimmerAndHood::DIMMER_DIAL_VALUE_MIN..=wrx_2018::DimmerAndHood::DIMMER_DIAL_VALUE_MAX,
                );
            let hood_closed = rand::thread_rng().gen_bool(0.5);
            let dimmerandhood_frame = wrx_2018::DimmerAndHood::new(
                    dimmer_dial_value,
                    hood_closed,
                )
                .expect("Failed to create frame");
            if let Some(frame) = Frame::new(
                dimmerandhood_frame.id(),
                dimmerandhood_frame.data(),
            ) {
                socket.write_frame(frame).unwrap().await.unwrap();
            }
            let units = rand::thread_rng().gen_bool(0.5);
            let dashstate2verify_frame = wrx_2018::DashState2Verify::new(units)
                .expect("Failed to create frame");
            if let Some(frame) = Frame::new(
                dashstate2verify_frame.id(),
                dashstate2verify_frame.data(),
            ) {
                socket.write_frame(frame).unwrap().await.unwrap();
            }
            let odometer = rand::thread_rng()
                .gen_range(
                    wrx_2018::Odometer::ODOMETER_MIN..=wrx_2018::Odometer::ODOMETER_MAX,
                );
            let odometer_frame = wrx_2018::Odometer::new(odometer)
                .expect("Failed to create frame");
            if let Some(frame) = Frame::new(odometer_frame.id(), odometer_frame.data()) {
                socket.write_frame(frame).unwrap().await.unwrap();
            }
            let left_front_tire_pressure = rand::thread_rng()
                .gen_range(
                    wrx_2018::Tpms::LEFT_FRONT_TIRE_PRESSURE_MIN..=wrx_2018::Tpms::LEFT_FRONT_TIRE_PRESSURE_MAX,
                );
            let right_front_tire_pressure = rand::thread_rng()
                .gen_range(
                    wrx_2018::Tpms::RIGHT_FRONT_TIRE_PRESSURE_MIN..=wrx_2018::Tpms::RIGHT_FRONT_TIRE_PRESSURE_MAX,
                );
            let right_rear_tire_pressure = rand::thread_rng()
                .gen_range(
                    wrx_2018::Tpms::RIGHT_REAR_TIRE_PRESSURE_MIN..=wrx_2018::Tpms::RIGHT_REAR_TIRE_PRESSURE_MAX,
                );
            let left_rear_tire_pressure = rand::thread_rng()
                .gen_range(
                    wrx_2018::Tpms::LEFT_REAR_TIRE_PRESSURE_MIN..=wrx_2018::Tpms::LEFT_REAR_TIRE_PRESSURE_MAX,
                );
            let tpms_frame = wrx_2018::Tpms::new(
                    left_front_tire_pressure,
                    right_front_tire_pressure,
                    right_rear_tire_pressure,
                    left_rear_tire_pressure,
                )
                .expect("Failed to create frame");
            if let Some(frame) = Frame::new(tpms_frame.id(), tpms_frame.data()) {
                socket.write_frame(frame).unwrap().await.unwrap();
            }
            sleep(delay);
        }
    }
}
