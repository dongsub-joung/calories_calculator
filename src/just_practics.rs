use std::io::{stdin, BufRead};

use std::fs::File;
use std::io::prelude::*;

struct  Cals<T> {
    sport: T,
}

fn read_cal() -> std::io::Result<i32> {
    let mut file = File::open("cal.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    Ok(contents.parse::<i32>().unwrap())
}

fn file_save(cal: i32) -> std::io::Result<()> {
    let mut file = File::create("cal.txt")?;
    file.write_all(cal.to_string().as_bytes())?;
    
    Ok(())
}

fn input_cal() -> i32{
    let std= stdin();
    let input_cal= std.lock().lines()
        .next().unwrap().unwrap()
        .parse::<i32>().unwrap();
    
    input_cal
}

fn select_simple_menu()-> i32{
    println!("0. input cal");
    println!("1. Swimming: 250cal");
    println!("2. jogging: 250cal");
    println!("3. burpee: per 1.26cal");
    println!("4. weight training: 300cal");
    println!("99. food");

    let std= stdin();
    let input_number= std.lock().lines()
        .next().unwrap().unwrap()
        .parse::<i32>().unwrap();

    return input_number;
}

fn main(){
    let mut remained_cal: i32= match read_cal() {
        Ok(n) => n,
        Err(e) => 0,
        _ => 0
    };

    let selected_number= select_simple_menu();
    
    let swimming= Cals{ sport: 250};
    let jogging= Cals{ sport: 300};
    let BPtest= Cals{ sport: 250};
    let weight= Cals{ sport: 1.26};
    let user_input_cal= Cals{ sport: input_cal()};

    let cal_point= match selected_number {
        0 => user_input_cal,
        1 => swimming,
        2 => jogging,
        3 => BPtest,
        4 => weight,
        99 => swimming,
        _ => panic!(),
    }
}