use super::texture::Texture;
use glam::{Mat4, Vec2, Vec3};
use miniquad::*;
use std::path::Path;

pub struct Sprite {
    pub position: Vec2,
    pub size: Vec2,
    pub texture: Texture,
}

impl Sprite {
    /// Creates a new sprite at a given position. `image_path` is the path
    /// where the image we want to load is located.
    ///
    /// # Example
    /// ```rust
    /// use std::path::Path;
    /// use glam::Vec2;
    ///
    /// let image_path = Path::new("sprite.png");
    /// let position = Vec2::new(500.0, 500.0);
    /// let sprite = Sprite::new(position, image_path);
    /// ```
    pub fn new(ctx: &mut Context, position: Vec2, image_path: &Path) -> Self {
        let shader_params = shader::get_shader_params();
        let texture = Texture::from_path(ctx, image_path, shader_params);
        let (size_x, size_y) = texture.size;
        let size = Vec2::new(size_x as f32, size_y as f32);

        Self {
            position,
            size,
            texture,
        }
    }
}

impl EventHandler for Sprite {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        let (window_width, window_height) = ctx.screen_size();

        let (position_x, position_y): (f32, f32) = self.position.into();
        let translation = Vec3::new(position_x, position_y, 0.0);
        let translation_matrix = Mat4::from_translation(translation);

        let (scale_x, scale_y): (f32, f32) = self.size.into();
        let scale = Vec3::new(scale_x, scale_y, 1.0);
        let scale_matrix = Mat4::from_scale(scale);

        let ortho_matrix =
            Mat4::orthographic_rh_gl(0.0, window_width, window_height, 0.0, -1.0, 1.0);

        let mvp = ortho_matrix * translation_matrix * scale_matrix;

        let pipeline = self.texture.pipeline();
        let bindings = self.texture.bindings();

        ctx.apply_pipeline(pipeline);
        ctx.apply_bindings(bindings);
        ctx.apply_uniforms(&shader::Uniforms {
            offset: (0.0, 0.0),
            mvp,
        });

        ctx.draw(0, 6, 1);
    }
}

mod shader {
    use miniquad::*;

    use crate::core::graphics_handler::ShaderParams;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;
    attribute vec3 color0;
    attribute vec2 tex0;

    uniform vec2 offset;
    uniform mat4 mvp;

    varying lowp vec4 color;
    varying lowp vec2 texcoord;

    void main() {
        vec4 pos = vec4(pos + offset, 0, 1);
        gl_Position = mvp * pos;
        color = vec4(color0, 1.0);
        texcoord = tex0;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;
    varying lowp vec4 color;

    uniform sampler2D tex;

    void main() {
        gl_FragColor = texture2D(tex, texcoord);
    }
    "#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("offset", UniformType::Float2),
                    UniformDesc::new("mvp", UniformType::Mat4),
                ],
            },
        }
    }

    pub fn get_shader_params() -> ShaderParams {
        ShaderParams {
            vertex_shader: VERTEX,
            fragment_shader: FRAGMENT,
            meta: meta(),
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub offset: (f32, f32),
        pub mvp: glam::Mat4,
    }
}
