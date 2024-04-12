#![allow(clippy::inconsistent_digit_grouping)]

use std::fmt::Write;
use crate::constants::{INDIAN_SCALE, INTERNATIONAL_SCALE};

use crate::util::{convert_three_digits, };

mod constants;
mod util;

pub fn format_with_currency_in_words(amount: f64, number_scale: NumberScale,
                                     main_currency:&str,
                                     fractional_currency:&str,
                                     fractional_conversion_factor: u64) -> String {
    let mut words = String::new();
    let whole_amount = amount.trunc() as u64;
    let fractional_amount = ((amount * (fractional_conversion_factor as f64)).round() as u64) % fractional_conversion_factor;
    if whole_amount == 0 && fractional_amount == 0 {
        return format!("Zero {} Only", main_currency);
    }
    if whole_amount != 0 {
        let wrd = format_number_to_word(whole_amount, number_scale.clone());
        words.push_str(wrd.as_str());
        write!(words, " {}", main_currency).unwrap();
    } else {
        write!(words, "Zero {}", main_currency).unwrap();
    }
    if fractional_amount != 0 {
        write!(words, " And ").unwrap();
        let wrd = format_number_to_word(fractional_amount, number_scale);
        words.push_str(wrd.as_str());
        write!(words, " {}", fractional_currency).unwrap();
    }
    write!(words, " Only").unwrap();
    words
}

#[derive(Debug,PartialEq,Clone)]
pub enum NumberScale {
    International,
    Indian,
}
impl NumberScale{
    fn get_number_scale(&self)-> &[&str]{
        match self{
            NumberScale::International => { &INTERNATIONAL_SCALE }
            NumberScale::Indian => {&INDIAN_SCALE }
        }
    }
}
/// Converts a given number to its word representation using the specified number scale.
///
/// This function takes a non-negative integer `amount` and a `NumberScale` enum value
/// representing the desired number scale system (Indian or International). It returns
/// the word representation of the number using the corresponding number scale.
///
/// # Arguments
///
/// * `amount` - A non-negative integer representing the number to be converted to words.
/// * `scale_type` - A `NumberScale` enum value specifying the number scale system to be used
///                  (Indian or International).
///
/// # Returns
///
/// A `String` containing the word representation of the given number using the specified
/// number scale system.
///
/// # Examples
///
/// ```
///
/// use number_wordify::{format_number_to_word, NumberScale};
/// let amount = 1234567;
/// let indian_words = format_number_to_word(amount, NumberScale::Indian);
/// let international_words = format_number_to_word(amount, NumberScale::International);
///
/// assert_eq!(indian_words, "Twelve Lakh Thirty Four Thousand Five Hundred Sixty Seven");
/// assert_eq!(international_words, "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven");
/// ```
///
/// # Notes
///
/// - The function supports numbers up to 9,999,999,999,999,999,999 (Indian scale) or
///   999,999,999,999,999 (International scale).
/// - For the Indian number scale, the function uses the following units:
///   - Thousand
///   - Lakh (100,000)
///   - Crore (10,000,000)
///   - Arab (1,000,000,000)
///   - Kharab (100,000,000,000)
/// - For the International number scale, the function uses the following units:
///   - Thousand
///   - Million
///   - Billion
/// - The function assumes that the `convert_three_digits` function is defined elsewhere
///   in the code to convert a three-digit number to its word representation.
pub fn format_number_to_word(amount: u64, scale_type: NumberScale) -> String {
    let scales = scale_type.get_number_scale();
    let mut words = String::new();
    let mut rupees = amount;
    if rupees == 0 {
        return words;
    }
    let mut scale_index = 0;
    let mut divider = 1000;
    while rupees > 0 {
        if scale_index >= 1 && scale_type == NumberScale::Indian {
            divider = 100;
        }
        if rupees % divider != 0 {
            let scale = if scale_index > 0 {
                format!(" {}", scales[scale_index])
            } else {
                "".to_string()
            };
            words = format!(
                "{}{}{}",
                convert_three_digits(rupees % divider).trim(),
                scale,
                if words.is_empty() { "" } else { " " }
            ) + &words;
        }
        rupees /= divider;
        scale_index += 1;
    }
    words
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::format_with_currency_in_words;
    use crate::NumberScale::{Indian, International};

    #[rstest]
    #[case(272000000000.29, "Two Hundred Seventy Two Billion Rupees And Twenty Nine Paisa Only")]
    #[case(27200000000.29, "Twenty Seven Billion Two Hundred Million Rupees And Twenty Nine Paisa Only")]
    #[case(2720000000.29, "Two Billion Seven Hundred Twenty Million Rupees And Twenty Nine Paisa Only")]
    #[case(272000000.29, "Two Hundred Seventy Two Million Rupees And Twenty Nine Paisa Only")]
    #[case(27200000.29, "Twenty Seven Million Two Hundred Thousand Rupees And Twenty Nine Paisa Only")]
    #[case(2720000.29, "Two Million Seven Hundred Twenty Thousand Rupees And Twenty Nine Paisa Only")]
    #[case(272000.29, "Two Hundred Seventy Two Thousand Rupees And Twenty Nine Paisa Only")]
    #[case(27200.90, "Twenty Seven Thousand Two Hundred Rupees And Ninety Paisa Only")]
    #[case(2720.90, "Two Thousand Seven Hundred Twenty Rupees And Ninety Paisa Only")]
    #[case(272.90, "Two Hundred Seventy Two Rupees And Ninety Paisa Only")]
    #[case(27.29, "Twenty Seven Rupees And Twenty Nine Paisa Only")]
    #[case(0.0, "Zero Rupees Only")]
    #[case(0.01, "Zero Rupees And One Paisa Only")]
    #[case(0.99, "Zero Rupees And Ninety Nine Paisa Only")]
    #[case(1.0, "One Rupees Only")]
    #[case(1.01, "One Rupees And One Paisa Only")]
    #[case(1.99, "One Rupees And Ninety Nine Paisa Only")]
    #[case(999.99, "Nine Hundred Ninety Nine Rupees And Ninety Nine Paisa Only")]
    #[case(1000.0, "One Thousand Rupees Only")]
    #[case(1000.01, "One Thousand Rupees And One Paisa Only")]
    #[case(1000.99, "One Thousand Rupees And Ninety Nine Paisa Only")]
    #[case(999999.99, "Nine Hundred Ninety Nine Thousand Nine Hundred Ninety Nine Rupees And Ninety Nine Paisa Only")]
    #[case(1000000.0, "One Million Rupees Only")]
    #[case(1000000.01, "One Million Rupees And One Paisa Only")]
    #[case(1000000.99, "One Million Rupees And Ninety Nine Paisa Only")]
    #[case(999999999.99, "Nine Hundred Ninety Nine Million Nine Hundred Ninety Nine Thousand Nine Hundred Ninety Nine Rupees And Ninety Nine Paisa Only")]
    #[case(1000000000.0, "One Billion Rupees Only")]
    #[case(1000000000.01, "One Billion Rupees And One Paisa Only")]
    #[case(1000000000.99, "One Billion Rupees And Ninety Nine Paisa Only")]
    #[case(999999999999.99, "Nine Hundred Ninety Nine Billion Nine Hundred Ninety Nine Million Nine Hundred Ninety Nine Thousand Nine Hundred Ninety Nine Rupees And Ninety Nine Paisa Only")]
    fn test_combinations(#[case]amt: f64, #[case]words: &str) {
        let result = format_with_currency_in_words(amt, International, "Rupees", "Paisa",100);
        assert_eq!(result, words)
    }

    #[rstest]
    #[case(2_72_00_00_00_000.29, "Two Kharab Seventy Two Arab Rupees And Twenty Nine Paisa Only")]
    #[case(27_20_00_00_000.29, "Twenty Seven Arab Twenty Crore Rupees And Twenty Nine Paisa Only")]
    #[case(2_72_00_00_000.29, "Two Arab Seventy Two Crore Rupees And Twenty Nine Paisa Only")]
    #[case(27_20_00_000.29, "Twenty Seven Crore Twenty Lakh Rupees And Twenty Nine Paisa Only")]
    #[case(2_72_00_000.29, "Two Crore Seventy Two Lakh Rupees And Twenty Nine Paisa Only")]
    #[case(27_20_000.29, "Twenty Seven Lakh Twenty Thousand Rupees And Twenty Nine Paisa Only")]
    #[case(2_72_000.29, "Two Lakh Seventy Two Thousand Rupees And Twenty Nine Paisa Only")]
    #[case(27_200.90, "Twenty Seven Thousand Two Hundred Rupees And Ninety Paisa Only")]
    #[case(2_720.90, "Two Thousand Seven Hundred Twenty Rupees And Ninety Paisa Only")]
    #[case(272.90, "Two Hundred Seventy Two Rupees And Ninety Paisa Only")]
    #[case(27.29, "Twenty Seven Rupees And Twenty Nine Paisa Only")]
    #[case(0.0, "Zero Rupees Only")]
    #[case(0.01, "Zero Rupees And One Paisa Only")]
    #[case(0.99, "Zero Rupees And Ninety Nine Paisa Only")]
    #[case(1.0, "One Rupees Only")]
    #[case(1.01, "One Rupees And One Paisa Only")]
    #[case(1.99, "One Rupees And Ninety Nine Paisa Only")]
    #[case(999.99, "Nine Hundred Ninety Nine Rupees And Ninety Nine Paisa Only")]
    #[case(1000.0, "One Thousand Rupees Only")]
    #[case(1000.01, "One Thousand Rupees And One Paisa Only")]
    #[case(1000.99, "One Thousand Rupees And Ninety Nine Paisa Only")]
    #[case(999999.99, "Nine Lakh Ninety Nine Thousand Nine Hundred Ninety Nine Rupees And Ninety Nine Paisa Only")]
    #[case(1000000.0, "Ten Lakh Rupees Only")]
    #[case(1000000.01, "Ten Lakh Rupees And One Paisa Only")]
    #[case(1000000.99, "Ten Lakh Rupees And Ninety Nine Paisa Only")]
    #[case(999999999.99, "Ninety Nine Crore Ninety Nine Lakh Ninety Nine Thousand Nine Hundred Ninety Nine Rupees And Ninety Nine Paisa Only")]
    #[case(1000000000.0, "One Arab Rupees Only")]
    #[case(1000000000.01, "One Arab Rupees And One Paisa Only")]
    #[case(1000000000.99, "One Arab Rupees And Ninety Nine Paisa Only")]
    #[case(9_99_99_99_99_999.99, "Nine Kharab Ninety Nine Arab Ninety Nine Crore Ninety Nine Lakh Ninety Nine Thousand Nine Hundred Ninety Nine Rupees And Ninety Nine Paisa Only")]
    fn test_combinations_indian(#[case]amt: f64, #[case]words: &str) {
        let result = format_with_currency_in_words(amt, Indian, "Rupees", "Paisa",100);
        assert_eq!(result, words)
    }
}