#[macro_export]
macro_rules! nd_vec {
    [ $( [ $( $x:expr ),* $(,)? ] ),* $(,)? ] => {
        vec![$(nd_vec![$($x),*]),*]
    };

    [ $( $x:expr ),* $(,)? ] => {
        vec![$($x),*]
    };
}

#[macro_export]
macro_rules! nd_vec_string {
    [ $( [ $( $x:expr ),* $(,)? ] ),* $(,)? ] => {
        vec![$(nd_vec_string![$($x),*]),*]
    };

    [ $( $x:expr ),* $(,)? ] => {
        vec![$($x.to_owned()),*]
    };
}
