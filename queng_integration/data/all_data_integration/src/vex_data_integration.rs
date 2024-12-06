use vex_data_integration::ImsVexDataIntegration;

const ID: &str = "VexDataIntegration";

#[derive(Debug, Default, Clone, Copy)]
pub struct VexDataIntegration {
    integration: ImsVexDataIntegration,
}

impl VexDataIntegration {
    pub fn new() -> Self {
        let integration = ImsVexDataIntegration::new();
        Self { integration }
    }
}
