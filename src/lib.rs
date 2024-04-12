use crate::constants::MILLION_SCALES;
use crate::util::convert_three_digits;

pub mod indian_money_system;
mod constants;
mod util;


pub fn money_to_words(amount: f64) -> String {
    let mut words = String::new();
    let mut rupees = amount.trunc() as u64; // Extract the whole number part of the amount as rupees
    let paisa = ((amount * 100.0).round() as u64) % 100; // Calculate the paisa by multiplying the amount by 100, rounding, and taking the remainder when divided by 100

    if rupees == 0 && paisa == 0 {
        return "Zero Rupees Only".to_string(); // If both rupees and paisa are zero, return "Zero Rupees Only"
    }

    let mut scale_index = 0; // Initialize the scale index to keep track of the current scale (thousands, millions, billions)
    while rupees > 0 {
        if rupees % 1000 != 0 {
            let scale = if scale_index > 0 { format!(" {}", MILLION_SCALES[scale_index]) } else { "".to_string() }; // Get the appropriate scale word based on the scale index
            words = format!("{}{}{}", convert_three_digits(rupees % 1000).trim(), scale, if words.is_empty() { "" } else { " " }) + &words; // Convert the current three digits of rupees to words, append the scale, and add it to the existing words
        }
        rupees /= 1000; // Divide the rupees by 1000 to move to the next scale
        scale_index += 1; // Increment the scale index
    }

    if !words.is_empty() {
        words = words.trim().to_string() + " Rupees"; // If there are rupees, append "Rupees" to the words
    }

    if paisa > 0 {
        if !words.is_empty() {
            words += " And "; // If there are rupees and paisa, append "And" to the words
        } else {
            words = "Zero Rupees And ".to_string(); // If there are no rupees but paisa, start the words with "Zero Rupees And"
        }
        words += &convert_three_digits(paisa).trim(); // Convert the paisa to words and append it to the words
        words += if paisa == 1 { " Paisa" } else { " Paisa" }; // Append "Paisa" (singular or plural) to the words based on the paisa value
    }

    words += " Only"; // Append "Only" to the words

    words // Return the final words string
}


#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::money_to_words;

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
        let result = money_to_words(amt);
        assert_eq!(result, words)
    }

    
}