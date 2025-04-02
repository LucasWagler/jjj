#[macro_export]
macro_rules! join {
    ($sep:literal, [$head:literal $(, $tail:literal)*] ) => {
        concat!( $head $(, $sep, $tail)* )
    }
}
