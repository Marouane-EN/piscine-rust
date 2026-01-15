pub fn rotate(input: &str, key: i8) -> String {
    let mut res = String::new();
    for c in input.chars() {
        if c.is_ascii_uppercase() {
            let first = 'A' as u8;
            res.push((((((c as i8) - (first as i8) + key + 26) % 26) as u8) + first) as char);
        } else if c.is_ascii_lowercase() {
            let first = 'a' as u8;
            res.push((((((c as i8) - (first as i8) + key + 26) % 26) as u8) + first) as char);
        } else {
            res.push(c);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotates_a_by_26() {
        assert_eq!(rotate("a", 26), "a");
    }

    #[test]
    fn rotates_m_by_13() {
        assert_eq!(rotate("m", 13), "z");
    }

    #[test]
    fn rotates_a_by_15() {
        assert_eq!(rotate("a", 15), "p");
    }

    #[test]
    fn rotates_word_miss_by_5() {
        assert_eq!(rotate("MISS", 5), "RNXX");
    }

    #[test]
    fn decodes_rot13_sentence() {
        assert_eq!(
            rotate("Gur svir obkvat jvmneqf whzc dhvpxyl.", 13),
            "The five boxing wizards jump quickly."
        );
    }

    #[test]
    fn decodes_rot5_sentence() {
        assert_eq!(
            rotate("Mtb vznhpqd ifky ozrunsl ejgwfx ajc", 5),
            "Ryg aesmuvi nkpd tewzsxq jolbkc foh"
        );
    }

    #[test]
    fn rotates_sentence_with_numbers_by_4() {
        assert_eq!(rotate("Testing with numbers 1 2 3", 4), "Xiwxmrk amxl ryqfivw 1 2 3");
    }

    #[test]
    fn rotates_testing_by_negative_14() {
        assert_eq!(rotate("Testing", -14), "Fqefuzs");
    }

    #[test]
    fn rotates_a_by_negative_1() {
        assert_eq!(rotate("a", -1), "z");
    }
}
