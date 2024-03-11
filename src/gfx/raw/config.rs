pub type HasDynamicOffset = bool;
pub type Order = u32;

#[derive(Debug, Clone, Copy, Default)]
pub enum StepMode {
    #[default]
    Vertex,
    Instance,
}

#[derive(Debug, Clone, Copy)]
pub enum VertexFormat {
    Uint8x2 = 0,
    Uint8x4 = 1,
    Sint8x2 = 2,
    Sint8x4 = 3,
    Unorm8x2 = 4,
    Unorm8x4 = 5,
    Snorm8x2 = 6,
    Snorm8x4 = 7,
    Uint16x2 = 8,
    Uint16x4 = 9,
    Sint16x2 = 10,
    Sint16x4 = 11,
    Unorm16x2 = 12,
    Unorm16x4 = 13,
    Snorm16x2 = 14,
    Snorm16x4 = 15,
    Float16x2 = 16,
    Float16x4 = 17,
    Float32 = 18,
    Float32x2 = 19,
    Float32x3 = 20,
    Float32x4 = 21,
    Uint32 = 22,
    Uint32x2 = 23,
    Uint32x3 = 24,
    Uint32x4 = 25,
    Sint32 = 26,
    Sint32x2 = 27,
    Sint32x3 = 28,
    Sint32x4 = 29,
    Float64 = 30,
    Float64x2 = 31,
    Float64x3 = 32,
    Float64x4 = 33,
}

#[derive(Debug, Clone, Copy)]
pub struct Attribute {
    pub offset: u64,
    pub shader_location: u32,
    pub format: VertexFormat,
}

#[derive(Default, Debug)]
pub enum IndexFormat {
    Uint16 = 0,
    #[default]
    Uint32 = 1,
}

#[derive(Debug, Clone, Copy)]
pub enum ShaderStages {
    None,
    Vertex,
    Fragment,
    Compute,
    VertexFragment,
}

pub enum SamplerBindingType {
    Filtering,
    NonFiltering,
    Comparison,
}

pub enum TextureSampleType {
    Float { filterable: bool },
    Depth,
    Sint,
    Uint,
}

pub enum TextureViewDimension {
    D1,
    D2,
    D2Array,
    Cube,
    CubeArray,
    D3,
}

pub enum BindingType {
    Uniform(HasDynamicOffset),
    Texture {
        sample_type: TextureSampleType,
        view_dimension: TextureViewDimension,
        multisampled: bool,
    },
    Sampler(SamplerBindingType),
}

pub struct BindGroupLayoutEntry {
    pub binding: u32,
    pub visibility: ShaderStages,
    pub ty: BindingType,
    pub count: Option<u32>,
}

pub enum Resource<'a> {
    Uniform(&'a wgpu::Buffer),
}

pub struct BindGroupEntry<'a> {
    pub binding: u32,
    pub resource: Resource<'a>,
    pub size: Option<u64>,
}

pub struct ContextConfig {
    pub width: u32,
    pub height: u32,
    pub vsync: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum AddressMode {
    #[default]
    ClampToEdge = 0,
    Repeat = 1,
    MirrorRepeat = 2,
    ClampToBorder = 3,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum FilterMode {
    #[default]
    Nearest = 0,
    Linear = 1,
}

pub enum CompareFunction {
    Never = 1,
    Less = 2,
    Equal = 3,
    LessEqual = 4,
    Greater = 5,
    NotEqual = 6,
    GreaterEqual = 7,
    Always = 8,
}

pub enum SamplerBorderColor {
    TransparentBlack,
    OpaqueBlack,
    OpaqueWhite,
    Zero,
}

pub struct TextureConfig {
    pub address_mode_u: AddressMode,
    pub address_mode_v: AddressMode,
    pub address_mode_w: AddressMode,
    pub mag_filter: FilterMode,
    pub min_filter: FilterMode,
    pub mipmap_filter: FilterMode,
    pub lod_min_clamp: f32,
    pub lod_max_clamp: f32,
    pub compare: Option<CompareFunction>,
    pub anisotropy_clamp: u16,
    pub border_color: Option<SamplerBorderColor>,
}

impl Default for TextureConfig {
    fn default() -> Self {
        Self {
            address_mode_u: Default::default(),
            address_mode_v: Default::default(),
            address_mode_w: Default::default(),
            mag_filter: Default::default(),
            min_filter: Default::default(),
            mipmap_filter: Default::default(),
            lod_min_clamp: 0.0,
            lod_max_clamp: 32.0,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LoadOp {
    Clear(Color),
    Load,
}

#[derive(Debug, Default, Clone)]
pub struct PipelineConfig<'a> {
    pub shader_source: &'a str,
    pub vertex_buffer_layouts: &'a [wgpu::VertexBufferLayout<'a>],
    pub bind_group_layouts: &'a [(Order, &'a wgpu::BindGroupLayout)],
    pub depth_stencil_write_enabled: bool,
}
