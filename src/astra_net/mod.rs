pub mod layer;
pub mod activation;
use crate::astra_net::layer::Layer;

use ndarray::{Array1, Array2, Zip};



pub struct Net{
    layers: Vec<Box<dyn Layer>>,
    learning_rate: f64,
}

impl Net{
    pub fn new() -> Self {
        Self { 
            layers: Vec::new(),
            learning_rate: 0.001
             }
    }

    pub fn set_learning_rate(&mut self, learning_rate: f64){
        self.learning_rate = learning_rate;
    }

    pub fn add_layer(&mut self, layer: Box<dyn Layer>){
        self.layers.push(layer);
    }

    pub fn feed_forward(&mut self, input: &Array1<f64>) -> Array1<f64> {
        let mut output = input.to_owned();
        for l in self.layers.iter_mut(){
            output = l.feed_forward(&output);
        }
        output
    }

    pub fn back_propagation(&mut self, input: &Array1<f64>, target: &Array1<f64>) {

        let output: Array1<f64> = self.feed_forward(input);

        if output.iter().any(|v| v.is_nan()){
            panic!("Panic in net bp after feed forward");
        }

        let mut error: Array1<f64> = Array1::from(output.to_owned());

        Zip::from(&mut error)
            .and(target)
            .for_each(|x, &y| *x -= y);

        // let error_sum = error.sum();
        // println!("{:#?}", error_sum);

        // if error_sum == 0.0 ||  error_sum.is_nan(){
        //     println!("OUTPUT {:#?}", output);
        //     println!("TARGET {:#?}", target);
        // }
        
        

        for l in self.layers.iter_mut().rev() {
            error = l.back_propagation(error, 0.01);
            println!("ERROR FORM NET BP LOOP {:#?}", error);
        }

    }
}
