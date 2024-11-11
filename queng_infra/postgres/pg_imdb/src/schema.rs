// @generated automatically by Diesel CLI.

pub mod imdb {
    pub mod sql_types {
        #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "integration_message_config", schema = "imdb"))]
        pub struct IntegrationMessageConfig;
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::IntegrationMessageConfig;

        imdb.integration_config (integration_id) {
            #[max_length = 255]
            integration_id -> Varchar,
            integration_version -> Int2,
            ims_integration_type -> Int2,
            online -> Bool,
            exchange_id -> Int2,
            integration_message_config -> IntegrationMessageConfig,
        }
    }
}
