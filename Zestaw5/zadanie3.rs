// Napisz funkcję szyfruj(), która dla danego napisu
// zwróci ten sam napis zaszyfrowany prostym szyfrem odwracającym — klucz określa długość odwracanych fragmentów.

fn main() {
    println!(
        "1: {}, 2: {}, 3: {}, 4: {}, 5: {}, 6: {}, 7: {}, 8: {}, 9: {}",
        szyfruj("Aladyn", 2) == "lAdany",
        szyfruj("Aladyn", 3) == "alAnyd",
        szyfruj("Aladyn", 4) == "dalAny",
        szyfruj("Aladyn", 5) == "ydalAn",
        szyfruj("koza", 3) == "zoka",
        szyfruj("kaszanka", 3) == "saknazak",
        szyfruj("kot Mruczek", 9) == "zcurM tokke",
        szyfruj("kot Mruczek", 1) == "kot Mruczek",
        szyfruj("kot Mruczek", 2) == "ok trMcuezk"
    );
}

fn szyfruj(napis: &str, klucz: i64) -> String {
    let mut new_napis = String::new();
    let mut temp_napis = String::new();
    let chars = napis.chars();
    let mut i = 0;
    for l in chars {
        temp_napis.push(l);
        i += 1;
        if i % klucz == 0 {
            for letter in temp_napis.chars().rev() {
                new_napis.push(letter);
            }
            temp_napis.clear();
            i = 0;
        }
    }
    if i != 0 {
        for letter in temp_napis.chars().rev() {
            new_napis.push(letter);
        }
        temp_napis.clear();
    }
    new_napis
}
