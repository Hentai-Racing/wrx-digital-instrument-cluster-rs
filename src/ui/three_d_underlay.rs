use three_d::{
    context::{
        COLOR_ATTACHMENT0, FRAMEBUFFER, LINEAR, RGBA, TEXTURE_2D, TEXTURE_MAG_FILTER,
        TEXTURE_MIN_FILTER, UNSIGNED_BYTE,
    },
    degrees, vec3, Camera, ClearState, ColorMaterial, CpuMesh, Gm, HasContext, Mesh, Positions,
    RenderTarget, Srgba,
};

pub struct ModelContainer {
    context: three_d::Context,
    model: Gm<Mesh, ColorMaterial>,
}

impl ModelContainer {
    pub fn new(context: three_d::Context) -> Self {
        let cpu_mesh = CpuMesh {
            positions: Positions::F32(vec![
                vec3(0.5, -0.5, 0.0),  // bottom right
                vec3(-0.5, -0.5, 0.0), // bottom left
                vec3(0.0, 0.5, 0.0),   // top
            ]),
            colors: Some(vec![
                Srgba::new(255, 0, 0, 255), // bottom right
                Srgba::new(0, 255, 0, 255), // bottom left
                Srgba::new(0, 0, 255, 255), // top
            ]),
            ..Default::default()
        };

        let model = Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial::default());

        Self { context, model }
    }

    pub fn render(&self, width: u32, height: u32) -> slint::Image {
        let context = &self.context;

        unsafe {
            let texture = context.create_texture().unwrap();
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
                vec3(0.0, 1.0, 0.0),
                degrees(45.0),
                0.1,
                10.0,
            );

            renderer.clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0));
            renderer.render(camera, &self.model, &[]);

            slint::BorrowedOpenGLTextureBuilder::new_gl_2d_rgba_texture(
                texture.0,
                (width as _, height as _).into(),
            )
            .build()
        }
    }
}
