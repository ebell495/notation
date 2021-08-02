#![feature(arc_new_cyclic)]

pub use notation_proto;

pub mod entry;
pub mod bar;
pub mod slice;
pub mod parse;
pub mod section;
pub mod tab;
pub mod track;
pub mod util;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::entry::ModelEntry;
    #[doc(hidden)]
    pub use crate::bar::{Bar, BarLayer};
    #[doc(hidden)]
    pub use crate::slice::{Slice};
    #[doc(hidden)]
    pub use crate::parse::ParseError;
    #[doc(hidden)]
    pub use crate::section::{Form, Section};
    #[doc(hidden)]
    pub use crate::tab::{Tab, TabBar};
    #[doc(hidden)]
    pub use crate::track::Track;
    #[doc(hidden)]
    pub use notation_proto::prelude::Tab as ProtoTab;
    #[doc(hidden)]
    pub use notation_proto::prelude::*;
}
