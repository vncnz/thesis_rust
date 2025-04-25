use std::collections::HashMap;
use memory_stats::memory_stats;

#[path = "common.rs"] mod common;
use common::{create_node, get_from_map, recostruct_alignment, recostruct_subproblems, tree_prune, write_file, TreeNode, TREE_MODE};

pub fn build_tree(seq1: &str, seq2: &str, match_score: i32, mismatch: i32, gap: i32) -> (i32, usize) {

    let m = seq1.len();
    let n = seq2.len();
    let m1 = m + 1;
    let n1 = n + 1;

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
        let points = 0; // std::cmp::max(0, get_from_map(&tree, &(i-1)).points + gap);
        create_node(i, i - 1, points, &mut tree);
    }

    let mut ratio: f64 = 100.0;
    for j in 1..n1 {

        ratio = (100.0 * (tree.len() as f64) / ((m1*j) as f64)) as f64;
        if j % std::cmp::max(1 as usize, (n/20) as usize) == 0 {
            println!("\nRow j={} ({}) tree is {}%", j, seq2.as_bytes()[j - 1] as char, ratio.round());

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
            tree_prune((j-1)*m1 - 1, &mut tree, &max_pos, &m1, &Vec::new(), &Vec::new()); // qui prune dell'ultimo elemento della riga appena abbandonata
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

            /* if match_mismatch_delta_points > delete && match_mismatch_delta_points > insert {
                create_node(w, wdiag, match_mismatch_delta_points, &mut tree);
                // L'elemento in diagonale ovviamente non è leaf ma per la versione con percorsi compressi ci serve comunque valutarla!
            } else if delete > insert { // * Preferiamo il movimento orizzontale! ... appunto, mica è questo -.-'
                create_node(w, wup, delete, &mut tree);
            } else {
                create_node(w, wleft, insert, &mut tree);
            } */

            if insert >= match_mismatch_delta_points && insert >= delete {    // * Preferiamo il movimento orizzontale!
                create_node(w, wleft, insert, &mut tree);
            } else if match_mismatch_delta_points >= delete {                // * In alternativa, il diagonale
                create_node(w, wdiag, match_mismatch_delta_points, &mut tree);
            } else {                                                        // * Infine, il verticale
                create_node(w, wup, delete, &mut tree);
            }
           
            tree_prune(wdiag, &mut tree, &max_pos, &m1, &Vec::new(), &Vec::new()); // prune dell'elemento in diagonale

            // dp[1][j] = std::cmp::max(match_mismatch_delta_points, std::cmp::max(delete, std::cmp::max(insert, 0)));

            // Traccia il punteggio massimo e la sua posizione

            /* if w == 81 || w == 82 {
                let for_drawer = serde_json::json!({
                    "dependences": [],
                    "tree": &tree,
                    "seq1": &seq1,
                    "seq2": &seq2,
                    "max_pos": &max_pos,
                    "max_points": &max_score,
                    "w": &w
                });
                println!("\n(Element {:?} computed)\n{:?}\n\n", w, serde_json::to_string(&for_drawer).unwrap());
            } */
        }
        let last_idx = j*m1 + m1 - 1;
        let last_node = get_from_map(&tree, &last_idx);
        let last_node_points = last_node.points;
        if last_node_points > max_score {
            // println!("Il nuovo nodo {} (punti {}) rimpiazza {} (punti {})?. Elimino il vecchio?", last_idx, last_node_points, max_pos, max_score);
            if max_pos > 0 && max_pos < (j-1)*m1 && last_node.children.len() == 0 {
                // println!("sì");
                tree_prune(max_pos, &mut tree, &((j+1)*m1 -1), &m1, &Vec::new(), &Vec::new());
            } else {
                // println!("no (max_pos={} j={}/m1={})", max_pos, j, m1);
            }
            max_score = last_node_points;
            max_pos = last_idx;
        }
        // if j < 8 { print_hash_map(&tree); }

        // if j < 10 { print_hash_map(&tree); }
    }

    for i in 0..m1 {
        let idx = (n1-1)*m1 + i;
        let node = get_from_map(&tree, &idx);
        let node_points = node.points;
        if node_points > max_score {
            max_score = node_points;
            max_pos = idx;
        }
    }

    println!("\nMatrix size {} x {} = {}", m1, n1, m1*n1);
    println!("Tree size {} nodes ({}% of matrix size)", tree.len(), (ratio * 100.).round() / 100.);

    let mut waypoints:Vec<((usize, usize), (usize, usize))> = vec!();
    let mut fullpath:Vec<(usize, usize)> = vec!();
    if TREE_MODE {
        waypoints = recostruct_subproblems(max_pos, &tree, seq1, seq2, m1, &HashMap::new());
    } else {
        fullpath = recostruct_alignment(max_pos, &tree, seq1, seq2, m1, &HashMap::new());
    }

    let for_drawer = serde_json::json!({
        "dependences": [],
        "tree": &tree,
        "seq1": &seq1,
        "seq2": &seq2,
        "max_pos": &max_pos,
        "max_points": &max_score,
        "waypoints": &waypoints,
        "fullpath": &fullpath
    });

    let written = write_file(&for_drawer);
    if written.is_err() { panic!("\n❌ Error writing file"); }
    else { println!("\n✅ Full tree written in /tmp/alignment_tree.json ready for drawing"); }

    (max_score, max_pos)
}