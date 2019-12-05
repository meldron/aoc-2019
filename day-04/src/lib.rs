#[allow(dead_code)]
#[inline]
pub fn number_to_digits(i: usize) -> Vec<u8> {
    let mut v = Vec::new();
    let mut n = i;

    loop {
        if n == 0 {
            break;
        }

        let d = (n % 10) as u8;
        v.insert(0, d);

        n /= 10;
    }

    v
}

#[inline]
pub fn number_to_digits_string(i: usize) -> Vec<u8> {
    i.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

pub fn verify_password(p: usize) -> Result<usize, String> {
    let digits = number_to_digits_string(p);

    if digits.len() != 6 {
        return Err("Password must have 6 digits".to_owned());
    }

    let mut same_same = false;
    let mut last_digit = std::u8::MIN;

    for d in digits {
        if d < last_digit {
            return Err("Decreasing digit".to_owned());
        }

        if last_digit == d {
            same_same = true;
        }

        last_digit = d;
    }

    if !same_same {
        return Err("Two adjacent digits have to be the same".to_owned());
    }

    Ok(p)
}

pub fn verify_password_2(p: usize) -> Result<usize, String> {
    let digits = number_to_digits_string(p);

    if digits.len() != 6 {
        return Err("Password must have 6 digits".to_owned());
    }

    let mut double = false;
    let mut digit_count: usize = 0;
    let mut last_digit = std::u8::MIN;

    for d in digits {
        if d < last_digit {
            return Err("Decreasing digit".to_owned());
        }

        if last_digit != d {
            if digit_count == 2 {
                double = true;
            }
            digit_count = 0;
        }

        digit_count += 1;

        last_digit = d;
    }

    if digit_count == 2 {
        double = true;
    }

    if !double {
        return Err("No valid double".to_owned());
    }

    Ok(p)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn number_to_digit_valid_output() {
        let d = number_to_digits(85_712_648);
        assert_eq!(d, vec!(8, 5, 7, 1, 2, 6, 4, 8));
    }

    #[test]
    fn number_to_digit_string_valid_output() {
        let d = number_to_digits_string(85_712_648);
        assert_eq!(d, vec!(8, 5, 7, 1, 2, 6, 4, 8));
    }

    #[test]
    fn test_verify_password_1() {
        assert_eq!(verify_password(111_111), Ok(111_111));
        assert_eq!(verify_password(122_345), Ok(122_345));
        assert_eq!(verify_password(135_679), Err("Two adjacent digits have to be the same".to_owned()));
        assert_eq!(verify_password(223_450), Err("Decreasing digit".to_owned()));
    }

    #[test]
    fn test_verify_password_2() {
        assert_eq!(verify_password_2(122_345), Ok(122_345));
        assert_eq!(verify_password_2(111_122), Ok(111_122));

        assert_eq!(verify_password_2(123_444), Err("No valid double".to_owned()));
        assert_eq!(verify_password_2(135_679), Err("No valid double".to_owned()));
        assert_eq!(verify_password_2(223_450), Err("Decreasing digit".to_owned()));
    }
}
