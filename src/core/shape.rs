use super::vertex::Vertex;
use glam::{Mat4, Vec2, Vec3};
use miniquad::*;

pub enum ShapeType {
    SQUARE,
    TRIANGLE,
}

pub struct Shape {
    pub shape_type: ShapeType,
    pipeline: Pipeline,
    bindings: Bindings,
}

impl Shape {
    /// Creates a new shape.
    pub fn new(ctx: &mut Context, shape_type: ShapeType) -> Self {
        match shape_type {
            ShapeType::SQUARE => Self::new_square(ctx),
            ShapeType::TRIANGLE => Self::new_triangle(ctx),
        }
    }

    /// Creates a square at the middle of the screen.
    /// Shorthand for `Shape::new(ctx, ShapeType::SQUARE)`.
    pub fn new_square(ctx: &mut Context) -> Self {
        let vertices: [Vertex; 4] = [
            Vertex::new(-0.5, 0.5),
            Vertex::new(0.5, 0.5),
            Vertex::new(0.5, -0.5),
            Vertex::new(-0.5, -0.5),
        ];

        // TODO: Separate all the vertex/index/pipeline/bindings logic into a module.
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let pixels: [u8; 4 * 4 * 4] = [0xFF; 64];
        let texture = Texture::from_rgba8(ctx, 4, 4, &pixels);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![texture],
        };

        let shader = Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::meta()).unwrap();

        // set the pipeline's parameters, as well as its shader attributes
        let pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("offset", VertexFormat::Float2),
                VertexAttribute::new("scale", VertexFormat::Mat4),
            ],
            shader,
        );

        Self {
            pipeline,
            bindings,
            shape_type: ShapeType::SQUARE,
        }
    }

    /// Creates a triangle at the middle of the screen.
    /// Shorthand for `Shape::new(ctx, ShapeType::TRIANGLE)`.
    pub fn new_triangle(ctx: &mut Context) -> Self {
        let vertices: [Vertex; 3] = [
            Vertex::new(-0.5, -0.5),
            Vertex::new(0., 0.),
            Vertex::new(0.5, -0.5),
        ];

        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices: [u16; 3] = [0, 1, 2];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let pixels: [u8; 4 * 4 * 4] = [0xFF; 64];
        let texture = Texture::from_rgba8(ctx, 4, 4, &pixels);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![texture],
        };

        let shader = Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::meta()).unwrap();

        // set the pipeline's parameters, as well as its shader attributes
        let pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("offset", VertexFormat::Float2),
                VertexAttribute::new("projection", VertexFormat::Mat4),
            ],
            shader,
        );

        Self {
            pipeline,
            bindings,
            shape_type: ShapeType::TRIANGLE,
        }
    }
}

impl EventHandler for Shape {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(Default::default());

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        ctx.apply_uniforms(&shader::Uniforms {
            offset: Vec2::new(0.0, 0.0),
            projection: Mat4::from_scale(Vec3::new(1.0, 1.0, 1.0)),
        });

        match &self.shape_type {
            ShapeType::SQUARE => {
                ctx.draw(0, 6, 1);
            }
            ShapeType::TRIANGLE => {
                ctx.draw(0, 3, 1);
            }
        }

        ctx.end_render_pass();
        ctx.commit_frame();
    }
}

mod shader {
    use glam::{Mat4, Vec2};
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;
    attribute vec2 uv;

    uniform vec2 offset;
    uniform mat4 projection;

    varying lowp vec2 texcoord;

    void main() {
        gl_Position = vec4(pos + offset, 0, 1);
        texcoord = uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;

    uniform sampler2D tex;

    void main() {
        gl_FragColor = texture2D(tex, texcoord);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("offset", UniformType::Float2),
                    UniformDesc::new("projection", UniformType::Mat4),
                ],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub offset: Vec2,
        pub projection: Mat4,
    }
}
