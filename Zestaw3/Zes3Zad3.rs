// Napisz funkcję rand_perm() permutującą w miejscu w sposób losowy wartości tablicy przekazanej w argumencie.

fn main(){
    let mut seed = 1231;
    let mut my_arr = [1, 2, 3, 4, 5, 6, 7, 8];
    rand_perm(&mut my_arr, &mut seed);
    for elem in my_arr {
        println!("{}", elem);
    }
}

fn rand_perm(arr: &mut [i32], seed: &mut i32){
    let lenght = arr.len();
    for i in 0..lenght {
        swap_arr(arr, i, rand(seed, 0, (lenght as i32) - 1) as usize);
    }
}

fn rand(seed: &mut i32, min_rand: i32, max_rand: i32) -> i32 {
    let a = 75;
    let c = 74;
    let m = 65537;
    *seed = (*seed*a + c) % m;
    *seed%(max_rand-min_rand+1)+min_rand
}

fn swap_arr(arr: &mut [i32], i: usize, j: usize){
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}
