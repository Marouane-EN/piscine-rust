
pub fn delete_and_backspace(s: &mut String) {
    let mut new_s = String::new();
    let mut skip = 0;
    for c in s.chars() {
        if c != '-' && c != '+' {
            if skip != 0 {
                skip -= 1;
                continue;
            }
            new_s.push(c);
            continue;
        }
        if c == '+' {
            skip += 1;
            continue;
        }
        if c == '-' {
            new_s.pop();
            continue;
        }
    }
    *s = new_s;
}

pub fn do_operations(v: &mut [String]) {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut sum = 0;
    let mut skip = false;
    for s in v {
        for c in s.chars() {
            match c {
                '+' => {
                    if !num2.is_empty() {
                        sum = num1.parse::<i32>().unwrap() + num2[1..].parse::<i32>().unwrap();
                        num1 = sum.to_string();
                        num2 = "".to_owned();
                        skip = true;
                    }
                    num2.push(c);
                }
                '-' => {
                    if !num2.is_empty() {
                        sum = num1.parse::<i32>().unwrap() - num2[1..].parse::<i32>().unwrap();
                        num1 = sum.to_string();
                        num2 = "".to_owned();
                        skip = true;
                    }
                    num2.push(c);
                }
                _ => {
                    if num2.is_empty() && !skip {
                        num1.push(c);
                    } else {
                        num2.push(c);
                    }
                }
            }
        }
        if !num2.is_empty() {
            if num2.contains('+') {
                sum = num1.parse::<i32>().unwrap() + num2[1..].parse::<i32>().unwrap();
            } else {
                sum = num1.parse::<i32>().unwrap() - num2[1..].parse::<i32>().unwrap();
            }
        }
        *s = sum.to_string();
        num1 = "".to_owned();
        num2 = "".to_owned();
        skip = false;
        sum = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = "bpp--o+er+++sskroi-++lcw".to_owned();
        delete_and_backspace(&mut a);
        assert_eq!(a, "borrow");
    }
    #[test]
    fn it_works1() {
        let mut b = ["2+2+2".to_owned(), "3+2".to_owned(), "10-3".to_owned(), "5+5".to_owned()];
        do_operations(&mut b);
        assert_eq!(b, ["6", "5", "7", "10"]);
    }
     #[test]
    fn it_works2() {
        let b = "+2".to_owned();
        assert_eq!(b[1..].parse::<i32>().unwrap(), 2);
    }
}
