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

#[macro_export]
macro_rules! output_stream_entries {
    ($stream:expr, $entry_name:ident) => {{
        use futures::StreamExt;
        use serde::ser::SerializeSeq;
        use serde::Serializer;

        let mut stream = $stream;
        let mut ser = serde_json::Serializer::with_formatter(
            std::io::stdout(),
            serde_json::ser::PrettyFormatter::new(),
        );
        let mut serializer = ser.serialize_seq(None)?;

        while let Some(item) = stream.next().await {
            let item = item?;
            for entry in paste::paste! { item. $entry_name } {
                serializer.serialize_element(&entry)?;
            }
        }
        serializer.end()?;
    }};
}

#[macro_export]
macro_rules! output_stream_entries_debug {
    ($stream:expr, $entry_name:expr) => {{
        use futures::StreamExt;
        let mut stream = $stream;
        while let Some(item) = stream.next().await {
            let item = item?;
            for entry in paste::paste! { item. $entry_name } {
                log::debug!("{entry:#?}");
            }
        }
    }};
}
