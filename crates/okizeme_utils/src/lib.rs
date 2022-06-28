pub fn countdown(val: u8) -> u8 {
    if val > 0 {
        val - 1
    } else {
        0
    }
}

pub trait MotionGroups {
    fn y_negative(&self) -> bool;
    fn y_positive(&self) -> bool;
    fn x_positive(&self) -> bool;
    fn x_negative(&self) -> bool;
}

impl MotionGroups for u8 {
    fn x_positive(&self) -> bool {
        matches!(self, 6 | 3 | 9)
    }

    fn y_positive(&self) -> bool {
        matches!(self, 7 | 8 | 9)
    }

    fn x_negative(&self) -> bool {
        matches!(self, 4 | 1 | 7)
    }

    fn y_negative(&self) -> bool {
        matches!(self, 1 | 2 | 3)
    }
}

