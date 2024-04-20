macro_rules! offset {
    ( $x:expr, $y:expr, $width:expr ) => {
        $y * $width + $x
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
