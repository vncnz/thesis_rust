use std::collections::HashMap;
// use memory_stats::memory_stats;

#[path = "common.rs"] mod common;
use common::{create_node, get_from_map, recostruct_alignment, recostruct_subproblems, tree_prune, write_file, TreeNode, TREE_MODE};

pub fn build_tree(seq_s: &str, seq_t: &str, match_score: i32, mismatch: i32, gap: i32) -> (i32, usize) {

    let n: usize = seq_s.len();
    let m: usize = seq_t.len();
    let n1 = n + 1;
    let m1 = m + 1;

    // Inizializza il punteggio massimo, la sua posizione, il relativo TreeNode
    let mut max_score = 0;
    // let mut max_node: TreeNode = TreeNode { pos: 0, parent: 0, children: [0,0,0], depth: 0};
    let mut max_pos = 0;

    // Inizializza il dict
    let mut tree = HashMap::with_capacity(m * 2);
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
    for i in 1..n1 {
        let points = 0; // std::cmp::max(0, get_from_map(&tree, &(i-1)).points + gap);
        create_node(i, i - 1, points, &mut tree);
    }

    let mut ratio: f64 = 100.0;
    let mut max_nodes = 0;
    for j in 1..m1 {

        let tree_len = tree.len();
        if tree_len > max_nodes { max_nodes = tree_len; }
        ratio = (100.0 * (tree_len as f64) / ((n1*j) as f64)) as f64;
        if j % std::cmp::max(1 as usize, (m/20) as usize) == 0 {
            println!("Row j={} ({}) tree has {} nodes ({}%)", j, seq_t.as_bytes()[j - 1] as char, tree_len, (ratio * 100.).round() / 100.);

            // Prints out memory consumption
            /* if let Some(usage) = memory_stats() {
                println!("Current physical memory usage: {}", usage.physical_mem);
                println!("Current virtual memory usage: {}", usage.virtual_mem);
            } else {
                println!("Couldn't get the current memory usage :(");
            } */
        }

        let points = std::cmp::max(0, get_from_map(&tree, &(j*n1 - n1)).points + gap);
        create_node(j*n1, (j - 1)*n1, points, &mut tree);
        if j > 1 {
            tree_prune((j-1)*n1 - 1, &mut tree, &max_pos, &n1, &Vec::new(), &Vec::new()); // qui prune dell'ultimo elemento della riga appena abbandonata
        }

        for i in 1..n1 {
            // if j == 27 { println!("\nRow i={}", i); }
            let w: usize = j*n1 + i;
            let wdiag: usize = w - n1 - 1;
            let wup: usize = w - n1;
            let wleft: usize = w - 1;
            // println!("w={} m1={} j={} i={}", w, &m1, &j, &i);
            let match_mismatch_delta_points = get_from_map(&tree, &wdiag).points
                + if seq_s.as_bytes()[i - 1] == seq_t.as_bytes()[j - 1] { match_score }
                  else { mismatch };

            let delete = get_from_map(&tree, &wup).points + gap;
            let insert = get_from_map(&tree, &wleft).points + gap;

            if insert >= match_mismatch_delta_points && insert >= delete {    // * Preferiamo il movimento orizzontale!
                create_node(w, wleft, insert, &mut tree);
            } else if match_mismatch_delta_points >= delete {                // * In alternativa, il diagonale
                create_node(w, wdiag, match_mismatch_delta_points, &mut tree);
            } else {                                                        // * Infine, il verticale
                create_node(w, wup, delete, &mut tree);
            }
           
            tree_prune(wdiag, &mut tree, &max_pos, &n1, &Vec::new(), &Vec::new()); // prune dell'elemento in diagonale

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
        let last_idx = j*n1 + n1 - 1;
        let last_node = get_from_map(&tree, &last_idx);
        let last_node_points = last_node.points;
        if last_node_points > max_score {
            if max_pos > 0 && max_pos < (j-1)*n1 && last_node.children.len() == 0 {
                tree_prune(max_pos, &mut tree, &((j+1)*n1 -1), &n1, &Vec::new(), &Vec::new());
            }
            max_score = last_node_points;
            max_pos = last_idx;
        }

    }

    for i in 0..n1 {
        let idx = (m1-1)*n1 + i;
        let node = get_from_map(&tree, &idx);
        let node_points = node.points;
        if node_points > max_score {
            max_score = node_points;
            max_pos = idx;
        }
    }

    println!("\nMatrix size {} x {} = {}", n1, m1, n1*m1);
    println!("Tree size {} nodes ({}% of matrix size)", tree.len(), (ratio * 100.).round() / 100.);
    // println!("Max tree size before computing last row was {}", max_nodes);

    let mut waypoints:Vec<((usize, usize), (usize, usize))> = vec!();
    let mut fullpath:Vec<(usize, usize)> = vec!();
    if TREE_MODE {
        waypoints = recostruct_subproblems(max_pos, &tree, seq_s, seq_t, n1, &HashMap::new());
    } else {
        fullpath = recostruct_alignment(max_pos, &tree, seq_s, seq_t, n1, &HashMap::new());
    }

    let for_drawer = serde_json::json!({
        "dependences": [],
        "tree": &tree,
        "seq1": &seq_s,
        "seq2": &seq_t,
        "max_pos": &max_pos,
        "max_points": &max_score,
        "waypoints": &waypoints,
        "fullpath": &fullpath,
        "treemode": &TREE_MODE
    });

    let written = write_file(&for_drawer);
    if written.is_err() { panic!("\n❌ Error writing file"); }
    else { println!("\n✅ Full tree written in /tmp/alignment_tree.json ready for drawing"); }

    (max_score, max_pos)
}