// Zadanie 7
// Napisz program, który wyświetla cyfry danej liczby całkowitej (od końca).

fn main() {
    let number = 12345678;
    let mut tens = 10;
    let mut prev_tens = 1;
    while tens < number {
        print!("{}", (number%tens)/prev_tens);
        prev_tens = tens;
        tens *= 10;
    }
    println!("{}", (number%tens)/prev_tens);
}
