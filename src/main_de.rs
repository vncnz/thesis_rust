use std::collections::HashMap;
// use memory_stats::memory_stats;
// use cli_clipboard;

#[path = "common.rs"] mod common;
use common::{create_node, get_from_map, print_hash_map, recostruct_alignment, recostruct_subproblems, tree_prune, write_file, TreeNode, TREE_MODE};

fn create_concatenated_alternatives_string (seq: &str) -> (String, HashMap<usize, Vec<usize>>) {
    let mut faked: String = String::new();
    let mut dependences: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut building_alternative: u8 = 0;
    let mut start: usize = 0;
    let mut derivates = Vec::new();
    for c in seq.chars() {
        match c {
            'A' | 'C' | 'G' | 'T' => {
                faked.push(c);
            },
            '[' => {
                if building_alternative > 0 {
                    panic!("Nested alternatives not supported!");
                }
                building_alternative = 1;
                start = faked.len() - 1;
                dependences.insert(faked.len() + 1, [start + 1].to_vec());
            },
            ']' => {
                if building_alternative == 0 {
                    panic!("Closing an alternative never opened!");
                } else if building_alternative == 1 {
                    panic!("Closing a fake alternative, at least an OR is mandatory!");
                }

                let end = faked.len() + 1;
                derivates.push(faked.len());
                derivates.insert(0, start + 1);
                dependences.insert(end, derivates);
                derivates = Vec::new();
                building_alternative = 0;
            },
            '|' => {
                if building_alternative == 0 {
                    panic!("Alternative never opened!");
                }
                derivates.push(faked.len());
                dependences.insert(faked.len() + 1, [start + 1].to_vec());
                building_alternative = 2;
            },
            _ => {
                panic!("Wrong char in input sequence!");
            }
        };
    }
    if building_alternative != 0 {
        panic!("Alternative never closed!");
    }
    println!("dependences: {:?}", dependences);
    (faked, dependences)
}


pub fn build_tree(seq_s: &str, seq: &str, match_score: i32, mismatch: i32, gap: i32) -> (i32, usize) {

    let (seqq, dependences) = create_concatenated_alternatives_string(seq);
    let seq_t = &seqq;

    let n: usize = seq_s.len();
    let m: usize = seq_t.len();
    let n1 = n + 1;
    let m1 = m + 1;

    let mut lines_to_keep: Vec<usize> = Vec::new();
    let mut dont_skip: Vec<usize> = Vec::new();
    for (_key, dep) in dependences.iter() {
        // if !lines_to_keep.contains(key) { lines_to_keep.push(*key); }
        // if !dont_skip.contains(key) { dont_skip.push(*key); }
        for d in dep {
            if !lines_to_keep.contains(d) { lines_to_keep.push(*d); }
            if !dont_skip.contains(d) { dont_skip.push(*d); }
            println!("lines_to_keep: {:?}   dont_skip: {:?}", &lines_to_keep, &dont_skip);
        }
    }

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
        if j > 1 {
            tree_prune((j-1)*n1 - 1, &mut tree, &max_pos, &n1, &lines_to_keep, &dont_skip); // qui prune dell'ultimo elemento della riga appena abbandonata
        }

        let mut uprow: Vec<usize> = ((j-1)*n1 .. j*n1).collect();
        if dependences.contains_key(&j) {
            let deps = get_from_map(&dependences, &j);
            println!("La riga {} è speciale: {:?}", j, deps);
            match deps.len() {
                1 => {
                    println!("Riga con una dipendenza unica: {}", deps[0]);
                    uprow = (deps[0]*n1 .. (deps[0]+1)*n1).collect();
                    println!("lines_to_keep: {:?}", &lines_to_keep);
                },
                2 => {
                    panic!("La riga {} è speciale ma ha due valori associati: {:?}", j, deps);
                },
                _ => {
                    // * In questo caso il formato è [start, variant1, ..., variantN]
                    println!("Riga con più dipendenze: {:?}", deps);
                    uprow.clear();
                    for i in 0..n1 {
                        let mut max = 1;
                        for d in 2..deps.len() {
                            println!("Comparing {} ({}) with {} ({})", deps[d]*n1 + i, get_from_map(&tree, &(deps[d]*n1 + i)).points, deps[max]*n1 + i, get_from_map(&tree, &(deps[max]*n1 + i)).points);
                            if get_from_map(&tree, &(deps[d]*n1 + i)).points > get_from_map(&tree, &(deps[max]*n1 + i)).points {
                                max = d;
                            }
                        }
                        uprow.push(deps[max]*n1 + i);
                    }
                    // uprow = (deps[1]*m1 .. (deps[1]+1)*m1).collect();
                }
            }
            println!("uprow indeces: {:?} with m1={}", uprow, n1);
        }
        create_node(j*n1, uprow[0], points, &mut tree);
        for i in 1..n1 {
            // if j == 27 { println!("\nRow i={}", i); }
            
            let w: usize = j*n1 + i;
            let wdiag: usize = uprow[i-1];
            let wup: usize = uprow[i];
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

            tree_prune(wdiag, &mut tree, &max_pos, &n1, &lines_to_keep, &dont_skip);
        }

        let last_idx = j*n1 + n1 - 1;
        let last_node = get_from_map(&tree, &last_idx);
        let last_node_points = last_node.points;
        if last_node_points > max_score {
            if max_pos > 0 && max_pos < (j-1)*n1 && last_node.children.len() == 0 {
                tree_prune(max_pos, &mut tree, &((j+1)*n1 -1), &n1, &lines_to_keep, &dont_skip);
            }
            max_score = last_node_points;
            max_pos = last_idx;
        }

        // * Eventually clean all rows previously blocked by split/merge hops
        if dependences.contains_key(&j) {
            let deps = get_from_map(&dependences, &j);
            if deps.len() > 2 {
                dont_skip.retain(|&x| x != deps[0]);
                println!("                         removed {} from dont_skip {:?}", &deps[0], &dont_skip);
                for d in deps {
                    lines_to_keep.retain(|&x| x != *d);
                    println!("                         removed {} from lines_to_keep {:?}", &d, &lines_to_keep);
                }
                // This is a closing-alternative node, we can clean up all the previously blocked rows!
                for d in deps {
                    println!("Pruning row {} start", d);
                    for i in 0..n1 {
                        let w = d*n1 + i;
                        tree_prune(w, &mut tree, &max_pos, &n1, &lines_to_keep, &dont_skip);
                    }
                }
            }
        }
        // if j == 7 { print_hash_map(&tree); }
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

    println!("{} -> {} with dependences={:?} and dont_delete={:?}", seq, seq_t, dependences, dont_skip);
    println!("Matrix size {} x {} = {}", n1, m1, n1*m1);
    println!("Tree size {} nodes ({}% of matrix size)", tree.len(), (ratio * 100.).round() / 100.);
    // println!("Max tree size before computing last row was {}", max_nodes);

    let mut waypoints:Vec<((usize, usize), (usize, usize))> = vec!();
    let mut fullpath:Vec<(usize, usize)> = vec!();
    if TREE_MODE {
        waypoints = recostruct_subproblems(max_pos, &tree, seq_s, seq_t, n1, &dependences);
    } else {
        fullpath = recostruct_alignment(max_pos, &tree, seq_s, seq_t, n1, &dependences);
    }

    let for_drawer = serde_json::json!({
        "dependences": &dependences,
        "tree": &tree,
        "seq1": &seq_s,
        "seq2": &seq_t,
        "max_pos": &max_pos,
        "max_points": &max_score,
        "waypoints": &waypoints,
        "fullpath": &fullpath,
        "treemode": &TREE_MODE
    });

    // println!("{:?}", &fullpath);
    let written = write_file(&for_drawer);
    if written.is_err() { panic!("\n❌ Error writing file"); }
    else { println!("\n✅ Full tree written in /tmp/alignment_tree.json ready for drawing"); }

    // cli_clipboard::set_contents(serde_json::to_string(&for_drawer).unwrap().to_owned()).unwrap();

    (max_score, max_pos)
}