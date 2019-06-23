//Name:Muhammad Ali Khan
//Ph# 03101097145
//Rust Hackathon
use std::io;
use std::io::{stdin, stdout, Read, Write};
fn main() {
    loop {
        print!("\n1)Calculate area of circle\n2)Check positive or negative\n3)Check divisibility\n4)Volume of sphere\n5)N-times string\n6)Even-Odd check\n7)Vowel Tester\n8)Triangle area\n9)Interest Calculate\n10)Euclidean distance\n11)Feet-CM Converter\n12)BMI calculate\n13)Sum of n Positive integers\n14)Digits sum of a number\n15)Decimal to Binary Converter\n16)Binary to Decimal Converter\n17)Vowel and Consonant Counter\n18)Palindrome tester\n19)Count Alphabets,Numbers and Special Characters\n20)Right Star Pyramid\n21)Right Number Pyramid\n22)Number Pattern\n23)Days Calculator.\nAny other number to exit.\nInput: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().parse::<u32>().expect("Input a number");
        clrscr();
        match input {
            1 => calc_area(),
            2 => check_sign(),
            3 => check_if_divisible(),
            4 => sphere_vol(),
            5 => n_strings(),
            6 => check_even_odd(),
            7 => check_vowel(),
            8 => triangle_area(),
            9 => calc_interest(),
            10 => euclidean_dist(),
            11 => cvt_feet_cm(),
            12 => bmi(),
            13 => sum_n(),
            14 => sum_of_a_number(),
            15 => dec_bin(),
            16 => bin_to_dec(),
            17 => vowel_consonant_count(),
            18 => palindrome_check(),
            19 => count_text(),
            20 => right_pyramid(),
            21 => right_pyramid_num(),
            22 => number_pattern(),
            23 => days_calculator(),
            _ => break,
        }
    }

}
fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn dec_bin() {
    print!("Enter a decimal number: ");
    io::stdout().flush().unwrap();
    let mut var = String::new();
    io::stdin()
        .read_line(&mut var)
        .expect("Failed to read line");
    let mut var = var.trim().parse::<u32>().expect("Input a number");
    let dec = var;
    let mut remainder: u32;
    let mut binary: u32;
    binary = 0;
    let mut base = 1;

    while var > 0 {
        remainder = var % 2;
        binary += remainder * base;
        var /= 2;
        base *= 10;
    }
    println!("Binary representation of {} is {}", dec, binary);
    pause();
}
fn clrscr() {
    for _ in 0..50 {
        println!();
    }
}
fn calc_area() {
    print!("Input radius: ");
    io::stdout().flush().unwrap();
    let mut var = String::new();
    io::stdin()
        .read_line(&mut var)
        .expect("Failed to read line");
    let mut var = var.trim().parse::<f32>().expect("Input a number");
    var = 3.142 * var * var;
    println!("Area of circle is {}", var);
    pause();
}
fn check_sign() {
    print!("Enter number: ");
    io::stdout().flush().unwrap();
    let mut var = String::new();
    io::stdin()
        .read_line(&mut var)
        .expect("Failed to read line");
    let var = var.trim().parse::<i32>().expect("Input a number");
    if var == 0 {
        println!("Zero entered");
    } else if var > 0 {
        println!("Positive number entered.");
    } else {
        println!("Negative number entered.");
    }
    pause();
}
fn check_if_divisible() {
    print!("Enter numerator: ");
    io::stdout().flush().unwrap();
    let mut numerator = String::new();
    io::stdin()
        .read_line(&mut numerator)
        .expect("Failed to read line");

    print!("Enter denominator: ");
    io::stdout().flush().unwrap();
    let mut denominator = String::new();
    io::stdin()
        .read_line(&mut denominator)
        .expect("Failed to read line");
    let numerator = numerator.trim().parse::<u32>().expect("Input a number");
    let denominator = denominator.trim().parse::<u32>().expect("Input a number");
    if numerator % denominator == 0 {
        println!(
            "Number {} is completely divisible by {} ",
            numerator, denominator
        );
    } else {
        println!(
            "Number {} is not completely divisible by {} ",
            numerator, denominator
        );
    }
    pause();
}
fn sphere_vol() {

    print!("Enter radius of sphere: ");
    io::stdout().flush().unwrap();
    let mut r = String::new();
    io::stdin().read_line(&mut r).expect("Failed to read line");
    let r = r.trim().parse::<f32>().expect("Input a number");
    let vol = (4.0 / 3.0) * 3.142 * (r * r * r);
    println!("Volume of sphere with radius {} is {}", r, vol);
    pause();
}
fn n_strings() {
    print!("Enter String: ");
    io::stdout().flush().unwrap();
    let mut string = String::new();
    io::stdin()
        .read_line(&mut string)
        .expect("Failed to read line");
    string.pop();
    print!("How many copies of String you need: ");
    io::stdout().flush().unwrap();
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let mut n = n.trim().parse::<u32>().expect("Input a number");
    print!("{} copies of {} are ", n, string);
    io::stdout().flush().unwrap();
    loop {
        if n <= 0 {
            println!();
            break;
        }
        n -= 1;
        print!("{}", string);
        io::stdout().flush().unwrap();
    }
    pause();
}
fn check_even_odd() {
    print!("Enter number: ");
    io::stdout().flush().unwrap();
    let mut var = String::new();
    io::stdin()
        .read_line(&mut var)
        .expect("Failed to read line");
    let var = var.trim().parse::<u32>().expect("Input a number");
    if var % 2 == 0 {
        println!("{} is even", var);
    } else {
        println!("{} is odd", var);
    }
    pause();
}
fn check_vowel() {
    let mut flag = false;
    print!("Enter a character: ");
    io::stdout().flush().unwrap();
    let mut var = String::new();
    io::stdin()
        .read_line(&mut var)
        .expect("Failed to read line");
    loop {
        if var.len() == 1 {
            break;
        }
        var.pop();
    }
    if var == "a" || var == "A" {
        flag = true;
    }
    if var == "e" || var == "E" {
        flag = true;
    }
    if var == "i" || var == "I" {
        flag = true;
    }
    if var == "o" || var == "O" {
        flag = true;
    }
    if var == "u" || var == "U" {
        flag = true;
    }
    if flag {
        println!("{} is a vowel.", var);
    } else {
        println!("{} is not a vowel.", var);
    }
    pause();
}
fn triangle_area() {
    print!("Enter magnitude of Triangle base: ");
    io::stdout().flush().unwrap();
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read line");
    let b = b.trim().parse::<f32>().expect("Input a number");
    print!("Enter Magnitude of Triangle Height: ");
    io::stdout().flush().unwrap();
    let mut h = String::new();
    io::stdin().read_line(&mut h).expect("Failed to read line");
    let h = h.trim().parse::<f32>().expect("Input a number");
    let result = (h * b) / 2.0;
    println!(
        "Area of a Triangle with Height {} and Base {} is {}",
        h, b, result
    );
    pause();
}
fn calc_interest() {
    print!("Please enter principal amount: ");
    io::stdout().flush().unwrap();
    let mut principal = String::new();
    io::stdin()
        .read_line(&mut principal)
        .expect("Failed to read line");
    let principal = principal.trim().parse::<f32>().expect("Input a number");
    print!("Please enter rate of interest in %: ");
    io::stdout().flush().unwrap();
    let mut rate = String::new();
    io::stdin()
        .read_line(&mut rate)
        .expect("Failed to read line");
    let rate = rate.trim().parse::<f32>().expect("Input a number");
    print!("Enter number of years for investment: ");
    io::stdout().flush().unwrap();
    let mut time = String::new();
    io::stdin()
        .read_line(&mut time)
        .expect("Failed to read line");
    let time = time.trim().parse::<f32>().expect("Input a number");
    let mut result = principal;
    let mut temp = time;
    loop {
        if temp == 0.0 {
            break;
        }
        temp -= 1.0;
        result += result * rate;
    }
    println!(
        "After {} years your principal amount {} over an interest rate of {}% will be {}.",
        time, principal, rate, result
    );
    pause();
}
fn euclidean_dist() {
    print!("Enter Co-ordinate for x1: ");
    io::stdout().flush().unwrap();
    let mut x1 = String::new();
    io::stdin().read_line(&mut x1).expect("Failed to read line");
    let x1 = x1.trim().parse::<u32>().expect("Input a number");
    print!("Enter Co-ordinate for x2: ");
    io::stdout().flush().unwrap();
    let mut x2 = String::new();
    io::stdin().read_line(&mut x2).expect("Failed to read line");
    let x2 = x2.trim().parse::<u32>().expect("Input a number");
    print!("Enter Co-ordinate for y1: ");
    io::stdout().flush().unwrap();
    let mut y1 = String::new();
    io::stdin().read_line(&mut y1).expect("Failed to read line");
    let y1 = y1.trim().parse::<u32>().expect("Input a number");
    print!("Enter Co-ordinate for y2: ");
    io::stdout().flush().unwrap();
    let mut y2 = String::new();
    io::stdin().read_line(&mut y2).expect("Failed to read line");
    let y2 = y2.trim().parse::<u32>().expect("Input a number");
    let x = x2 - x1;
    let y = y2 - y1;
    let dist = ((x * x) + (y * y)) * 1 / 2;
    println!(
        "Distance between points ({},{}) and ({},{}) is {} ",
        x1, y1, x2, y2, dist
    );
    pause();
}
fn cvt_feet_cm() {
    print!("Enter height in feet: ");
    io::stdout().flush().unwrap();
    let mut feet = String::new();
    io::stdin()
        .read_line(&mut feet)
        .expect("Failed to read line");
    let feet = feet.trim().parse::<f32>().expect("Input a number");
    let cm = feet * 30.48;
    println!("There are {}Cm in {}ft.", cm, feet);
    pause();
}
fn bmi() {

    print!("Enter height in cm: ");
    io::stdout().flush().unwrap();
    let mut cm = String::new();
    io::stdin().read_line(&mut cm).expect("Failed to read line");
    let cm = cm.trim().parse::<f32>().expect("Input a number");

    print!("Enter weight in KG: ");
    io::stdout().flush().unwrap();
    let mut kg = String::new();
    io::stdin().read_line(&mut kg).expect("Failed to read line");
    let kg = kg.trim().parse::<f32>().expect("Input a number");
    let bmi = (kg / (cm * cm)) * 10000.0;
    println!("Your BMI is {}.", bmi);
    pause();
}
fn sum_n() {
    print!("Enter value of n: ");
    io::stdout().flush().unwrap();
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n = n.trim().parse::<u32>().expect("Input a number");
    let sum = (n * (n + 1)) / 2;
    println!("Sum of n positive integers till {} is {}.", n, sum);
    pause();
}
fn sum_of_a_number() {

    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let mut n = n.trim().parse::<u32>().expect("Input a number");
    let mut sum = 0;
    let mut temp;
    print!("Sum of ");
    loop {
        if n <= 0 {
            break;
        } else if sum != 0 {
            print!(" + ");
        }
        temp = n % 10;
        sum += temp;
        n /= 10;
        print!("{}", temp);
    }
    println!(" is {}", sum);
    pause();
}
fn bin_to_dec() {

    print!("Enter a binary number: ");
    io::stdout().flush().unwrap();
    let mut bin = String::new();
    io::stdin()
        .read_line(&mut bin)
        .expect("Failed to read line");
    let mut bin = bin.trim().parse::<u32>().expect("Input a number");
    let orig_bin = bin;
    let mut sum = 0;
    let mut counter_1 = 0;
    let mut counter_2;
    let mut temp;
    loop {
        if bin <= 0 {
            break;
        }
        temp = bin % 10;
        if temp == 1 {
            if counter_1 == 0 {
                sum += 1;
            } else {
                let mut temp_var = 1;
                counter_2 = counter_1;
                while counter_2 != 0 {
                    temp_var *= 2;
                    counter_2 -= 1;
                }
                sum += temp_var;
            }

        }
        bin /= 10;
        counter_1 += 1;
    }
    println!("Decimal representation of {} is {}", orig_bin, sum);
    pause();
}
fn vowel_consonant_count() {

    let mut vow = 0;
    let mut con = 0;
    print!("Enter text: ");
    io::stdout().flush().unwrap();
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");
    text.pop();
    for x in text.chars() {
        if x == 'a' || x == 'A' {
            vow += 1;
        } else if x == 'e' || x == 'E' {
            vow += 1;
        } else if x == 'i' || x == 'I' {
            vow += 1;
        } else if x == 'o' || x == 'O' {
            vow += 1;
        } else if x == 'u' || x == 'U' {
            vow += 1;
        } else {
            con += 1;
        }
    }
    println!("Vowel: {}\nConsonant: {}", vow, con);
    pause();
}
fn palindrome_check() {

    print!("Enter text: ");
    io::stdout().flush().unwrap();
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");
    text.pop();
    let text = text.to_string();
    let half = text.len() / 2;
    if text.bytes().take(half).eq(text.bytes().rev().take(half)) {
        println!("Text {} is a Palindrome", text);
    } else {
        println!("Text {} is not a Palindrome", text);
    }
    pause();
}
fn count_text() {
    let mut num = 0;
    let mut alphabet = 0;
    let mut special = 0;
    let mut spaces = 0;
    print!("Enter text: ");
    io::stdout().flush().unwrap();
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");
    text.pop();
    for x in text.chars() {
        let temp = x as u8;
        if temp >= 48 && temp <= 57 {
            num += 1;
        } else if temp >= 65 && temp <= 90 || temp >= 97 && temp <= 122 {
            alphabet += 1;
        } else if temp == 32 {
            spaces += 1;
        } else {
            special += 1;
        }

    }
    println!(
        "Numbers: {}\nAlphabets: {}\nSpecial Characters: {}\nSpaces: {}",
        num, alphabet, special, spaces
    );
    pause();
}
fn right_pyramid() {

    for i in 1..6 {
        for _ in 0..i {
            print!("*");
        }
        println!();
    }
    for i in (1..5).rev() {
        for _ in 0..i {
            print!("*");
        }
        println!();
    }
    pause();
}
fn right_pyramid_num() {
    let mut j = 1;
    for i in 1..6 {
        for _ in 0..i {
            print!("{}", j);
            j += 1;
        }
        j = 1;
        println!();
    }
    for i in (1..5).rev() {
        for _ in 0..i {
            print!("{}", j);
            j += 1;
        }
        j = 1;
        println!();
    }
    pause();
}
fn number_pattern() {
    for i in 1..10 {
        for _ in 0..i {
            print!("{}", i);
        }
        println!();
    }
    pause();
}
fn days_calculator() {
    print!("Enter a date in (dd/mm/yyyy) format: ");
    io::stdout().flush().unwrap();
    let mut date_1 = String::new();
    io::stdin()
        .read_line(&mut date_1)
        .expect("Failed to read line");
    print!("Enter a date in (dd/mm/yyyy) format: ");
    io::stdout().flush().unwrap();
    let mut date_2 = String::new();
    io::stdin()
        .read_line(&mut date_2)
        .expect("Failed to read line");
    date_1.pop();
    date_2.pop();
    let day_1 = &date_1[0..2];
    let day_2 = &date_2[0..2];

    let month_1 = &date_1[3..5];
    let month_2 = &date_2[3..5];

    let year_1 = &date_1[6..10];
    let year_2 = &date_2[6..10];

    let day_1 = day_1.trim().parse::<u32>().expect("Input a number");
    let day_2 = day_2.trim().parse::<u32>().expect("Input a number");
    let month_1 = month_1.trim().parse::<u32>().expect("Input a number");
    let month_2 = month_2.trim().parse::<u32>().expect("Input a number");
    let year_1 = year_1.trim().parse::<u32>().expect("Input a number");
    let year_2 = year_2.trim().parse::<u32>().expect("Input a number");

    let year: u32;
    let month: u32;
    let day: u32;

    if year_1 > year_2 {
        year = year_1 - year_2;
    } else {
        year = year_2 - year_1;
    }

    if month_1 > month_2 {
        month = month_1 - month_2;
    } else {
        month = month_2 - month_1;
    }

    if day_1 > day_2 {
        day = day_1 - day_2;
    } else {
        day = day_2 - day_1;
    }
    let result = day + (month * 30) + (year * 365);
    println!(
        "There are {} days in between {} and {}.",
        result, date_1, date_2
    );
    pause();
}
