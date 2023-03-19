// Zadanie 9
// Napisz program, który znajduje wszystkie trójki pitagorejskie o wartościach nie większych niż dana.
// Zakładamy, że 0 < a < b < c.

fn main() {
    let mut a: i128 = 1;
    let mut b: i128 = 1;
    let mut c: i128 = 1;
    let dana = 100;
    while a <= dana {
        while b <= dana {
            while c*c <= dana {
                if a*a + b*b == c*c {
                    println!("A = {} B = {} C = {}", a, b ,c);
                }
                c += 1;
            }
            b += 1;
            c = 1;
        }
        a += 1;
        b = 1;
    }
}
