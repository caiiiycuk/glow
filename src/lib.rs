#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use self::native::*;

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
pub use self::web::*;

use bitflags::bitflags;

/// The type of the shader.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ShaderType {
    Fragment = 0x8B30,
    Vertex = 0x8B31,
    Geometry = 0x8DD9,
    TessEvaluation = 0x8E87,
    TessControl = 0x8E88,
    Compute = 0x91B9,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Parameter {
    /// If enabled, blend the computed fragment color values with the values in the color buffers.
    Blend = 0x0BE2,
    /// If enabled, clip geometry against user-defined half space 0.
    ClipDistance0 = 0x3000,
    /// If enabled, clip geometry against user-defined half space 1.
    ClipDistance1 = 0x3001,
    /// If enabled, clip geometry against user-defined half space 2.
    ClipDistance2 = 0x3002,
    /// If enabled, clip geometry against user-defined half space 3.
    ClipDistance3 = 0x3003,
    /// If enabled, clip geometry against user-defined half space 4.
    ClipDistance4 = 0x3004,
    /// If enabled, clip geometry against user-defined half space 5.
    ClipDistance5 = 0x3005,
    /// If enabled, clip geometry against user-defined half space 6.
    ClipDistance6 = 0x3006,
    /// If enabled, clip geometry against user-defined half space 7.
    ClipDistance7 = 0x3007,
    /// If enabled, apply the currently selected logical operation to the computed fragment color and color buffer values.
    ColorLogicOp = 0x0BF2,
    /// If enabled, cull polygons based on their winding in window coordinates.
    CullFace = 0x0B44,
    /// If enabled, debug messages are produced by a debug context. When disabled, the debug message log is silenced. Note that in a non-debug context, very few, if any messages might be produced, even when DebugOutput is enabled.
    DebugOutput = 0x92E0,
    /// If enabled, debug messages are produced synchronously by a debug context. If disabled, debug messages may be produced asynchronously. In particular, they may be delayed relative to the execution of GL commands, and the debug callback function may be called from a thread other than that in which the commands are executed.
    DebugOutputSynchronous = 0x8242,
    /// If enabled, the −wc≤zc≤wc plane equation is ignored by view volume clipping (effectively, there is no near or far plane clipping).
    DepthClamp = 0x864F,
    /// If enabled, do depth comparisons and update the depth buffer. Note that even if the depth buffer exists and the depth mask is non-zero, the depth buffer is not updated if the depth test is disabled.
    DepthTest = 0x0B71,
    /// If enabled, dither color components or indices before they are written to the color buffer.
    Dither = 0x0BD0,
    /// If enabled and the value of GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING for the framebuffer attachment corresponding to the destination buffer is GL_SRGB, the R, G, and B destination color values (after conversion from fixed-point to floating-point) are considered to be encoded for the sRGB color space and hence are linearized prior to their use in blending.
    FramebufferSrgb = 0x8DB9,
    /// If enabled, draw lines with correct filtering. Otherwise, draw aliased lines.
    LineSmooth = 0x0B20,
    /// If enabled, use multiple fragment samples in computing the final color of a pixel.
    Multisample = 0x809D,
    /// If enabled, and if the polygon is rendered in GL_FILL mode, an offset is added to depth values of a polygon's fragments before the depth comparison is performed.
    PolygonOffsetFill = 0x8037,
    /// If enabled, and if the polygon is rendered in GL_LINE mode, an offset is added to depth values of a polygon's fragments before the depth comparison is performed.
    PolygonOffsetLine = 0x2A02,
    /// If enabled, an offset is added to depth values of a polygon's fragments before the depth comparison is performed, if the polygon is rendered in GL_POINT mode.
    PolygonOffsetPoint = 0x2A01,
    /// If enabled, draw polygons with proper filtering. Otherwise, draw aliased polygons. For correct antialiased polygons, an alpha buffer is needed and the polygons must be sorted front to back.
    PolygonSmooth = 0x0B41,
    /// Enables primitive restarting. If enabled, any one of the draw commands which transfers a set of generic attribute array elements to the GL will restart the primitive when the index of the vertex is equal to the primitive restart index.
    PrimitiveRestart = 0x8F9D,
    /// Enables primitive restarting with a fixed index. If enabled, any one of the draw commands which transfers a set of generic attribute array elements to the GL will restart the primitive when the index of the vertex is equal to the fixed primitive index for the specified index type. The fixed index is equal to 2n−1 where n is equal to 8 for GL_UNSIGNED_BYTE, 16 for GL_UNSIGNED_SHORT and 32 for GL_UNSIGNED_INT.
    PrimitiveRestartFixedIndex = 0x8D69,
    /// If enabled, primitives are discarded after the optional transform feedback stage, but before rasterization. Furthermore, when enabled, glClear, glClearBufferData, glClearBufferSubData, glClearTexImage, and glClearTexSubImage are ignored.
    RasterizerDiscard = 0x8C89,
    /// If enabled, compute a temporary coverage value where each bit is determined by the alpha value at the corresponding sample location. The temporary coverage value is then ANDed with the fragment coverage value.    
    SampleAlphaToCoverage = 0x809E,
    ///If enabled, each sample alpha value is replaced by the maximum representable alpha value.
    SampleAlphaToOne = 0x809F,
    /// If enabled, the fragment's coverage is ANDed with the temporary coverage value. If GL_SAMPLE_COVERAGE_INVERT is set to GL_TRUE, invert the coverage value.
    SampleCoverage = 0x80A0,
    /// If enabled, the active fragment shader is run once for each covered sample, or at fraction of this rate as determined by the current value of GL_MIN_SAMPLE_SHADING_VALUE.
    SampleShading = 0x8C36,
    /// If enabled, the sample coverage mask generated for a fragment during rasterization will be ANDed with the value of GL_SAMPLE_MASK_VALUE before shading occurs.
    SampleMask = 0x8E51,
    /// If enabled, discard fragments that are outside the scissor rectangle.
    ScissorTest = 0x0C11,
    /// If enabled, do stencil testing and update the stencil buffer.
    StencilTest = 0x0B90,
    /// If enabled, cubemap textures are sampled such that when linearly sampling from the border between two adjacent faces, texels from both faces are used to generate the final sample value. When disabled, texels from only a single face are used to construct the final sample value.
    TextureCubeMapSeamless = 0x884F,
    /// If enabled and a vertex or geometry shader is active, then the derived point size is taken from the (potentially clipped) shader builtin gl_PointSize and clamped to the implementation-dependent point size range.
    ProgramPointSize = 0x8642,
}

/// A buffer binding target.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BufferBindingTarget {
    /// Vertex attributes.
    ArrayBuffer = 0x8892,
    /// Atomic counter storage.
    AtomicCounterBuffer = 0x92C0,
    /// Buffer copy source.
    CopyReadBuffer = 0x8F36,
    /// Buffer copy destination.
    CopyWriteBuffer = 0x8F37,
    /// Indirect compute dispatch commands.
    DispatchIndirectBuffer = 0x90EE,
    /// Indirect command arguments.
    DrawIndirect = 0x8F3F,
    /// Vertex array indices.
    ElementArray = 0x8893,
    /// Pixel read target.
    PixelPack = 0x88EB,
    /// Texture data source.
    PixelUnpack = 0x88EC,
    /// Query result buffer.
    Query = 0x9192,
    /// Read-write storage for shaders.
    ShaderStorage = 0x90D2,
    /// Texture data buffer.
    Texture = 0x8C2A,
    /// Transform feedback buffer.
    TransformFeedback = 0x8C8E,
    /// Uniform block storage.
    Uniform = 0x8A11,
}

/// The kind of primitive to render.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PrimitiveMode {
    Points = 0x0000,
    LineStrip = 0x0003,
    LineLoop = 0x0002,
    Lines = 0x0001,
    LineStripAdjacency = 0x000B,
    LinesAdjacency = 0x000A,
    TriangleStrip = 0x0005,
    TriangleFan = 0x0006,
    Triangles = 0x0004,
    TriangleStripAdjacency = 0x000D,
    TrianglesAdjacency = 0x000C,
    Patches = 0x000E,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PixelStoreI32Parameter {
    PackRowLength = 0x0D02,
    PackImageHeight = 0x806C,
    PackSkipRows = 0x0D03,
    PackSkipPixels = 0x0D04,
    PackSkipImages = 0x806B,
    PackAlignment = 0x0D05,
    UnpackRowLength = 0x0CF2,
    UnpackImageHeight = 0x806E,
    UnpackSkipRows = 0x0CF3,
    UnpackSkipPixels = 0x0CF4,
    UnpackSkipImages = 0x806D,
    UnpackAlignment = 0x0CF5,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PixelStoreBoolParameter {
    PackSwapBytes = 0x0D00,
    PackLsbFirst = 0x0D01,
    UnpackSwapBytes = 0x0CF0,
    UnpackLsbFirst = 0x0CF1,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FrontFace {
    Clockwise = 0x0900,
    CounterClockwise = 0x0901,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CullFace {
    Front = 0x0404,
    Back = 0x0405,
    FrontAndBack = 0x0408,
}

/// The buffers to clear.
bitflags! {
    pub struct ClearMask: u32 {
        const Color = 0x00004000;
        const Stencil = 0x00000400;
        const Depth = 0x00000100;
    }
}

pub(crate) const COMPILE_STATUS: u32 = 0x8B81;
pub(crate) const INFO_LOG_LENGTH: u32 = 0x8B84;
pub(crate) const LINK_STATUS: u32 = 0x8B82;

pub trait RenderingContext {
    type Shader: Copy
        + Clone
        + std::fmt::Debug
        + Eq
        + std::hash::Hash
        + Ord
        + PartialEq
        + PartialOrd;
    type Program: Copy
        + Clone
        + std::fmt::Debug
        + Eq
        + std::hash::Hash
        + Ord
        + PartialEq
        + PartialOrd;
    type Buffer: Copy
        + Clone
        + std::fmt::Debug
        + Eq
        + std::hash::Hash
        + Ord
        + PartialEq
        + PartialOrd;
    type VertexArray: Copy
        + Clone
        + std::fmt::Debug
        + Eq
        + std::hash::Hash
        + Ord
        + PartialEq
        + PartialOrd;

    unsafe fn create_shader(&self, shader_type: ShaderType) -> Result<Self::Shader, String>;

    unsafe fn delete_shader(&self, shader: Self::Shader);

    unsafe fn shader_source(&self, shader: Self::Shader, source: &str);

    unsafe fn compile_shader(&self, shader: Self::Shader);

    unsafe fn get_shader_compile_status(&self, shader: Self::Shader) -> bool;

    unsafe fn get_shader_info_log(&self, shader: Self::Shader) -> String;

    unsafe fn create_program(&self) -> Result<Self::Program, String>;

    unsafe fn delete_program(&self, program: Self::Program);

    unsafe fn attach_shader(&self, program: Self::Program, shader: Self::Shader);

    unsafe fn detach_shader(&self, program: Self::Program, shader: Self::Shader);

    unsafe fn link_program(&self, program: Self::Program);

    unsafe fn get_program_link_status(&self, program: Self::Program) -> bool;

    unsafe fn get_program_info_log(&self, program: Self::Program) -> String;

    unsafe fn use_program(&self, program: Option<Self::Program>);

    unsafe fn create_buffer(&self) -> Result<Self::Buffer, String>;

    unsafe fn bind_buffer(&self, target: BufferBindingTarget, buffer: Option<Self::Buffer>);

    unsafe fn draw_arrays(&self, mode: PrimitiveMode, first: i32, count: i32);

    unsafe fn create_vertex_array(&self) -> Result<Self::VertexArray, String>;

    unsafe fn delete_vertex_array(&self, vertex_array: Self::VertexArray);

    unsafe fn bind_vertex_array(&self, vertex_array: Option<Self::VertexArray>);

    unsafe fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32);

    unsafe fn supports_f64_precision() -> bool;

    unsafe fn clear_depth_f64(&self, depth: f64);

    unsafe fn clear_depth_f32(&self, depth: f32);

    unsafe fn clear_stencil(&self, stencil: i32);

    unsafe fn clear(&self, mask: ClearMask);

    unsafe fn pixel_store_i32(&self, parameter: PixelStoreI32Parameter, value: i32);

    unsafe fn pixel_store_bool(&self, parameter: PixelStoreBoolParameter, value: bool);

    unsafe fn enable(&self, parameter: Parameter);

    unsafe fn disable(&self, parameter: Parameter);

    unsafe fn front_face(&self, value: FrontFace);

    unsafe fn cull_face(&self, value: CullFace);

    unsafe fn color_mask(&self, red: bool, green: bool, blue: bool, alpha: bool);

    unsafe fn blend_color(&self, red: f32, green: f32, blue: f32, alpha: f32);

    unsafe fn line_width(&self, width: f32);

    unsafe fn polygon_offset(&self, factor: f32, units: f32);
}

pub trait RenderLoop {
    type Window;

    fn run<F: FnMut(&mut bool) + 'static>(&self, callback: F);
}