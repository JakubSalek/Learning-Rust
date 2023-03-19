// Zadanie 2
// Napisz program, który wyświetla liczbę dni miesiąca na podstawie jego numeru i numeru roku.

fn main() {
    let miesiac = 2;
    let rok = 100;
    let mut przestepny = false;
    if ((rok % 4 == 0) && (rok % 100 != 0)) || rok % 400 == 0 {
        przestepny = true;
    }
    if (miesiac == 1)
        || (miesiac == 3)
        || (miesiac == 5)
        || (miesiac == 7)
        || (miesiac == 8)
        || (miesiac == 10)
        || (miesiac == 12)
    {
        println!("31");
    } else if miesiac == 2 {
        if przestepny {
            println!("29");
        } else {
            println!("28");
        }
    } else {
        println!("30");
    }
}
