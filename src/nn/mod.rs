use crate::engine::Value;
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

trait Module {
    fn zero_grad(&self) -> Value;
    fn parameters(&self) -> Vec<Value>;
}

#[derive(Clone, Debug)]
struct Neuron {
    weights: Vec<Value>,
    bias: Value,
    non_lin: bool,
}

impl Neuron {
    fn new(nin: usize, non_lin: bool) -> Neuron {
        let mut rng = thread_rng();
        let generator = Uniform::from(0.01..1.00);

        Neuron {
            weights: vec![Value::new(generator.sample(&mut rng)); nin],
            bias: Value::new(generator.sample(&mut rng)),
            non_lin,
        }
    }
    fn call(&self, x: &Vec<Value>) -> Value {
        let act: Value = self
            .weights
            .iter()
            .zip(x.iter())
            .map(move |(x, y)| x * y)
            .fold(self.bias.clone(), |a, b| a + b);

        if self.non_lin {
            return act.tanh();
        }

        act
    }
}

struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    fn new(nin: usize, nout: usize, non_lin: bool) -> Layer {
        Layer {
            neurons: vec![Neuron::new(nin, non_lin); nout],
        }
    }

    fn call(&self, x: &Vec<Value>) -> Vec<Value> {
        self.neurons.iter().map(move |n| n.call(x)).collect()
    }
}

pub struct MLP {
    layers: Vec<Layer>,
}

impl MLP {
    pub fn new(nin: usize, nout: Vec<usize>) -> MLP {
        let sz = {
            let mut sz = vec![nin];
            sz.extend(&nout);
            sz
        };

        let layers = (0..nout.len())
            .map(|i| Layer::new(sz[i], sz[i + 1], i != nout.len() - 1))
            .collect::<Vec<Layer>>();

        MLP { layers }
    }

    pub fn call(&self, x: &Vec<Value>) -> Vec<Value> {
        let mut out: Vec<Value> = x.clone();
        for layer in self.layers.iter() {
            out = layer.call(&out);
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_neuron() {
        let n = Neuron::new(6, true);

        assert_eq!(6, n.weights.len());
    }
    #[test]
    fn create_output_from_neuron() {
        let mut rng = thread_rng();
        let generator = Uniform::from(0.01..1.00);
        let x: Vec<Value> = vec![Value::new(generator.sample(&mut rng)); 3];

        let n = Neuron::new(3, true);
        let out = n.call(&x);

        assert_eq!(3, n.weights.len());
        assert_eq!(0.0, out.grad);
    }
    #[test]
    fn create_output_from_layer() {
        let mut rng = thread_rng();
        let generator = Uniform::from(0.01..1.00);
        let l = Layer::new(3, 3, true);
        let x: Vec<Value> = vec![Value::new(generator.sample(&mut rng)); 3];
        let out = l.call(&x);

        assert_eq!(3, out.len());
    }
    #[test]
    fn create_output_from_mlp() {
        let mut rng = thread_rng();
        let generator = Uniform::from(-1.00..=1.00);
        let x: Vec<Value> = vec![Value::new(generator.sample(&mut rng)); 3];

        let m = MLP::new(2, vec![3, 3, 1]);
        let out: Vec<Value> = m.call(&x);

        assert_eq!(1, out.len());
        assert_eq!(3, m.layers.len());
    }
}
