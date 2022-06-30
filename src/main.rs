use std::io;

fn main() {

    loop{
        //Input
        println!("Input a temperature: ");
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("Failed to read line.");
        //Make sure it's a number
        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        //Convert the input
        println!("Your temperature in Farenheit: {:.1}", c_to_f(temp));
        println!("Your temperature in Celcius: {:.1}", f_to_c(temp));

    }
}
//Convert Celcius to Farenheit
fn c_to_f(temp: f32) -> f32{
    let temp = temp * 1.8 + 32.0;
    return temp;
}
//Convert Farenheit to Celcius
fn f_to_c(temp: f32) -> f32{
    let temp = (temp-32.0) / 1.8;
    return temp;
}