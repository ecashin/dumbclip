use lv2_core::prelude::*;
use urid::*;

#[uri("https://github.com/ecashin/dumbclip")]
struct DumbClip {
    _sample_rate: f64,
}

#[derive(PortCollection)]
struct SimpleAmpPorts {
    gain: InputPort<Control>,
    input: InputPort<InPlaceAudio>,
    output: OutputPort<InPlaceAudio>,
}

impl Plugin for DumbClip {
    type Ports = SimpleAmpPorts;
    type InitFeatures = ();
    type AudioFeatures = ();
    fn new(plugin_info: &PluginInfo, _features: &mut Self::InitFeatures) -> Option<Self> {
        println!("dumbclip sample rate: {:?}", plugin_info.sample_rate());
        Some(Self {
            _sample_rate: plugin_info.sample_rate(),
        })
    }

    fn run(&mut self, ports: &mut SimpleAmpPorts, _: &mut (), _: u32) {
        let coef = if *(ports.gain) > -90.0 {
            10.0_f32.powf(*(ports.gain) * 0.05)
        } else {
            0.0
        };

        for (input_sample, output_sample) in Iterator::zip(ports.input.iter(), ports.output.iter())
        {
            let sample = input_sample.get() * coef;
            output_sample.set(sample.clamp(-1.0, 1.0));
        }
    }
}

lv2_descriptors!(DumbClip);
