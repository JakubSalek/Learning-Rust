// Zadanie 1
// Napisz program, który wyświetla informację o przestępności danego roku.

fn main() {
    let rok = 400;
    if ((rok % 4 == 0) && (rok % 100 != 0)) || rok % 400 == 0 {
        println!("Rok {} jest przestępny", rok);
    } else {
        println!("Rok {} nie jest przestępny", rok);
    }
}
