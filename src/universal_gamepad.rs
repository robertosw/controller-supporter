pub struct UniversalGamepad {
    pub sticks: Sticks,
    pub triggers: Triggers,
    pub buttons: Buttons,
    pub other: Other,
}
impl UniversalGamepad {
    pub fn nothing_pressed() -> Self {
        Self {
            sticks: Sticks {
                left: Stick { x: 0, y: 0, pressed: false },
                right: Stick { x: 0, y: 0, pressed: false },
            },
            triggers: Triggers { left: 0, right: 0 },
            buttons: Buttons {
                bumpers: Bumpers { left: false, right: false },
                dpad: DPad {
                    up: false,
                    down: false,
                    left: false,
                    right: false,
                },
                main: MainButtons {
                    upper: false,
                    lower: false,
                    left: false,
                    right: false,
                },
                specials: SpecialButtons {
                    right: false,
                    left: false,
                    logo: false,
                },
            },
            other: Other {
                touchpad: None,
                gyroscope: None,
            },
        }
    }
}

// ----- //

pub struct Sticks {
    pub left: Stick,
    pub right: Stick,
}
impl Sticks {
    pub fn allfalse() -> Self {
        Self {
            left: Stick { x: 0, y: 0, pressed: false },
            right: Stick { x: 0, y: 0, pressed: false },
        }
    }
}

pub struct Stick {
    pub x: u8,
    pub y: u8,
    pub pressed: bool,
}

// ----- //

pub struct Triggers {
    pub left: u8,
    pub right: u8,
}

// ----- //

pub struct Buttons {
    pub bumpers: Bumpers,
    pub dpad: DPad,
    pub main: MainButtons,
    pub specials: SpecialButtons,
}

pub struct Bumpers {
    pub left: bool,
    pub right: bool,
}
impl Bumpers {
    pub fn allfalse() -> Self {
        Self { left: false, right: false }
    }
}

pub struct DPad {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}
impl DPad {
    pub fn allfalse() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}

pub struct MainButtons {
    pub upper: bool,
    pub lower: bool,
    pub left: bool,
    pub right: bool,
}
impl MainButtons {
    pub fn allfalse() -> Self {
        Self {
            upper: false,
            lower: false,
            left: false,
            right: false,
        }
    }
}

pub struct SpecialButtons {
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
            right: false,
            left: false,
            logo: false,
        }
    }
}

// ----- //

pub struct Other {
    pub touchpad: Option<Touchpad>,
    pub gyroscope: Option<Gyroscope>,
}

pub struct Gyroscope {
    pub x_coord: u8,
    pub y_coord: u8,
    pub z_coord: u8,
}

pub struct Touchpad {
    pub x_coord: u8,
    pub y_coord: u8,
    pub touched: bool,
    pub pressed: bool,
}
