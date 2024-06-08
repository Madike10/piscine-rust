//In this exercise, you'll create the function spell that will spell a generated number.
//
// Here are some examples of what your function should return:
//
// 1 -> "one"
// 14 -> "fourteen".
// 96 -> "ninety-six"
// 100 -> "one hundred".
// 101 -> "one hundred one"
// 348 -> "three hundred forty-eight"
// 1002 -> "one thousand two".
// 1000000 -> "one million"
// Only positive numbers will be tested, up to "one million".

fn u_to_s(u: u32) -> &'static str {
    match u {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",
        100 => "hundred",
        1000 => "thousand",
        1000000 => "million",
        _ => panic!("unexpected value"),
    }
}

fn spell_util(num: u32, result: &mut String) {
    match num {
        1..=20 | 30 | 40 | 50 | 60 | 70 | 80 | 90 => result.push_str(u_to_s(num)),
        21..=99 => {
            result.push_str(u_to_s(num - num % 10));
            result.push('-');
            spell_util(num % 10, result);
        }
        100..=999 => {
            spell_util(num / 100, result);
            result.push_str(" hundred ");
            if num % 100 != 0 {
                spell_util(num % 100, result);
            }
        }
        1000..=999_999 => {
            spell_util(num / 1000, result);
            result.push_str(" thousand ");
            if num % 1000 != 0 {
                spell_util(num % 1000, result);
            }
        }
        _ => (),
    }
}

pub fn spell(num: u32) -> String {
    if num == 0 {
        return String::from("zero");
    }
    if num == 1_000_000 {
        return String::from("one million");
    }

    let mut result = String::new();
    spell_util(num, &mut result);

    result.trim().to_string()
}
