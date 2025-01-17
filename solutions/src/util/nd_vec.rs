#[macro_export]
macro_rules! nd_vec {
    [ $( [ $( $x:expr ),* $(,)? ] ),* $(,)? ] => {
        vec![$(nd_vec![$($x),*]),*]
    };

    [ $( $x:expr ),* $(,)? ] => {
        vec![$($x.into()),*]
    };
}
