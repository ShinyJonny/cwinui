/// Computes the offset of coordinates [`x`, `y`] in a flat buffer where rows of
/// size `width` are stored sequentially.
///
/// Automatically casts all values to `usize`.
macro_rules! offset {
    ($x:expr, $y:expr, $width:expr) => {
        $y as usize * $width as usize + $x as usize
    }
}

pub(crate) use offset;


/// Const version of `std::cmp::max`.
macro_rules! max {
    ($a:expr, $b:expr) => {
        if $a <= $b { $b } else { $a }
    }
}

pub(crate) use max;


/// Const version of `std::cmp::min`.
macro_rules! min {
    ($a:expr, $b:expr) => {
        if $a <= $b { $a } else { $b }
    }
}
pub(crate) use min;
