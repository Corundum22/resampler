use super::core;
use crate::algorithms::*;

impl core::Resampler {
    // Selects and calls an interpolation algorithm based on
    // the provided string
    pub fn do_resample(&mut self, interpolation_type: String) {
        match interpolation_type.to_lowercase().as_str() {
            "lerp" | "linear" | "l" => self.do_lerp(),
            "piecewise_constant" | "piecewise" | "piece" | "p"
                => self.do_piecewise_constant(),
            "tangentless_hermite" | "th" => self.do_tangentless_hermite(),
            "catmull_rom" | "cat" | "cr" => self.do_catmull_rom_spline(),
            _ => panic!("Interpolation type not found!"),
        }
    }

    // Performs piecewise constant interpolation and outputs
    // to output_samples
    fn do_piecewise_constant(&mut self) {
        let mut f: f32 = 0.0;
        
        while f < (self.input_samples.len() - 1) as f32 {
            let k = f.floor() as usize;

            self.output_samples.push(piecewise_constant(
                    self.input_samples[k],
                    self.input_samples[k + 1],
                    f.fract()
                )
            );
            f += self.ratio;
        }
    }

    // Performs lerp and outputs to output_samples
    fn do_lerp(&mut self) {
        let mut f: f32 = 0.0;
        
        while f < (self.input_samples.len() - 1) as f32 {
            let k = f.floor() as usize;

            self.output_samples.push(lerp(
                    self.input_samples[k],
                    self.input_samples[k + 1],
                    f.fract()
                )
            );
            f += self.ratio;
        }
    }

    // Performs cubic spline interpolation without the tangent
    // components and outputs to output_samples
    fn do_tangentless_hermite(&mut self) {
        let mut f: f32 = 0.0;
        
        while f < (self.input_samples.len() - 1) as f32 {
            self.output_samples.push(tangentless_hermite(
                    self.input_samples[f.floor() as usize],
                    self.input_samples[f.floor() as usize + 1],
                    f.fract()
                )
            );
            f += self.ratio;
        }
    }

    // Performs Catmull-Rom cubic spline interpolation and
    // outputs to output_samples
    fn do_catmull_rom_spline(&mut self) {
        self.repeat_last(2);

        let mut f = 0.0;
        let m_get =
            | pos_behind: f32, pos_ahead: f32 | -> f32 {
                (pos_ahead - pos_behind) * 0.5
        };
        let mut m_current = 0.0;

        // TODO: properly handle beginning conditions of input_samples
        
        while f <= (self.input_samples.len() - 3) as f32 {
            self.output_samples.push((|| {
                let k = f.floor() as usize;
                let position = [
                    self.input_samples[k],
                    self.input_samples[k + 1],
                    self.input_samples[k + 2],
                ];

                let m_prev = m_current;
                m_current = m_get(position[0], position[2]);

                return even_hermite(
                    position[0],
                    position[1],
                    m_prev,
                    m_current,
                    f.fract(),
                );
            })());
            f += self.ratio;
        }

        self.remove_last(2);
    }
}
