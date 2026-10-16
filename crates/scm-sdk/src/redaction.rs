//! Shared safe formatting for option objects containing caller data.
macro_rules! redacted_debug {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl std::fmt::Debug for $ty {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(concat!(stringify!($ty), "([REDACTED])"))
                }
            }
        )+
    };
}
pub(crate) use redacted_debug;
