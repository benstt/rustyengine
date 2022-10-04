use super::graphics_handler::{GraphicsHandler, ShaderParams};
use image::io::Reader as ImageReader;
use image::GenericImageView;
use miniquad::*;
use std::path::Path;

/// A GPU allocated texture.
pub struct Texture {
    /// The size of the image loaded.
    size: (u32, u32),
    wrap_mode: TextureWrap,
    #[doc(hidden)]
    graphics_handler: GraphicsHandler,
}

impl Texture {
    /// Loads a texture from a path to the GPU. Stores the pipeline and bindings
    /// in it to later draw onto the screen.
    ///
    /// # Example
    /// ```rust
    /// use std::path::Path;
    ///
    /// let path = Path::new("sprite.png");
    /// let shader_params = shader::get_shader_params(); // somewhere else
    /// let texture = Texture::from_path(ctx, path, shader_params);
    /// ```
    pub fn from_path(ctx: &mut Context, path: &Path, shader_params: ShaderParams) -> Self {
        Self::with_params(
            ctx,
            path,
            shader_params,
            TextureParams {
                filter: FilterMode::Nearest,
                ..Default::default()
            },
        )
    }

    /// Loads a texture to the GPU with the texture wrap mode specified.
    pub fn with_params(
        ctx: &mut Context,
        path: &Path,
        shader_params: ShaderParams,
        texture_params: TextureParams,
    ) -> Self {
        // open the image and get its dimensions and bytes
        info!("Trying to open an image with path {:?}", path);
        let img = ImageReader::open(path)
            .expect("Cannot open file.")
            .decode()
            .unwrap();

        let img_dimensions = img.dimensions();
        let img_bytes = img.as_bytes();

        info!(
            "Allocating a texture with size of {:?} and dimensions {:?}",
            img_bytes.len(),
            img_dimensions
        );

        let graphics_handler = GraphicsHandler::from_texture_with_params(
            ctx,
            img_dimensions,
            img_bytes,
            shader_params,
            texture_params,
        );

        Self {
            graphics_handler,
            size: img_dimensions,
            wrap_mode: texture_params.wrap,
        }
    }

    /// Returns the pipeline used to render to the screen.
    pub const fn pipeline(&self) -> &Pipeline {
        self.graphics_handler.pipeline()
    }

    /// Returns the bindings used to render to the screen.
    pub const fn bindings(&self) -> &Bindings {
        self.graphics_handler.bindings()
    }

    pub const fn size(&self) -> (u32, u32) {
        self.size
    }

    pub const fn wrap_mode(&self) -> TextureWrap {
        self.wrap_mode
    }
}
