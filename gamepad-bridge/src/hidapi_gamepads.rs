use crate::{hidapi_fn::GamepadModel, hidapi_gamepad::*};

fn read(model: GamepadModel) {
    let read_input: [u8; 48] = [0 as u8; 48];
    let mut gamepad_output: UniversalGamepad = UniversalGamepad::nothing_pressed();

    match model {
        GamepadModel::PS5 => {
            gamepad_output.bumpers = Bumpers {
                left: match read_input[1] {
                    1 => true,
                    _ => false,
                },
                right: match read_input[2] {
                    2 => true,
                    _ => false,
                },
            };
        }
        GamepadModel::PS4 => {}
    }
}
