// Napisz funkcje co_drugi_znak(), która zwróci napis zawierający co drugi znak z danego napisu.

fn main() {
    println!("Nowy napis = {}", co_drugi_znak("Ala ma kota"));
}

fn co_drugi_znak(napis: &str) -> String {
    let mut new_napis = String::new();
    for (i, letter) in napis.chars().enumerate() {
        if i % 2 == 0 {
            new_napis.push(letter);
        }
    }
    new_napis
}
