use std::{path::PathBuf, thread::JoinHandle};

#[derive(Clone)]
pub struct GameControllerSimple {
    pub path: PathBuf,
    pub mac: String,
}

pub struct GameController {
    pub path: PathBuf,
    pub mac: String,
    pub thread_handle: Option<JoinHandle<()>>,
}

pub struct GameControllerCollection {
    pub first: Option<GameController>,
    pub second: Option<GameController>,
}
impl GameControllerCollection {
    /// How many controllers are being used?
    pub fn len(&self) -> u8 {
        let mut _count: u8 = 0;

        match &self.first {
            None => (),
            Some(_) => _count += 1,
        };
        match &self.second {
            None => (),
            Some(_) => _count += 1,
        };

        return _count;
    }

    /// Is this Controller already known?
    pub fn contains(&self, new_ctrl: GameControllerSimple) -> bool {
        let mut _first_contains: bool;
        let mut _second_contains: bool;

        match &self.first {
            None => _first_contains = false,
            Some(ctrl) => {
                if ctrl.mac == new_ctrl.mac {
                    _first_contains = true;
                }
                _first_contains = false;
            }
        };
        match &self.second {
            None => _second_contains = false,
            Some(ctrl) => {
                if ctrl.mac == new_ctrl.mac {
                    _second_contains = true;
                }
                _second_contains = false;
            }
        };

        return _first_contains || _second_contains;
    }

    /// WARNING: If the collection is already full, nothing will be changed <br>
    /// Adds the given controller to the collection in the top most place
    pub fn add(&mut self, new_ctrl: GameControllerSimple) {
        match &self.first {
            None => {
                self.first = Some(GameController {
                    path: new_ctrl.path,
                    mac: new_ctrl.mac,
                    thread_handle: None,
                });
                return;
            }
            Some(_) => (),
        };

        match &self.second {
            None => {
                self.second = Some(GameController {
                    path: new_ctrl.path,
                    mac: new_ctrl.mac,
                    thread_handle: None,
                });
                return;
            }
            Some(_) => (),
        };
    }
}
