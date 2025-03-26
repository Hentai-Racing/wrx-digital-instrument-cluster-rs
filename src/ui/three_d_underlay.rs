use std::time::Instant;
use three_d::{
    context::{
        NativeFramebuffer, COLOR_ATTACHMENT0, FRAMEBUFFER, LINEAR, RGBA, TEXTURE_2D,
        TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, UNSIGNED_BYTE,
    },
    degrees, radians, vec3, Camera, ClearState, ColorMaterial, CpuMesh, Geometry, Gm, HasContext,
    Mat4, Mesh, Positions, RenderTarget, ScissorBox, Srgba, Viewport,
};

pub struct ModelContainer {
    context: three_d::Context,
    camera: Camera,
    model: Gm<Mesh, ColorMaterial>,
    time: Instant,

    framebuffer: Option<NativeFramebuffer>,
}

impl ModelContainer {
    pub fn new(context: three_d::Context) -> Self {
        let size = 0.25;

        let framebuffer = unsafe { context.create_framebuffer().ok() };

        let camera = Camera::new_perspective(
            Viewport::new_at_origo(1, 1),
            vec3(0.0, 0.0, 2.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, -1.0, 0.0), // negative because it is flipped for some reason
            degrees(45.0),
            0.1,
            10.0,
        );

        let cpu_mesh = CpuMesh {
            positions: Positions::F32(vec![
                vec3(size, -size, 0.0),  // bottom right
                vec3(-size, -size, 0.0), // bottom left
                vec3(0.0, size, 0.0),    // top
            ]),
            colors: Some(vec![
                Srgba::new(255, 0, 0, 255), // bottom right
                Srgba::new(0, 255, 0, 255), // bottom left
                Srgba::new(0, 0, 255, 255), // top
            ]),
            ..Default::default()
        };

        let mut model = Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial::default());

        model.set_animation(|time| Mat4::from_angle_y(radians(time * 0.005)));

        let time = Instant::now();

        Self {
            context,
            camera,
            model,
            time,

            framebuffer,
        }
    }

    pub fn render(
        &mut self,
        width: u32,
        height: u32,
        window_width: u32,
        window_height: u32,
    ) -> slint::Image {
        let context = &self.context;

        unsafe {
            if let (Some(texture), Some(framebuffer)) =
                (context.create_texture().ok(), self.framebuffer)
            {
                context.bind_texture(TEXTURE_2D, Some(texture));
                context.tex_image_2d(
                    TEXTURE_2D,
                    0,
                    RGBA as _,
                    width as _,
                    height as _,
                    0,
                    RGBA,
                    UNSIGNED_BYTE,
                    three_d::context::PixelUnpackData::Slice(None),
                );
                context.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as _);
                context.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as _);

                context.bind_framebuffer(FRAMEBUFFER, self.framebuffer);
                context.framebuffer_texture_2d(
                    FRAMEBUFFER,
                    COLOR_ATTACHMENT0,
                    TEXTURE_2D,
                    Some(texture),
                    0,
                );

                let renderer = RenderTarget::from_framebuffer(context, width, height, framebuffer);
                let scissor_box = ScissorBox {
                    x: 0,
                    y: 0,
                    width: window_width,
                    height: window_height,
                };

                if self.camera.viewport() != renderer.viewport() {
                    self.camera.set_viewport(renderer.viewport());
                }

                self.model
                    .animate(self.time.elapsed().as_millis() as f32 * 0.5);

                renderer.clear_partially(
                    scissor_box,
                    ClearState::color_and_depth(0.0, 0.0, 0.0, 0.0, 1.0),
                );

                renderer.render_partially(scissor_box, &self.camera, &self.model, &[]);

                slint::BorrowedOpenGLTextureBuilder::new_gl_2d_rgba_texture(
                    texture.0,
                    (width as _, height as _).into(),
                )
                .build()
            } else {
                slint::Image::default()
            }
        }
    }
}
