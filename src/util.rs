macro_rules! pos {
    ( $width:expr, $y:expr, $x:expr ) => {
        $y * $width + $x
    }
}

pub(crate) use pos;
