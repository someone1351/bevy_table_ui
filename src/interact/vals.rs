
#[derive(PartialEq,Debug,Copy,Clone)]
pub enum FocusMove {Left,Right,Up,Down,Prev,Next}

impl FocusMove {
    pub fn ind(&self) -> Option<usize> {
        match self {
            Self::Left => Some(0),
            Self::Up => Some(1),
            Self::Right => Some(2),
            Self::Down => Some(3),
            Self::Prev => None,
            Self::Next => None,
        }
    }
    pub fn rev(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Prev => Self::Next,
            Self::Next => Self::Prev,
        }
    }

    pub fn horizontal(&self) -> bool {
        FocusMove::Left.eq(self) || FocusMove::Right.eq(self)
    }
    pub fn vertical(&self) -> bool {
        FocusMove::Up.eq(self) || FocusMove::Down.eq(self)
    }
    pub fn positive(&self) -> bool {
        FocusMove::Down.eq(self) || FocusMove::Right.eq(self) || FocusMove::Next.eq(self)
    }
    pub fn negative(&self) -> bool {
        FocusMove::Up.eq(self) || FocusMove::Left.eq(self) || FocusMove::Prev.eq(self)
    }
    pub fn tab(&self) -> bool {
        FocusMove::Prev.eq(self) || FocusMove::Next.eq(self)
    }
}