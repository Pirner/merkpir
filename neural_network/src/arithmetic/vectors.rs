/* vector functions */
pub fn add_vectors(x: &[f64], y: &[f64]) -> Vec<f64> {

    /* return vector*/
    let mut v: Vec<f64> = Vec::new();

    if x.len() == y.len() {
        for it in x.iter().zip(y.iter()) {
            let (xi, yi) = it;
            v.push(xi + yi);
        }

    /* only vectors of the same length can be added */
    } else {
        panic!("Error in add_vectors, vectors must be same size but x.len is {} and y.len is {}", x.len(), y.len());
    }
    /* return the added vector */
    v
}


pub fn scale_vector(x: &[f64], s: f64) -> Vec<f64> {

    let mut v: Vec<f64> = Vec::new();

    for i in 0..x.len() {
        v.push(x[i] * s);
    }
    /*return scaled vector*/
    v
}