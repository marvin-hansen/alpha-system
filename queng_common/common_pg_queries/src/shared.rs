use common_config::prelude::{Endpoint, ServiceID};

pub(crate) fn service_ids_to_string(ids: &[ServiceID]) -> String {
    let id_strings: Vec<String> = ids.iter().map(|id| id.as_u8().to_string()).collect();
    format!("{{{}}}", id_strings.join(","))
}

//  ARRAY[ row(name, version, base_uri, port, protocol)::Endpoint,
//         row(name, version, base_uri, port, protocol)::Endpoint
//      ]

pub(crate) fn service_endpoints_to_string(endpoints: &[Endpoint]) -> String {
    let mut endpoints_strings: Vec<String> = Vec::new();

    for e in endpoints.iter() {
        endpoints_strings.push(format!(
            "row('{}',{},'{}',{},'{}')::Endpoint",
            e.name(),
            e.version(),
            e.uri(),
            e.port(),
            e.protocol().to_string()
        ));
    }

    format!("ARRAY[{}]", endpoints_strings.join(","))
}
