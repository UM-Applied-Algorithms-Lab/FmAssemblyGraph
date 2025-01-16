mod assembly_graph;
mod program_args;

use assembly_graph::AssemblyGraph;
use clap::Parser;
use program_args::ProgramArgs;
use std::{fs::File, io, path::Path};

fn main() {
    let args = ProgramArgs::parse();

    let input_dir_path = Path::new(args.input_dir());
    let graph_output_path = Path::new(args.output_path());
    let tmp_concat_fastq_path = Path::new(args.temp_concat_fastq_path());

    concat_fastq_files(input_dir_path, tmp_concat_fastq_path);

    assembly_graph
        .write_wg_file(Path::new(args.output_path()))
        .expect("could not write wg file");
}

fn concat_fastq_files(input_dir: &Path, output_path: &Path) {
    let mut output = File::create(output_path).expect("could not create output fastq file");
    //iterate over all .fq files in input_dir
    for entry in input_dir
        .read_dir()
        .expect("could not read input directory")
    {
        // let entry = entry.unwrap();
        let path = entry
            .expect("could not read path to file in input directory")
            .path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name.ends_with(".fq") {
                let mut input_file = File::open(file_name).expect("unable to open fastq file");
                io::copy(&mut input_file, &mut output).expect("unable to add fastq to output file");
            }
        }
    }
}