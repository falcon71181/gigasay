struct Chars {
    arrow: &'static str,
    top: &'static str,
    bottom: &'static str,
    left: &'static str,
    right: &'static str,
    single_left: &'static str,
    single_right: &'static str,
    angled_up_right: &'static str,
    angled_up_left: &'static str,
    angled_down_right: &'static str,
    angled_down_left: &'static str,
}

static SAY_CHARS: Chars = Chars {
    arrow: "\\",
    top: "-",
    bottom: "- ",
    left: "|",
    right: "|",
    single_left: "<",
    single_right: ">",
    angled_up_right: "/",
    angled_up_left: "\\",
    angled_down_right: "\\",
    angled_down_left: "/",
};

fn closest_no_multiple_of_four(n: &mut usize) -> usize {
    if *n == 0 {
        closest_no_multiple_of_four(&mut (*n + 1))
    } else if *n % 4 == 0 {
        *n
    } else {
        closest_no_multiple_of_four(&mut (*n + 1))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn closest_no_test() {
        assert_eq!(closest_no_multiple_of_four(&mut 1), 4);
        assert_eq!(closest_no_multiple_of_four(&mut 3), 4);
        assert_eq!(closest_no_multiple_of_four(&mut 5), 8);
        assert_eq!(closest_no_multiple_of_four(&mut 8), 8);
        assert_eq!(closest_no_multiple_of_four(&mut 0), 4);
    }
}
