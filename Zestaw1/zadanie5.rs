// Zadanie 5
// Napisz program, który dla danych dwóch poprawnych pór jednej doby
// (w postaci całkowitych godzin, minut i sekund) wyświetla różnicę czasów
// (także w postaci analogicznej trójki, z minutami i sekundami w przedziale [0;59]).

fn main() {
    let h1 = 12;
    let m1 = 55;
    let s1 = 23;
    let h2 = 14;
    let m2 = 11;
    let s2 = 55;
    let one_in_sec = h1 * 3600 + m1 * 60 + s1;
    let two_in_sec = h2 * 3600 + m2 * 60 + s2;
    let mut difference;
    if one_in_sec > two_in_sec {
        difference = one_in_sec - two_in_sec;
    } else {
        difference = two_in_sec - one_in_sec;
    }
    let hours_final = difference / 3600;
    difference -= hours_final * 3600;
    let minutes_final = difference / 60;
    difference -= minutes_final * 60;
    println!(
        "Różnica czasów wynosi: {} godzin, {} minut, {} sekund",
        hours_final, minutes_final, difference
    );
}
