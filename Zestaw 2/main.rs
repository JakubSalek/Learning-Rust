// Dane: funkcja, <a, b>, EPSILON, N, pochodna

// Wyniki: miejsce zerowe funkcji

fn main() {
    println!("Wartość funkcji: {}", newt(5.0, 0.001, 10000));
    println!(
        "Wartość funkcji rekurencyjnej: {}",
        newt_recursive(6.0, 0.001, 0, 10000)
    );
}

fn newt_recursive(a: f64, eps: f64, i: u128, n: u128) -> f64 {
    let new_a = a - fun(a) / poch(a);
    if i >= n || fun(new_a) <= eps || a - new_a <= eps {
        a
    } else {
        newt_recursive(new_a, eps, i + 1, n)
    }
}

fn newt(a: f64, eps: f64, n: u128) -> f64 {
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

fn fun(x: f64) -> f64 {
    (x * x) - 4.0
}

fn poch(x: f64) -> f64 {
    2.0 * x
}
