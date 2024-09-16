pub mod builder;
pub mod harness;
pub mod types;

pub mod prelude {
    pub use crate::builder::Builder as TestigoBuilder;
    pub use crate::harness::Harness as TestigoHarnes;
    pub use testigo_macro::testigo;
    pub use testigo_types::Test as TestigoTest;
}
