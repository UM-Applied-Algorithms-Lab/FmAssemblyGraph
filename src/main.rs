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

    let _assembly_graph = AssemblyGraph::new(tmp_concat_fastq_path, args.kmer_len())
        .expect("could not generate assembly graph")
        .write_wg_file(graph_output_path)
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

#[cfg(test)]
mod tests {
    use crate::{assembly_graph::AssemblyGraph, concat_fastq_files};
    use std::path::Path;

    #[test]
    fn can_generate_assembly_graph() {
        let input_dir_path = Path::new("test/data/fastqs");
        let concatenated_fastq_path = Path::new("test/data/concatenated.fastq");
        let graph_output_path = Path::new("test/output");
        {
            //delete the output graph file if it already exists
            let _ = std::fs::remove_file(graph_output_path);
            assert!(
                !graph_output_path.exists(),
                "graph file could notn be deleted"
            );

            //delete the output graph file if it already exists
            let _ = std::fs::remove_file(concatenated_fastq_path);
            assert!(
                !concatenated_fastq_path.exists(),
                "concatenated read file could notn be deleted"
            );
        }
        {
            concat_fastq_files(input_dir_path, concatenated_fastq_path);
            let assembly_graph = AssemblyGraph::new(graph_output_path, 10)
                .expect("could not generate assembly graph");

            assembly_graph
                .write_wg_file(Path::new("tests/output"))
                .expect("could not write wg file");

            assert!(graph_output_path.exists(), "graph file was not created");
        }
        {
            //cleanup after the test
            //delete the output graph file if it already exists
            let _ = std::fs::remove_file(graph_output_path);
            assert!(
                !graph_output_path.exists(),
                "graph file could notn be deleted"
            );

            //delete the output graph file if it already exists
            let _ = std::fs::remove_file(concatenated_fastq_path);
            assert!(
                !concatenated_fastq_path.exists(),
                "concatenated read file could notn be deleted"
            );
        }
    }
}
