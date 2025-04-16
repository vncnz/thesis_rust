use std::collections::HashMap;
use memory_stats::memory_stats;
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


pub fn build_tree(seq1: &str, seq: &str, match_score: i32, mismatch: i32, gap: i32) -> (i32, usize) {

    let (seqq, dependences) = create_concatenated_alternatives_string(seq);
    let seq2 = &seqq;

    let m = seq1.len();
    let n = seq2.len();
    let m1 = m + 1;
    let n1 = n + 1;

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

        ratio = ((100 * tree.len() / (m1*j)) as f64).round();
        if j % std::cmp::max(1 as usize, (n/20) as usize) == 0 {
            println!("\nRow j={} ({}) tree is {}%", j, seq2.as_bytes()[j - 1] as char, ratio);

            if let Some(usage) = memory_stats() {
                println!("Current physical memory usage: {}", usage.physical_mem);
                println!("Current virtual memory usage: {}", usage.virtual_mem);
            } else {
                println!("Couldn't get the current memory usage :(");
            }
        }

        let points = std::cmp::max(0, get_from_map(&tree, &(j*m1 - m1)).points + gap);
        if j > 1 {
            tree_prune((j-1)*m1 - 1, &mut tree, &max_pos, &m1, &lines_to_keep, &dont_skip); // qui prune dell'ultimo elemento della riga appena abbandonata
        }

        let mut uprow: Vec<usize> = ((j-1)*m1 .. j*m1).collect();
        if dependences.contains_key(&j) {
            let deps = get_from_map(&dependences, &j);
            println!("La riga {} è speciale: {:?}", j, deps);
            match deps.len() {
                1 => {
                    println!("Riga con una dipendenza unica: {}", deps[0]);
                    uprow = (deps[0]*m1 .. (deps[0]+1)*m1).collect();
                    println!("lines_to_keep: {:?}", &lines_to_keep);
                },
                2 => {
                    panic!("La riga {} è speciale ma ha due valori associati: {:?}", j, deps);
                },
                _ => {
                    // * In questo caso il formato è [start, variant1, ..., variantN]
                    println!("Riga con più dipendenze: {:?}", deps);
                    uprow.clear();
                    for i in 0..m1 {
                        let mut max = 1;
                        for d in 2..deps.len() {
                            println!("Comparing {} ({}) with {} ({})", deps[d]*m1 + i, get_from_map(&tree, &(deps[d]*m1 + i)).points, deps[max]*m1 + i, get_from_map(&tree, &(deps[max]*m1 + i)).points);
                            if get_from_map(&tree, &(deps[d]*m1 + i)).points > get_from_map(&tree, &(deps[max]*m1 + i)).points {
                                max = d;
                            }
                        }
                        uprow.push(deps[max]*m1 + i);
                    }
                    // uprow = (deps[1]*m1 .. (deps[1]+1)*m1).collect();
                }
            }
            println!("uprow indeces: {:?} with m1={}", uprow, m1);
        }
        create_node(j*m1, uprow[0], points, &mut tree);
        for i in 1..m1 {
            // if j == 27 { println!("\nRow i={}", i); }
            
            let w: usize = j*m1 + i;
            let wdiag: usize = uprow[i-1];
            let wup: usize = uprow[i];
            let wleft: usize = w - 1;
            // println!("w={} m1={} j={} i={}", w, &m1, &j, &i);
            let match_mismatch_delta_points = get_from_map(&tree, &wdiag).points
                + if seq1.as_bytes()[i - 1] == seq2.as_bytes()[j - 1] { match_score }
                  else { mismatch };

            // println!("w={}, wleft={}, wdiag={}, wtop={}", &w, &wleft, &wdiag, &wup);
            let delete = get_from_map(&tree, &wup).points + gap;
            let insert = get_from_map(&tree, &wleft).points + gap;

            /* if match_mismatch_delta_points > delete && match_mismatch_delta_points > insert {
                create_node(w, wdiag, match_mismatch_delta_points, &mut tree);
                // L'elemento in diagonale ovviamente non è leaf ma per la versione con percorsi compressi ci serve comunque valutarla?
            } else if delete > insert { // * Preferiamo il movimento orizzontale!
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

            tree_prune(wdiag, &mut tree, &max_pos, &m1, &lines_to_keep, &dont_skip);
        }

        let last_idx = j*m1 + m1 - 1;

        let last_node = get_from_map(&tree, &last_idx);
        let last_node_points = last_node.points;
        if last_node_points > max_score {
            if max_pos > 0 && max_pos < (j-1)*m1 && last_node.children.len() == 0 {
                tree_prune(max_pos, &mut tree, &((j+1)*m1 -1), &m1, &lines_to_keep, &dont_skip);
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
                    for i in 0..(m1-1) {
                        let w = d*m1 + i;
                        tree_prune(w, &mut tree, &max_pos, &m1, &lines_to_keep, &dont_skip);
                    }
                }
            }
        }
        if j == 7 { print_hash_map(&tree); }
    }

    /* if tree.len() < 170 {
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
    } */

    println!("{} -> {} with dependences={:?} and dont_delete={:?}", seq, seq2, dependences, dont_skip);
    println!("Matrix size {} x {} = {}", m1, n1, m1*n1);
    println!("Tree size {} nodes ({}% of matrix size)", tree.len(), (ratio * 100.).round() / 100.);

    let mut waypoints:Vec<((usize, usize), (usize, usize))> = vec!();
    let mut fullpath:Vec<(usize, usize)> = vec!();
    if TREE_MODE {
        waypoints = recostruct_subproblems(max_pos, &tree, seq1, seq2, m1, &dependences);
    } else {
        fullpath = recostruct_alignment(max_pos, &tree, seq1, seq2, m1, &dependences);
    }

    let for_drawer = serde_json::json!({
        "dependences": &dependences,
        "tree": &tree,
        "seq1": &seq1,
        "seq2": &seq2,
        "max_pos": &max_pos,
        "max_points": &max_score,
        "waypoints": &waypoints,
        "fullpath": &fullpath
    });
    
    // println!("\n\n{:?}\n\n", serde_json::to_string(&for_drawer).unwrap());
    let written = write_file(&for_drawer);
    if written.is_err() { panic!("\n❌ Error writing file"); }
    else { println!("\n✅ Full tree written in /tmp/alignment_tree.json ready for drawing"); }

    // cli_clipboard::set_contents(serde_json::to_string(&for_drawer).unwrap().to_owned()).unwrap();

    (max_score, max_pos)
}