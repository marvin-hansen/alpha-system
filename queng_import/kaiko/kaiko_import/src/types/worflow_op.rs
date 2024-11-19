use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MetaDataDBWOp {
    all_op: WorkflowOpAll,
    assets_op: WorkflowOp,
    exchanges_op: WorkflowOp,
    instruments_op: WorkflowOp,
}

impl MetaDataDBWOp {
    #[must_use]
    pub const fn new(
        all_op: WorkflowOpAll,
        assets_op: WorkflowOp,
        exchanges_op: WorkflowOp,
        instruments_op: WorkflowOp,
    ) -> Self {
        Self {
            all_op,
            assets_op,
            exchanges_op,
            instruments_op,
        }
    }
}

impl MetaDataDBWOp {
    #[must_use]
    pub const fn all_op(&self) -> WorkflowOpAll {
        self.all_op
    }

    #[must_use]
    pub const fn assets_op(&self) -> WorkflowOp {
        self.assets_op
    }

    #[must_use]
    pub const fn exchanges_op(&self) -> WorkflowOp {
        self.exchanges_op
    }

    #[must_use]
    pub const fn instruments_op(&self) -> WorkflowOp {
        self.instruments_op
    }
}

impl Display for MetaDataDBWOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum WorkflowOpAll {
    NoOPAll, // Nothing to do.
    ImportAll,
    ImportSample(usize, usize, usize),
    UpdateAll,
    UpdatePartial,
    ImportPartial,
}

impl Display for WorkflowOpAll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum WorkflowOp {
    NoOP, // Nothing to do.
    ImportAssets,
    ImportExchanges,
    ImportInstruments,
    UpdateAssets,
    UpdateExchanges,
    UpdateInstruments,
}

impl Display for WorkflowOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}
