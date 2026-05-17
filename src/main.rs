use std::io;

fn main() {
    loop {
        let mut number_input = String::new();

        println!("Please enter a number n for generate the nth Fibonacci number:");
        io::stdin()
            .read_line(&mut number_input)
            .expect("Failed to read line");

        let number: usize = match number_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        if number == 0 {
            println!("Please enter a number greater than 0.");
            continue;
        }

        let mut a = vec![0; number];

        if number >= 1 {
            a[0] = 1;
            println!("The 1st Fibonacci number is {}", a[0]);
        }
        if number >= 2 {
            a[1] = 1;
            println!("The 2st Fibonacci number is {}", a[1]);
        }

        for index in 2..number {
            a[index] = a[index - 1] + a[index - 2];

            println!("The {}th Fibonacci number is {}", index + 1, a[index]);
        }
        break;
    }
}
