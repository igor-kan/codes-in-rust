//! Gradient Descent Momentum

pub fn momentum_update(v: f64, beta: f64, lr: f64, grad: f64) -> f64 { beta * v + lr * grad }
