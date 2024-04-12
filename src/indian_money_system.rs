#![allow(clippy::inconsistent_digit_grouping)]

use std::fmt::Write;

const DIGITS: [&str; 10] = ["", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"];
const TEENS: [&str; 10] = ["Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen"];
const TENS: [&str; 10] = ["", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"];
const SCALES: [&str; 6] = ["", "Thousand", "Lakh", "Crore", "Arab", "Kharab"];

pub fn money_to(amount: f64) -> String {
    let mut words = String::new();

    let rupees = amount.trunc() as u64;
    let paisa = ((amount * 100.0).round() as u64) % 100;
    if rupees == 0 && paisa == 0 {
        return "Zero Rupees Only".to_string();
    }
    if rupees != 0 {
        let p =number_to_word(rupees );
        words.push_str(p.as_str());
        write!(words, " Rupees").unwrap();
    } else {
        write!(words, "Zero Rupees").unwrap();
    }
    if paisa != 0 {
        write!(words, " And ").unwrap();
        let p=number_to_word(paisa);
        words.push_str(p.as_str());
        write!(words, " Paisa").unwrap();
    }
    write!(words, " Only").unwrap();
    words
}

pub fn number_to_word(amount: u64)->String {
    let mut words=String::new();
    let mut rupees = amount;
    if rupees==0{
        return words;
    }
    let mut scale_index = 0;
    let mut divider = 1000;
    while rupees > 0 {
        if scale_index >= 1 {
            divider = 100;
        }
        if rupees % divider != 0 {
            let scale = if scale_index > 0 { format!(" {}", SCALES[scale_index]) } else { "".to_string() };
            words = format!("{}{}{}", convert_three_digits(rupees % divider ).trim(), scale, if words.is_empty() { "" } else { " " }) + &words;
        }
        rupees /= divider;
        scale_index += 1;
    }
    words
}

pub fn money_to_words_i(mut amount: f64) -> String {
    let mut words = String::new();

    let mut rupees = amount.trunc() as u64;
    let paisa = ((amount * 100.0).round() as u64) % 100;
    if rupees == 0 && paisa == 0 {
        return "Zero Rupees Only".to_string();
    }
    let mut scale_index = 0;
    let mut divider = 1000;
    while rupees > 0 {
        if scale_index >= 1 {
            divider = 100;
        }
        if rupees % divider != 0 {
            let scale = if scale_index > 0 { format!(" {}", SCALES[scale_index]) } else { "".to_string() };
            words = format!("{}{}{}", convert_three_digits(rupees % divider).trim(), scale, if words.is_empty() { "" } else { " " }) + &words;
        }
        rupees /= divider;
        scale_index += 1;
    }

    if !words.is_empty() {
        words = words.trim().to_string() + " Rupees";
    }

    if paisa > 0 {
        if !words.is_empty() {
            words += " And ";
        } else {
            words = "Zero Rupees And ".to_string();
        }
        words += &convert_three_digits(paisa).trim();
        words += " Paisa";
    }

    words += " Only";

    words
}


fn convert_three_digits(num: u64) -> String {
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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::indian_money_system::{money_to, money_to_words_i};

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
    fn test_combinations(#[case]amt: f64, #[case]words: &str) {
        let result =  money_to(amt);
        assert_eq!(result, words)
    }
}