use super::vertex::Vertex;
use miniquad::*;

/// High level helper to handle graphics.
/// It can generate a new texture, as well as apply shaders to it.
pub struct GraphicsHandler {
    /// The pipeline used to render vertex and fragment shaders.
    pipeline: Pipeline,
    /// Binds vertex and index buffers as well as textures.
    bindings: Bindings,
}

/// Represents settings for a shader.
pub struct ShaderParams {
    /// The vertex shader.
    pub vertex_shader: &'static str,
    /// The fragment shader.
    pub fragment_shader: &'static str,
    /// Information about the shader.
    pub meta: ShaderMeta,
}

impl GraphicsHandler {
    /// Creates a new instance by defining a `Pipeline` with the `primitive_type` and `shader_params` in it.
    /// Constructs a binding by creating a vertex and index buffers with the provided vertices and indices arrays.
    pub fn new(
        ctx: &mut Context,
        vertices: &[Vertex],
        indices: &[u16],
        primitive_type: PrimitiveType,
        shader_params: ShaderParams,
    ) -> Self {
        debug!("creating new GraphicsHandler");

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

    /// Returns its pipeline.
    pub const fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    /// Returns its bindings.
    pub const fn bindings(&self) -> &Bindings {
        &self.bindings
    }
}
