// Napisz funkcję wizytowka, która otrzymuje w dwóch parametrach napisowych imię i nazwisko,
// a zwraca napis powstały z pierwszej litery imienia, kropki, spacji i nazwiska,
// przy czym w wyniku pierwsza litera imienia i nazwiska mają być duże, pozostałe małe.
// Na przykład, dla danych "jan" oraz "KOWALSKI" funkcja ma zwracać napis "J. Kowalski".

fn main(){
    println!("Wynik: {}", wizytowka("Jakub", "Salek"));
}

fn wizytowka(imie: &str, nazwisko: &str) -> String {
    let mut wynik = String::new();
    wynik.push(imie.chars().next().unwrap().to_ascii_uppercase());
    wynik.push_str(". ");
    let mut chars = nazwisko.chars();
    wynik.push(chars.next().unwrap().to_ascii_uppercase());
    for l in chars {
        wynik.push(l.to_ascii_lowercase());
    }
    wynik
}
