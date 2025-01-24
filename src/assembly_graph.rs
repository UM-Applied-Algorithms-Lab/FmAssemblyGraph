use std::{collections::HashMap, fs::File, io::Write, path::Path};

use awry::{alphabet::Symbol, fm_index::FmIndex, search::SearchRange};

pub(crate) struct AssemblyGraph {
    graph_edges: HashMap<String, u64>,
}

struct StringIdMap {
    map: HashMap<String, u64>,
    current_string_id: u64,
}
impl StringIdMap {
    const STARTING_STRING_ID: u64 = 2;

    fn new() -> Self {
        Self {
            map: HashMap::new(),
            current_string_id: Self::STARTING_STRING_ID,
        }
    }
    fn get_node_id(&mut self, string: &String) -> u64 {
        if self.map.contains_key(string) {
            return self.map[string];
        } else {
            let string_id = self.map.insert(string.clone(), self.current_string_id);
            self.current_string_id += 1;
            return string_id.expect("unable to find given node id");
        }
    }
}

impl AssemblyGraph {
    const ALPHABET_CHARS: &str = "AGCT";
    pub(crate) fn new(read_set_path: &Path, kmer_len: usize) -> anyhow::Result<Self> {
        let mut assembly_graph = Self {
            graph_edges: HashMap::new(),
        };
        let fm_index = Self::build_fm_index(read_set_path)?;

        Self::create_edges(&mut assembly_graph, &fm_index, kmer_len);
        anyhow::Ok(assembly_graph)
    }

    pub(crate) fn add_edge(&mut self, string: String, count: u64) {
        self.graph_edges.insert(string, count);
    }

    fn build_fm_index(read_set_path: &Path) -> anyhow::Result<FmIndex> {
        let awry_build_args = awry::fm_index::FmBuildArgs {
            input_file_src: read_set_path.to_path_buf(),
            suffix_array_output_src: None,
            suffix_array_compression_ratio: Some(1 << 62),
            lookup_table_kmer_len: None,
            alphabet: awry::alphabet::SymbolAlphabet::Nucleotide,
            max_query_len: Some(100),
            remove_intermediate_suffix_array_file: true,
        };

        return awry::fm_index::FmIndex::new(&awry_build_args);
    }

    fn create_edges(assembly_graph: &mut AssemblyGraph, fm_index: &FmIndex, kmer_len: usize) {
        for c in Self::ALPHABET_CHARS.chars() {
            let current_string: String = c.to_string();
            let search_range =
                fm_index.initial_search_range(Symbol::new_ascii(fm_index.alphabet(), c));
            Self::create_edges_recursive(
                assembly_graph,
                fm_index,
                kmer_len,
                current_string,
                search_range,
            );
        }
    }

    fn create_edges_recursive(
        assembly_graph: &mut AssemblyGraph,
        fm_index: &FmIndex,
        kmer_len: usize,
        string: String,
        search_range: SearchRange,
    ) {
        let search_range_len = search_range.clone().len();
        if search_range_len == 0 {
            return;
        }
        if string.len() == kmer_len {
            assembly_graph.add_edge(string, search_range_len);
            return;
        }

        for c in Self::ALPHABET_CHARS.chars() {
            let next_string = string.clone() + &c.to_string();
            let search_range = fm_index.update_range_with_symbol(
                search_range.clone(),
                Symbol::new_ascii(fm_index.alphabet(), c),
            );
            Self::create_edges_recursive(
                assembly_graph,
                fm_index,
                kmer_len,
                next_string,
                search_range,
            );
        }
    }

    //function that returns an iterator of (u64, u64, u64) made from the graph_edges hashmap
    pub(crate) fn write_wg_file(&self, output_path: &Path) -> anyhow::Result<&Self> {
        std::fs::create_dir_all(
            output_path
                .parent()
                .expect("output file does not name a file source"),
        )
        .expect("could not create output directory");
        let mut string_id_map: StringIdMap = StringIdMap::new();

        let mut output = File::create(output_path)?;
        for (string, count) in self.graph_edges.iter() {
            let from_node_string = string[0..string.len() - 1].to_string();
            let to_node_string = string[1..string.len()].to_string();
            let from_node_id = string_id_map.get_node_id(&from_node_string);
            let to_node_id = string_id_map.get_node_id(&to_node_string);
            writeln!(
                output,
                "{}\t{}\t{}\t{}",
                from_node_id, to_node_id, count, string
            )?;
        }

        return Ok(&self);
    }
}
