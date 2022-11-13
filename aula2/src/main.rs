const SECONDS_IN_MINUTE: u32 = 60;
const MINUTES_IN_HOUR: u32 = 60;
const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE + MINUTES_IN_HOUR;

fn main() { // escope 1

    // let total = 30;
    let mut total = 30; // set variable to mutable
    println!("Trabalhou {total} horas.");

    total = 44;
    // total = "quarenta e  quatro" // can't change variable type
    println!("Trabalhou {total} horas.");

    let total = "quarenta e quatro";
    println!("Trabalhou {total} horas.");

    let total = 50;
    { // escope 2

        let total = 40;
        println!("Trabalhou {total} horas.");

    } // fim 2
    println!("Trabalhou {total} horas.");

    let total_in_seconds = total * SECONDS_IN_HOUR;
    println!("Trablhou {total_in_seconds}s.");

} // fim 1
