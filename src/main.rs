use std::io;

fn main() {
    println!("This program will convert temperatures from Fahrenheit to Celcius");

    loop {
        println!("Please input the temperature you would like to convert");

        let mut inputed_temp = String::new();
        io::stdin()
            .read_line(&mut inputed_temp)
            .expect("Failed to read line");

        let temp_to_number: i32 = inputed_temp.trim().parse().unwrap();
        let converted_temp = { (temp_to_number - 32) * 5 / 9 };
        println!("The converted temperature is {}C", converted_temp);
    }
}
