use std::time::Instant;
use three_d::{
    context::{
        COLOR_ATTACHMENT0, FRAMEBUFFER, LINEAR, RGBA, TEXTURE_2D, TEXTURE_MAG_FILTER,
        TEXTURE_MIN_FILTER, UNSIGNED_BYTE,
    },
    degrees, radians, vec3, Camera, ClearState, ColorMaterial, CpuMesh, Geometry, Gm, HasContext,
    Mat4, Mesh, Positions, RenderTarget, Srgba,
};

pub struct ModelContainer {
    context: three_d::Context,
    model: Gm<Mesh, ColorMaterial>,
    time: Instant,
}

impl ModelContainer {
    pub fn new(context: three_d::Context) -> Self {
        let size = 0.25;

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
            model,
            time,
        }
    }

    pub fn render(&mut self, width: u32, height: u32) -> slint::Image {
        let context = &self.context;

        unsafe {
            let texture = context.create_texture().unwrap();
            context.bind_texture(TEXTURE_2D, Some(texture));
            context.tex_image_2d(
                TEXTURE_2D,
                0,
                RGBA as _, // !Required in order for Slint to make an image
                width as _,
                height as _,
                0,
                RGBA,
                UNSIGNED_BYTE,
                three_d::context::PixelUnpackData::Slice(None),
            );
            context.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as _);
            context.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as _);

            let framebuffer = context.create_framebuffer().unwrap();
            context.bind_framebuffer(FRAMEBUFFER, Some(framebuffer));
            context.framebuffer_texture_2d(
                FRAMEBUFFER,
                COLOR_ATTACHMENT0,
                TEXTURE_2D,
                Some(texture),
                0,
            );

            let renderer = RenderTarget::from_framebuffer(context, width, height, framebuffer);

            let camera = Camera::new_perspective(
                renderer.viewport(),
                vec3(0.0, 0.0, 2.0),
                vec3(0.0, 0.0, 0.0),
                vec3(0.0, -1.0, 0.0), // negative because it is flipped for some reason
                degrees(45.0),
                0.1,
                10.0,
            );

            self.model
                .animate(self.time.elapsed().as_millis() as f32 * 0.5);

            renderer.clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 0.0, 1.0));
            renderer.render(camera, &self.model, &[]);

            slint::BorrowedOpenGLTextureBuilder::new_gl_2d_rgba_texture(
                texture.0,
                (width as _, height as _).into(),
            )
            .build()
        }
    }
}
