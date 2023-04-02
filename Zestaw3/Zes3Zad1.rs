// Stwórz generator liczb pseudolosowych, którego ziarno przechowywane będzie na zewnątrz
// i podawane w pierwszym parametrze, a w parametrze drugim i trzecim podawany będzie przedział losowanych liczb.

fn main(){
    let mut seed = 3231;
    while true {
        println!("Wylosowana liczba to: {}", rand(&mut seed, 0, 100));
    }
}

fn rand(seed: &mut i32, min_rand: i32, max_rand: i32) -> i32 {
    let a = 75;
    let c = 74;
    let m = 65537;
    *seed = (*seed*a + c) % m;
    *seed%(max_rand-min_rand+1)+min_rand
}
