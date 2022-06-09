use std::io;

fn main() {
    let mut f;

    loop {
        println!("Convert fahrenheit or celsius? (f/c)");

        f = String::new();

        io::stdin().read_line(&mut f)
            .expect("Failed to read line");

        if f.trim() == "f" {
            convert_f_to_c();

            break;
        } else if f.trim() == "c" {
            convert_c_to_f();

            break;
        }
    }
}

fn convert_f_to_c() {
    let mut f = String::new();

    loop {
        println!("Input fahrenheit number");

        io::stdin().read_line(&mut f)
            .expect("Failed to read line");

        let c: f64 = match f.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{} celsius", (c - 32.0) / 1.8);

        break;
    }
}

fn convert_c_to_f() {
    let mut c = String::new();

    loop {
        println!("Input celsius number");

        io::stdin().read_line(&mut c)
            .expect("Failed to read line");

        let f: f64 = match c.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{} fahrenheit", (f * 1.8) + 32.0);

        break;
    }
}
