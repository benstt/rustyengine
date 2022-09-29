use super::color::Color;
use super::graphics_handler::GraphicsHandler;
use super::vertex::Vertex;
use glam::{Mat4, Vec2, Vec3};
use miniquad::*;

const NUMBER_OF_SIDES_IN_CIRCLE: usize = 20;

#[derive(Debug)]
pub enum ShapeType {
    Square(f32),
    SquareLines(f32),
    Rectangle(f32, f32),
    RectangleLines(f32, f32),
    Triangle(f32, f32),
    TriangleLines(f32, f32),
    Circle(f32),
    CircleLines(f32),
    Line(f32, f32),
}

pub enum ShapeCenterPosition {
    Middle,
    TopLeft,
}

/// Represents the shape settings that can be applied.
pub struct ShapeParams {
    /// Represents the center position of the shape. Can be either at the middle or top-left.
    pub center: ShapeCenterPosition,
}

impl Default for ShapeParams {
    fn default() -> Self {
        Self {
            center: ShapeCenterPosition::TopLeft,
        }
    }
}

#[repr(C)]
pub struct Shape {
    pub shape_type: ShapeType,
    pub position: Vec2,
    pub size: Vec2,
    pub params: ShapeParams,
    graphics_handler: GraphicsHandler,
}

impl Shape {
    pub fn new(ctx: &mut Context, shape_type: ShapeType, position: Vec2, color: Color) -> Self {
        Self::with_params(ctx, shape_type, position, color, Default::default())
    }

    pub fn with_params(
        ctx: &mut Context,
        shape_type: ShapeType,
        position: Vec2,
        color: Color,
        params: ShapeParams,
    ) -> Self {
        debug!("creating shape {:?} at {}", shape_type, position);
        match &shape_type {
            ShapeType::Square(size) => Self::new_square(ctx, position, *size, color, params),
            ShapeType::SquareLines(size) => {
                Self::new_square_lines(ctx, position, *size, color, params)
            }
            ShapeType::Rectangle(x, y) => {
                let size = Vec2::new(*x, *y);
                Self::new_rectangle(ctx, position, size, color, params)
            }
            ShapeType::RectangleLines(x, y) => {
                let size = Vec2::new(*x, *y);
                Self::new_rectangle_lines(ctx, position, size, color, params)
            }
            ShapeType::Triangle(x, y) => {
                let size = Vec2::new(*x, *y);
                Self::new_triangle(ctx, position, size, color, params)
            }
            ShapeType::TriangleLines(x, y) => {
                let size = Vec2::new(*x, *y);
                Self::new_triangle_lines(ctx, position, size, color, params)
            }
            ShapeType::Circle(r) => Self::new_circle(ctx, position, *r, color, params),
            _ => unimplemented!(),
            // ShapeType::Line(x1, y1) => {
            //     let end = Vec2::new(*x1, *y1);
            //     Self::new_line(ctx, position, end, color)
            // }
        }
    }

    /// Creates a square with the position and size given.
    fn new_square(
        ctx: &mut Context,
        position: Vec2,
        size: f32,
        color: Color,
        params: ShapeParams,
    ) -> Self {
        let size = Vec2::new(size, size);
        Self::new_rectangle(ctx, position, size, color, params)
    }

    /// Creates a square's contour.
    fn new_square_lines(
        ctx: &mut Context,
        position: Vec2,
        size: f32,
        color: Color,
        params: ShapeParams,
    ) -> Self {
        let size = Vec2::new(size, size);
        Self::new_rectangle_lines(ctx, position, size, color, params)
    }

    /// Creates a rectangle with the position and size given.
    fn new_rectangle(
        ctx: &mut Context,
        position: Vec2,
        size: Vec2,
        color: Color,
        params: ShapeParams,
    ) -> Self {
        let vertices: [Vertex; 4] = [
            Vertex::new(-1.0, 1.0, color),
            Vertex::new(1.0, 1.0, color),
            Vertex::new(1.0, -1.0, color),
            Vertex::new(-1.0, -1.0, color),
        ];

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let shader_params = shader::get_shader_params();
        let draw_mode = PrimitiveType::Triangles;
        let graphics_handler =
            GraphicsHandler::new(ctx, &vertices, &indices, draw_mode, shader_params);

        Self {
            position,
            size,
            graphics_handler,
            params,
            shape_type: ShapeType::Rectangle(size.x, size.y),
        }
    }

    /// Creates a rectangle's contour.
    fn new_rectangle_lines(
        ctx: &mut Context,
        position: Vec2,
        size: Vec2,
        color: Color,
        params: ShapeParams,
    ) -> Self {
        let vertices: [Vertex; 8] = [
            Vertex::new(-1.0, 1.0, color),
            Vertex::new(1.0, 1.0, color),
            Vertex::new(1.0, 1.0, color),
            Vertex::new(1.0, -1.0, color),
            Vertex::new(1.0, -1.0, color),
            Vertex::new(-1.0, -1.0, color),
            Vertex::new(-1.0, -1.0, color),
            Vertex::new(-1.0, 1.0, color),
        ];
        // TODO: make center lines transparent
        let indices: [u16; 24] = [
            0, 1, 4, 1, 4, 5, 1, 5, 6, 1, 2, 6, 3, 7, 2, 2, 7, 6, 0, 4, 3, 3, 4, 7,
        ];
        let shader_params = shader::get_shader_params();
        let draw_mode = PrimitiveType::Lines;
        let graphics_handler =
            GraphicsHandler::new(ctx, &vertices, &indices, draw_mode, shader_params);

        Self {
            position,
            size,
            graphics_handler,
            params,
            shape_type: ShapeType::RectangleLines(size.x, size.y),
        }
    }

    /// Creates a triangle with the position and size given.  
    fn new_triangle(
        ctx: &mut Context,
        position: Vec2,
        size: Vec2,
        color: Color,
        params: ShapeParams,
    ) -> Self {
        let vertices: [Vertex; 3] = [
            Vertex::new(-1.0, -1.0, color),
            Vertex::new(0.0, 1.0, color),
            Vertex::new(1.0, -1.0, color),
        ];
        let indices: [u16; 3] = [0, 1, 2];
        let shader_params = shader::get_shader_params();
        let draw_mode = PrimitiveType::Triangles;
        let graphics_handler =
            GraphicsHandler::new(ctx, &vertices, &indices, draw_mode, shader_params);

        Self {
            position,
            size,
            graphics_handler,
            params,
            shape_type: ShapeType::Triangle(size.x, size.y),
        }
    }

    /// Creates a triangle's contour.
    fn new_triangle_lines(
        ctx: &mut Context,
        position: Vec2,
        size: Vec2,
        color: Color,
        params: ShapeParams,
    ) -> Self {
        let vertices: [Vertex; 6] = [
            Vertex::new(-1.0, -1.0, color),
            Vertex::new(0.0, 1.0, color),
            Vertex::new(0.0, 1.0, color),
            Vertex::new(1.0, 1.0, color),
            Vertex::new(1.0, 1.0, color),
            Vertex::new(-1.0, -1.0, color),
        ];

        // TODO: fix triangle lines indices
        let indices: [u16; 12] = [0, 1, 2, 3, 4, 5, 2, 3, 1, 4, 5, 1];
        let shader_params = shader::get_shader_params();
        let draw_mode = PrimitiveType::Lines;
        let graphics_handler =
            GraphicsHandler::new(ctx, &vertices, &indices, draw_mode, shader_params);

        Self {
            position,
            size,
            graphics_handler,
            params,
            shape_type: ShapeType::TriangleLines(size.x, size.y),
        }
    }

    /// Creates a new circle with the given position and radius.
    fn new_circle(
        ctx: &mut Context,
        position: Vec2,
        radius: f32,
        color: Color,
        params: ShapeParams,
    ) -> Self {
        // https://github.com/not-fl3/macroquad/blob/master/src/shapes.rs#L126
        let mut vertices = Vec::<Vertex>::with_capacity(NUMBER_OF_SIDES_IN_CIRCLE + 2);
        let mut indices = Vec::<u16>::with_capacity(NUMBER_OF_SIDES_IN_CIRCLE * 3);

        let (x, y): (f32, f32) = (-1.0, 1.0);
        for i in 0..NUMBER_OF_SIDES_IN_CIRCLE + 1 {
            let rx =
                (i as f32 / NUMBER_OF_SIDES_IN_CIRCLE as f32 * std::f32::consts::PI * 2.).cos();
            let ry =
                (i as f32 / NUMBER_OF_SIDES_IN_CIRCLE as f32 * std::f32::consts::PI * 2.).sin();

            let vertex = Vertex::new(x * rx, y * ry, color);
            vertices.push(vertex);

            if i != NUMBER_OF_SIDES_IN_CIRCLE {
                indices.extend_from_slice(&[0, i as u16 + 1, i as u16 + 2]);
            }
        }

        let size = Vec2::new(radius, radius);
        let shader_params = shader::get_shader_params();
        let draw_mode = PrimitiveType::Triangles;
        let graphics_handler =
            GraphicsHandler::new(ctx, &vertices, &indices, draw_mode, shader_params);

        Self {
            position,
            size,
            graphics_handler,
            shape_type: ShapeType::Circle(radius),
            params,
        }
    }
}

impl EventHandler for Shape {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        let (window_width, window_height) = ctx.screen_size();

        // TODO: Adjust position to be in respect to the size of the screen
        let (position_x, position_y): (f32, f32) = self.position.into();
        let translation = Vec3::new(position_x, position_y, 0.0);
        let translation_matrix = Mat4::from_translation(translation);

        // TODO: Scale in respect to `ShapeCenterPosition`
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
            ShapeType::Square(_) | ShapeType::Rectangle(_, _) => ctx.draw(0, 6, 1),
            ShapeType::SquareLines(_) | ShapeType::RectangleLines(_, _) => ctx.draw(0, 24, 1),
            ShapeType::Triangle(_, _) => ctx.draw(0, 3, 1),
            ShapeType::TriangleLines(_, _) => ctx.draw(0, 12, 1),
            ShapeType::Circle(_) => ctx.draw(0, NUMBER_OF_SIDES_IN_CIRCLE as i32 * 3, 1),
            _ => unimplemented!(),
            // ShapeType::Line(_, _, _, _) => ctx.draw(0, 2, 1),
        }
    }
}

// TODO: add macro to print a warning when the size of a shape is negative
// macro_rules! warn_on_negative_size {}

mod shader {
    use miniquad::*;

    use crate::core::graphics_handler::ShaderParams;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;
    attribute vec3 color0;
    attribute vec2 uv;

    uniform vec2 offset;
    uniform mat4 mvp;

    varying lowp vec2 texcoord;
    varying lowp vec4 color;

    void main() {
        vec4 pos = vec4(pos + offset, 0, 1);
        gl_Position = mvp * pos;
        color = vec4(color0, 1.0);
        texcoord = uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec4 color;
    varying lowp vec2 texcoord;

    uniform sampler2D tex;

    void main() {
        gl_FragColor = color * texture2D(tex, texcoord);
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
