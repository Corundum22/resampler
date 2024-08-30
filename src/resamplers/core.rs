use crate::filedata::FileData;

pub struct Resampler {
    pub(super) input_samples: Vec<f32>,
    pub(super) output_samples: Vec<f32>,
    pub(super) ratio: f32,
    pub(super) input_datatype: FileData,
    pub(super) output_datatype: FileData,
    pub(super) source_name: String,
    pub(super) dest_name: String,
    pub(super) row_len: Vec<u32>,
}

impl Resampler {
    pub fn from_args(
        source: String,
        dest: String,
        input_ratio: f32,
        output_ratio: f32
    ) -> Resampler {
        Resampler {
            input_samples: vec![],
            output_samples: vec![],
            ratio: input_ratio / output_ratio,
            input_datatype: FileData::from_string(&source),
            output_datatype: FileData::from_string(&dest),
            source_name: source.clone(),
            dest_name: dest.clone(),
            row_len: Vec::new(),
        }
    }

    // Extends the end of input_samples by repeating the last
    // value n times
    pub(super) fn repeat_last(&mut self, n: usize) {
        // Sets end values for algorithms that use later values
        if let Some(last_ref) = self.input_samples.last() {
            let last_val = *last_ref;
            self.input_samples.append(&mut vec![last_val; n]);
        }
    }

    // Removes the last n input_samples
    pub(super) fn remove_last(&mut self, n: usize) {
        for _k in 0..n {
            self.input_samples.pop();
        }
    }
}
