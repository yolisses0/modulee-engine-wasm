use wasm_bindgen::prelude::wasm_bindgen;

// This value is based on the audio context block size. It should probably be
// replaced later.
const BUFFER_SIZE: usize = 128;

#[wasm_bindgen]
pub struct Graph {
    buffer: [f32; BUFFER_SIZE],
    graph: modulee_engine::graph::Graph,
}

#[wasm_bindgen]
impl Graph {
    #[wasm_bindgen]
    pub fn set_nodes_from_json(&mut self, nodes_data_json: String) {
        if let Err(e) = self.graph.set_nodes_from_json(&nodes_data_json) {
            log::error!("Failed to set nodes from JSON: {}\n{}", e, nodes_data_json);
        }
    }

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            buffer: [0.; BUFFER_SIZE],
            graph: modulee_engine::graph::Graph::new(),
        }
    }

    #[wasm_bindgen]
    pub fn get_buffer_pointer(&self) -> *const f32 {
        self.buffer.as_ptr()
    }

    #[wasm_bindgen]
    pub fn process_block(&mut self) {
        self.graph.process_block(&mut self.buffer, BUFFER_SIZE);
    }
}
