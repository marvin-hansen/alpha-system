use common::prelude::DataType;

pub(crate) enum Command {
    StartData(Vec<String>, DataType),
    StopData(u16),
    StopAllData,
    ReconnectData(u16),
}
