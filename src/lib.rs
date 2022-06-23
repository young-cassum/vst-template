#[macro_use]
extern crate vst;

use std::sync::Arc;
use vst::buffer::AudioBuffer;

struct Plugin {
    params: Arc<PluginParameters>,
}

struct PluginParameters {}

impl Default for Plugin {
    fn default() -> Plugin {
        Plugin {
            params: Arc::new(PluginParameters {}),
        }
    }
}

impl vst::prelude::PluginParameters for PluginParameters {
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            _ => 0.0,
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            _ => "",
        }
        .to_string()
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            _ => "".to_string(),
        }
    }

    fn set_parameter(&self, index: i32, _value: f32) {
        match index {
            _ => {}
        }
    }
}

impl vst::prelude::Plugin for Plugin {
    fn new(_host: vst::prelude::HostCallback) -> Self {
        Default::default()
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        for (input_buffer, output_buffer) in buffer.zip() {
            for (_input_sample, _output_sample) in input_buffer.iter().zip(output_buffer) {}
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn vst::prelude::PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn vst::prelude::PluginParameters>
    }

    fn get_info(&self) -> vst::prelude::Info {
        vst::prelude::Info {
            name: "VST".to_string(),
            unique_id: 666420, // used by hosts to differentiate between plugins
            category: vst::prelude::Category::Effect,

            inputs: 2,
            outputs: 2,
            parameters: 1,

            ..Default::default()
        }
    }
}

plugin_main!(Plugin);
