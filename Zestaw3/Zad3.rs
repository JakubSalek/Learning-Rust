// Napisz funkcję trójargumentową, która poprzestawia wartości swoich argumentów 
// (dla ustalenia uwagi: typu i32) tak, by były uporządkowane niemalejąco

fn main(){
    let mut x1 = 0;
    let mut x2 = 5;
    let mut x3 = 2;
    println!("x1: {} x2: {} x3: {}", x1, x2, x3);
    sort(&mut x1, &mut x2, &mut x3);
    println!("x1: {} x2: {} x3: {}", x1, x2, x3);

}

fn sort(x1: &mut i32, x2: &mut i32, x3: &mut i32){
    if *x1 > *x2 && *x2 < *x3 {
        swap(x1, x2);
        if *x3 < *x2{
            swap(x2, x3);
        }
    } else if *x1 > *x3 && *x3 < *x2 {
        swap(x1, x3);

    }
    if *x3 < *x2{
        swap(x2, x3);
    }
}

fn swap(x1: &mut i32, x2: &mut i32){
    let temp = *x1;
    *x1 = *x2;
    *x2 = temp;
}
