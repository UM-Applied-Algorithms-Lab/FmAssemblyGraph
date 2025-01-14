use clap::Parser;

#[derive(Parser, Debug)]
pub(crate)struct ProgramArgs {
    #[arg(short, long)]
    input_dir: String,

    #[arg(short, long)]
    output_path: String,

    #[arg(short, long)]
    kmer_len: usize,

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
}