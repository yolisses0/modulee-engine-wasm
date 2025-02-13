use log::debug;
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

    #[wasm_bindgen]
    pub fn set_groups_from_json(&mut self, groups_data_json: String) {
        if let Err(e) = self.graph.set_groups_from_json(&groups_data_json) {
            log::error!(
                "Failed to set groups from JSON: {}\n{}",
                e,
                groups_data_json
            );
        }

        debug!("{:#?}", self.graph);
    }

    #[wasm_bindgen]
    pub fn set_note_on(&mut self, pitch: f32) {
        self.graph.set_note_on(pitch);

        debug!("{:#?}", self.graph);
    }

    #[wasm_bindgen]
    pub fn set_note_off(&mut self, pitch: f32) {
        self.graph.set_note_off(pitch);
    }

    #[wasm_bindgen]
    pub fn set_main_group_id(&mut self, main_group_id: usize) {
        self.graph.set_main_group_id(main_group_id)
    }
}
