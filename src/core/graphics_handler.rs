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
        info!("Creating new GraphicsHandler");

        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let pixels: [u8; 4 * 4 * 4] = [0xFF; 64];
        let texture = Texture::from_rgba8(ctx, 4, 4, &pixels);

        let bindings = Bindings {
            index_buffer,
            vertex_buffers: vec![vertex_buffer],
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

    /// Allocates a new texture into the GPU and returns the pipeline
    /// and bindings with the shaders to render it.
    pub fn from_texture(
        ctx: &mut Context,
        img_size: (u32, u32),
        img_bytes: &[u8],
        shader_params: ShaderParams,
    ) -> Self {
        Self::from_texture_with_params(
            ctx,
            img_size,
            img_bytes,
            shader_params,
            TextureParams {
                ..Default::default()
            },
        )
    }

    /// Allocates a new texture into the GPU with the texture wrap mode specified.
    pub fn from_texture_with_params(
        ctx: &mut Context,
        img_size: (u32, u32),
        img_bytes: &[u8],
        shader_params: ShaderParams,
        texture_params: TextureParams,
    ) -> Self {
        info!("Creating new GraphicsHandler from a texture");

        let (img_width, img_height) = img_size;
        let vertices: [Vertex; 4] = [
            Vertex::with_tex(-1.0, 1.0, 0.0, 1.0),
            Vertex::with_tex(1.0, 1.0, 1.0, 1.0),
            Vertex::with_tex(1.0, -1.0, 1.0, 0.0),
            Vertex::with_tex(-1.0, -1.0, 0.0, 0.0),
        ];
        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];

        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);
        let texture = Texture::new(
            ctx,
            TextureAccess::Static,
            Some(img_bytes),
            TextureParams {
                width: img_width,
                height: img_height,
                ..texture_params
            },
        );

        let bindings = Bindings {
            index_buffer,
            vertex_buffers: vec![vertex_buffer],
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
        let pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("color0", VertexFormat::Float3),
                VertexAttribute::new("tex0", VertexFormat::Float2),
            ],
            shader,
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
