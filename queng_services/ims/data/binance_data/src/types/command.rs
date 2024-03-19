use common::prelude::DataType;

pub(crate) enum Command {
    StartData(u32, Vec<String>, DataType),
    StopData(u32),
    StopAllData,
    ReconnectData(u32),
}
