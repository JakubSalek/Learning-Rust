// Zadania z zestawów 1b/2a zmodyfikuj tak, by obliczenia były prowadzone nie dla sztywno zakodowanych funkcji
// i ich pochodnych, lecz by funkcję i pochodną można było przekazać do funkcji obliczającej miejsce zerowe 
// (wskazówka: funkcja tablicuj na wykładzie).


// Dane: funkcja, <a, b>, EPSILON, N, pochodna

// Wyniki: miejsce zerowe funkcji

fn main() {
    println!("Wartość funkcji: {}", newt(fun1, poch1, 5.0, 0.001, 10000));
    println!(
        "Wartość funckji 2: {}",
        newt(fun2, poch2, 5.0, 0.001, 10000)
    );
    println!("Wartość funkcji 4: {}", newt(fun3, poch3, 5.0, 0.001, 10000))
}

fn newt(fun: fn(f64) -> f64, poch: fn(f64) -> f64, a: f64, eps: f64, n: u128) -> f64 {
    // WHILE
    // let mut x1 = a;
    // let mut x2 = a+1.0;
    // let mut i = 0;
    // while i < n && fun(x1) > eps && x2 - x1 > eps{
    //     x2 = x1;
    //     x1 = x2 - (fun(x2)/poch(x2));
    //     i += 1;
    // }
    // x1

    // LOOP
    // let mut x1 = a;
    // let mut x2;
    // let mut i = 0;
    // loop {
    //     x2 = x1 - fun(x1) / poch(x1);
    //     if i >= n || fun(x2) <= eps || x1 - x2 <= eps {
    //         break x2;
    //     }
    //     x1 = x2;
    //     i += 1;
    // }

    // FOR
    let mut x1 = a;
    let mut x2 = a + 1.0;
    for _i in 0..n {
        x2 = x1 - fun(x1) / poch(x1);
        if fun(x2) <= eps || x1 - x2 <= eps {
            break;
        }
        x1 = x2;
    }
    x2
}

fn fun1(x: f64) -> f64 {
    (x * x) - 4.0
}

fn poch1(x: f64) -> f64 {
    2.0 * x
}

fn fun2(x: f64) -> f64 {
    x * x - 4.0 * x + 3.0
}

fn poch2(x: f64) -> f64 {
    2.0 * x - 4.0
}

fn fun3(x: f64) -> f64 {
    0.5 * x * x - 2.0 * x
}

fn poch3(x: f64) -> f64 {
    x - 2.0
}
