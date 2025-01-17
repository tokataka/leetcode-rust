#[macro_export]
macro_rules! option_vec {
    [ $( $x:tt ),* $(,)? ] => {
        vec![
            $(match stringify!($x) {
                "null" => None,
                x => Some(x.parse().expect("Usage: option_vec![any | null, ...]"))
            }),*
        ]
    };
}
