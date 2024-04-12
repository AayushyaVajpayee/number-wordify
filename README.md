# Number Wordify

Number Wordify is a Rust library that provides functions to convert numbers into their word representations. It supports both Indian and International number scales and allows formatting numbers with currency units.

## Features

- Convert numbers up to 9,999,999,999,999,999,999 (Indian scale) or 999,999,999,999,999 (International scale) to their word representations.
- Support for Indian number scale (Thousand, Lakh, Crore, Arab, Kharab) and International number scale (Thousand, Million, Billion).
- Format numbers with currency units (e.g., Rupees and Paise, Dollars and Cents).
- Comprehensive test suite to ensure accuracy and reliability.

## Installation

To use Number Wordify in your Rust project, add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
number-wordify = "0.1.0"
```

Here are some examples of how to use Number Wordify:
```rust
use number_wordify::{format_number_to_word, format_with_currency_in_words, NumberScale};

let amount = 1234567;
let indian_words = format_number_to_word(amount, NumberScale::Indian);
let international_words = format_number_to_word(amount, NumberScale::International);

println!("Indian: {}", indian_words);
println!("International: {}", international_words);

let amount_with_currency = 1234567.89;
let indian_currency_words = format_with_currency_in_words(amount_with_currency, NumberScale::Indian, "Rupees", "Paise", 100);
let international_currency_words = format_with_currency_in_words(amount_with_currency, NumberScale::International, "Dollars", "Cents", 100);

println!("Indian Currency: {}", indian_currency_words);
println!("International Currency: {}", international_currency_words);
```
Output of above:
```
Indian: Twelve Lakh Thirty Four Thousand Five Hundred Sixty Seven
International: One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven
Indian Currency: Twelve Lakh Thirty Four Thousand Five Hundred Sixty Seven Rupees And Eighty Nine Paise Only
International Currency: One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven Dollars And Eighty Nine Cents Only
```