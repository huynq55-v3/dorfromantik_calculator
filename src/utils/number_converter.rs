pub struct NumberSystemConverter {
    unicode_letters: Vec<char>,
    number_base: usize,
}

impl NumberSystemConverter {
    pub fn new() -> Self {
        let excluded: Vec<char> = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let unicode_areas: Vec<(u32, u32)> = vec![(48, 57), (65, 90), (97, 122)];

        let mut unicode_letters = Vec::new();
        for (start, end) in unicode_areas {
            for code in start..=end {
                if let Some(ch) = std::char::from_u32(code) {
                    if !excluded.contains(&ch) {
                        unicode_letters.push(ch);
                    }
                }
            }
        }
        let number_base = unicode_letters.len();

        Self {
            unicode_letters,
            number_base,
        }
    }

    pub fn decode_number_as_long(&self, encoded_number: &str, number_can_be_negative: bool) -> i64 {
        let mut num: i64 = 0;
        let chars: Vec<char> = encoded_number.chars().collect();
        let len = chars.len();

        for (i, &ch) in chars.iter().enumerate() {
            let num2 = self.unicode_letters.iter().position(|&c| c == ch).unwrap_or(0) as i64;
            let exponent = (len - 1 - i) as u32;
            let power = (self.number_base as i64).pow(exponent);
            num += num2 * power;
        }

        if number_can_be_negative {
            num = num - 2147483647 - 1;
        }
        num
    }

    pub fn decode_number_as_digits(&self, encoded_number: &str, new_base: i64) -> Vec<usize> {
        let mut val = self.decode_number_as_long(encoded_number, false);
        let mut digits = Vec::new();

        if val == 0 {
            digits.push(0);
        } else {
            while val > 0 {
                digits.push((val % new_base) as usize);
                val /= new_base;
            }
            digits.reverse();
        }

        while digits.len() < 10 {
            digits.insert(0, 0);
        }

        digits
    }
}
