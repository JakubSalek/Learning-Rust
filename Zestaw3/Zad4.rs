// Wyświetl tabelę widzialnych znaków ASCII wraz kodami (od 33 do 126).

fn main() {
    let mut i: u8 = 33;
    while i <= 126 {
        println!("Znak: {} Numer: {}", i as char, i);
        i += 1;
    }
}
