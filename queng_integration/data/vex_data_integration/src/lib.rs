#[derive(Debug, Clone, Copy)]
pub struct ImsVexDataIntegration;

impl Default for ImsVexDataIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl ImsVexDataIntegration {
    pub fn new() -> Self {
        Self {}
    }
}
//
// impl ImsDataIntegration for ImsVexDataIntegration {
// }
