#[macro_export]
macro_rules! is_bit_offset_set {
    ($val:expr, $offset:expr) => { 
        if $val & (1 << $offset) == 1 {
            true
        } else {
            false
        }
    }
}

#[macro_export]
macro_rules! is_bit_set {
    ($val:expr, $bit:expr) => {
        if $val & $bit == 1 {
            true
        } else {
            false
        }
    }
}
