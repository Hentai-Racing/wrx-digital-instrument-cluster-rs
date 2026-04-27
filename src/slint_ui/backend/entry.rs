use crate::application::settings::SETTINGS;
use crate::slint_generatedApp::{App, BevyTextureGlobal};
use crate::slint_ui::backend::bevy_adapter::slint_bevy_adapter;

use bevy::prelude::*;
use slint::{ComponentHandle, Weak};
use tokio::time::{self, Duration, Instant};

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

const MODEL_PATH: &str = "models/CarConcept.glb"; // place model files in resources/models/
const CAMERA_START_HEIGHT: f32 = 0.6;
const CAMERA_END_HEIGHT: f32 = 3.0;
const CAMERA_START_RADIUS: f32 = 4.0;
const CAMERA_END_RADIUS: f32 = 6.5;
const ORBIT_DURATION_SECS: f32 = 3.0;
const ORBIT_HOLD_SECS: f32 = 0.75;

pub fn make_app() -> Result<App, Box<dyn std::error::Error>> {
    let mut wgpu_settings = slint::wgpu_27::WGPUSettings::default();
    wgpu_settings.device_required_limits = slint::wgpu_27::wgpu::Limits::default()
        .using_resolution(slint::wgpu_27::wgpu::Limits::downlevel_defaults());
    slint::BackendSelector::new()
        .require_wgpu_27(slint::wgpu_27::WGPUConfiguration::Automatic(wgpu_settings))
        .select()?;
    let app_window = App::new().unwrap();

    let bevy_channels: Rc<
        RefCell<
            Option<(
                crossbeam::channel::Receiver<slint::wgpu_27::wgpu::Texture>,
                crossbeam::channel::Sender<slint_bevy_adapter::ControlMessage>,
            )>,
        >,
    > = Rc::new(RefCell::new(None));
    let frames: Arc<AtomicU32> = Default::default();

    let app_weak = app_window.as_weak();
    let bevy_channels_setup = bevy_channels.clone();
    let restart_animation: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let last_focused = Rc::new(Cell::new(false));

    {
        let frames = frames.clone();
        let restart_animation_setup = restart_animation.clone();
        let restart_animation_frame = restart_animation.clone();
        let last_focused = last_focused.clone();
        app_window
            .window()
            .set_rendering_notifier(move |state, graphics_api| match state {
                slint::RenderingState::RenderingSetup => {
                    let slint::GraphicsAPI::WGPU27 {
                        instance,
                        device,
                        queue,
                        ..
                    } = graphics_api
                    else {
                        eprintln!("RenderingSetup: not a WGPU27 backend, skipping");
                        return;
                    };

                    let restart_animation = restart_animation_setup.clone();
                    let channels = slint_bevy_adapter::run_bevy_app_with_slint(
                        instance.clone(),
                        device.clone(),
                        queue.clone(),
                        |_app| {},
                        move |mut app| {
                            app.insert_resource(AnimationControl {
                                restart: restart_animation.clone(),
                            })
                            .add_systems(Startup, setup)
                            .add_systems(Update, (animate_camera, monitor_scene_loading))
                            .insert_resource(ClearColor(Color::NONE))
                            .run();
                        },
                    );

                    *bevy_channels_setup.borrow_mut() = Some(channels);
                }
                slint::RenderingState::BeforeRendering => {
                    let Some(app) = app_weak.upgrade() else {
                        return;
                    };

                    frames.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    app.window().request_redraw();

                    let bevy_global = app.global::<BevyTextureGlobal>();
                    let focused = bevy_global.get_bevy_widget_focused();
                    if focused && !last_focused.get() {
                        restart_animation_frame.store(true, Ordering::Relaxed);
                    }
                    last_focused.set(focused);

                    let channels = bevy_channels_setup.borrow();
                    let Some((new_texture_receiver, control_message_sender)) = channels.as_ref()
                    else {
                        return;
                    };

                    let Ok(new_texture) = new_texture_receiver.try_recv() else {
                        return;
                    };

                    if let Some(old_texture) = bevy_global.get_texture().to_wgpu_27_texture() {
                        let _ = control_message_sender.try_send(
                            slint_bevy_adapter::ControlMessage::ReleaseFrontBufferTexture {
                                texture: old_texture,
                            },
                        );
                    }

                    let requested_width = bevy_global.get_requested_texture_width().round() as u32;
                    let requested_height =
                        bevy_global.get_requested_texture_height().round() as u32;
                    if requested_width > 0 && requested_height > 0 {
                        let _ = control_message_sender.try_send(
                            slint_bevy_adapter::ControlMessage::ResizeBuffers {
                                width: requested_width,
                                height: requested_height,
                            },
                        );
                    }

                    if let Ok(image) = new_texture.try_into() {
                        bevy_global.set_texture(image);
                    }
                }
                _ => {}
            })
            .unwrap_or_else(|e| eprintln!("set_rendering_notifier failed: {e:?}"));
    }

    {
        let frames = frames.clone();
        tokio::spawn(async move {
            let mut last = Instant::now();
            let mut interval = time::interval(Duration::from_millis(100));

            loop {
                interval.tick().await;
                let secs = last.elapsed().as_secs_f32();

                SETTINGS.developer.system_info.fps.set_value(
                    ((frames.swap(0, std::sync::atomic::Ordering::Relaxed) as f32) / secs) as i32,
                );

                last = Instant::now();
            }
        });
    }
    app_window.show()?;

    Ok(app_window)
}

#[derive(Resource, Clone)]
struct AnimationControl {
    restart: Arc<AtomicBool>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(DirectionalLight {
        illuminance: 100_000.0,
        ..default()
    });
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, CAMERA_START_HEIGHT, CAMERA_START_RADIUS)
            .looking_at(Vec3::ZERO, Vec3::Y),
        PointLight::default(),
    ));
    let handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset(MODEL_PATH));
    commands.spawn(SceneRoot(handle));
}

fn animate_camera(
    mut cameras: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
    control: Res<AnimationControl>,
    mut start_secs: Local<f32>,
) {
    if control.restart.swap(false, Ordering::Relaxed) {
        *start_secs = time.elapsed_secs();
    }
    let elapsed = (time.elapsed_secs() - *start_secs - ORBIT_HOLD_SECS).max(0.0);
    let raw = (elapsed / ORBIT_DURATION_SECS).clamp(0.0, 1.0);
    let t = raw * raw * (3.0 - 2.0 * raw);
    let theta = t * std::f32::consts::PI;
    let (sin, cos) = ops::sin_cos(theta);
    let radius = CAMERA_START_RADIUS + (CAMERA_END_RADIUS - CAMERA_START_RADIUS) * t;
    let height = CAMERA_START_HEIGHT + (CAMERA_END_HEIGHT - CAMERA_START_HEIGHT) * t;

    for mut transform in cameras.iter_mut() {
        transform.translation = Vec3::new(sin * radius, height, cos * radius);
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}

fn monitor_scene_loading(
    mut done: Local<bool>,
    asset_server: Res<AssetServer>,
    scenes: Query<&SceneRoot>,
) {
    if *done {
        return;
    }
    for scene in &scenes {
        match asset_server.get_load_state(scene.0.id()) {
            Some(bevy::asset::LoadState::Loaded) => {
                *done = true;
            }
            Some(bevy::asset::LoadState::Failed(e)) => {
                error!("Model failed to load: {e:?}");
                *done = true;
            }
            _ => {}
        }
    }
}
