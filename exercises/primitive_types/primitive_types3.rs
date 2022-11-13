// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.

fn main() {
    let a = [40, 7, 4, 60, 1, 60, 65, 41, 80, 41, 33, 57, 77, 47, 2, 76, 39, 79, 69, 4, 66, 85, 86, 72, 47, 98, 69, 54, 89, 73, 10, 17, 20, 95, 40, 61, 24, 30, 27, 57, 93, 77, 21, 89, 98, 7, 56, 96, 20, 73, 81, 8, 43, 57, 35, 47, 28, 17, 96, 31, 18, 49, 65, 70, 22, 6, 59, 75, 23, 61, 33, 87, 93, 25, 29, 60, 94, 11, 94, 24, 66, 35, 13, 5, 38, 24, 48, 87, 56, 49, 30, 21, 65, 8, 64, 100, 33, 80, 69, 71];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
