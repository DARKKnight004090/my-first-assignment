const FREEZ_PT:i32 = 32;

fn fahrenheit_to_celsius(f: f64) -> f64 {

    return (f - 32.0) * (0.556);
    
}

fn celsius_to_fahrenheit(c: f64) -> f64 {

    return ((1.8) * c) + 32.0;
    
}

fn main() {
    
    let mut start_f = 90.0;
    let mut start_c =  fahrenheit_to_celsius(start_f);
    
    println!("1st °F value = {}°C", start_c);
    
    for offset in 1..=5 {
        let mut next_f = start_f + offset as f64;
        let mut next_c = fahrenheit_to_celsius(next_f);
        println!("{}°F value = {}°C", next_f, next_c);
    }
    
    
}