/*
 * @file led.rs
 * @brief LED state management
 * @author Kevin Thomas
 * @date 2025
 *
 * MIT License
 *
 * Copyright (c) 2025 Kevin Thomas
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

//! FILE: led.rs
//!
//! DESCRIPTION:
//! LED State Management for RP2350.
//!
//! BRIEF:
//! Provides LED state enumeration and blink controller.
//!
//! AUTHOR: Kevin Thomas
//! CREATION DATE: December 6, 2025
//! UPDATE DATE: December 6, 2025

use crate::config::BLINK_DELAY_MS;

/// LED state enumeration.
///
/// # Details
/// Represents the current state of the LED.
/// Used for state tracking and transitions.
///
/// # Variants
/// * `On` - LED is currently on (high)
/// * `Off` - LED is currently off (low)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum LedState {
    On,
    Off,
}

/// LED controller with state tracking.
///
/// # Details
/// Maintains LED state and blink timing configuration.
/// Provides methods for state transitions and queries.
///
/// # Fields
/// * `state` - Current LED state
/// * `delay_ms` - Blink delay in milliseconds
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub struct LedController {
    state: LedState,
    delay_ms: u64,
}

impl Default for LedController {
    /// Returns default LedController instance.
    ///
    /// # Details
    /// Delegates to new() for initialization.
    ///
    /// # Returns
    /// * `Self` - New LedController with default values
    #[allow(dead_code)]
    fn default() -> Self {
        Self::new()
    }
}

impl LedController {
    /// Creates new LED controller with default settings.
    ///
    /// # Details
    /// Initializes controller with LED off.
    ///
    /// # Returns
    /// * `Self` - New LedController instance
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            state: LedState::Off,
            delay_ms: BLINK_DELAY_MS,
        }
    }

    /// Toggles LED state and returns new state.
    ///
    /// # Details
    /// Transitions LED from On to Off or Off to On.
    ///
    /// # Returns
    /// * `LedState` - New LED state after toggle
    #[allow(dead_code)]
    pub fn toggle(&mut self) -> LedState {
        self.state = match self.state {
            LedState::On => LedState::Off,
            LedState::Off => LedState::On,
        };
        self.state
    }

    /// Returns current blink delay.
    ///
    /// # Details
    /// Delay used for blink timing in milliseconds.
    ///
    /// # Returns
    /// * `u64` - Delay in milliseconds
    #[allow(dead_code)]
    pub fn delay_ms(&self) -> u64 {
        self.delay_ms
    }
}

/// Converts LedState to boolean for GPIO control.
///
/// # Details
/// Maps On state to true (high), Off state to false (low).
///
/// # Arguments
/// * `state` - LED state to convert
///
/// # Returns
/// * `bool` - true for On, false for Off
#[allow(dead_code)]
pub fn led_state_to_level(state: LedState) -> bool {
    matches!(state, LedState::On)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== LedState Enum Tests ====================

    #[test]
    fn test_led_state_on_exists() {
        let _state = LedState::On;
    }

    #[test]
    fn test_led_state_off_exists() {
        let _state = LedState::Off;
    }

    #[test]
    fn test_led_state_equality_on() {
        assert_eq!(LedState::On, LedState::On);
    }

    #[test]
    fn test_led_state_equality_off() {
        assert_eq!(LedState::Off, LedState::Off);
    }

    #[test]
    fn test_led_state_inequality() {
        assert_ne!(LedState::On, LedState::Off);
        assert_ne!(LedState::Off, LedState::On);
    }

    #[test]
    fn test_led_state_copy() {
        let state = LedState::On;
        let copy = state;
        assert_eq!(state, copy);
    }

    #[test]
    fn test_led_state_clone() {
        let state = LedState::Off;
        #[allow(clippy::clone_on_copy)]
        let cloned = state.clone();
        assert_eq!(state, cloned);
    }

    #[test]
    fn test_led_state_debug_on() {
        let state = LedState::On;
        let debug_str = format!("{:?}", state);
        assert_eq!(debug_str, "On");
    }

    #[test]
    fn test_led_state_debug_off() {
        let state = LedState::Off;
        let debug_str = format!("{:?}", state);
        assert_eq!(debug_str, "Off");
    }

    #[test]
    fn test_led_state_size() {
        assert_eq!(core::mem::size_of::<LedState>(), 1);
    }

    // ==================== led_state_to_level Function Tests ====================

    #[test]
    fn test_led_state_to_level_on() {
        assert!(led_state_to_level(LedState::On));
    }

    #[test]
    fn test_led_state_to_level_off() {
        assert!(!led_state_to_level(LedState::Off));
    }

    #[test]
    fn test_led_state_to_level_on_returns_true() {
        assert_eq!(led_state_to_level(LedState::On), true);
    }

    #[test]
    fn test_led_state_to_level_off_returns_false() {
        assert_eq!(led_state_to_level(LedState::Off), false);
    }

    #[test]
    fn test_led_state_to_level_consistent() {
        for _ in 0..10 {
            assert!(led_state_to_level(LedState::On));
            assert!(!led_state_to_level(LedState::Off));
        }
    }

    // ==================== LedController::new() Tests ====================

    #[test]
    fn test_new_controller() {
        let ctrl = LedController::new();
        assert_eq!(ctrl.delay_ms(), BLINK_DELAY_MS);
    }

    #[test]
    fn test_new_controller_state_off() {
        let ctrl = LedController::new();
        let expected = LedController {
            state: LedState::Off,
            delay_ms: BLINK_DELAY_MS,
        };
        assert_eq!(ctrl, expected);
    }

    #[test]
    fn test_new_controller_delay_matches_config() {
        let ctrl = LedController::new();
        assert_eq!(ctrl.delay_ms(), 500);
    }

    #[test]
    fn test_new_returns_consistent_value() {
        let ctrl1 = LedController::new();
        let ctrl2 = LedController::new();
        assert_eq!(ctrl1, ctrl2);
    }

    // ==================== LedController::default() Tests ====================

    #[test]
    fn test_default_equals_new() {
        let default = LedController::default();
        let new = LedController::new();
        assert_eq!(default, new);
    }

    #[test]
    fn test_default_delay() {
        let default = LedController::default();
        assert_eq!(default.delay_ms(), BLINK_DELAY_MS);
    }

    #[test]
    fn test_default_state_off() {
        let mut default = LedController::default();
        assert_eq!(default.toggle(), LedState::On);
    }

    // ==================== LedController::toggle() Tests ====================

    #[test]
    fn test_toggle_off_to_on() {
        let mut ctrl = LedController::new();
        assert_eq!(ctrl.toggle(), LedState::On);
    }

    #[test]
    fn test_toggle_on_to_off() {
        let mut ctrl = LedController::new();
        ctrl.toggle();
        assert_eq!(ctrl.toggle(), LedState::Off);
    }

    #[test]
    fn test_toggle_returns_new_state() {
        let mut ctrl = LedController::new();
        let state = ctrl.toggle();
        assert_eq!(state, LedState::On);
    }

    #[test]
    fn test_multiple_toggles() {
        let mut ctrl = LedController::new();
        assert_eq!(ctrl.toggle(), LedState::On);
        assert_eq!(ctrl.toggle(), LedState::Off);
        assert_eq!(ctrl.toggle(), LedState::On);
        assert_eq!(ctrl.toggle(), LedState::Off);
    }

    #[test]
    fn test_toggle_10_times() {
        let mut ctrl = LedController::new();
        for i in 0..10 {
            let state = ctrl.toggle();
            if i % 2 == 0 {
                assert_eq!(state, LedState::On);
            } else {
                assert_eq!(state, LedState::Off);
            }
        }
    }

    #[test]
    fn test_toggle_100_times() {
        let mut ctrl = LedController::new();
        for i in 0..100 {
            let state = ctrl.toggle();
            if i % 2 == 0 {
                assert_eq!(state, LedState::On);
            } else {
                assert_eq!(state, LedState::Off);
            }
        }
    }

    #[test]
    fn test_toggle_preserves_delay() {
        let mut ctrl = LedController::new();
        let delay_before = ctrl.delay_ms();
        ctrl.toggle();
        ctrl.toggle();
        ctrl.toggle();
        assert_eq!(ctrl.delay_ms(), delay_before);
    }

    // ==================== LedController::delay_ms() Tests ====================

    #[test]
    fn test_delay_ms_returns_correct_value() {
        let ctrl = LedController::new();
        assert_eq!(ctrl.delay_ms(), BLINK_DELAY_MS);
    }

    #[test]
    fn test_delay_ms_is_500() {
        let ctrl = LedController::new();
        assert_eq!(ctrl.delay_ms(), 500);
    }

    #[test]
    fn test_delay_ms_immutable_after_toggle() {
        let mut ctrl = LedController::new();
        ctrl.toggle();
        assert_eq!(ctrl.delay_ms(), BLINK_DELAY_MS);
    }

    #[test]
    fn test_delay_ms_within_config_range() {
        let ctrl = LedController::new();
        assert!(ctrl.delay_ms() >= crate::config::MIN_BLINK_DELAY_MS);
        assert!(ctrl.delay_ms() <= crate::config::MAX_BLINK_DELAY_MS);
    }

    // ==================== LedController Struct Tests ====================

    #[test]
    fn test_initial_state_off() {
        let ctrl = LedController::new();
        let expected = LedController {
            state: LedState::Off,
            delay_ms: BLINK_DELAY_MS,
        };
        assert_eq!(ctrl, expected);
    }

    #[test]
    fn test_controller_size() {
        assert!(core::mem::size_of::<LedController>() <= 16);
    }

    #[test]
    fn test_controller_alignment() {
        assert!(core::mem::align_of::<LedController>() <= 8);
    }

    // ==================== Trait Implementation Tests ====================

    #[test]
    fn test_led_state_debug() {
        let state = LedState::On;
        let debug_str = format!("{:?}", state);
        assert!(debug_str.contains("On"));
    }

    #[test]
    fn test_led_controller_clone() {
        let ctrl1 = LedController::new();
        let ctrl2 = ctrl1;
        assert_eq!(ctrl1.delay_ms(), ctrl2.delay_ms());
    }

    #[test]
    fn test_led_controller_copy() {
        let ctrl1 = LedController::new();
        let ctrl2 = ctrl1;
        assert_eq!(ctrl1, ctrl2);
    }

    #[test]
    fn test_led_controller_partial_eq() {
        let ctrl1 = LedController::new();
        let ctrl2 = LedController::new();
        assert_eq!(ctrl1, ctrl2);
    }

    #[test]
    fn test_led_controller_eq_reflexive() {
        let ctrl = LedController::new();
        assert_eq!(ctrl, ctrl);
    }

    #[test]
    fn test_led_controller_eq_symmetric() {
        let ctrl1 = LedController::new();
        let ctrl2 = LedController::new();
        assert_eq!(ctrl1, ctrl2);
        assert_eq!(ctrl2, ctrl1);
    }

    #[test]
    fn test_led_controller_debug() {
        let ctrl = LedController::new();
        let debug_str = format!("{:?}", ctrl);
        assert!(debug_str.contains("LedController"));
    }

    #[test]
    fn test_led_controller_debug_contains_state() {
        let ctrl = LedController::new();
        let debug_str = format!("{:?}", ctrl);
        assert!(debug_str.contains("Off"));
    }

    #[test]
    fn test_led_controller_debug_contains_delay() {
        let ctrl = LedController::new();
        let debug_str = format!("{:?}", ctrl);
        assert!(debug_str.contains("500"));
    }

    // ==================== State Transition Tests ====================

    #[test]
    fn test_state_machine_cycle() {
        let mut ctrl = LedController::new();
        // Start: Off
        assert_eq!(ctrl.toggle(), LedState::On);
        // Now: On
        assert_eq!(ctrl.toggle(), LedState::Off);
        // Now: Off (back to start)
    }

    #[test]
    fn test_independent_controllers() {
        let mut ctrl1 = LedController::new();
        let mut ctrl2 = LedController::new();

        ctrl1.toggle();

        // ctrl2 should still be at initial state
        assert_eq!(ctrl2.toggle(), LedState::On);
    }

    #[test]
    fn test_toggle_and_level_consistency() {
        let mut ctrl = LedController::new();

        let state1 = ctrl.toggle();
        assert_eq!(led_state_to_level(state1), true);

        let state2 = ctrl.toggle();
        assert_eq!(led_state_to_level(state2), false);
    }

    // ==================== Edge Case Tests ====================

    #[test]
    fn test_many_controllers() {
        let controllers: Vec<LedController> = (0..100).map(|_| LedController::new()).collect();

        for ctrl in controllers {
            assert_eq!(ctrl.delay_ms(), BLINK_DELAY_MS);
        }
    }

    #[test]
    fn test_controller_in_option() {
        let maybe_ctrl: Option<LedController> = Some(LedController::new());
        assert!(maybe_ctrl.is_some());
        assert_eq!(maybe_ctrl.unwrap().delay_ms(), BLINK_DELAY_MS);
    }

    #[test]
    fn test_controller_in_result() {
        let result: Result<LedController, ()> = Ok(LedController::new());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().delay_ms(), BLINK_DELAY_MS);
    }
}
