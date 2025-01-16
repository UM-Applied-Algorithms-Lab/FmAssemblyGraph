use clap::Parser;

#[derive(Parser, Debug)]
pub(crate)struct ProgramArgs {
    #[arg(short, long)]
    input_dir: String,

    #[arg(short, long)]
    output_path: String,

    #[arg(short, long)]
    kmer_len: usize,
    
    #[arg(short,long, default_value_t = String::from("/tmp/tmp_concat.fastq"))]
    temp_concat_fastq_path: String,

}

impl ProgramArgs {
    pub(crate) fn input_dir(&self) -> &str {
        &self.input_dir
    }

    pub(crate) fn output_path(&self) -> &str {
        &self.output_path
    }

    pub(crate) fn kmer_len(&self) -> usize {
        self.kmer_len
    }
    
    pub(crate) fn temp_concat_fastq_path(&self) -> &str {
        &self.temp_concat_fastq_path
    }
}