use std::usize;

use textwrap::wrap;
use unicode_width::UnicodeWidthStr;

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

fn closest_no_multiple_of_four(n: usize) -> usize {
    let mut max: usize = n;
    if n == 0 {
        max += 1;
    }
    while max % 4 != 0 {
        max += 1;
    }
    max
}

fn main() {
    let message = "fnonfi34nfi n34oif34 nf3i4fnssss";
    let mut lines = wrap(message, 100 as usize);
    let mut longest = lines.iter().map(|line| line.width_cjk()).max().unwrap();
    longest = closest_no_multiple_of_four(longest);

    println!("{}", SAY_CHARS.top.repeat(longest + 1));
    // lines.iter().map(|line| println!("{}", line[0]));
    println!("{}", SAY_CHARS.bottom.repeat((longest / 2) + 1));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn closest_no_test() {
        assert_eq!(closest_no_multiple_of_four(1), 4);
        assert_eq!(closest_no_multiple_of_four(3), 4);
        assert_eq!(closest_no_multiple_of_four(5), 8);
        assert_eq!(closest_no_multiple_of_four(8), 8);
        assert_eq!(closest_no_multiple_of_four(0), 4);
    }
}
