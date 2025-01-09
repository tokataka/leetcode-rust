#[macro_export]
macro_rules! nd_vec {
    [ $( [ $( $x:expr ),* $(,)? ] ),* $(,)? ] => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(nd_vec![$($x),*]);
            )*
            temp_vec
        }
    };

    [ $( $x:expr ),* $(,)? ] => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x.into());
            )*
            temp_vec
        }
    };
}
