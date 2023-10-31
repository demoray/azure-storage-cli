/// args!(a, b) expands to the following:
/// ````
/// if let Some(value) = b {
///     a = a.b(value);
/// }
/// ```
///
/// `args!(a, b, c)` expands to:
///
/// ```rust
/// if let Some(value) = c {
///     a = a.c(value);
/// }
/// if let Some(value) = b {
///     a = a.b(value);
/// }
/// ```
#[macro_export]
macro_rules! args {
    ($builder:ident, $name:ident) => {
        if let Some(value) = $name {
            $builder = $builder.$name(value);
        }
    };
    ($builder:ident, $($name:ident),+) => {
        $(args!($builder, $name);)*
    };
}
