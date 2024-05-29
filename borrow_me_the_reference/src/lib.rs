pub fn delete_and_backspace(s: &mut String) {
    let mut res = String::new();
    let mut res2 = String::new();
    let mut final_string = String::new();
    for i in s.chars() {
        if i == '-' {
            res.pop();
        } else {
            res.push(i);
        }
    }
    for i in res.chars().rev() {
        if i == '+' {
            res2.pop();
        } else {
            res2.push(i);
        }
    }
    for i in res2.chars().rev() {
        final_string.push(i);
    }
    *s = final_string;
}

pub fn do_operations(v: &mut Vec<String>) {
    let mut res_vec = vec![];

    for val in &mut *v {
        let temp: Vec<&str> = if val.contains("-") {
            val.split("-").collect()
        } else {
            val.split("+").collect()
        };

        let mut first_value = 0;
        let mut second_value = 0;

        let result: Result<i32, _> = temp[0].to_string().parse();
        match result {
            Ok(value) => first_value = value,
            Err(e) => println!(" {}", e),
        }

        let result: Result<i32, _> = temp[1].to_string().parse();
        match result {
            Ok(value) => second_value = value,
            Err(e) => println!(" {}", e),
        }

        let result = if val.contains("-") {
            first_value - second_value
        } else {
            first_value + second_value
        };

        res_vec.push(result.to_string());
    }
    *v = res_vec;

    // println!("{:?}", );
}

fn main() {
    // let mut a = String::from("bpp--o+er+++sskroi-++lcw");
    let mut b: Vec<String> = vec![
        "2+2".to_string(),
        "3+2".to_string(),
        "10-3".to_string(),
        "5+5".to_string(),
    ];

    // delete_and_backspace(&mut a);
    do_operations(&mut b);

    println!("{:?}", b);
}
