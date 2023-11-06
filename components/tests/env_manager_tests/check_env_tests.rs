use common::prelude::*;
use components::prelude::get_env_type;

#[test]
fn test_get_env_type() {
    assert_eq!(get_env_type(), EnvironmentType::UnknownEnv);
}