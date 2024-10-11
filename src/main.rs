use core::prelude::v1;
use std::clone;

#[derive(Debug)]
struct  Vector {
    v: Vec<f32> 
    
}
impl Vector {
    fn new()-> Self {
        Vector {v : Vec::new()}
    }
    fn from(vec : Vec<f32>) -> Self{
        Vector {v: vec}
    }
    fn len(&self) -> usize {
        self.v.len()
    }
    fn clone(&self) -> Self {
        Vector {v: self.v.clone()}
    }
    fn add(mut self, w: Vector) -> Self{
        assert_eq!(self.len(), w.len(), "Vectors must have the same length");
        for i in 0..self.len(){
            self.v[i] += w.v[i];
        }
       self
    }
    fn sub(mut self, w: Vector) -> Self{
        assert_eq!(self.len(), w.len(), "Vectors must have the same length");
        for i in 0..self.len(){
            self.v[i] -= w.v[i];
        }
       self
    }

    fn vector_sum(vectors: Vec<Vector>) -> Self {
        assert!(!vectors.is_empty(), "No vectors provided for summation");
        let vector_len = vectors[0].len();
        for v in &vectors {
            assert_eq!(v.len(), vector_len, "All vectors must have the same length");
        }
        let mut result_vector = vec![0.0; vector_len];
        for v in &vectors {
            for i in 0..vector_len {
                result_vector[i] += v.v[i]
            }
        }
        Vector {v: result_vector}

    }

    fn scalar_multiply(mut self, c:f32) -> Self {
        for i in 0..self.len() {
            self.v[i] *= c;
        }
        self
    }
    
    fn vector_mean(vectors: Vec<Vector>) -> Self {
        assert!(!vectors.is_empty(), "No vectors provided for summation");
        let vector_len = vectors[0].len();
        for v in &vectors {
            assert_eq!(v.len(), vector_len, "All vectors must have the same length");
        }
        let n = vectors.len() as f32;
        Vector::vector_sum(vectors).scalar_multiply(1.0/n)

    }
    
    fn dot(&self, w:&Vector) -> f32 {
        assert_eq!(self.len(), w.len(), "Vectors must have the same length");
        let mut dot = 0.0;
        for i in 0..self.len() {
            dot += self.v[i] * w.v[i];
        }
        dot

    }

    fn sum_of_squares(&self) -> f32 {
        self.dot(self)
    }
}

fn main() {
    let v1  = Vector::from(vec![1.0, 2.0, 3.0]);
    let v2 = Vector::from(vec![4.0, 5.0, 6.0]);
    let v3 = Vector::from(vec![5.0, 6.0]);
    println!("{:?}", v1.sum_of_squares()); 

}

