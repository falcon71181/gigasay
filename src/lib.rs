use std::fmt::format;

pub struct Options {
    pub width: u32,
}

struct Chars {
    arrow: &'static str,
    top: &'static str,
    bottom: &'static str,
    line: &'static str,
    single_left: &'static str,
    single_right: &'static str,
    angled_up: &'static str,
}

static GIGACHAD: &str = "
⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⢴⣮⣽⣿⣿⣿⣿⣿⣯⣭⣭⣿⣢⢄⡀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⣴⣿⣾⣿⣿⣿⣿⣾⣿⣿⣿⣿⣿⣿⣿⣿⣷⣿⢆⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢀⣾⣿⣿⣿⣿⣿⣿⡿⠛⠋⠙⣉⠛⣛⣿⣿⣿⠟⠛⢧⢷⠀⠀
⠀⠀⠀⠀⠀⠀⡼⣿⣿⣿⣿⣿⣿⠯⠄⠀⠀⠀⠀⣦⣤⣽⣿⣟⣗⣄⠈⢣⡗⠀
⠀⠀⠀⠀⠀⢠⢿⣿⣿⣿⣿⣿⣿⡴⠚⠉⠀⢀⣤⣬⣬⣿⣿⣿⠹⣿⡇⠀⣿⠀
⠀⠀⠀⠀⠀⢸⢸⣿⣿⣿⣿⣿⠋⠀⠀⢠⠴⠟⣛⣿⣿⣿⣿⣿⣶⣾⣰⡀⢹⡢
⠀⠀⠀⠀⠀⣸⢾⠟⠻⣿⣿⠇⠀⠀⠀⠐⢿⣿⣿⣿⣿⣿⣿⡟⢻⢻⣿⣿⣶⡇
⠀⠀⠀⠀⢀⣾⣏⣐⡄⠀⣯⡀⠀⠀⠀⠀⠀⠙⢿⣿⣿⣿⣿⠄⠘⣿⣿⣿⣷⡅
⠀⠀⠀⠀⢸⣤⣿⣿⠀⠀⣿⣷⡀⠀⠀⠀⣠⣶⣿⣿⣿⣿⠇⣄⣀⠸⡾⣷⡄⡇
⠀⠀⠀⠀⠈⠣⣃⡈⢉⣸⣿⡻⣿⣮⣴⣾⡏⢀⣽⣿⣿⣿⣶⣶⣶⣴⣇⣿⠀⣱
⠀⠀⠀⠀⠀⠀⡏⡏⠁⣿⢿⣆⣿⣿⣿⣿⣧⣿⣿⣿⣛⣿⣿⣿⣿⡦⣾⡟⢠⣃
⠀⠀⠀⠀⠀⠀⣧⡇⢠⡏⢂⢹⣿⣿⣿⣿⣿⣿⣿⣿⡷⣬⣭⣙⡛⢳⣼⣿⣿⣎
⠀⠀⠀⠀⠀⢠⢿⠀⠘⣿⣧⣵⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣾⣿⣿⣥⣿⣿⢿⡿
⠀⠀⠀⠀⠀⢸⡟⠀⠀⠙⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⣸⣿⢻⡿⠀
⠀⠀⠀⠀⠀⣯⡇⠀⠀⠀⠀⠈⠙⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⢸⠁⠀
⠀⠀⠀⢀⣴⠟⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣜⡆⠀
⣒⠶⡛⠛⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⣿⣿⣿⣿⣿⣿⣿⡿⣠⡟⠀⠀
";

static SAY_CHARS: Chars = Chars {
    arrow: "\\",
    top: "-",
    bottom: "- ",
    line: "|",
    single_left: "<",
    single_right: ">",
    angled_up: "/",
};

fn generate_gigasay_message(message: &str, opts: Options) -> String {
    format!(
        "
    {}
  {}
 {}
  {}
   {}
    {}",
        message,
        SAY_CHARS.angled_up,
        SAY_CHARS.single_left,
        SAY_CHARS.arrow,
        SAY_CHARS.arrow,
        SAY_CHARS.arrow
    )
}

fn closest_no_multiple_of_four(n: &mut usize) -> usize {
    if *n == 0 {
        closest_no_multiple_of_four(&mut (*n + 1))
    } else if *n % 4 == 0 {
        *n
    } else {
        closest_no_multiple_of_four(&mut (*n + 1))
    }
}

pub fn print_message(message: &str, opts: Options) {
    println!("{}{}", generate_gigasay_message(message, opts), GIGACHAD);
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
