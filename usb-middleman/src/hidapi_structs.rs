pub struct UniversalController {
    pub sticks: Sticks,
    pub triggers: Triggers,
    pub bumpers: Bumpers,
    pub buttons: MainButtons,
    pub specials: SpecialButtons,
}
pub struct Sticks {
    pub left: Stick,
    pub right: Stick,
}
pub struct Triggers {
    pub left: u8,
    pub right: u8,
}
pub struct Bumpers {
    pub left: bool,
    pub right: bool,
}
impl Bumpers {
    pub fn allfalse() -> Self {
        Self {
            left: false,
            right: false,
        }
    }
}

pub struct MainButtons {
    pub dpad_up: bool,
    pub dpad_down: bool,
    pub dpad_left: bool,
    pub dpad_right: bool,
    pub upper: bool,
    pub lower: bool,
    pub left: bool,
    pub right: bool,
}
impl MainButtons {
    pub fn allfalse() -> Self {
        Self {
            dpad_up: false,
            dpad_down: false,
            dpad_left: false,
            dpad_right: false,
            upper: false,
            lower: false,
            left: false,
            right: false,
        }
    }
}

pub struct SpecialButtons {
    pub touchpad: bool,

    /// menu button
    pub right: bool,

    /// Share button for PS Controllers
    pub left: bool,

    /// PS-Button or XBOX Button
    pub logo: bool,
}
impl SpecialButtons {
    pub fn allfalse() -> Self {
        Self {
            touchpad: false,
            right: false,
            left: false,
            logo: false,
        }
    }
}

pub struct Stick {
    pub x: u8,
    pub y: u8,
    pub pressed: bool,
}
