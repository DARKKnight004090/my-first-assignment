fn is_even(n:i32) -> bool {
    n % 2 == 0
}

fn main() {

    let my_array: [i32;10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    //For Loop w/ FizzBuzz
    for idx in 0..my_array.len() {
    
        if(is_even(my_array[idx]) == true)
        {
            println!("{} is even", my_array[idx]);
        }
        else
        {
            println!("{} is odd", my_array[idx]);
        }
        if(my_array[idx] % 3 == 0)
        {
            println!("Fizz");
        }
        if(my_array[idx] % 5 == 0)
        {
            println!("Buzz");
        }
        if(my_array[idx] % 3 == 0 && my_array[idx] % 5 == 0)
        {
            println!("FizzBuzz");
        }
    }
    
    //While Loop for Sum
    let mut counter = my_array.len();
    let mut sum = 0;
    while counter > 0 {
        counter -= 1;
        sum += my_array[counter];
    }
    
    println!("The sum of all the elements is: {}", sum);
    
    //Finding the Largest Value
    let mut largest = my_array[0];
    
    for idx in 1..my_array.len() {
        if(my_array[idx] > largest) {
            largest = my_array[idx];
        }
    }
    
    println!("Largest value: {}", largest);
}
