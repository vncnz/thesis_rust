use std::collections::HashMap;
// use std::fs;
// use serde_json;
use itertools::Itertools;
use colored::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub static TREE_MODE: bool = true;

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeNode {
    pub(crate) pos: usize,
    pub(crate) parent: usize,
    pub(crate) children: Vec<usize>,
    pub(crate) depth: u32,
    pub(crate) points: i32
}

pub fn print_hash_map(map: &HashMap<usize, TreeNode>) {
    // for (key, value) in &*map {
    for key in map.keys().sorted() {
        // println!("{:3} / {:?}", &key, &value);
        println!("{:3} / {:?}", &key, map[key]);
    }
    // map.clear();
}/*
pub fn print_path_to_root_compressed(starting: usize, map: &HashMap<usize, TreeNode>) {
    let mut path = format!("{}", map[&starting].pos);
    let mut cnode = &map[&starting];
    while cnode.parent != cnode.pos {
        cnode = get_from_map(map, &cnode.parent); // &map[&cnode.parent];
        // println!("{:?}", &cnode);
        let f = format!(" -> {:?}", &cnode.pos);
        path.push_str(&f);
    }
    println!("{}", path);
}

pub fn print_path_to_root_full(starting: usize, map: &HashMap<usize, TreeNode>) {
    let mut cnode = &map[&starting];
    println!("{:?}", cnode);
    while cnode.parent != cnode.pos {
        cnode = get_from_map(map, &cnode.parent); // &map[&cnode.parent];
        println!("{:?}", &cnode);
    }
} */

pub fn get_from_map<'a, K: Eq + std::hash::Hash + std::fmt::Debug, V>(map: &'a HashMap<K, V>, key: &K) -> &'a V {
    map.get(key).expect(&format!("Key {:?} not found in map", key))
}
pub fn get_mut_from_map<'a, K: Eq + std::hash::Hash + std::fmt::Debug, V>(map: &'a mut HashMap<K, V>, key: &K) -> &'a mut V {
    map.get_mut(key).expect(&format!("Key {:?} not found in map", key))
}

/*
#[derive(Debug)]
pub struct Relationship {
    pub(crate) d: bool,
    pub(crate) v: bool,
    pub(crate) h: bool
}
pub fn nodes_relationship (current_node: usize, parent: usize, m1: usize) -> Relationship {
    // println!("Computing relation between {} and {}", current_node, parent);
    let vmov0 = (current_node % m1 - parent % m1) as i64;
    let hmov0 = (current_node / m1) as i64 - (parent / m1) as i64;
    let r = Relationship { d: hmov0 == vmov0, v: vmov0 == 0, h: hmov0 == 0 };
    // println!("Computing relation between {} and {} {:?}", current_node, parent, r);
    r
} */

pub fn create_node(w: usize, parent: usize, points: i32, tree: &mut HashMap<usize, TreeNode>) {
    // println!("{}, child of {}", w, &parent);
    let p = tree.get_mut(&parent).unwrap();
    p.children.push(w);
    let n = TreeNode {
        pos: w,
        parent: parent,
        children: Vec::new(),
        depth: p.depth + 1,
        points: points
    };
    tree.insert(n.pos, n);
}

pub fn tree_prune(w: usize, tree: &mut HashMap<usize, TreeNode>, protected: &usize, m1: &usize, lines_to_keep: &Vec<usize>, dont_skip: &Vec<usize>) {
    let mut current_node: usize = w;
    let mut children_num: usize = 100;
    let mut row = &current_node / m1;

    // println!("tree_prune on {}", w);

    while current_node != *protected && !lines_to_keep.contains(&row) {
        let parent_id;

        // Primo riferimento mutabile: recuperiamo il nodo corrente
        {
            let n: &mut TreeNode = get_mut_from_map(tree, &current_node); // tree.get_mut(&current_node).expect(&format!("Key not found in tree: {}", current_node));
            // println!("      working on {:?}", n);

            // Se il nodo ha figli, non facciamo nulla e interrompiamo il ciclo
            if n.children.len() > 0 {
                children_num = n.children.len();
                break;
            }

            // Altrimenti, raccogliamo le informazioni sul nodo corrente
            parent_id = n.parent;
        }

        // let children_num: usize;
        {
            // Recuperiamo il genitore del nodo corrente
            let p: &mut TreeNode = get_mut_from_map(tree, &parent_id); // tree.get_mut(&parent_id).expect(&format!("Key not found in tree: {}", parent_id));
            
            // Rimuoviamo il nodo corrente dalla lista dei figli del genitore
            if let Some(pos) = p.children.iter().position(|x: &usize| *x == current_node) {
                p.children.swap_remove(pos);
            }

            // Verifichiamo se il genitore ha altri figli
            children_num = p.children.len();

            // Rimuoviamo il nodo corrente dall'albero
            tree.remove(&current_node);
            // println!("Elimino elemento {}", &current_node);
        }

        // Se il genitore non ha più figli, proseguiamo potando il genitore
        current_node = parent_id;
        row = current_node / m1;

        if children_num > 0 {
            break;
        }
    }

    let n: &mut TreeNode = get_mut_from_map(tree, &current_node);
    if dont_skip.contains(&(&n.parent / m1)) {
        println!("Don't delete {}, This line needs to be kept: {} {:?}", current_node, &row, &lines_to_keep);
        return;
    }

    if current_node != *protected && children_num == 1 && current_node != 0 && !dont_skip.contains(&(current_node / m1)) {
        if TREE_MODE {
            //* Se eliminiamo questo nodo indipendentemente dall'eventuale cambio direzione arriviamo alla versione solo albero, senza percorsi completi.
            // La possiamo chiamare "tree mode".
            skip_node(current_node, tree);
            //}
        } else {
            let vmov0 = (current_node % m1 - n.parent % m1) as i64;
            let vmov1 = (n.children[0] % m1 - n.pos % m1) as i64;
            let hmov0 = (current_node / m1) as i64 - (n.parent / m1) as i64;
            let hmov1 = (n.children[0] / m1) as i64 - (n.pos / m1) as i64;
            let diag = hmov0 == vmov0 && hmov1 == vmov1;
            let left = hmov0 == 0 && hmov1 == 0;
            let up = vmov0 == 0 && vmov1 == 0;
            if diag || left || up {
                // println!("Exited on {} and I can skip it (diag [to be extended]) {:?}", current_node, n);
                skip_node(current_node, tree);
            }
        }
    } // Questo abilita la "nuova versione"
}

pub fn skip_node (w: usize, tree: &mut HashMap<usize, TreeNode>) {
    // return;
    // println!("Skipping node {}", &w);

    let w0: usize;
    let w2: usize;

    {
        let n: &mut TreeNode = get_mut_from_map(tree, &w);
        w0 = n.parent;
        w2 = n.children[0];
    }

    {
        let n0: &mut TreeNode = get_mut_from_map(tree, &w0);

        // Sostituiamo il nodo corrente dalla lista dei figli del genitore
        if let Some(pos) = n0.children.iter().position(|x: &usize| *x == w) {
            n0.children[pos] = w2;
        }
    }

    {
        let n2: &mut TreeNode = get_mut_from_map(tree, &w2);
        n2.parent = w0;
    }

    // Rimuoviamo il nodo corrente dall'albero
    tree.remove(&w);
}

pub fn recostruct_alignment(max_points_pos: usize, map: &HashMap<usize, TreeNode>, seq1: &str, seq2: &str, m1: usize, dependences: &HashMap<usize, Vec<usize>>) -> Vec<(usize, usize)> {
    let seq1v: Vec<char> = seq1.chars().collect();
    let seq2v: Vec<char> = seq2.chars().collect();
    let end_pos = (seq1v.len() + 1) * (seq2v.len() + 1) - 1;
    // println!("end pos is {}", end_pos);
    let mut a: String = "".to_owned();
    let mut b: String = "".to_owned();
    // let mut cnode = &map[&max_points_pos];
    let mut cnode = &TreeNode { pos: end_pos, parent: max_points_pos, children: [].to_vec(), depth: 0, points: 0 };
    if end_pos == max_points_pos {
        cnode = get_from_map(map, &end_pos);
    }

    let mut hmov = cnode.pos % m1 != cnode.parent % m1;
    let mut vmov = cnode.pos / m1 != cnode.parent / m1;
    let mut p = cnode.pos;
    let mut parent = cnode.parent;

    let mut coords = vec![];

    let mut ssafe = seq1.len() * seq2.len() + 3;
    while ssafe > 0 && p > 0 {
        // println!("\n   ssafe={} p={} hmov={} vmov={} x={} y={} cnode={:?}", &ssafe, &p, &hmov, &vmov, p%m1, p/m1, &cnode);
        ssafe -= 1;

        // println!("Indeces x={} ({}) y={} ({})", p%m1, seq1v[p%m1 -1], p/m1, seq2v[p/m1 -1]);
        if vmov { b.insert(0, seq2v[(p / m1 -1) as usize]); }
        else { b.insert(0, '-'); }

        if hmov { a.insert(0, seq1v[p % m1 -1]); }
        else { a.insert(0, '-'); }

        let row_number = p / m1;
        
        if dependences.contains_key(&(row_number)) { // && get_from_map(&dependences, &row_number).len() == 1 {
            /* let v = get_from_map(&dependences, &row_number);
            if v.len() == 1 {
                let oldp = p;
                p = (get_from_map(&dependences, &row_number)[0]* m1) + (p % m1);
                println!("Teleporting p from {} to {}", oldp, p);
            } else {
                p = cnode.parent; // * In fase di costruzione albero mi garantisco la presenza di un nodo nell'ultima riga dell'alternativa scelta
            } */
            p = cnode.parent; // * In fase di costruzione albero mi garantisco la presenza di un nodo nell'ultima riga dell'alternativa scelta
            // println!("{} for row_number {} {:?}", "Using parent".yellow(), &row_number, &dependences);
        } else {
            // println!("{}", "NOT using parent".cyan());
            if vmov  { p = p - m1; }
            if hmov { p = p - 1; }
        }

        coords.push((p % m1, p / m1));

        if &p == &parent {
            cnode = get_from_map(map, &p);
            parent = cnode.parent;
            // println!("p={} parent={} node={:?}", &p, &parent, &cnode);
            hmov = p % m1 != parent % m1;
            vmov = p / m1 != parent / m1;
        }
        // println!("{}", a);
        // println!("{}", b);
    }

    if ssafe == 0 {
        panic!("{}", "Infinite cycle detected in alignment recostruction".red().bold());
    }

    coords.reverse();

    println!("\n\n{}", "Alignment completed".green());
    // println!("{:?}", coords);
    if a.len() < 50 {
        println!("{}", a);
        println!("{}", b);
    }

    coords
}

pub fn recostruct_subproblems(max_points_pos: usize, map: &HashMap<usize, TreeNode>, seq1: &str, seq2: &str, m1: usize, dependences: &HashMap<usize, Vec<usize>>) -> Vec<((usize, usize), (usize, usize))> {
    // !! Da testare e da modificare per la versione de-strings!
    // let seq1v: Vec<char> = seq1.chars().collect();
    // let seq2v: Vec<char> = seq2.chars().collect();
    let end_pos = (seq1.len() + 1) * (seq2.len() + 1) - 1;
    // println!("end pos is {}", end_pos);

    let mut cnode = &TreeNode { pos: end_pos, parent: max_points_pos, children: [].to_vec(), depth: 0, points: 0 };
    if end_pos == max_points_pos {
        cnode = get_from_map(map, &end_pos);
    }

    let mut pos = cnode.pos;
    let mut parent = cnode.parent;

    let mut coords = vec![];

    let mut ssafe = seq1.len() * seq2.len() + 3;

    coords.push((end_pos % m1, end_pos / m1));

    while ssafe > 0 && pos > 0 {
        // println!("\n   ssafe={} p={} hmov={} vmov={} x={} y={} cnode={:?}", &ssafe, &p, &hmov, &vmov, p%m1, p/m1, &cnode);
        ssafe -= 1;

        let hparent = parent % m1;
        let vparent = parent / m1;

        coords.push((hparent, vparent));

        cnode = get_from_map(map, &parent);
        pos = cnode.pos;
        parent = cnode.parent;
        // println!("{}", a);
        // println!("{}", b);
    }

    if ssafe == 0 {
        panic!("{}", "Infinite cycle detected in alignment recostruction".red().bold());
    }

    coords.reverse();

    println!("\n\n{}", "Alignment completed".green());
    println!("{:?}", coords);

    for window in coords[..coords.len().min(100)].windows(2) {
        let [couple0, couple1] = window else { continue };
        if dependences.contains_key(&couple1.1) {
            println!("{:?} / {:?} Skipping rectangle (alternatives skipping)", couple0, couple1);
        } else if couple0.0 == couple1.0 {
            println!("{:?} / {:?} Same column (no computation needed)", couple0, couple1);
        } else if couple0.1 == couple1.1 {
            println!("{:?} / {:?} Same row (no computation needed)", couple0, couple1);
        } else {
            println!("{:?} / {:?} Submatrix (to be computed {} elements)", couple0, couple1, (couple1.0 - couple0.0) * (couple1.1 - couple0.1));
        }
    }

    coords
        .windows(2)
        .map(|w| (w[0], w[1]))
        .filter(|w| !dependences.contains_key(&w.1.1))
        .collect()
}


use std::fs::File;
use std::io::{BufWriter, Write};

pub fn write_file(v: &Value) -> std::io::Result<()> {
    let file = File::create("/tmp/alignment_tree.json")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, v)?;
    writer.flush()?;
    Ok(())
}










//TODO: To be removed, plain version of tree_prune is obtainable from tree_prune with two additional empty arrays
/* pub fn tree_prune_plain(w: usize, tree: &mut HashMap<usize, TreeNode>, protected: &usize, m1: &usize) {
    let mut current_node: usize = w;
    let mut children_num: usize = 100;

    // println!("tree_prune on {}", w);

    while current_node != *protected {
        let parent_id;

        // Primo riferimento mutabile: recuperiamo il nodo corrente
        {
            let n: &mut TreeNode = get_mut_from_map(tree, &current_node); // tree.get_mut(&current_node).expect(&format!("Key not found in tree: {}", current_node));
            // println!("      working on {:?}", n);

            // Se il nodo ha figli, non facciamo nulla e interrompiamo il ciclo
            if n.children.len() > 0 {
                children_num = n.children.len();
                break;
            }

            // Altrimenti, raccogliamo le informazioni sul nodo corrente
            parent_id = n.parent;
        }

        // let children_num: usize;
        {
            // Recuperiamo il genitore del nodo corrente
            let p: &mut TreeNode = get_mut_from_map(tree, &parent_id); // tree.get_mut(&parent_id).expect(&format!("Key not found in tree: {}", parent_id));
            
            // Rimuoviamo il nodo corrente dalla lista dei figli del genitore
            if let Some(pos) = p.children.iter().position(|x: &usize| *x == current_node) {
                p.children.swap_remove(pos);
            }

            // Verifichiamo se il genitore ha altri figli
            children_num = p.children.len();

            // Rimuoviamo il nodo corrente dall'albero
            tree.remove(&current_node);
            // println!("Elimino elemento {}", &current_node);
        }

        // Se il genitore non ha più figli, proseguiamo potando il genitore
        current_node = parent_id;
        // row = current_node / m1;

        if children_num > 0 {
            break;
        }
    }

    if current_node != *protected && children_num == 1 && current_node != 0 {
        let n: &mut TreeNode = get_mut_from_map(tree, &current_node);
        let vmov0 = (current_node % m1 - n.parent % m1) as i64;
        let vmov1 = (n.children[0] % m1 - n.pos % m1) as i64;
        let hmov0 = (current_node / m1) as i64 - (n.parent / m1) as i64;
        let hmov1 = (n.children[0] / m1) as i64 - (n.pos / m1) as i64;
        let diag = hmov0 == vmov0 && hmov1 == vmov1;
        let left = hmov0 == 0 && hmov1 == 0;
        let up = vmov0 == 0 && vmov1 == 0;
        if diag || left || up {
            // println!("Exited on {} and I can skip it (diag [to be extended]) {:?}", current_node, n);
            skip_node(current_node, tree);
        } else if TREE_MODE {
            // println!("Exited on {} and I can work on this node to skip it?", current_node);
            // * In questo caso c'è un "cambio di direzione". Se eliminiamo questo nodo arriviamo alla versione solo albero, senza percorsi completi.
            // La possiamo chiamare "tree mode"
            skip_node(current_node, tree);
        }
    } // Questo abilita la "nuova versione"
} */
