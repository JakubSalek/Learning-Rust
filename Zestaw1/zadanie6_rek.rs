// Zadanie 6
// Napisz program, który oblicza silnię dla danej liczby.

fn main() {
    let silnia = silnia_rek(3, 1, 1);
    println!("Silnia wynosi: {}", silnia);
}

fn silnia_rek(n: i32, i: i32, mut silnia: i32) -> i32 {
    if n + 1 == i {
        return silnia;
    }
    silnia *= i;
    silnia_rek(n, i + 1, silnia)
}
