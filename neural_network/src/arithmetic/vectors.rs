/* vector functions */
pub fn add_vectors(x: &[f64], y: &[f64]) -> Vec<f64> {

    let mut v: Vec<f64> = Vec::new();
    if x.len() == y.len() {

        for i in 0..x.len() {
            v.push(x[i] + y[i])
        }

    } else {
        panic!("Error in add_vectors, vectors must be same size but x.len is {} and y.len is {}", x.len(), y.len());
    }
    v
}