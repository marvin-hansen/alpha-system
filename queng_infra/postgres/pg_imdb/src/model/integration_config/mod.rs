use crate::model::integration_message_config_type::MessageConfig;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};

mod integration_config_impl;
mod integration_config_type_conversion;
mod integration_config_type_create_conversion;
mod integration_config_type_update_conversion;

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name=crate::schema::imdb::integration_config,  primary_key(integration_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IntegrationConfig {
    integration_id: String,
    integration_version: i32,
    ims_integration_type: i32,
    online: bool,
    exchange_id: i32,
    integration_message_config: MessageConfig,
}

#[derive(Debug, Clone, Queryable, Insertable)]
#[diesel(table_name=crate::schema::imdb::integration_config,  primary_key(integration_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateIntegrationConfig {
    integration_id: String,
    integration_version: i32,
    ims_integration_type: i32,
    online: bool,
    exchange_id: i32,
    integration_message_config: MessageConfig,
}

impl CreateIntegrationConfig {
    pub fn new(
        integration_id: String,
        integration_version: i32,
        ims_integration_type: i32,
        online: bool,
        exchange_id: i32,
        integration_message_config: MessageConfig,
    ) -> Self {
        Self {
            integration_id,
            integration_version,
            ims_integration_type,
            online,
            exchange_id,
            integration_message_config,
        }
    }
}

#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::imdb::integration_config,  primary_key(integration_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateIntegrationConfig {
    integration_id: String,
    integration_version: i32,
    ims_integration_type: i32,
    online: bool,
    exchange_id: i32,
    integration_message_config: MessageConfig,
}

impl UpdateIntegrationConfig {
    pub fn new(
        integration_id: String,
        integration_version: i32,
        ims_integration_type: i32,
        online: bool,
        exchange_id: i32,
        integration_message_config: MessageConfig,
    ) -> Self {
        Self {
            integration_id,
            integration_version,
            ims_integration_type,
            online,
            exchange_id,
            integration_message_config,
        }
    }
}
