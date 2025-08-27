use wasm_bindgen::prelude::wasm_bindgen;

// This value is based on the audio context block size. It should probably be
// replaced later.
const BUFFER_SIZE: usize = 128;

#[wasm_bindgen]
pub struct Graph {
    buffer_0: [f32; BUFFER_SIZE],
    buffer_1: [f32; BUFFER_SIZE],
    graph: modulee_engine::Graph,
}

#[wasm_bindgen]
impl Graph {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            buffer_0: [0.; BUFFER_SIZE],
            buffer_1: [0.; BUFFER_SIZE],
            graph: modulee_engine::Graph::default(),
        }
    }

    #[wasm_bindgen]
    pub fn get_buffer_0_pointer(&self) -> *const f32 {
        self.buffer_0.as_ptr()
    }

    #[wasm_bindgen]
    pub fn get_buffer_1_pointer(&self) -> *const f32 {
        self.buffer_1.as_ptr()
    }

    #[wasm_bindgen]
    pub fn process_block(&mut self) {
        for i in 0..BUFFER_SIZE {
            self.graph.process();
            let output_values = self.graph.get_output_values();
            self.buffer_0[i] = output_values.0;
            self.buffer_1[i] = output_values.1;
        }
    }

    #[wasm_bindgen]
    pub fn set_graph(&mut self, graph_data_json: String) {
        if let Err(e) = self.graph.update_from_json(&graph_data_json) {
            log::error!("Failed to set graph from JSON: {}\n{}", e, graph_data_json);
        }
        // debug!("{:#?}", self.graph);
    }

    #[wasm_bindgen]
    pub fn set_note_on(&mut self, pitch: f32) {
        self.graph.set_note_on(pitch);
    }

    #[wasm_bindgen]
    pub fn set_note_off(&mut self, pitch: f32) {
        self.graph.set_note_off(pitch);
    }

    #[wasm_bindgen]
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.graph.set_sample_rate(sample_rate);
    }

    #[wasm_bindgen]
    pub fn update_control(&mut self, id: usize, value: f32) {
        let control_update_data = modulee_engine::ControlUpdateData { id, value };
        self.graph.update_control(&control_update_data);
    }
}
