use std::io;

fn main() {
    'measure: loop {
        println!("Enter your current temperature measure. [C]elcius or [F]ahrenheint");

        let mut measure_type = String::new();

        io::stdin()
            .read_line(&mut measure_type)
            .expect("Failed to read measure type");

        let measure_type = measure_type.trim().to_uppercase();

        if measure_type != "C" && measure_type != "F" {
            println!("Invalid option");
            continue;
        }

        loop {
            let mut temperature = String::new();

            println!("Enter the temperature");

            io::stdin()
                .read_line(&mut temperature)
                .expect("Failed to read temperature");

            let temperature: f32 = match temperature.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid temperature");
                    continue;
                }
            };

            let converted = match measure_type.as_str() {
                "C" => temperature * 1.8 + 32.0,
                "F" => (temperature - 32.0) / 1.8,
                &_ => continue,
            };

            let label = if measure_type == "C" {
                "Fahrenheit"
            } else {
                "Celcius"
            };

            println!("{temperature} degrees converted to {label} is: {converted}");

            break 'measure;
        }
    }
}
