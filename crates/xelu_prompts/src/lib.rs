// https://thoseawesomeguys.com/prompts/
use bevy::input::gamepad::GamepadButtonType;
use bevy::input::keyboard::KeyCode;
use leafwing_input_manager::user_input::InputKind;

pub fn texture_path_for_input(kind: &InputKind) -> Option<&'static str> {
    match kind {
        InputKind::Keyboard(key) => {
            match key {
                KeyCode::Key0 => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/0_Key_Dark.png"),
                KeyCode::Key1 => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/1_Key_Dark.png"),
                KeyCode::Key2 => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/2_Key_Dark.png"),
                KeyCode::Key3 => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/3_Key_Dark.png"),
                KeyCode::Key4 => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/4_Key_Dark.png"),
                KeyCode::Key5 => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/5_Key_Dark.png"),
                KeyCode::Key6 => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/6_Key_Dark.png"),
                KeyCode::Key7 => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/7_Key_Dark.png"),
                KeyCode::Key8 => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/8_Key_Dark.png"),
                KeyCode::Key9 => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/9_Key_Dark.png"),

                KeyCode::Numpad0 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/0_Key_Dark.png")
                }
                KeyCode::Numpad1 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/1_Key_Dark.png")
                }
                KeyCode::Numpad2 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/2_Key_Dark.png")
                }
                KeyCode::Numpad3 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/3_Key_Dark.png")
                }
                KeyCode::Numpad4 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/4_Key_Dark.png")
                }
                KeyCode::Numpad5 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/5_Key_Dark.png")
                }
                KeyCode::Numpad6 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/6_Key_Dark.png")
                }
                KeyCode::Numpad7 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/7_Key_Dark.png")
                }
                KeyCode::Numpad8 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/8_Key_Dark.png")
                }
                KeyCode::Numpad9 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/9_Key_Dark.png")
                }

                KeyCode::F1 => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F1_Key_Dark.png"),
                KeyCode::F2 => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F2_Key_Dark.png"),
                KeyCode::F3 => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F3_Key_Dark.png"),
                KeyCode::F4 => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F4_Key_Dark.png"),
                KeyCode::F5 => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F5_Key_Dark.png"),
                KeyCode::F6 => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F6_Key_Dark.png"),
                KeyCode::F7 => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F7_Key_Dark.png"),
                KeyCode::F8 => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F8_Key_Dark.png"),
                KeyCode::F9 => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F9_Key_Dark.png"),
                KeyCode::F10 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F10_Key_Dark.png")
                }
                KeyCode::F11 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F11_Key_Dark.png")
                }
                KeyCode::F12 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F12_Key_Dark.png")
                }
                KeyCode::F13 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F13_Key_Dark.png")
                }
                KeyCode::F14 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F14_Key_Dark.png")
                }
                KeyCode::F15 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F15_Key_Dark.png")
                }
                KeyCode::F16 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F16_Key_Dark.png")
                }
                KeyCode::F17 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F17_Key_Dark.png")
                }
                KeyCode::F18 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F18_Key_Dark.png")
                }
                KeyCode::F19 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F19_Key_Dark.png")
                }
                KeyCode::F20 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F20_Key_Dark.png")
                }
                KeyCode::F21 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F21_Key_Dark.png")
                }
                KeyCode::F22 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F22_Key_Dark.png")
                }
                KeyCode::F23 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F23_Key_Dark.png")
                }
                KeyCode::F24 => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F24_Key_Dark.png")
                }

                // Alt_Key_Dark.png
                KeyCode::AltLeft => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Alt_Key_Dark.png")
                }
                KeyCode::AltRight => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Alt_Key_Dark.png")
                }
                // Arrow_Down_Key_Dark.png
                KeyCode::Down => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Down_Key_Dark.png")
                }
                // Arrow_Left_Key_Dark.png
                KeyCode::Left => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Left_Key_Dark.png")
                }
                // Arrow_Right_Key_Dark.png
                KeyCode::Right => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Right_Key_Dark.png")
                }
                // Arrow_Up_Key_Dark.png
                KeyCode::Up => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Up_Key_Dark.png"),
                // Asterisk_Key_Dark.png
                KeyCode::Asterisk => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Asterisk_Key_Dark.png")
                }
                // Backspace_Alt_Key_Dark.png
                // Backspace_Key_Dark.png
                // KeyCode::Back => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Backspace_Alt_Key_Dark.png"),
                KeyCode::Back => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Backspace_Key_Dark.png")
                }
                // Bracket_Left_Key_Dark.png
                KeyCode::BracketLeft => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Bracket_Left_Key_Dark.png")
                }
                // Bracket_Right_Key_Dark.png
                KeyCode::BracketRight => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Bracket_Right_Key_Dark.png")
                }
                // Caps_Lock_Key_Dark.png
                KeyCode::Capital => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Caps_Lock_Key_Dark.png")
                }
                // Command_Key_Dark.png
                KeyCode::SuperLeft => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Command_Key_Dark.png")
                }
                // Ctrl_Key_Dark.png
                KeyCode::ControlLeft => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Ctrl_Key_Dark.png")
                }
                KeyCode::ControlRight => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Ctrl_Key_Dark.png")
                }
                // Del_Key_Dark.png
                KeyCode::Delete => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Del_Key_Dark.png")
                }
                // End_Key_Dark.png
                KeyCode::End => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/End_Key_Dark.png")
                }
                // Enter_Alt_Key_Dark.png
                // KeyCode::Return => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Enter_Alt_Key_Dark.png"),
                // Enter_Tall_Key_Dark.png
                // KeyCode::Return => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Enter_Tall_Key_Dark.png"),
                // Enter_Key_Dark.png
                KeyCode::Return => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Enter_Key_Dark.png")
                }

                // Esc_Key_Dark.png
                KeyCode::Escape => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Esc_Key_Dark.png")
                }
                // Home_Key_Dark.png
                KeyCode::Home => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Home_Key_Dark.png")
                }
                // Insert_Key_Dark.png
                KeyCode::Insert => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Insert_Key_Dark.png")
                }
                // Mark_Left_Key_Dark.png "<"
                // Mark_Right_Key_Dark.png ">"
                // Minus_Key_Dark.png
                KeyCode::Minus => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Minus_Key_Dark.png")
                }
                // Mouse_Left_Key_Dark.png
                // Mouse_Middle_Key_Dark.png
                // Mouse_Right_Key_Dark.png
                // Mouse_Simple_Key_Dark.png
                // Num_Lock_Key_Dark.png
                KeyCode::Numlock => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Num_Lock_Key_Dark.png")
                }
                // Page_Down_Key_Dark.png
                KeyCode::PageDown => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Page_Down_Key_Dark.png")
                }
                // Page_Up_Key_Dark.png
                KeyCode::PageUp => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Page_Up_Key_Dark.png")
                }
                // Plus_Key_Dark.png
                KeyCode::Plus => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Plus_Key_Dark.png")
                }
                // Plus_Tall_Key_Dark.png
                KeyCode::NumpadAdd => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Plus_Tall_Key_Dark.png")
                }
                // Print_Screen_Key_Dark.png
                KeyCode::Snapshot => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Print_Screen_Key_Dark.png")
                }
                // Question_Key_Dark.png
                KeyCode::Slash => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Question_Key_Dark.png")
                }
                // Quote_Key_Dark.png
                KeyCode::Apostrophe => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Quote_Key_Dark.png")
                }
                // Semicolon_Key_Dark.png
                KeyCode::Semicolon => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Semicolon_Key_Dark.png")
                }
                // Shift_Alt_Key_Dark.png
                KeyCode::ShiftLeft => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Shift_Alt_Key_Dark.png")
                }
                // Shift_Key_Dark.png
                KeyCode::ShiftRight => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Shift_Key_Dark.png")
                }
                // Slash_Key_Dark.png
                KeyCode::Backslash => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Slash_Key_Dark.png")
                }
                // Space_Key_Dark.png
                KeyCode::Space => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Space_Key_Dark.png")
                }
                // Tab_Key_Dark.png
                KeyCode::Tab => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Tab_Key_Dark.png")
                }
                // Tilda_Key_Dark.png
                KeyCode::Grave => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Tilda_Key_Dark.png")
                }
                // Win_Key_Dark.png
                KeyCode::SuperRight => {
                    Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Win_Key_Dark.png")
                }

                KeyCode::A => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/A_Key_Dark.png"),
                KeyCode::B => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/B_Key_Dark.png"),
                KeyCode::C => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/C_Key_Dark.png"),
                KeyCode::D => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/D_Key_Dark.png"),
                KeyCode::E => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/E_Key_Dark.png"),
                KeyCode::F => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/F_Key_Dark.png"),
                KeyCode::G => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/G_Key_Dark.png"),
                KeyCode::H => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/H_Key_Dark.png"),
                KeyCode::I => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/I_Key_Dark.png"),
                KeyCode::J => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/J_Key_Dark.png"),
                KeyCode::K => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/K_Key_Dark.png"),
                KeyCode::L => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/L_Key_Dark.png"),
                KeyCode::M => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/M_Key_Dark.png"),
                KeyCode::N => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/N_Key_Dark.png"),
                KeyCode::O => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/O_Key_Dark.png"),
                KeyCode::P => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/P_Key_Dark.png"),
                KeyCode::Q => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Q_Key_Dark.png"),
                KeyCode::R => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/R_Key_Dark.png"),
                KeyCode::S => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/S_Key_Dark.png"),
                KeyCode::T => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/T_Key_Dark.png"),
                KeyCode::U => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/U_Key_Dark.png"),
                KeyCode::V => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/V_Key_Dark.png"),
                KeyCode::W => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/W_Key_Dark.png"),
                KeyCode::X => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/X_Key_Dark.png"),
                KeyCode::Y => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Y_Key_Dark.png"),
                KeyCode::Z => Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Z_Key_Dark.png"),
                _ => None,
            }
        }
        InputKind::Mouse(button) => match button {
            bevy::input::mouse::MouseButton::Left => {
                Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Mouse_Left_Key_Dark.png")
            }
            bevy::input::mouse::MouseButton::Right => {
                Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Mouse_Right_Key_Dark.png")
            }
            bevy::input::mouse::MouseButton::Middle => {
                Some("textures/xelu_prompts/Keyboard & Mouse/Dark/Mouse_Middle_Key_Dark.png")
            }
            bevy::input::mouse::MouseButton::Other(_) => None,
        },
        InputKind::GamepadButton(button) => {
            match button {
                // XboxSeriesX_A.png
                GamepadButtonType::South => {
                    Some("textures/xelu_prompts/Xbox Series/XboxSeriesX_A.png")
                }
                // XboxSeriesX_B.png
                GamepadButtonType::East => {
                    Some("textures/xelu_prompts/Xbox Series/XboxSeriesX_B.png")
                }
                // XboxSeriesX_Diagram.png
                // XboxSeriesX_Diagram_Simple.png
                // XboxSeriesX_Dpad.png
                // XboxSeriesX_Dpad_Down.png
                GamepadButtonType::DPadDown => {
                    Some("textures/xelu_prompts/Xbox Series/XboxSeriesX_Dpad_Down.png")
                }
                // XboxSeriesX_Dpad_Left.png
                GamepadButtonType::DPadLeft => {
                    Some("textures/xelu_prompts/Xbox Series/XboxSeriesX_Dpad_Left.png")
                }
                // XboxSeriesX_Dpad_Right.png
                GamepadButtonType::DPadRight => {
                    Some("textures/xelu_prompts/Xbox Series/XboxSeriesX_Dpad_Right.png")
                }
                // XboxSeriesX_Dpad_Up.png
                GamepadButtonType::DPadUp => {
                    Some("textures/xelu_prompts/Xbox Series/XboxSeriesX_Dpad_Up.png")
                }
                // XboxSeriesX_LB.png
                GamepadButtonType::LeftTrigger => {
                    Some("textures/xelu_prompts/Xbox Series/XboxSeriesX_LB.png")
                }
                // XboxSeriesX_Left_Stick.png
                // GamepadButtonType::LeftStick => Some("textures/xelu_prompts/Xbox Series/XboxSeriesX_Left_Stick.png"),
                // XboxSeriesX_Left_Stick_Click.png
                GamepadButtonType::LeftThumb => {
                    Some("textures/xelu_prompts/Xbox Series/XboxSeriesX_Left_Stick_Click.png")
                }
                // XboxSeriesX_LT.png
                GamepadButtonType::LeftTrigger2 => {
                    Some("textures/xelu_prompts/Xbox Series/XboxSeriesX_LT.png")
                }
                // XboxSeriesX_Menu.png
                GamepadButtonType::Start => {
                    Some("textures/xelu_prompts/Xbox Series/XboxSeriesX_Menu.png")
                }
                // XboxSeriesX_RB.png
                GamepadButtonType::RightTrigger => {
                    Some("textures/xelu_prompts/Xbox Series/XboxSeriesX_RB.png")
                }
                // XboxSeriesX_Right_Stick.png
                // GamepadButtonType::RightStick => Some("textures/xelu_prompts/Xbox Series/XboxSeriesX_Right_Stick.png"),
                // XboxSeriesX_Right_Stick_Click.png
                GamepadButtonType::RightThumb => {
                    Some("textures/xelu_prompts/Xbox Series/XboxSeriesX_Right_Stick_Click.png")
                }
                // XboxSeriesX_RT.png
                GamepadButtonType::RightTrigger2 => {
                    Some("textures/xelu_prompts/Xbox Series/XboxSeriesX_RT.png")
                }
                // XboxSeriesX_Share.png
                // XboxSeriesX_View.png
                GamepadButtonType::Select => {
                    Some("textures/xelu_prompts/Xbox Series/XboxSeriesX_View.png")
                }
                // XboxSeriesX_X.png
                GamepadButtonType::West => {
                    Some("textures/xelu_prompts/Xbox Series/XboxSeriesX_X.png")
                }
                // XboxSeriesX_Y.png
                GamepadButtonType::North => {
                    Some("textures/xelu_prompts/Xbox Series/XboxSeriesX_Y.png")
                }
                _ => None,
            }
        }
        _ => None,
    }
}
