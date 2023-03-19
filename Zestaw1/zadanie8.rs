// Zadanie 8

fn main() {
    let number = 11;
    let mut tens = 10;
    let mut prev_tens = 1;
    let mut sum = 0;
    while tens < number {
        sum += (number%tens)/prev_tens;
        prev_tens = tens;
        tens *= 10;
    }
    sum += (number%tens)/prev_tens;
    println!("Suma wynosi {}", sum);
}
