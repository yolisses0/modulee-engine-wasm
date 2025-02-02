use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Graph {
    graph: modulee_engine::graph::Graph,
}

#[wasm_bindgen]
impl Graph {
    #[wasm_bindgen]
    pub fn get_debug_value(&self) -> f32 {
        self.graph.get_debug_value()
    }

    #[wasm_bindgen]
    pub fn set_debug_string(&mut self, debug_string: String) {
        self.graph.set_debug_string(&debug_string);
        let string_value = self.graph.get_debug_string();
        log::debug!("string value: {}", string_value);
    }

    #[wasm_bindgen]
    pub fn set_nodes(&mut self, nodes_data: String) {
        if let Err(e) = self.graph.set_nodes_from_json(&nodes_data) {
            log::error!("Failed to set nodes from JSON: {}\n{}", e, nodes_data);
        }
    }

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            graph: modulee_engine::graph::Graph::new(),
        }
    }

    #[wasm_bindgen]
    pub fn process_block(&mut self, buffer: &mut [f32], length: usize) {
        self.graph.process_block(buffer, length);
    }
}
