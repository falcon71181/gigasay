use textwrap::wrap;
use unicode_width::UnicodeWidthStr;

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
    bottom: " -",
    line: "|",
    single_left: "<",
    single_right: ">",
    angled_up: "/",
};

// TODO: opts for future
fn generate_gigasay_message(message: &str, _opts: Options) -> String {
    let mut lines = wrap(message, 100 as usize);
    let mut longest = lines.iter().map(|line| line.width_cjk()).max().unwrap();
    longest = closest_no_multiple_of_four(&mut longest);
    let chars = &SAY_CHARS;
    format!(
        "
    {}
  {}
 {}
  {}
   {}
    {}",
        chars.top.repeat(longest + 1),
        if lines.len() == 1 {
            format!("{} {} {}", chars.single_left, lines[0], chars.single_right)
        } else {
            let mut result = format!(
                "{} {}{}{}",
                chars.angled_up,
                lines[0],
                " ".repeat(longest - lines[0].width() + 1),
                chars.angled_up
            );
            lines.remove(0);
            let last = lines.pop().unwrap();

            for line in lines {
                result = format!(
                    "{}\n{} {}{}{}",
                    result,
                    chars.line,
                    line,
                    " ".repeat(longest - line.width() + 1),
                    chars.line,
                );
            }

            format!(
                "{}\n{} {}{}{}",
                result,
                chars.angled_up,
                last,
                " ".repeat(longest - last.width() + 1),
                chars.angled_up,
            )
        },
        chars.bottom.repeat((longest / 2) + 1),
        SAY_CHARS.angled_up,
        SAY_CHARS.arrow,
        SAY_CHARS.arrow,
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
