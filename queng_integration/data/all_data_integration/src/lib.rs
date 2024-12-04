use enum_dispatch::enum_dispatch;
use mock_data_integration::MockDataIntegration;

#[enum_dispatch]
pub enum ImsDataIntegration {
    MockDataIntegration,
}
