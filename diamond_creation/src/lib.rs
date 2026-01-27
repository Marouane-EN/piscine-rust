pub fn get_diamond(c: char) -> Vec<String> {
    let t = ((c as u8) - b'A') as i32;
    let size = t * 2 + 1;
    let mut diamond = Vec::new();

    for i in 0..=t {
        diamond.push(make_line(i, t, size));
    }

    for i in (0..t).rev() {
        diamond.push(make_line(i, t, size));
    }

    diamond
}

fn make_line(i: i32, t: i32, size: i32) -> String {
    let current_char = (b'A' + (i as u8)) as char;
    let mut line = vec![' '; size as usize];

    let l_p = (t - i) as usize;
    let r_p = (t + i) as usize;

    line[l_p] = current_char;
    line[r_p] = current_char;

    line.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
