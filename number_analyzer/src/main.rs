fn is_even(n: i32) -> bool{
    if n % 2 == 0{
        return true
    }else {
        return false
    }
        
}
fn main() {
    // creating an array
    let array = [7, 10, 15, 3, 6, 23, 30, 50, 18, 2];
    for &idx in array.iter(){

        if idx % 3 == 0 && idx % 5 == 0{
            println!("FizzBuzz");
        }else if idx % 3 == 0{
            println!("Fizz");
        }else if idx % 5 == 0{
            println!("Buzz");
        }else{
            if is_even(idx){
                println!("{} is even", idx);
            }else {
                println!("{} is odd", idx);
            }
        }
    }

    // while loop that gets sum of all numbers of the array
    let mut sum = 0;
    let mut idx = 0;
    while idx < array.len(){
        sum += array[idx];
        idx += 1;
    }
    println!("{} is the sum of all numbers in the array.", sum);

    // a for loop that finds the largest number in the array
    let mut largest = array[0];
    for &ndx in array.iter(){
        if ndx > largest{
            largest = ndx;
        }
    }
    println!("{} is the largest number in the array.", largest);
}
