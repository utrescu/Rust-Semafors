use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = "in.txt";

    if args.len() > 2 {
        panic!("Please enter ONE filename as a param");
    }

    if args.len() == 2 {
        filename = &args[1];
    }

    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("Unable to read file {:?}", filename),
    };

    let buffer = BufReader::new(file);

    let mut countlines = 0;

    for line in buffer.lines() {
        countlines += 1;
        let semafors = match line {
            Ok(line) => line,
            Err(_) => panic!("Error reading line {}", countlines),
        };
        println!("{}", calcula_semafors(semafors));
    }
}

/**
 * Calcula el número de semàfors que es poden fer si es segueix l'ordre.
 */
fn calcula_semafors(mut semafors: String) -> u32 {
    let mut count_semaphors = 0;

    // println!("... Buscant {}", semafors);

    loop {
        match elimina(semafors, "RAV") {
            Ok(s) => {
                semafors = s;
                count_semaphors += 1;
            }
            Err(_) => {
                break;
            }
        }
    }
    count_semaphors
}

/**
 * Elimina una instància de cada un dels caràcters de la cadena rebuda
 *
 * Si no els troba tots retorna zero.
 */
fn elimina(semafors: String, caracters: &str) -> Result<String, u8> {
    let mut result = String::new();
    let mut index = 0;

    for car in semafors.chars() {
        if index < 3 && car == caracters.chars().nth(index).unwrap() {
            index += 1;
        } else {
            result.push(car);
        }
    }

    match index == 3 {
        true => Result::Ok(result),
        false => Result::Err(0),
    }
}
