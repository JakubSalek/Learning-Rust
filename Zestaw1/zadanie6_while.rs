// Zadanie 6
// Napisz program, który oblicza silnię dla danej liczby.

fn main(){
    let n = 4;
    let mut silnia = 1;
    let mut i = 1;
    while i != n+1 {
        silnia *= i;
        i+=1;
    }
    println!("Silnia wynosi: {}", silnia);
}
