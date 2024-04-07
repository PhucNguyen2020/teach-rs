fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    // TODO
    let mut largest = input[0];
    let mut smallest = input[0];
    for i in input {
        if i > largest {
            largest = i;
        }
        if i < smallest {
            smallest = i;
        }
    }
    println!("{} is largest and {} is smallest",largest,smallest);
}
