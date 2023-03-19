// Zadanie 4
// Napisz program służący do konwersji wartości temperatury podanej w stopniach Fagrenheita na stopnie w skali Celsjusza

fn main() {
    let farenheit = 50.0;
    let celsius = ((farenheit - 32.0) * 5.0) / 9.0;
    println!(
        "Temperatura wynosi {} stopni Celsjusza i {} stopni Farenheita",
        celsius, farenheit
    );
}
