/* imports for the network */
mod arithmetic;


fn main() {
    println!("Test functions for neural network WIP!");

    let xs: [f64; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let ys: [f64; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

    println!("{:?}", arithmetic::vectors::add_vectors(&xs, &ys));

    println!("{:?}", arithmetic::vectors::scale_vector(&xs, -3.0));
}