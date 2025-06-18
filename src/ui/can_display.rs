use crate::{App, CanDisplay, SCanFrameDisplay};
use embedded_can::{Frame, Id};
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel, Weak};

pub struct CanFrameDisplay {
    ui: Weak<App>,
}

impl CanFrameDisplay {
    // TODO: add bit view
    // TODO: implement byte changes
    pub fn new(ui: Weak<App>) -> Self {
        Self { ui }
    }

    pub fn update(&mut self, frame: &impl Frame, force: bool) {
        let frame_data = frame.data().to_owned();
        let frame_dlc = frame.dlc().to_owned();
        let frame_id = frame.id().to_owned();
        let raw_id = match &frame_id {
            Id::Extended(id) => id.as_raw() as i32,
            Id::Standard(id) => id.as_raw() as i32,
        };

        let _ = self.ui.upgrade_in_event_loop(move |ui| {
            let can_display = ui.global::<CanDisplay>();

            if !(can_display.get_running() || force) {
                return;
            }

            let display_frames = can_display.get_CanFrames();
            if let Some(frames) = display_frames
                .as_any()
                .downcast_ref::<VecModel<SCanFrameDisplay>>()
            {
                let mut is_new = false;
                let frame = frames
                    .iter()
                    .find_map(|frame| {
                        if frame.id == raw_id {
                            Some(frame)
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| {
                        is_new = true;

                        let formatted_frame_id = match frame_id {
                            Id::Extended(id) => {
                                let left = id.as_raw() >> 16;
                                let right = id.as_raw() | 0xFFFF;

                                format!("{left:04X}_{right:04X}")
                            }
                            Id::Standard(id) => format!("{:03X}", id.as_raw()),
                        };

                        frames.push(SCanFrameDisplay {
                            dlc: frame_dlc as i32,
                            formatted_id: formatted_frame_id.into(),
                            id: raw_id,
                            data: ModelRc::new(VecModel::default()),
                            bit_display: ModelRc::new(VecModel::from(vec![false; frame_dlc])),
                            ..Default::default()
                        });

                        frames.iter().last().unwrap() //* guaranteed unwrap
                    });

                if let (Some(data), Some(bit_display)) = (
                    frame.data.as_any().downcast_ref::<VecModel<SharedString>>(),
                    frame.bit_display.as_any().downcast_ref::<VecModel<bool>>(),
                ) {
                    let format_data = frame_data.iter().enumerate().map(|(i, byte)| {
                        match bit_display.row_data(i) {
                            Some(true) => format!("{byte:08b}").into(),
                            _ => format!("{byte:02X}").into(),
                        }
                    });

                    data.set_vec(Vec::from_iter(format_data));
                }
            }

            can_display.set_CanFrames(display_frames);
        });
    }
}
