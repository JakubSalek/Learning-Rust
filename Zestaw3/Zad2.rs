// Napisz funkcję dwuargumentową, która zamieni wartości podanych w argumentach zmiennych (dla ustalenia uwagi: typu i32).

fn main(){
    let mut x1 = 5;
    let mut x2 = 10;
    println!("x1: {} x2: {}", x1, x2);
    swap(&mut x1, &mut x2);
    println!("x1: {} x2: {}", x1, x2);
}

fn swap(x1: &mut i32, x2: &mut i32){
    let temp = *x1;
    *x1 = *x2;
    *x2 = temp;
}
