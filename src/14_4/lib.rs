#[macro_export]
macro_rules! impl_display {
    ($type: ty, $field: ident) => {
        impl ::std::fmt::Display for $type {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", self.$field)
            }
        }
    };
    ($type: ty: $first: ident $($field: ident)*) => {
        impl ::std::fmt::Display for $type {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", self.$first)?;
                $(write!(f, " {}", self.$field)?;)*
                Ok(())
            }
        }
    };
    ($type: ty: $first: ident, $($field: ident),*) => {
        impl ::std::fmt::Display for $type {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", self.$first)?;
                $(write!(f, ", {}", self.$field)?;)*
                Ok(())
            }
        }
    };
}