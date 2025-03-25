use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Instant;
use three_d::{
    context::{
        NativeFramebuffer, NativeTexture, COLOR_ATTACHMENT0, DRAW_FRAMEBUFFER,
        DRAW_FRAMEBUFFER_BINDING, FRAMEBUFFER, LINEAR, RGBA, TEXTURE_2D, TEXTURE_BINDING_2D,
        TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, UNSIGNED_BYTE,
    },
    degrees, radians, vec3, Camera, ClearState, ColorMaterial, CpuMesh, Geometry, Gm, HasContext,
    Mat4, Mesh, Positions, Program, RenderTarget, ScissorBox, Srgba, Vector3, VertexBuffer,
    Viewport,
};

// Taken from https://github.com/slint-ui/slint/blob/029857291bf0c95aa09f5c7a82b5246ac27d1b4b/examples/opengl_texture/main.rs#L11
macro_rules! define_scoped_binding {
    (struct $binding_ty_name:ident => $obj_name:path, $param_name:path, $binding_fn:ident, $target_name:path) => {
        struct $binding_ty_name {
            saved_value: Option<$obj_name>,
            gl: Rc<three_d::Context>,
        }

        impl $binding_ty_name {
            unsafe fn new(gl: &Rc<three_d::Context>, new_binding: Option<$obj_name>) -> Self {
                let saved_value =
                    NonZeroU32::new(gl.get_parameter_i32($param_name) as u32).map($obj_name);

                gl.$binding_fn($target_name, new_binding);
                Self {
                    saved_value,
                    gl: gl.clone(),
                }
            }
        }

        impl Drop for $binding_ty_name {
            fn drop(&mut self) {
                unsafe {
                    self.gl.$binding_fn($target_name, self.saved_value);
                }
            }
        }
    };
    (struct $binding_ty_name:ident => $obj_name:path, $param_name:path, $binding_fn:ident) => {
        struct $binding_ty_name {
            saved_value: Option<$obj_name>,
            gl: Rc<three_d::Context>,
        }

        impl $binding_ty_name {
            unsafe fn new(gl: &Rc<three_d::Context>, new_binding: Option<$obj_name>) -> Self {
                let saved_value =
                    NonZeroU32::new(gl.get_parameter_i32($param_name) as u32).map($obj_name);

                gl.$binding_fn(new_binding);
                Self {
                    saved_value,
                    gl: gl.clone(),
                }
            }
        }

        impl Drop for $binding_ty_name {
            fn drop(&mut self) {
                unsafe {
                    self.gl.$binding_fn(self.saved_value);
                }
            }
        }
    };
}

const SIZE: f32 = 0.25;

define_scoped_binding!(struct ScopedTextureBinding => NativeTexture, TEXTURE_BINDING_2D, bind_texture, TEXTURE_2D);
define_scoped_binding!(struct ScopedFramebufferBinding => NativeFramebuffer, DRAW_FRAMEBUFFER_BINDING, bind_framebuffer, DRAW_FRAMEBUFFER);

pub struct ModelTexture {
    texture: three_d::context::Texture,
    framebuffer: three_d::context::Framebuffer,

    width: u32,
    height: u32,

    context: Rc<three_d::Context>,
}

impl ModelTexture {
    pub fn new(context: &Rc<three_d::Context>, width: u32, height: u32) -> Self {
        unsafe {
            let framebuffer = context
                .create_framebuffer()
                .expect("Failed to create framebuffer");
            let texture = context.create_texture().expect("Failed to create texture");

            let _scoped_texture_binding = ScopedTextureBinding::new(&context, Some(texture));

            let old_unpack_alignment =
                context.get_parameter_i32(three_d::context::UNPACK_ALIGNMENT);
            let old_unpack_row_length =
                context.get_parameter_i32(three_d::context::UNPACK_ROW_LENGTH);
            let old_unpack_skip_pixels =
                context.get_parameter_i32(three_d::context::UNPACK_SKIP_PIXELS);
            let old_unpack_skip_rows =
                context.get_parameter_i32(three_d::context::UNPACK_SKIP_ROWS);

            context.pixel_store_i32(three_d::context::UNPACK_ALIGNMENT, 1);
            context.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as _);
            context.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as _);
            context.pixel_store_i32(three_d::context::UNPACK_ROW_LENGTH, width as i32);
            context.pixel_store_i32(three_d::context::UNPACK_SKIP_PIXELS, 0);
            context.pixel_store_i32(three_d::context::UNPACK_SKIP_ROWS, 0);

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

            let _saved_framebuffer_binding =
                ScopedFramebufferBinding::new(&context, Some(framebuffer));

            context.framebuffer_texture_2d(
                FRAMEBUFFER,
                COLOR_ATTACHMENT0,
                TEXTURE_2D,
                Some(texture),
                0,
            );

            context.pixel_store_i32(three_d::context::UNPACK_ALIGNMENT, old_unpack_alignment);
            context.pixel_store_i32(three_d::context::UNPACK_ROW_LENGTH, old_unpack_row_length);
            context.pixel_store_i32(three_d::context::UNPACK_SKIP_PIXELS, old_unpack_skip_pixels);
            context.pixel_store_i32(three_d::context::UNPACK_SKIP_ROWS, old_unpack_skip_rows);

            Self {
                texture,
                framebuffer,

                width,
                height,

                context: context.clone(),
            }
        }
    }

    unsafe fn with_texture_as_active_framebuffer<R>(&self, callback: impl FnOnce() -> R) -> R {
        let _saved_fbo = ScopedFramebufferBinding::new(&self.context, Some(self.framebuffer));
        callback()
    }
}

impl Drop for ModelTexture {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_framebuffer(self.framebuffer);
            self.context.delete_texture(self.texture);
        }
    }
}

pub struct ModelContainer {
    context: Rc<three_d::Context>,

    camera: Camera,
    model: Gm<Mesh, ColorMaterial>,
    start_time: Instant,

    displayed_texture: ModelTexture,
    next_texture: ModelTexture,
}

impl ModelContainer {
    pub fn new(context: three_d::Context, x: u32, y: u32, width: u32, height: u32) -> Self {
        let context = Rc::new(context);

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
                vec3(SIZE, -SIZE, 0.0),  // bottom right
                vec3(-SIZE, -SIZE, 0.0), // bottom left
                vec3(0.0, SIZE, 0.0),    // top
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

        let start_time = Instant::now();

        let displayed_texture = ModelTexture::new(&context, width, height);
        let next_texture = ModelTexture::new(&context, width, height);

        Self {
            context,
            camera,
            model,
            start_time,
            displayed_texture,
            next_texture,
        }
    }

    pub fn render(&mut self, x: i32, y: i32, width: u32, height: u32) -> slint::Image {
        let context = &self.context;

        unsafe {
            if self.next_texture.width != width || self.next_texture.height != height {
                let mut new_texture = ModelTexture::new(context, width, height);
                std::mem::swap(&mut self.next_texture, &mut new_texture);
            }

            self.next_texture.with_texture_as_active_framebuffer(|| {
                self.model
                    .animate(self.start_time.elapsed().as_millis() as f32 * 0.5);

                let renderer = RenderTarget::from_framebuffer(
                    context,
                    width,
                    height,
                    self.next_texture.framebuffer,
                );

                let scissor = ScissorBox {
                    x: 0,
                    y: 0,
                    width,
                    height,
                };

                self.camera.set_viewport(renderer.viewport());

                renderer.clear_partially(
                    scissor,
                    ClearState::color_and_depth(0.0, 0.0, 0.0, 0.0, 1.0),
                );

                // renderer.render_partially(scissor, &self.camera, &self.model, &[]);
            });

            let result_texture = slint::BorrowedOpenGLTextureBuilder::new_gl_2d_rgba_texture(
                self.next_texture.texture.0,
                (width as _, height as _).into(),
            )
            .build();

            std::mem::swap(&mut self.next_texture, &mut self.displayed_texture);

            result_texture
        }
    }
}
