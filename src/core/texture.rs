use super::graphics_handler::{GraphicsHandler, ShaderParams};
use image::io::Reader as ImageReader;
use image::GenericImageView;
use miniquad::*;
use std::path::Path;

pub struct Texture {
    pub size: (u32, u32),
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
    /// let texture = Texture::from_path(ctx, path);
    /// ```
    pub fn from_path(ctx: &mut Context, path: &Path, shader_params: ShaderParams) -> Self {
        // open the image and get its dimensions and bytes
        let img = ImageReader::open(path)
            .expect("Cannot open file.")
            .decode()
            .unwrap();

        let img_dimensions = img.dimensions();
        let img_bytes = img.as_bytes();

        let graphics_handler =
            GraphicsHandler::from_texture(ctx, img_dimensions, img_bytes, shader_params);

        Self {
            graphics_handler,
            size: img_dimensions,
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
}
