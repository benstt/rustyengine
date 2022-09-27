use super::graphics_handler::{self, GraphicsHandler, ShaderParams};
use super::vertex::Vertex;
use glam::{Mat4, Vec2, Vec3};
use miniquad::*;

const NUMBER_OF_SIDES_IN_CIRCLE: usize = 20;

pub enum ShapeType {
    SQUARE,
    RECTANGLE,
    TRIANGLE,
}

pub enum ShapeCenterPosition {
    MIDDLE,
    TOPLEFT,
}

/// Represents the shape settings that can be applied.
#[repr(C)]
pub struct ShapeParams {
    /// Represents the center position of the shape. Can be either at the middle or top-left.
    pub center: ShapeCenterPosition,
}

impl Default for ShapeParams {
    fn default() -> Self {
        Self {
            center: ShapeCenterPosition::TOPLEFT,
        }
    }
}

#[repr(C)]
pub struct Shape {
    pub shape_type: ShapeType,
    pub position: Vec2,
    pub size: Vec2,
    pub params: Option<ShapeParams>,
    graphics_handler: GraphicsHandler,
}

impl Shape {
    pub fn new<T>(ctx: &mut Context, shape_type: ShapeType, position: Vec2, size: T) -> Self
    where
        // we want to either pass a f32 or a vector
        T: Into<f32> + Into<Vec2>,
    {
        match &shape_type {
            ShapeType::SQUARE => todo!(),
            ShapeType::RECTANGLE => todo!(),
            ShapeType::TRIANGLE => todo!(),
        }
    }

    /// Creates a square with the position and size given.
    pub fn new_square(ctx: &mut Context, position: Vec2, size: f32) -> Self {
        let size = Vec2::new(size, size);
        Self::new_rectangle(ctx, position, size)
    }

    /// Creates a rectangle with the position and size given.
    pub fn new_rectangle(ctx: &mut Context, position: Vec2, size: Vec2) -> Self {
        let vertices: [Vertex; 4] = [
            Vertex::new(-1.0, 1.0),
            Vertex::new(1.0, 1.0),
            Vertex::new(1.0, -1.0),
            Vertex::new(-1.0, -1.0),
        ];

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let shader_params = shader::get_shader_params();
        let graphics_handler = GraphicsHandler::new(ctx, &vertices, &indices, shader_params);

        Self {
            position,
            size,
            params: None,
            graphics_handler,
            shape_type: ShapeType::SQUARE,
        }
    }

    /// Creates a triangle with the position and size given.  
    pub fn new_triangle(ctx: &mut Context, position: Vec2, size: Vec2) -> Self {
        let vertices: [Vertex; 3] = [
            Vertex::new(-1.0, -1.0),
            Vertex::new(0., 1.0),
            Vertex::new(1.0, -1.0),
        ];
        let indices: [u16; 3] = [0, 1, 2];
        let shader_params = shader::get_shader_params();
        let graphics_handler = GraphicsHandler::new(ctx, &vertices, &indices, shader_params);

        Self {
            position,
            size,
            params: None,
            graphics_handler,
            shape_type: ShapeType::TRIANGLE,
        }
    }

    pub fn new_circle(ctx: &mut Context, position: Vec2, radius: f32) -> Self {
        let mut vertices = Vec::<Vertex>::with_capacity(NUMBER_OF_SIDES_IN_CIRCLE + 2);
        let mut indices = Vec::<u16>::with_capacity(NUMBER_OF_SIDES_IN_CIRCLE * 3);

        let (x, y): (f32, f32) = position.into();
        vertices.push(Vertex::new(x, y));
        for i in 0..NUMBER_OF_SIDES_IN_CIRCLE + 1 {
            let rx =
                (i as f32 / NUMBER_OF_SIDES_IN_CIRCLE as f32 * std::f32::consts::PI * 2.).cos();
            let ry =
                (i as f32 / NUMBER_OF_SIDES_IN_CIRCLE as f32 * std::f32::consts::PI * 2.).sin();

            let vertex = Vertex::new(x + radius * rx, y + radius * ry);
            vertices.push(vertex);

            if i != NUMBER_OF_SIDES_IN_CIRCLE {
                indices.extend_from_slice(&[0, i as u16 + 1, i as u16 + 2]);
            }
        }

        let size = Vec2::new(radius, radius);
        let shader_params = shader::get_shader_params();
        let graphics_handler = GraphicsHandler::new(ctx, &vertices, &indices, shader_params);

        Self {
            position,
            size,
            params: None,
            graphics_handler,
            shape_type: ShapeType::TRIANGLE,
        }
    }

    /// Constructs the shape with additional shape parameters.
    pub fn with_params(mut self, params: ShapeParams) -> Self {
        self.params = Some(params);
        self
    }
}

impl EventHandler for Shape {
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
            Mat4::orthographic_rh_gl(0.0, window_width, 0.0, window_height, -1.0, 1.0);

        let mvp = ortho_matrix * translation_matrix * scale_matrix;

        ctx.apply_pipeline(&self.graphics_handler.pipeline());
        ctx.apply_bindings(&self.graphics_handler.bindings());
        ctx.apply_uniforms(&shader::Uniforms {
            offset: (0.0, 0.0),
            mvp,
        });

        match &self.shape_type {
            ShapeType::SQUARE | ShapeType::RECTANGLE => ctx.draw(0, 6, 1),
            ShapeType::TRIANGLE => ctx.draw(0, 3, 1),
        }
    }
}

mod shader {
    use miniquad::*;

    use crate::core::graphics_handler::ShaderParams;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;
    attribute vec2 uv;

    uniform vec2 offset;
    uniform mat4 mvp;

    varying lowp vec2 texcoord;

    void main() {
        vec4 pos = vec4(pos + offset, 0, 1);
        gl_Position = mvp * pos;
        texcoord = uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;

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
