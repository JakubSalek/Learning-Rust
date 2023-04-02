// Napisz funkcję swap_arr()

fn main(){
    let mut my_arr = [1, 2, 3, 4, 5, 6, 7];
    for el in my_arr{
        println!("{}", el);
    }
    println!("===================");
    swap_arr(&mut my_arr, 0, 3);
    for el in my_arr{
        println!("{}", el);
    }
}

fn swap_arr(arr: &mut [i32], i: usize, j: usize){
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}
