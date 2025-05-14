use std::io::{stdin, BufRead};

use std::fs::File;
use std::io::prelude::*;


fn input_cal() -> i32{
    let std= stdin();
    let input_cal= std.lock().lines()
        .next().unwrap().unwrap()
        .parse::<i32>().unwrap();
    
    input_cal
}


fn file_save(cal: i32) -> std::io::Result<()> {
    let mut file = File::create("cal.txt")?;
    file.write_all(cal.to_string().as_bytes())?;
    
    Ok(())
}

fn read_cal() -> std::io::Result<i32> {
    let mut file = File::open("cal.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    Ok(contents.parse::<i32>().unwrap())
}

fn BPtest()-> i32{
    println!("how many you do?");

    let std= stdin();
    let times= std.lock().lines()
        .next().unwrap().unwrap()
        .parse::<f32>().unwrap();

    (times* 1.26) as i32
}

fn daily_minus() -> i32{
    println!("took a food");

    let std= stdin();
    let cal= std.lock().lines()
        .next().unwrap().unwrap()
        .parse::<i32>().unwrap();

    2500-cal
}

fn select_simple_menu()-> i32{
    const SWIMMING:i32 = 250;
    const JOGGING:i32= 250;
    const WEIGHT_TRAINING: i32= 300;

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

    let selcted= match input_number {
        0 => return input_cal(),
        1 => return SWIMMING,
        2 => return JOGGING,
        3 => return BPtest(),
        4 => return WEIGHT_TRAINING,
        99 => return daily_minus(),
        _ => 0
    };

    return selcted;
}

fn main() {
    // 156000
    let mut remained_cal: i32= match read_cal() {
        Ok(n) => n,
        Err(e) => 0,
        _ => 0
    };


    let minus= select_simple_menu();
    
    remained_cal-= minus;

    println!("Remained: {}", remained_cal);
    println!("Remained: {}", remained_cal/7800 + 1);

    file_save(remained_cal);

}