fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; //necessary to add the semi colon here
    let mut sum = 0; //mut is required to edit the value of sum (in the loop or outside? idk?)

    for &number in &numbers {
        sum += number;
    }

    println!("The sum of the numbers is: {}", sum);
}
