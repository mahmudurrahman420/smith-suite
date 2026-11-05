use std::collections::BTreeMap;
use texsmith_core::BakedMap;

#[derive(Clone, Debug, PartialEq)]
/// Row-major maps, with BakedMap::components() floats per texel
pub struct BakeOutput {
    pub resolution: u32,
    pub height: u32,
    pub maps: BTreeMap<BakedMap, Vec<f32>>,
    pub tracer: String,
}
