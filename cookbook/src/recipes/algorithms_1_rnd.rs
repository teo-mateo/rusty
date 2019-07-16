extern crate rand;
use rand::distributions::{Alphanumeric, Distribution, Normal, Standard, Uniform};
use rand::Rng;
use std::fmt;

#[allow(dead_code)]
pub fn cook_1_random_numbers() {
    println!("___cook_1_random_numbers",);

    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}

#[allow(dead_code)]
pub fn cook_2_random_numbers_within_range() {
    println!("___cook_2_random_numbers_within_range");

    println!("Integer: {}", rand::thread_rng().gen_range(0, 10));
    println!("Float: {}", rand::thread_rng().gen_range(0., 1000.));
}

pub fn cook_2_random_bool() {
    println!("___cook_2_random_bool",);
    println!(
        "Random bool: {}",
        rand::thread_rng().gen_bool(rand::thread_rng().gen::<f64>())
    );
}

pub fn cook_3_random_with_normal_distribution() {
    println!("___cook_3_random_with_normal_distribution",);

    let mut rng = rand::thread_rng();
    let normal = Normal::new(2.0, 3.0);
    let v = normal.sample(&mut rng);
    println!("{} is from a N(2, 9) distribution", v)
}

pub fn cook_2_roll_the_dice() {
    println!("___cook_2_roll_the_dice");

    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the dice: {}", throw);
        if throw == 6 {
            return;
        }
    }
}

pub fn cook_4_random_values_of_custom_tuple() {
    println!("___cook_4_random_values_of_custom_tuple",);

    let mut rng = rand::thread_rng();
    let tuple = rng.gen::<(i8, f32)>();
    println!("Here is a random tuple: ({}, {})", tuple.0, tuple.1);
}

pub fn cook_5_random_custom_type() {
    println!("___cook_5_random_custom_type");

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Distribution<Point> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
            let (rand_x, rand_y) = rng.gen();
            Point {
                x: rand_x,
                y: rand_y,
            }
        }
    }

    let mut rng = rand::thread_rng();
    let random_point: Point = rng.gen();
    println!("Random Point: {:?}", random_point);
}

pub fn cook_6_random_password_from_alfanum_characters() {
    println!("___cook_6_random_password_from_alfanum_characters",);

    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();

    println!("Random string: {}", random_string);
}

pub fn cook_7_random_password_from_userdefined_characters() {
    println!("___cook_7_random_password_from_userdefined_characters");

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();
    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            char::from(CHARSET[idx])
        })
        .collect();
    println!("{:?}", password);
}
