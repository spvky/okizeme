use std::fmt;

use bevy::{
    prelude::GamepadButtonType,
    reflect::{FromReflect, Reflect},
};

#[derive(Debug, Default, Clone, Copy, Reflect, FromReflect)]
#[repr(transparent)]
pub struct ButtonMask(pub u8);

impl ButtonMask {
    pub fn new(value: u8) -> Self {
        ButtonMask(value)
    }

    pub fn with_buttons(buttons: &str) -> Self {
        let mut binary_representation = 0_u8;
        for button in buttons.chars().into_iter() {
            let bit_to_set = match button {
                'a' => 0b0000_0001,
                'b' => 0b0000_0010,
                'c' => 0b0000_0100,
                'd' => 0b0000_1000,
                'e' => 0b0001_0000,
                'f' => 0b0010_0000,
                'g' => 0b0100_0000,
                'h' => 0b1000_0000,
                _ => 0,
            };
            binary_representation |= bit_to_set;
        }

        ButtonMask(binary_representation)
    }

    pub fn raw_value(&self) -> u8 {
        self.0
    }

    pub fn any(&self) -> bool {
        self.0 != 0
    }

    pub fn contains(&self, button: char) -> bool {
        let shift: u8 = match button {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return false,
        };

        self.is_bit_set(shift)
    }

    fn is_bit_set(&self, position: u8) -> bool {
        (self.0 & (1 << position)) != 0
    }
}

impl fmt::Display for ButtonMask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut button_string = String::new();
        if self.is_bit_set(0) {
            button_string.push('a')
        }
        if self.is_bit_set(1) {
            button_string.push('b')
        }
        if self.is_bit_set(2) {
            button_string.push('c')
        }
        if self.is_bit_set(3) {
            button_string.push('d')
        }
        if self.is_bit_set(4) {
            button_string.push('e')
        }
        if self.is_bit_set(5) {
            button_string.push('f')
        }
        if self.is_bit_set(6) {
            button_string.push('g')
        }
        if self.is_bit_set(7) {
            button_string.push('h')
        }
        write!(f, "{}", button_string)
    }
}

#[derive(Debug, Default, Clone, Copy, Reflect, FromReflect)]
#[repr(transparent)]
pub struct MotionMask(u8);

impl MotionMask {
    pub fn new(mut value: u8) -> Self {
        const LEFT_POS: u8 = 0;
        const RIGHT_POS: u8 = 1;
        const DOWN_POS: u8 = 2;
        const UP_POS: u8 = 3;

        const LEFT: u8 = 0b0000_0001;
        const RIGHT: u8 = 0b0000_0010;
        const DOWN: u8 = 0b0000_0100;

        fn is_bit_set(value: u8, position: u8) -> bool {
            (value & (1 << position)) != 0
        }

        fn unset_horizontal_bits(mut value: u8) -> u8 {
            value &= LEFT;
            value &= RIGHT;
            value
        }

        fn unset_the_down_bit(mut value: u8) -> u8 {
            value &= DOWN;
            value
        }

        if is_bit_set(value, LEFT_POS) && is_bit_set(value, RIGHT_POS) {
            value = unset_horizontal_bits(value);
        }

        if is_bit_set(value, DOWN_POS) && is_bit_set(value, UP_POS) {
            value = unset_the_down_bit(value);
        }

        MotionMask(value)
    }

    pub fn with_direction(motion: &str) -> Self {
        let mut binary_representation = 0_u8;
        for dir in motion.chars().into_iter() {
            let bit_to_set = match dir {
                'd' => 0b0000_0100,
                'u' => 0b0000_1000,
                'l' => 0b0000_0001,
                'r' => 0b0000_0010,
                _ => 0,
            };
            binary_representation |= bit_to_set;
        }
        MotionMask::new(binary_representation)
    }

    pub fn raw_value(&self) -> u8 {
        self.0
    }

    pub fn to_unicode(&self) -> char {
        const LEFT: u8 = 0b0000_0001;
        const RIGHT: u8 = 0b0000_0010;
        const DOWN: u8 = 0b0000_0100;
        const UP: u8 = 0b0000_1000;
        const DOWN_LEFT: u8 = 0b0000_0101;
        const DOWN_RIGHT: u8 = 0b0000_0110;
        const UP_LEFT: u8 = 0b0000_1001;
        const UP_RIGHT: u8 = 0b0000_1010;

        let mask = self.0;
        match mask {
            LEFT => char::from_u32(0x2190).unwrap(),
            RIGHT => char::from_u32(0x2192).unwrap(),
            DOWN => char::from_u32(0x2193).unwrap(),
            UP => char::from_u32(0x2191).unwrap(),
            DOWN_LEFT => char::from_u32(0x2199).unwrap(),
            DOWN_RIGHT => char::from_u32(0x2198).unwrap(),
            UP_LEFT => char::from_u32(0x2196).unwrap(),
            UP_RIGHT => char::from_u32(0x2197).unwrap(),
            0 => char::from_u32(0x2605).unwrap(),
            _ => ' ',
        }
    }

    pub fn to_numpad(&self, facing_right: bool) -> u8 {
        const LEFT: u8 = 0b0000_0001;
        const RIGHT: u8 = 0b0000_0010;
        const DOWN: u8 = 0b0000_0100;
        const UP: u8 = 0b0000_1000;
        const DOWN_LEFT: u8 = 0b0000_0101;
        const DOWN_RIGHT: u8 = 0b0000_0110;
        const UP_LEFT: u8 = 0b0000_1001;
        const UP_RIGHT: u8 = 0b0000_1010;

        let mask = self.0;
        let mut motion: u8 = 5;
        if facing_right {
            if mask == LEFT {
                motion = 4;
            }
            if mask == RIGHT {
                motion = 6;
            }
            if mask == DOWN {
                motion = 2;
            }
            if mask == UP {
                motion = 8;
            }
            if mask == DOWN_LEFT {
                motion = 1;
            }
            if mask == DOWN_RIGHT {
                motion = 3;
            }
            if mask == UP_LEFT {
                motion = 7;
            }
            if mask == UP_RIGHT {
                motion = 9;
            }
        } else {
            if mask == LEFT {
                motion = 6;
            }
            if mask == RIGHT {
                motion = 4;
            }
            if mask == DOWN {
                motion = 2;
            }
            if mask == UP {
                motion = 8;
            }
            if mask == DOWN_LEFT {
                motion = 3;
            }
            if mask == DOWN_RIGHT {
                motion = 1;
            }
            if mask == UP_LEFT {
                motion = 9;
            }
            if mask == UP_RIGHT {
                motion = 7;
            }
        }
        motion
    }
}

impl fmt::Display for MotionMask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_unicode())
    }
}
#[derive(Debug, Default, Clone, Copy, Reflect, FromReflect)]
pub struct Buttons {
    pub pressed: ButtonMask,
    pub held: ButtonMask,
    pub released: ButtonMask,
}
#[derive(Debug, Default, Clone, Copy, Reflect, FromReflect)]
#[repr(transparent)]
pub struct InputMask(pub u16);

impl InputMask {
    pub fn from_masks(buttons: ButtonMask, motion: MotionMask) -> Self {
        let mut base_value = (buttons.raw_value() as u16) << 8;
        base_value |= motion.raw_value() as u16;
        InputMask(base_value)
    }

    pub fn get_motion_mask(&self) -> MotionMask {
        MotionMask::new(self.0 as u8)
    }

    pub fn get_button_mask(&self) -> ButtonMask {
        let buttons_u16 = self.0 >> 8;
        ButtonMask(buttons_u16 as u8)
    }
}

impl fmt::Display for InputMask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let motion_string = self.get_motion_mask().to_string();
        let button_string = self.get_button_mask().to_string();
        write!(f, "{}{}", button_string, motion_string)
    }
}

#[test]
fn input_mask_test() {
    let masky = InputMask::from_masks(
        ButtonMask::with_buttons("abc"),
        MotionMask::with_direction("dr"),
    );
    assert_eq!(masky.to_string(), "abc↘".to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn detect_single_button() {
        let single_button_mask = ButtonMask(0b0000_1000);
        assert!(single_button_mask.contains('d'));
    }

    #[test]
    fn detect_multiple_buttons() {
        let multi_button_mask = ButtonMask(0b1010_0110);
        assert!(
            multi_button_mask.contains('h')
                && multi_button_mask.contains('f')
                && multi_button_mask.contains('b')
                && multi_button_mask.contains('c')
        )
    }

    #[test]
    fn motion_mask_to_numpad() {
        //
        let down_mask = MotionMask::new(0b0000_0100);
        let up_mask = MotionMask::new(0b0000_1000);
        let left_mask = MotionMask::new(0b0000_0001);
        let right_mask = MotionMask::new(0b0000_0010);
        let down_right_mask = MotionMask::new(0b0000_0110);
        let down_left_mask = MotionMask::new(0b0000_0101);
        let up_left_mask = MotionMask::new(0b0000_1001);
        let up_right_mask = MotionMask::new(0b0000_1010);
        let neutral_mask = MotionMask::new(0b0000_0000);
        // Universal Motions
        assert_eq!(neutral_mask.to_numpad(false), 5);
        assert_eq!(down_mask.to_numpad(false), 2);
        assert_eq!(up_mask.to_numpad(false), 8);

        // P1 Motions
        assert_eq!(left_mask.to_numpad(true), 4);
        assert_eq!(right_mask.to_numpad(true), 6);
        assert_eq!(down_left_mask.to_numpad(true), 1);
        assert_eq!(down_right_mask.to_numpad(true), 3);
        assert_eq!(up_left_mask.to_numpad(true), 7);
        assert_eq!(up_right_mask.to_numpad(true), 9);

        // P2 Motions
        assert_eq!(left_mask.to_numpad(false), 6);
        assert_eq!(right_mask.to_numpad(false), 4);
        assert_eq!(down_left_mask.to_numpad(false), 3);
        assert_eq!(down_right_mask.to_numpad(false), 1);
        assert_eq!(up_left_mask.to_numpad(false), 9);
        assert_eq!(up_right_mask.to_numpad(false), 7);
    }

    #[test]
    fn motion_mask_to_unicode() {
        let down_mask = MotionMask::new(0b0000_0100);
        let up_mask = MotionMask::new(0b0000_1000);
        let left_mask = MotionMask::new(0b0000_0001);
        let right_mask = MotionMask::new(0b0000_0010);
        let down_right_mask = MotionMask::new(0b0000_0110);
        let down_left_mask = MotionMask::new(0b0000_0101);
        let up_left_mask = MotionMask::new(0b0000_1001);
        let up_right_mask = MotionMask::new(0b0000_1010);
        let neutral_mask = MotionMask::new(0b0000_0000);

        assert_eq!('↓', down_mask.to_unicode());
        assert_eq!('↑', up_mask.to_unicode());
        assert_eq!('←', left_mask.to_unicode());
        assert_eq!('→', right_mask.to_unicode());
        assert_eq!('↙', down_left_mask.to_unicode());
        assert_eq!('↘', down_right_mask.to_unicode());
        assert_eq!('↖', up_left_mask.to_unicode());
        assert_eq!('↗', up_right_mask.to_unicode());
        assert_eq!('★', neutral_mask.to_unicode());
    }
}
