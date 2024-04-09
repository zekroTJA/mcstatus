macro_rules! export {
    ($( $pkg:tt )*) => {
        $(
            mod $pkg;
            pub use $pkg::*;
        )*
    };
}

export! {
    index
    server
}
