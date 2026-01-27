pub fn spell(n: u64) -> String {
    if n == 0 {
        return String::from("zero");
    }
    if n == 1_000_000 {
        return String::from("one million");
    }

    helper(n).trim().to_string()
}

fn helper(n: u64) -> String {
    let units = [
        "",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let tens = [
        "",
        "",
        "twenty",
        "thirty",
        "forty",
        "fifty",
        "sixty",
        "seventy",
        "eighty",
        "ninety",
    ];

    if n >= 1000 {
        let thousands = n / 1000;
        let remainder = n % 1000;
        let suffix = if remainder > 0 { format!(" {}", helper(remainder)) } else { String::new() };
        return format!("{} thousand{}", helper(thousands), suffix);
    }

    if n >= 100 {
        let hundreds = n / 100;
        let remainder = n % 100;
        let suffix = if remainder > 0 { format!(" {}", helper(remainder)) } else { String::new() };
        return format!("{} hundred{}", helper(hundreds), suffix);
    }

    if n >= 20 {
        let ten_val = n / 10;
        let unit_val = n % 10;
        if unit_val > 0 {
            return format!("{}-{}", tens[ten_val as usize], units[unit_val as usize]);
        } else {
            return tens[ten_val as usize].to_string();
        }
    }

    units[n as usize].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(spell(348), "three hundred forty-eight");
    }
    #[test]
    fn it_works1() {
        assert_eq!(spell(9996), "nine thousand nine hundred ninety-six");
    }
}
