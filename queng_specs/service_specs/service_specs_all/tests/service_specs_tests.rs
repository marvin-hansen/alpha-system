use service_specs_all::prelude::get_all_service_specs;

#[test]
fn test_get_all_service_specs() {
    let service_specs = get_all_service_specs();

    assert_eq!(service_specs.len(), 4); // Assuming there are 6 service specs in total
}
