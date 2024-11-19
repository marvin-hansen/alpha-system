use common_metadata::MetaExchange;

#[must_use]
pub fn meta_exchange_to_proto_exchange(
    meta_exchange: &MetaExchange,
) -> proto_mddb::proto::ProtoMetaExchange {
    proto_mddb::proto::ProtoMetaExchange {
        exchange_code: meta_exchange.code.to_string(),
        exchange_name: meta_exchange.name.to_string(),
        exchange_hash: meta_exchange.hash(),
    }
}

#[must_use]
pub fn proto_exchange_to_meta_exchange(
    proto_exchange: &proto_mddb::proto::ProtoMetaExchange,
) -> MetaExchange {
    MetaExchange {
        code: proto_exchange.exchange_code.to_string(),
        name: proto_exchange.exchange_name.to_string(),
        kaiko_legacy_slug: String::new(),
    }
}
