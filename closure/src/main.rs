// let v: Vec<u32> = vec![1, 2, 3]; 实现vec!宏

#[macro_export]
macro_rules! vec {
    ( $( $x: expr ), * ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}