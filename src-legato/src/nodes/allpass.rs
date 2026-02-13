use legato::{
    context::AudioContext,
    msg::{NodeMessage, RtValue},
    node::{Inputs, Node},
    ports::{PortBuilder, Ports},
    ring::RingBuffer,
};

pub struct Allpass {
    feedback: f32,
    delay_length: f32,
    delay_lines: Vec<RingBuffer>,
    capacity: usize,
    ports: Ports,
}

impl Allpass {
    pub fn new(chans: usize, feedback: f32, delay_length: f32, capacity: usize) -> Self {
        let delay_lines = vec![RingBuffer::new(capacity); chans];
        Self {
            feedback,
            delay_length,
            delay_lines,
            capacity,
            ports: PortBuilder::default()
                .audio_in(chans)
                .audio_out(chans)
                .build(),
        }
    }
}

impl Node for Allpass {
    fn process(&mut self, ctx: &mut AudioContext, inputs: &Inputs, outputs: &mut [&mut [f32]]) {
        for (c, (chan_in, chan_out)) in inputs.iter().zip(outputs.iter_mut()).enumerate() {
            // if let Some(chan_in_inner) = chan_in {
            //     for (sample_in, sample_out) in chan_in_inner.iter().zip(chan_out.iter_mut()) {
            //         let
            //     }
            // }
        }
    }
    fn ports(&self) -> &Ports {
        &self.ports
    }
    fn handle_msg(&mut self, msg: NodeMessage) {
        match msg {
            NodeMessage::SetParam(inner) => match (inner.param_name, inner.value) {
                ("feedback", RtValue::F32(val)) => self.feedback = val.clamp(0.0, 0.95),
                ("feedback", RtValue::U32(val)) => self.feedback = (val as f32).clamp(0.0, 0.95),
                ("delay_length", RtValue::F32(val)) => {
                    self.delay_length = (val as f32).clamp(0.0, self.capacity as f32)
                }
                ("delay_length", RtValue::U32(val)) => {
                    self.feedback = (val as f32).clamp(0.0, 0.95)
                }
                _ => (),
            },
            _ => (),
        }
    }
}
