use common::prelude::DataType;

pub(crate) enum DataCommand {
    Start(u32, Vec<String>, DataType),
    Stop(u32),
    StopAll,
}
