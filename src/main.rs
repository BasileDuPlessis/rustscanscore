use ndarray::prelude::*;

fn main() {
    println!("Hello, world!");
}

fn predict(x: &Array2<u64>, y: &Array2<u64>) -> Array2<u64> {  
    x.dot(y)
}

#[test]
fn test_predict() {
    let x = array![
        [1, 2],
        [3, 4]
    ];
    let y = array![
        [1, 2],
        [3, 4]
    ];
    let x_multiply_y = array![
        [7, 10],
        [15, 22]
    ];

    assert_eq!(predict(&x,&y), x_multiply_y);
}