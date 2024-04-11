const DIGITS: [&str; 10] = ["", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"];
const TEENS: [&str; 10] = ["Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen"];
const TENS: [&str; 10] = ["", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"];
const SCALES: [&str; 4] = ["", "Thousand", "Million", "Billion"];

pub fn money_to_words(amount: f64) -> String {
    let mut words = String::new();
    let mut rupees = amount.trunc() as u64;
    let paisa = ((amount * 100.0).round() as u64) % 100;

    if rupees == 0 && paisa == 0 {
        return "Zero Rupees Only".to_string();
    }

    let mut scale_index = 0;
    while rupees > 0 {
        if rupees % 1000 != 0 {
            words = format!("{} {}{}", convert_three_digits(rupees % 1000), SCALES[scale_index], if words.is_empty() { "" } else { " " }) + &words;
        }
        rupees /= 1000;
        scale_index += 1;
    }

    if !words.is_empty() {
        words += " Rupees";
    }

    if paisa > 0 {
        if !words.is_empty() {
            words += " And ";
        }
        words += &convert_three_digits(paisa);
        words += " Paisa";
    }

    if words.is_empty() {
        words += "Zero Rupees";
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

    words
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
    #[case(27200.90, "Twenty Seven Thousand Two Hundred  Rupees And Ninety Paisa Only")]
    #[case(2720.90, "Two Thousand Seven Hundred Twenty  Rupees And Ninety Paisa Only")]
    #[case(272.90, "Two Hundred Seventy Two  Rupees And Ninety Paisa Only")]
    #[case(27.29, "Twenty Seven  Rupees And Twenty Nine Paisa Only")]
    #[case(0.0, "Zero Rupees Only")]
    #[case(0.01, "Zero Rupees And One Paisa Only")]
    #[case(0.99, "Zero Rupees And Ninety Nine Paisa Only")]
    #[case(1.0, "One Rupee Only")]
    #[case(1.01, "One Rupee And One Paisa Only")]
    #[case(1.99, "One Rupee And Ninety Nine Paisa Only")]
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
    fn fsadt(#[case]amt: f64, #[case]words: &str) {
        let result = money_to_words(amt);
        assert_eq!(result, words)
    }
}