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

    fn get_node_id(&mut self, string: &str) -> u64 {
        // Use entry API for efficiency
        *self.map.entry(string.to_string()).or_insert_with(|| {
            let id = self.current_string_id;
            self.current_string_id += 1;
            id
        })
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
        Ok(assembly_graph)
    }

    pub(crate) fn add_edge(&mut self, string: String, count: u64) {
        // IMPORTANT: Store the original string AND reversed string
        self.graph_edges.insert(string.clone(), count);
        // Add the reversed string if needed for bidirectional traversal
        let reversed = string.chars().rev().collect::<String>();
        self.graph_edges.insert(reversed, count);
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

        awry::fm_index::FmIndex::new(&awry_build_args)
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
        if search_range.is_empty() {
            return;
        }

        if string.len() == kmer_len {
            assembly_graph.add_edge(string, search_range.len());
            return;
        }

        for c in Self::ALPHABET_CHARS.chars() {
            let next_string = string.clone() + &c.to_string();
            let next_search_range = fm_index.update_range_with_symbol(
                search_range.clone(),
                Symbol::new_ascii(fm_index.alphabet(), c),
            );

            // Only recurse if the search range is not empty
            if !next_search_range.is_empty() {
                Self::create_edges_recursive(
                    assembly_graph,
                    fm_index,
                    kmer_len,
                    next_string,
                    next_search_range,
                );
            }
        }
    }

    pub(crate) fn write_wg_file(&self, output_path: &Path) -> anyhow::Result<()> {
        // Create output directory if it doesn't exist
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut string_id_map = StringIdMap::new();
        let mut output = File::create(output_path)?;

        // Collect and sort edges for deterministic output (optional)
        let mut edges: Vec<_> = self.graph_edges.iter().collect();
        edges.sort_by(|a, b| a.0.cmp(b.0));

        for (string, count) in edges {
            // Ensure string is at least length 2 for slicing
            if string.len() < 2 {
                continue;
            }

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

        Ok(())
    }
}
