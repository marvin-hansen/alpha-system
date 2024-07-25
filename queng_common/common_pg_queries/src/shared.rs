use common_config::prelude::ServiceID;

pub(crate) fn service_ids_to_string(ids: &[ServiceID]) -> String {
    let id_strings: Vec<String> = ids.iter().map(|id| id.as_u8().to_string()).collect();
    format!("{{{}}}", id_strings.join(","))
}
