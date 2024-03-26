#![allow(non_snake_case)]

use Binary::Binary;
use ::Binary::QuBinary;
use Matrix::Matrix; 
use num_complex::Complex;
#[derive(Debug)]

pub struct Qubite{
    state: Matrix<Complex<f64>>,
    bites: i32,
    length: usize
}
impl Qubite{
    pub fn new(bites: i32, initial: Vec<i32>) -> Result<Qubite, &'static str>{
        if initial.len() as i32 != bites{
            Err("initial length is not equal to bites")
        }else{
            let b: QuBinary = Binary::<Matrix<Complex<f64>>>::new(initial)?;
            let mut v: Matrix<Complex<f64>> = b.get_state()[0].tensor(&b.get_state()[1]);
            for i in 2..bites{
                v = v.tensor(&b.get_state()[i as usize]);
            }
            Ok(Qubite{state: v, bites: bites, length: power(2, bites)})
        }
    }
    pub fn get_state(&self) -> Matrix<Complex<f64>>{
        self.state.clone()
    }
    pub fn get_bites(&self) -> i32{
        self.bites
    }
    pub fn get_length(&self) -> usize{
        self.length
    }
    
}
fn power(a: i32, b: i32) -> usize{
    a.pow(b as u32) as usize
}
