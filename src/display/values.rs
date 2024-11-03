use bevy::reflect::Reflect;


// #[derive(Debug)]
// pub struct UiTextAlignParseError;
// impl std::fmt::Display for UiTextAlignParseError { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f,"{self:?}") } }
// impl std::error::Error for UiTextAlignParseError { fn description(&self) -> &str { "" } }

#[derive(Reflect,Debug, Default, Clone,Copy,PartialEq,Eq)]
pub enum UiTextHAlign {
    #[default]
    Center,
    Left,
    Right,
}

#[derive(Reflect,Debug, Default, Clone,Copy,PartialEq,Eq)]
pub enum UiTextVAlign {
    #[default]
    Center,
    Top,
    Bottom,
}
impl ToString for UiTextHAlign {
    fn to_string(&self) -> String {
        match self {
            Self::Center => "center",
            Self::Left => "left",
            Self::Right => "right",
        }.to_string()
    }
}
impl ToString for UiTextVAlign {
    fn to_string(&self) -> String {
        match self {
            Self::Center => "center",
            Self::Top => "top",
            Self::Bottom => "bottom",
        }.to_string()
    }
}
impl std::str::FromStr for UiTextHAlign {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "left" => Ok(UiTextHAlign::Left),
            "right" => Ok(UiTextHAlign::Right),
            "center"|"" => Ok(UiTextHAlign::Center),
            _ => Err(())
        }
    }
}

impl std::str::FromStr for UiTextVAlign {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "top" => Ok(UiTextVAlign::Top),
            "bottom" => Ok(UiTextVAlign::Bottom),
            "center"|"" => Ok(UiTextVAlign::Center),
            _ => Err(())
        }
    }
}
