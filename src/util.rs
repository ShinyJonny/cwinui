macro_rules! offset {
    ( $x:expr, $y:expr, $width:expr ) => {
        $y * $width + $x
    }
}

pub(crate) use offset;
