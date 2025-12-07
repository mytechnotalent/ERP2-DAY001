/*
 * @file config.rs
 * @brief Application configuration constants
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

//! FILE: config.rs
//!
//! DESCRIPTION:
//! RP2350 Blink Configuration Constants.
//!
//! BRIEF:
//! Defines configuration constants for LED blink timing.
//! Contains delay intervals and GPIO pin configuration.
//!
//! AUTHOR: Kevin Thomas
//! CREATION DATE: December 6, 2025
//! UPDATE DATE: December 6, 2025

/// Default LED blink delay in milliseconds.
///
/// # Details
/// Configures the delay between LED state transitions.
/// Used for both ON and OFF durations.
///
/// # Value
/// 500 milliseconds
#[allow(dead_code)]
pub const BLINK_DELAY_MS: u64 = 500;

/// Minimum allowed blink delay in milliseconds.
///
/// # Details
/// Prevents excessively fast blinking which may cause issues.
///
/// # Value
/// 10 milliseconds
#[allow(dead_code)]
pub const MIN_BLINK_DELAY_MS: u64 = 10;

/// Maximum allowed blink delay in milliseconds.
///
/// # Details
/// Prevents excessively slow blinking for practical use.
///
/// # Value
/// 10000 milliseconds (10 seconds)
#[allow(dead_code)]
pub const MAX_BLINK_DELAY_MS: u64 = 10000;

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== BLINK_DELAY_MS Tests ====================

    #[test]
    fn test_blink_delay_default() {
        assert_eq!(BLINK_DELAY_MS, 500);
    }

    #[test]
    fn test_blink_delay_is_u64() {
        let _: u64 = BLINK_DELAY_MS;
    }

    #[test]
    fn test_blink_delay_non_zero() {
        assert!(BLINK_DELAY_MS > 0);
    }

    #[test]
    fn test_blink_delay_reasonable_range() {
        assert!(BLINK_DELAY_MS >= 100);
        assert!(BLINK_DELAY_MS <= 2000);
    }

    // ==================== MIN_BLINK_DELAY_MS Tests ====================

    #[test]
    fn test_min_delay_value() {
        assert_eq!(MIN_BLINK_DELAY_MS, 10);
    }

    #[test]
    fn test_min_delay_is_u64() {
        let _: u64 = MIN_BLINK_DELAY_MS;
    }

    #[test]
    fn test_min_delay_non_zero() {
        assert!(MIN_BLINK_DELAY_MS > 0);
    }

    #[test]
    fn test_min_delay_less_than_default() {
        assert!(MIN_BLINK_DELAY_MS < BLINK_DELAY_MS);
    }

    #[test]
    fn test_min_delay_practical_minimum() {
        assert!(MIN_BLINK_DELAY_MS >= 1);
    }

    // ==================== MAX_BLINK_DELAY_MS Tests ====================

    #[test]
    fn test_max_delay_value() {
        assert_eq!(MAX_BLINK_DELAY_MS, 10000);
    }

    #[test]
    fn test_max_delay_is_u64() {
        let _: u64 = MAX_BLINK_DELAY_MS;
    }

    #[test]
    fn test_max_delay_greater_than_default() {
        assert!(MAX_BLINK_DELAY_MS > BLINK_DELAY_MS);
    }

    #[test]
    fn test_max_delay_is_10_seconds() {
        assert_eq!(MAX_BLINK_DELAY_MS, 10 * 1000);
    }

    // ==================== Range Relationship Tests ====================

    #[test]
    fn test_delay_range_valid() {
        assert!(MIN_BLINK_DELAY_MS < MAX_BLINK_DELAY_MS);
    }

    #[test]
    fn test_default_within_range() {
        assert!(BLINK_DELAY_MS >= MIN_BLINK_DELAY_MS);
        assert!(BLINK_DELAY_MS <= MAX_BLINK_DELAY_MS);
    }

    #[test]
    fn test_range_span() {
        let span = MAX_BLINK_DELAY_MS - MIN_BLINK_DELAY_MS;
        assert!(span > 0);
        assert_eq!(span, 9990);
    }

    #[test]
    fn test_default_not_at_boundaries() {
        assert_ne!(BLINK_DELAY_MS, MIN_BLINK_DELAY_MS);
        assert_ne!(BLINK_DELAY_MS, MAX_BLINK_DELAY_MS);
    }

    // ==================== Arithmetic Safety Tests ====================

    #[test]
    fn test_no_overflow_on_double() {
        let doubled = BLINK_DELAY_MS.checked_mul(2);
        assert!(doubled.is_some());
    }

    #[test]
    fn test_no_overflow_max_doubled() {
        let doubled = MAX_BLINK_DELAY_MS.checked_mul(2);
        assert!(doubled.is_some());
    }

    #[test]
    fn test_min_subtraction_safe() {
        let result = BLINK_DELAY_MS.checked_sub(MIN_BLINK_DELAY_MS);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 490);
    }

    #[test]
    fn test_values_fit_in_u32() {
        assert!(BLINK_DELAY_MS <= u32::MAX as u64);
        assert!(MIN_BLINK_DELAY_MS <= u32::MAX as u64);
        assert!(MAX_BLINK_DELAY_MS <= u32::MAX as u64);
    }

    // ==================== Constant Immutability Tests ====================

    #[test]
    fn test_constants_are_const() {
        const _A: u64 = BLINK_DELAY_MS;
        const _B: u64 = MIN_BLINK_DELAY_MS;
        const _C: u64 = MAX_BLINK_DELAY_MS;
    }

    #[test]
    fn test_constants_usable_in_const_context() {
        const DOUBLE_DELAY: u64 = BLINK_DELAY_MS * 2;
        assert_eq!(DOUBLE_DELAY, 1000);
    }
}
