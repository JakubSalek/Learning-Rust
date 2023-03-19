// Zadanie 3
// Napisz program służący do konwersji wartości temperatury podanej w stopniach Celsjusza na stopnie w skali Fahrenheita

fn main() {
    let celsius = 10.0;
    let farenheit = 32.0 + ((9.0 / 5.0) * celsius);
    println!(
        "Temperatura wynosi {} stopni Celsjusza i {} stopni Farenheita",
        celsius, farenheit
    );
}
