// Napisz funkcje liczba_wystapien()

fn main() {
    println!("Liczba wystapien = {}", liczba_wystapien("ala ma kota", 'a'));
}

fn liczba_wystapien(napis: &str, znak: char) -> i64 {
    let mut i = 0;
    for letter in napis.chars(){
        if letter == znak {
            i+=1;
        }
    }
    i
}
