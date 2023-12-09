#![cfg_attr(feature = "bench", feature(test))]
#![cfg_attr(feature = "nightly", feature(portable_simd))]
#![allow(clippy::precedence)]

// pub mod arena_slices;
// pub mod array_vec;
pub mod bits;
pub mod bounded;
pub mod dijkstra;
pub mod dsu;
pub mod fact;
pub mod fenwick_tree;
// pub mod graph;
pub mod input;
pub mod jagged;
pub mod kd_tree;
pub mod macros;
// pub mod matrix;
pub mod cht;
pub mod complex;
pub mod d2;
pub mod d3;
pub mod disjoint_sparse_table;
pub mod experimental;
pub mod f2;
pub mod hld;
pub mod int;
pub mod li_chao;
pub mod mat_util;
pub mod max_flow;
pub mod min_cost_flow;
pub mod mod_int;
pub mod parser;
pub mod permutation;
pub mod poly;
pub mod push_relabel;
pub mod random;
pub mod rational;
pub mod scc;
pub mod segtree;
pub mod stdio;
pub mod trie;
pub mod two_sat;
pub mod util;

pub mod graph2;
pub mod graph3;
pub mod rbstree;
pub mod slice_arena;

pub mod dft;
pub mod rolling_hash;

pub mod cmat;

pub mod input2;
pub mod light_vec;
pub mod output;
pub mod output2;

pub mod bitset;

pub mod dft2;
pub mod modint2;
pub mod modint_poly;

pub mod counter;
pub mod float;
pub mod sa;
pub mod sandbox;
pub mod slope_trick;
pub mod stable_graph;

#[cfg(feature = "nightly")]
pub mod mod_int_vector;

#[cfg(test)]
mod tests;

#[cfg(all(test, feature = "bench"))]
pub mod bench;
