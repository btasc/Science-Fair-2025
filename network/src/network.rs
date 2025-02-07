mod components;

use components::*;
use std::collections::HashMap;

struct NeuralNetwork {
    neurons: Vec<i32>,
    connectors: Vec<i32>,
    connector_map: HashMap<(i32, i32), i32>,
}