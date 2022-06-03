mod span;
pub use span::{Span, Spanned};

pub type SourceId = usize;

#[macro_export]
macro_rules! trace {
    ($x: expr) => {
        #[cfg(feature = "trace")]
        {
            println!("{}", $x)
        }
    };
}
