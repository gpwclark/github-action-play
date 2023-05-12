use printer::stringerang;

fn main() {
    println!("{}", stringerang("hello world"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn returns_orig() {
        let my_str = "ok";
        let same = stringerang(my_str);
        assert_eq!(my_str, same);
    }
}

