use crate::constants::{DIGITS, TEENS, TENS};
/// Converts a number up to three digits to its word representation.
///
/// This function takes a number up to three digits (0-999) and converts it to its
/// corresponding word representation. It handles the conversion of hundreds, tens,
/// and ones separately and combines them to form the complete word representation.
///
/// # Arguments
///
/// * `num` - The number to be converted to words. It should be in the range 0-999.
///
/// # Returns
///
/// The word representation of the input number.
///
/// # Examples
///
/// ```rust,ignore
/// // This is a private function, so we cannot call it directly in the example.
/// // However, we can demonstrate its usage indirectly.
///
/// fn example_usage() {
///     let words = convert_three_digits(123);
///     assert_eq!(words, "One Hundred Twenty Three");
///
///     let words = convert_three_digits(50);
///     assert_eq!(words, "Fifty");
///
///     let words = convert_three_digits(9);
///     assert_eq!(words, "Nine");
/// }
/// ```
///
/// # Panics
///
/// This function does not panic for any input value. However, it expects the input
/// number to be in the range 0-999. Passing a number outside this range may lead to
/// unexpected behavior.
pub fn convert_three_digits(num: u64) -> String {
    let mut words = String::new();
    let hundred = num / 100;
    let ten = num % 100;
    let one = num % 10;

    if hundred > 0 {
        words += DIGITS[hundred as usize];
        words += " Hundred";
    }

    if ten >= 20 {
        if !words.is_empty() {
            words += " ";
        }
        words += TENS[(ten / 10) as usize];
        if one > 0 {
            words += " ";
            words += DIGITS[one as usize];
        }
    } else if ten >= 10 {
        if !words.is_empty() {
            words += " ";
        }
        words += TEENS[(ten % 10) as usize];
    } else if one > 0 {
        if !words.is_empty() {
            words += " ";
        }
        words += DIGITS[one as usize];
    }

    words.trim().to_string()
}