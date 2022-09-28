use super::vertex::Vertex;
use miniquad::*;

pub struct GraphicsHandler {
    pipeline: Pipeline,
    bindings: Bindings,
}

pub struct ShaderParams {
    pub vertex_shader: &'static str,
    pub fragment_shader: &'static str,
    pub meta: ShaderMeta,
}

impl GraphicsHandler {
    pub fn new(
        ctx: &mut Context,
        vertices: &[Vertex],
        indices: &[u16],
        primitive_type: PrimitiveType,
        shader_params: ShaderParams,
    ) -> Self {
        info!("Creating new GraphicsHandler");
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let pixels: [u8; 4 * 4 * 4] = [0xFF; 64];
        let texture = Texture::from_rgba8(ctx, 4, 4, &pixels);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![texture],
        };

        let shader = Shader::new(
            ctx,
            shader_params.vertex_shader,
            shader_params.fragment_shader,
            shader_params.meta,
        )
        .unwrap();

        // set the pipeline's parameters, as well as its shader attributes
        let pipeline = Pipeline::with_params(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("color0", VertexFormat::Float3),
            ],
            shader,
            PipelineParams {
                primitive_type,
                ..Default::default()
            },
        );

        Self { pipeline, bindings }
    }

    pub fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    pub fn bindings(&self) -> &Bindings {
        &self.bindings
    }
}
