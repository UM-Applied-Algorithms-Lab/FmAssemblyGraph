mod assembly_graph;
mod program_args;

use assembly_graph::AssemblyGraph;
use clap::Parser;
use program_args::ProgramArgs;
use std::{
    fs::{self, File},
    io,
    path::Path,
};

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

/// concatenates all fastq files in the input directory into a single fastq file
fn concat_fastq_files(input_dir: &Path, output_path: &Path) {
    let mut output_file = File::create(output_path).expect("could not create output fastq file");
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
            if is_fastq_file(&path) {
                let mut input_file = File::open(path).expect("unable to open fastq file");
                io::copy(&mut input_file, &mut output_file)
                    .expect("unable to add fastq to output file");
            }
        }
    }

    debug_assert!(
        file_contains_data(&output_path),
        "output fastq file is empty"
    );
}

fn is_fastq_file(path: &Path) -> bool {
    let file_name = path.file_name().unwrap().to_str().unwrap();
    file_name.ends_with(".fq") || file_name.ends_with("fastq")
}

fn file_contains_data(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => metadata.len() > 0,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::{assembly_graph::AssemblyGraph, concat_fastq_files};
    use std::path::Path;

    const TEST_DATA_DIR: &str = "test/data";
    const TEST_OUTPUT_DIR: &str = "test/output";

    #[test]
    fn can_generate_assembly_graph() {
        let input_dir_path = Path::new(TEST_DATA_DIR).join("fastqs");
        let concatenated_fastq_path = Path::new(TEST_DATA_DIR).join("concatenated.fastq");
        let graph_output_path = Path::new(TEST_OUTPUT_DIR).join("graph.asg");
        {
            //delete the output graph file if it already exists
            let _ = std::fs::remove_file(graph_output_path.as_path());
            assert!(
                !graph_output_path.as_path().exists(),
                "graph file could notn be deleted"
            );

            //delete the output graph file if it already exists
            let _ = std::fs::remove_file(concatenated_fastq_path.as_path());
            assert!(
                !concatenated_fastq_path.exists(),
                "concatenated read file could notn be deleted"
            );
        }
        {
            concat_fastq_files(input_dir_path.as_path(), concatenated_fastq_path.as_path());
            let assembly_graph = AssemblyGraph::new(concatenated_fastq_path.as_path(), 10)
                .expect("could not generate assembly graph");

            assembly_graph
                .write_wg_file(graph_output_path.as_path())
                .expect("could not write asg file");

            assert!(graph_output_path.exists(), "graph file was not created");
        }
        {
            //cleanup after the test
            //delete the output graph file if it already exists
            let _ = std::fs::remove_file(graph_output_path.as_path());
            assert!(
                !graph_output_path.as_path().exists(),
                "graph file could notn be deleted"
            );

            //delete the output graph file if it already exists
            let _ = std::fs::remove_file(concatenated_fastq_path.as_path());
            assert!(
                !concatenated_fastq_path.exists(),
                "concatenated read file could notn be deleted"
            );
        }
    }
}
