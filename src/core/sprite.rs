use crate::{VIRTUAL_RESOLUTION_X, VIRTUAL_RESOLUTION_Y, WINDOW_HEIGHT, WINDOW_WIDTH};

use super::texture::Texture;
use glam::{Mat4, Vec2, Vec3};
use miniquad::*;
use std::path::Path;

/// A sprite. Represents an image on the screen.
pub struct Sprite {
    /// The position where the sprite will be located, with a center at the
    /// top-left corner of the sprite.
    pub position: Vec2,
    /// The size of the sprite. It can be scaled without keeping track of the
    /// dimensions of the loaded texture.
    pub size: Vec2,
    /// The texture the sprite will be rendering.
    texture: Box<Texture>,
    /// The dimensions of the texture. This may be different than the size, as is
    /// the original size of the texture loaded, rather than the size of the sprite.
    dimensions: (u32, u32),
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
        Self::with_params(
            ctx,
            position,
            image_path,
            TextureParams {
                filter: FilterMode::Nearest,
                ..Default::default()
            },
        )
    }

    /// Creates a sprite at a given position, with the texture wrap mode specified.
    pub fn with_params(
        ctx: &mut Context,
        position: Vec2,
        image_path: &Path,
        texture_params: TextureParams,
    ) -> Self {
        info!("Creating new sprite at [{}, {}]", position.x, position.y);
        let shader_params = shader::get_shader_params();
        let texture = Box::new(Texture::with_params(
            ctx,
            image_path,
            shader_params,
            texture_params,
        ));
        let (size_x, size_y) = texture.size();
        let size = Vec2::new(size_x as f32, size_y as f32);

        Self {
            position,
            size,
            texture,
            dimensions: (size_x, size_y),
        }
    }

    /// Scales the sprite to a given width and height.
    pub fn scale_to(&mut self, x: f32, y: f32) {
        self.size.x = x;
        self.size.y = y;
    }
}

impl EventHandler for Sprite {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        // TODO: Move this code into mvp.rs
        let (original_width, original_height) = (WINDOW_WIDTH, WINDOW_HEIGHT);

        // we need to convert the given position & scale to virtual resolution
        // so we want to see what is the proportion between the original size
        // and the size we want to emulate
        let virtual_proportion_x = (original_width / VIRTUAL_RESOLUTION_X) as f32;
        let virtual_proportion_y = (original_height / VIRTUAL_RESOLUTION_Y) as f32;

        let (current_window_width, current_window_height) = ctx.screen_size();

        let (position_x, position_y): (f32, f32) = self.position.into();
        let position_x = position_x * virtual_proportion_x;
        let position_y = position_y * virtual_proportion_y;

        // get the proportion of the original window size
        // this way when resizing the window the sprite should be placed
        // at the same position relative to the window as before
        let (position_proportion_x, position_proportion_y): (f32, f32) = (
            position_x / original_width as f32,
            position_y / original_height as f32,
        );
        let translation = Vec3::new(
            current_window_width * position_proportion_x,
            current_window_height * position_proportion_y,
            0.0,
        );
        let translation_matrix = Mat4::from_translation(translation);

        let (scale_x, scale_y): (f32, f32) = self.size.into();
        let scale_x = scale_x * virtual_proportion_x;
        let scale_y = scale_y * virtual_proportion_y;

        // same as position
        let (scale_proportion_x, scale_proportion_y) = (
            // we divide by 2 as the center is at the middle of the sprite,
            // so it will actually scale both sides
            (scale_x / 2.0) / original_width as f32,
            (scale_y / 2.0) / original_height as f32,
        );
        let scale = Vec3::new(
            current_window_width * scale_proportion_x,
            current_window_height * scale_proportion_y,
            1.0,
        );
        let scale_matrix = Mat4::from_scale(scale);

        let ortho_matrix = Mat4::orthographic_rh_gl(
            0.0,
            current_window_width,
            current_window_height,
            0.0,
            -1.0,
            1.0,
        );

        // generate the mvp matrix
        let mvp = ortho_matrix * translation_matrix * scale_matrix;

        // set the pos of the image to be in respect to the top left corner
        // otherwise it will position the middle of the image in the specified coordinates
        let offset = (1.0, 1.0);

        // if the texture repeats, scale the texture coords
        // instead of scaling the image itself
        let tex_scale = match self.texture.wrap_mode() {
            TextureWrap::Repeat => (
                current_window_width / self.dimensions.0 as f32,
                current_window_height / self.dimensions.1 as f32,
            ),
            _ => (1.0, 1.0),
        };

        let pipeline = self.texture.pipeline();
        let bindings = self.texture.bindings();

        ctx.apply_pipeline(pipeline);
        ctx.apply_bindings(bindings);
        ctx.apply_uniforms(&shader::Uniforms {
            offset,
            mvp,
            tex_scale,
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
    uniform vec2 tex_scale;

    varying lowp vec4 color;
    varying lowp vec2 texcoord;

    void main() {
        vec4 pos = vec4(pos + offset, 0, 1);
        gl_Position = mvp * pos;
        color = vec4(color0, 1.0);
        texcoord = tex0 * tex_scale;
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
                    UniformDesc::new("tex_scale", UniformType::Float2),
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
        pub tex_scale: (f32, f32),
    }
}
