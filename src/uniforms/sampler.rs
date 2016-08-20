use ToGlEnum;
use gl;

/// Function to use for out-of-bounds samples.
///
/// This is how GL must handle samples that are outside the texture.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SamplerWrapFunction {
    /// Samples at coord `x + 1` map to coord `x`.
    Repeat,

    /// Samples at coord `x + 1` map to coord `1 - x`.
    Mirror,

    /// Samples at coord `x + 1` map to coord `1`.
    Clamp,

    /// Same as Mirror, but only for one repetition,
    MirrorClamp,

    /// Samples at coord `x + 1` map to the border color of the sampler.
    BorderClamp,
}

impl ToGlEnum for SamplerWrapFunction {
    #[inline]
    fn to_glenum(&self) -> gl::types::GLenum {
        match *self {
            SamplerWrapFunction::Repeat => gl::REPEAT,
            SamplerWrapFunction::Mirror => gl::MIRRORED_REPEAT,
            SamplerWrapFunction::Clamp => gl::CLAMP_TO_EDGE,
            SamplerWrapFunction::MirrorClamp => gl::MIRROR_CLAMP_TO_EDGE,
            SamplerWrapFunction::BorderClamp => gl::CLAMP_TO_BORDER,
        }
    }
}

/// The function that the GPU will use when loading the value of a texel.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MagnifySamplerFilter {
    /// The nearest texel will be loaded.
    Nearest,

    /// All nearby texels will be loaded and their values will be merged.
    Linear,
}

impl ToGlEnum for MagnifySamplerFilter {
    #[inline]
    fn to_glenum(&self) -> gl::types::GLenum {
        match *self {
            MagnifySamplerFilter::Nearest => gl::NEAREST,
            MagnifySamplerFilter::Linear => gl::LINEAR,
        }
    }
}

/// The function that the GPU will use when loading the value of a texel.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MinifySamplerFilter {
    /// The nearest texel will be loaded.
    ///
    /// Only uses the main texture, mipmaps are totally ignored.
    Nearest,

    /// All nearby texels will be loaded and their values will be merged.
    ///
    /// Only uses the main texture, mipmaps are totally ignored.
    Linear,

    /// The nearest texel of the nearest mipmap will be loaded.
    NearestMipmapNearest,

    /// Takes the nearest texel from the two nearest mipmaps, and merges them.
    LinearMipmapNearest,

    /// Same as `Linear`, but from the nearest mipmap.
    NearestMipmapLinear,

    /// Same as `Linear`, but from the two nearest mipmaps.
    LinearMipmapLinear,
}

impl ToGlEnum for MinifySamplerFilter {
    #[inline]
    fn to_glenum(&self) -> gl::types::GLenum {
        match *self {
            MinifySamplerFilter::Nearest => gl::NEAREST,
            MinifySamplerFilter::Linear => gl::LINEAR,
            MinifySamplerFilter::NearestMipmapNearest => gl::NEAREST_MIPMAP_NEAREST,
            MinifySamplerFilter::LinearMipmapNearest => gl::LINEAR_MIPMAP_NEAREST,
            MinifySamplerFilter::NearestMipmapLinear => gl::NEAREST_MIPMAP_LINEAR,
            MinifySamplerFilter::LinearMipmapLinear => gl::LINEAR_MIPMAP_LINEAR,
        }
    }
}

/// A sampler.
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Sampler<'t, T: 't>(pub &'t T, pub SamplerBehavior);

impl<'t, T: 't> Sampler<'t, T> {
    /// Builds a new `Sampler` with default parameters.
    pub fn new(texture: &'t T) -> Sampler<'t, T> {
        Sampler(texture, Default::default())
    }

    /// Changes the border color of the sampler.
    pub fn border_color(mut self, color: (u8, u8, u8, u8)) -> Sampler<'t, T> {
        self.1.border_color = color;
        self
    }

    /// Changes the wrap functions of all three coordinates.
    pub fn wrap_function(mut self, function: SamplerWrapFunction) -> Sampler<'t, T> {
        self.1.wrap_function = (function, function, function);
        self
    }

    /// Changes the minifying filter of the sampler.
    pub fn minify_filter(mut self, filter: MinifySamplerFilter) -> Sampler<'t, T> {
        self.1.minify_filter = filter;
        self
    }

    /// Changes the magnifying filter of the sampler.
    pub fn magnify_filter(mut self, filter: MagnifySamplerFilter) -> Sampler<'t, T> {
        self.1.magnify_filter = filter;
        self
    }

    /// Changes the magnifying filter of the sampler.
    pub fn anisotropy(mut self, level: u16) -> Sampler<'t, T> {
        self.1.max_anisotropy = level;
        self
    }
}

impl<'t, T: 't> Copy for Sampler<'t, T> {}

impl<'t, T: 't> Clone for Sampler<'t, T> {
    fn clone(&self) -> Self {
        *self
    }
}

/// Behavior of a sampler.
// TODO: GL_TEXTURE_MIN_LOD, GL_TEXTURE_MAX_LOD, GL_TEXTURE_LOD_BIAS,
//       GL_TEXTURE_COMPARE_MODE, GL_TEXTURE_COMPARE_FUNC
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct SamplerBehavior {
    /// Functions to use for the X, Y, and Z coordinates.
    pub wrap_function: (SamplerWrapFunction, SamplerWrapFunction, SamplerWrapFunction),

    /// The border color to use when one of the `wrap_function`'s is `BorderClamp`.
    // FIXME: These should probably be `f32`'s in line with other functions taking
    // colors as arguments, however float's are very impractical to use together with
    // `Hash` and `Eq`.
    pub border_color: (u8, u8, u8, u8),

    /// Filter to use when minifying the texture.
    pub minify_filter: MinifySamplerFilter,

    /// Filter to use when magnifying the texture.
    pub magnify_filter: MagnifySamplerFilter,

    /// `1` means no anisotropic filtering, any value above `1` sets the max anisotropy.
    ///
    /// ## Compatibility
    ///
    /// This parameter is always available. However it is ignored on hardware that does
    /// not support anisotropic filtering.
    ///
    /// If you set the value to a value higher than what the hardware supports, it will
    /// be clamped.
    pub max_anisotropy: u16,
}

impl Default for SamplerBehavior {
    #[inline]
    fn default() -> SamplerBehavior {
        SamplerBehavior {
            wrap_function: (
                SamplerWrapFunction::Mirror,
                SamplerWrapFunction::Mirror,
                SamplerWrapFunction::Mirror
            ),
            border_color: (0, 0, 0, 0),
            minify_filter: MinifySamplerFilter::LinearMipmapLinear,
            magnify_filter: MagnifySamplerFilter::Linear,
            max_anisotropy: 1,
        }
    }
}
