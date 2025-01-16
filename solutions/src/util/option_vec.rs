#[macro_export]
macro_rules! option_vec {
    [ $( $x:tt ),* $(,)? ] => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(
                    match stringify!($x) {
                        "null" => None,
                        x => Some(x.parse().expect("Usage: option_vec![any | null, ...]"))
                    }
                );
            )*

            temp_vec
        }
    };
}
