use std::collections::HashMap;
use memory_stats::memory_stats;

#[path = "common.rs"] mod common;
use common::{create_node, get_from_map, nodes_relationship, print_alignment, print_hash_map, print_path_to_root_compressed, print_path_to_root_full, tree_prune, TreeNode};

fn create_concatenated_alternatives_string (seq: &str) -> (String, HashMap<usize, Vec<usize>>) {
  let variants = HashMap::from([
      ('W', vec!["TTT", "CC"])
  ]);
  let mut faked: String = String::new();
  let mut dependences: HashMap<usize, Vec<usize>> = HashMap::new();
  // let mut last = -1;

  for c in seq.chars() {
      match c {
          'A' | 'C' | 'G' | 'T' => faked.push(c),
          _ => {
              let start = faked.len() - 1;
              let mut derivates = Vec::new();
              for i in 0..variants[&c].len() {
                  faked.push_str(variants[&c][i]);
                  derivates.push(faked.len() - 1);
                  dependences.insert(faked.len() - 1, [start].to_vec());
              }
              let end = faked.len();
              derivates.insert(0, start);
              dependences.insert(end, derivates);
              // println!("start: {}   end: {}   derivates: {:?}", start, end, derivates);
          }
      };
  }
  println!("dependences: {:?}", dependences);
  (faked, dependences)
}


pub fn two_rows_alignment(seq1: &str, seq: &str, match_score: i32, mismatch: i32, gap: i32) -> (i32, usize) {

    let (seqq, dependences) = create_concatenated_alternatives_string(seq);
    let seq2 = &seqq;

    let m = seq1.len();
    let n = seq2.len();
    let m1 = m + 1;
    let n1 = n + 1;

    // Inizializza la tabella DP. Si tratta delle due righe che nella versione py erano chiamate row_a e row_b
    // let mut dp = vec![vec![0; m1]; 2];

    // Inizializza il punteggio massimo, la sua posizione, il relativo TreeNode
    let mut max_score = 0;
    // let mut max_node: TreeNode = TreeNode { pos: 0, parent: 0, children: [0,0,0], depth: 0};
    let mut max_pos = 0;

    // Inizializza il dict
    let mut tree = HashMap::with_capacity(n * 2);
    // create_node(0, 0, &mut tree);
    let node = TreeNode {
        pos: 0,
        parent: 0,
        children: Vec::new(),
        depth: 0,
        points: 0
    };
    tree.insert(node.pos, node);

    // Inizializza la prima riga
    for i in 1..m1 {
        let points = std::cmp::max(0, get_from_map(&tree, &(i-1)).points + gap);
        create_node(i, i - 1, points, &mut tree);
    }

    // Riempie la tabella DP e traccia il punteggio massimo
    let mut ratio: f64 = 100.0;
    for j in 1..n1 {
        ratio = ((100 * tree.len() / (m1*j)) as f64).round();
        if j % std::cmp::max(1 as usize, (n/20) as usize) == 0 {
            println!("\nRow j={} tree is {}%", j, ratio);

            if let Some(usage) = memory_stats() {
                println!("Current physical memory usage: {}", usage.physical_mem);
                println!("Current virtual memory usage: {}", usage.virtual_mem);
            } else {
                println!("Couldn't get the current memory usage :(");
            }
        }
        let points = std::cmp::max(0, get_from_map(&tree, &(j*m1 - m1)).points + gap);
        create_node(j*m1, (j - 1)*m1, points, &mut tree);
        if j > 1 {
            tree_prune((j-1)*m1 - 1, &mut tree, &max_pos, &m1); // qui prune dell'ultimo elemento della riga appena abbandonata
        }

        let mut uprow = Vec::new();
        if dependences.contains_key(&j) {
            let deps = get_from_map(&dependences, &j);
            println!("La riga {} è speciale: {:?}", j, deps);
            match deps.len() {
                1 => {
                    uprow = (deps[0]*m1 .. (deps[0]+1)*m1).collect();
                },
                2 => {
                    panic!("La riga {} è speciale ma ha due valori associati: {:?}", j, deps);
                },
                _ => {
                    // TODO: scegliere valore per valore tra le righe disponibili in base allo score
                    uprow = (deps[1]*m1 .. (deps[1]+1)*m1).collect();
                }
            }
            println!("uprow indeces: {:?} with m1={}", uprow, m1);
        }
        for i in 1..m1 {
            // if j == 27 { println!("\nRow i={}", i); }
            
            let w: usize = j*m1 + i;
            let wdiag: usize = w - m1 - 1;
            let wup: usize = w - m1;
            let wleft: usize = w - 1;
            // println!("w={} m1={} j={} i={}", w, &m1, &j, &i);
            let match_mismatch_delta_points = get_from_map(&tree, &wdiag).points
                + if seq1.as_bytes()[i - 1] == seq2.as_bytes()[j - 1] { match_score }
                  else { mismatch };

            let delete = get_from_map(&tree, &wup).points + gap;
            let insert = get_from_map(&tree, &wleft).points + gap;

            if match_mismatch_delta_points > delete && match_mismatch_delta_points > insert {
                create_node(w, wdiag, match_mismatch_delta_points, &mut tree);
                // L'elemento in diagonale ovviamente non è leaf ma per la versione con percorsi compressi ci serve comunque valutarla?
                tree_prune(wdiag, &mut tree, &max_pos, &m1);
            } else if delete > insert { // * Preferiamo il movimento orizzontale!
                create_node(w, wup, delete, &mut tree);
                tree_prune(wdiag, &mut tree, &max_pos, &m1); // prune dell'elemento in diagonale
            } else {
                create_node(w, wleft, insert, &mut tree);
                tree_prune(wdiag, &mut tree, &max_pos, &m1); // prune dell'elemento in diagonale
            }

            // dp[1][j] = std::cmp::max(match_mismatch_delta_points, std::cmp::max(delete, std::cmp::max(insert, 0)));

            // Traccia il punteggio massimo e la sua posizione
        }
        let last_idx = j*m1 + m1 - 1;
        let last_node_points = get_from_map(&tree, &last_idx).points;
        if last_node_points > max_score {
            if max_pos > 0 && max_pos < (j-1)*m1 - 1 { tree_prune(max_pos, &mut tree, &((j+1)*m1 -1), &m1); }
            // Occhio ad eliminarlo solo se non ha figli, tree_node al momento non fa questo controllo
            max_score = last_node_points;
            max_pos = last_idx;
        }
        // if j < 8 { print_hash_map(&tree); }

        // if j < 10 { print_hash_map(&tree); }

    }

    println!("Matrix size {} x {} = {}", m1, n1, m1*n1);
    println!("Tree size {} nodes ({}% of matrix size)", tree.len(), ratio);
    println!("m is {} and m^2 is {}. n+m is {}", m, m*m, n+m);

    if tree.len() < 170 {
        println!("\nFull schema saved in memory");
        print_hash_map(&tree);
        println!("\nPath from best score to root (w={})", max_pos);
        print_path_to_root_full(max_pos, &tree);
    } else if tree.len() < 1_000 {
        println!("\nFull schema saved in memory too big to be printed, sorry");
        println!("\nPath from best score to root (w={})", max_pos);
        print_path_to_root_compressed(max_pos, &tree);
    } else {
        println!("\nFull schema saved in memory too big to be printed, sorry");
        println!("\nPath from best score to root (w={})", max_pos);
    }
    print_alignment(max_pos, &tree, seq1, seq2, m1);

  println!("{} -> {}, {:?}", seq, seq2, dependences);

  (max_score, max_pos)
}