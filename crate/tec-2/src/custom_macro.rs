#[macro_export]
macro_rules! map {
    () => (
        std::collections::HashMap::new()
    );
    ($($key:expr => $val:expr),+ $(,)?) => {
        vec![$(($key, $val)),+].into_iter().collect::<std::collections::HashMap<_, _>>()
    }
}

#[macro_export]
macro_rules! to_bytes {
    ($val:expr, $len:literal) => (
        {
            let mut bits = [0; $len];
            for i in 0..$len {
                bits[i] = ($val >> ($len - 1 - i)) & 1;
            }
            bits
        }
    )
}

