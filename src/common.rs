use std::collections::HashMap;
// use std::fs;
// use serde_json;
use itertools::Itertools;
use colored::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub static TREE_MODE: bool = false;

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

pub fn tree_prune(w: usize, tree: &mut HashMap<usize, TreeNode>, protected: &usize, n1: &usize, lines_to_keep: &Vec<usize>, dont_skip: &Vec<usize>) {

    let mut current_id: usize = w;
    let mut children_num: usize = 100;
    let mut row = &current_id / n1;

    while current_id != *protected && !lines_to_keep.contains(&row) {
        let parent_id;

        {
            let n: &mut TreeNode = get_mut_from_map(tree, &current_id);

            // Se il nodo ha figli, non facciamo nulla e interrompiamo il ciclo
            if n.children.len() > 0 {
                children_num = n.children.len();
                break;
            }

            // Altrimenti, ci salviamo il nodo del parent e continuiamo
            parent_id = n.parent;
        }

        {
            // Recuperiamo il genitore del nodo corrente
            let p: &mut TreeNode = get_mut_from_map(tree, &parent_id);
            
            // Rimuoviamo il nodo corrente dalla lista dei figli del genitore
            if let Some(pos) = p.children.iter().position(|x: &usize| *x == current_id) {
                p.children.swap_remove(pos);
            }

            // Recuperiamo il numero di figli del padre
            children_num = p.children.len();

            // Rimuoviamo il nodo corrente dall'albero
            tree.remove(&current_id);
        }

        // Se il genitore non ha più figli, proseguiamo potando il genitore
        current_id = parent_id;
        row = current_id / n1;

        if children_num > 0 {
            break;
        }
    }

    let n: &mut TreeNode = get_mut_from_map(tree, &current_id);
    if dont_skip.contains(&(&n.parent / n1)) {
        println!("Don't delete {}, This line needs to be kept: {} {:?}", current_id, &row, &lines_to_keep);
        return;
    }

    // Questo if abilita la "seconda ottimizzazione", se lo togliamo otteniamo percorsi con tutti i nodi intermedi
    if current_id != *protected && children_num == 1 && current_id != 0 && !dont_skip.contains(&(current_id / n1)) {
        if TREE_MODE {
            // Se eliminiamo questo nodo indipendentemente dall'eventuale cambio direzione arriviamo alla versione solo albero, senza percorsi completi.
            skip_node(current_id, tree);
            //}
        } else {
            // In questo caso eliminiamo il nodo solo se il genitore e il figlio sono nella stessa colonna e nella stessa riga
            let vmov0 = (current_id % n1 - n.parent % n1) as i64;
            let vmov1 = (n.children[0] % n1 - n.pos % n1) as i64;
            let hmov0 = (current_id / n1) as i64 - (n.parent / n1) as i64;
            let hmov1 = (n.children[0] / n1) as i64 - (n.pos / n1) as i64;
            let diag = hmov0 == vmov0 && hmov1 == vmov1;
            let left = hmov0 == 0 && hmov1 == 0;
            let up = vmov0 == 0 && vmov1 == 0;
            if diag || left || up {
                skip_node(current_id, tree);
            }
        }
    }
}

pub fn skip_node (w: usize, tree: &mut HashMap<usize, TreeNode>) {

    let w0: usize;
    let w2: usize;

    // Recuperiamo i nodi di nodo genitore e nodo figlio (unico)
    let n: &mut TreeNode = get_mut_from_map(tree, &w);
    w0 = n.parent;
    w2 = n.children[0];
    let n0: &mut TreeNode = get_mut_from_map(tree, &w0);

    // Sostituiamo il nodo corrente dalla lista dei figli del genitore
    if let Some(pos) = n0.children.iter().position(|x: &usize| *x == w) {
        n0.children[pos] = w2;
    }

    // Nel nodo "nipote" sostituiamo il genitore mettendoci il "nonno"
    let n2: &mut TreeNode = get_mut_from_map(tree, &w2);
    n2.parent = w0;

    // Rimuoviamo il nodo corrente dall'albero
    tree.remove(&w);
}

pub fn recostruct_alignment(max_points_pos: usize, map: &HashMap<usize, TreeNode>, seq_s: &str, seq_t: &str, n1: usize, dependences: &HashMap<usize, Vec<usize>>) -> Vec<(usize, usize)> {

    let vec_s: Vec<char> = seq_s.chars().collect();
    let vec_t: Vec<char> = seq_t.chars().collect();
    let end_pos = (vec_s.len() + 1) * (vec_t.len() + 1) - 1;
    let mut a: String = "".to_owned();
    let mut b: String = "".to_owned();
    let mut cnode = &TreeNode { pos: end_pos, parent: max_points_pos, children: [].to_vec(), depth: 0, points: 0 };
    if end_pos == max_points_pos {
        cnode = get_from_map(map, &end_pos);
    }

    let mut hmov = cnode.pos % n1 != cnode.parent % n1;
    let mut vmov = cnode.pos / n1 != cnode.parent / n1;
    let mut p = cnode.pos;
    let mut parent = cnode.parent;

    let mut coords = vec![];

    let mut ssafe = seq_s.len() * seq_t.len() + 3;
    while ssafe > 0 && p > 0 {
        // println!("\n   ssafe={} p={} hmov={} vmov={} x={} y={} cnode={:?}", &ssafe, &p, &hmov, &vmov, p%m1, p/m1, &cnode);
        ssafe -= 1;

        // println!("Indeces x={} ({}) y={} ({})", p%m1, seq1v[p%m1 -1], p/m1, seq2v[p/m1 -1]);
        if vmov { b.insert(0, vec_t[(p / n1 -1) as usize]); }
        else { b.insert(0, '-'); }

        if hmov { a.insert(0, vec_s[p % n1 -1]); }
        else { a.insert(0, '-'); }

        let row_number = p / n1;
        
        if dependences.contains_key(&(row_number)) {
            p = cnode.parent; // In fase di costruzione albero mi garantisco la presenza di un nodo nell'ultima riga dell'alternativa scelta
            // println!("{} for row_number {} {:?}", "Using parent".yellow(), &row_number, &dependences);
        } else {
            // println!("{}", "NOT using parent".cyan());
            if vmov  { p = p - n1; }
            if hmov { p = p - 1; }
        }

        coords.push((p % n1, p / n1));

        if &p == &parent {
            cnode = get_from_map(map, &p);
            parent = cnode.parent;
            // println!("p={} parent={} node={:?}", &p, &parent, &cnode);
            hmov = p % n1 != parent % n1;
            vmov = p / n1 != parent / n1;
        }
    }

    if ssafe == 0 {
        panic!("{}", "Infinite cycle detected in alignment recostruction".red().bold());
    }

    coords.reverse();

    // println!("{:?}", coords.iter().map(|x| { x.1 * n1 + x.0 }).collect::<Vec<(usize)>>());

    println!("\n\n{}", "Alignment completed".green());
    // println!("{:?}", coords);
    if a.len() < 50 {
        println!("{}", a);
        println!("{}", b);
    }

    // Restituisco la lista di coordinate dei nodi del percorso
    coords
}

pub fn recostruct_subproblems(max_points_pos: usize, map: &HashMap<usize, TreeNode>, seq_s: &str, seq_t: &str, n1: usize, dependences: &HashMap<usize, Vec<usize>>) -> Vec<((usize, usize), (usize, usize))> {
    // ! Da verificare meglio per la versione de-strings!
    let end_pos = (seq_s.len() + 1) * (seq_t.len() + 1) - 1;

    let mut cnode = &TreeNode { pos: end_pos, parent: max_points_pos, children: [].to_vec(), depth: 0, points: 0 };
    if end_pos == max_points_pos {
        cnode = get_from_map(map, &end_pos);
    }

    let mut pos = cnode.pos;
    let mut parent = cnode.parent;

    let mut coords = vec![];

    let mut ssafe = seq_s.len() * seq_t.len() + 3;

    coords.push((end_pos % n1, end_pos / n1));

    while ssafe > 0 && pos > 0 {
        ssafe -= 1;

        let hparent = parent % n1;
        let vparent = parent / n1;

        // println!("par={} ({},{})   pos={} ({},{})  cnode={:?}", &pos, pos%n1, pos/n1, &parent, &hparent, &vparent, &cnode);

        coords.push((hparent, vparent));

        cnode = get_from_map(map, &parent);
        pos = cnode.pos;
        parent = cnode.parent;
    }

    if ssafe == 0 {
        panic!("{}", "Infinite cycle detected in alignment recostruction".red().bold());
    }

    coords.reverse();

    println!("\n\n{}", "Alignment completed".green());
    println!("{:?}", coords);
    println!("{:?}", coords.iter().map(|x| { x.1 * n1 + x.0 }).collect::<Vec<(usize)>>());

    println!("{} (len {})", &seq_s, seq_s.len());
    println!("{} (len {})", &seq_t, seq_t.len());

    let mut s = "".to_string();
    let mut t = "".to_string();

    for window in coords[..coords.len().min(100)].windows(2) {
        let [couple0, couple1] = window else { continue };
        if dependences.contains_key(&couple1.1) {
            println!("{:?} / {:?} Skipping rectangle (alternatives skipping)", couple0, couple1);
        } else if couple0.0 == couple1.0 {
            println!("{:?} / {:?} Same column (no computation needed)", couple0, couple1);
            for x in couple0.1..couple1.1 {
                s += "-";
                t += seq_t.chars().nth(x).unwrap().to_string().as_str();
            }
        } else if couple0.1 == couple1.1 {
            println!("{:?} / {:?} Same row (no computation needed)", couple0, couple1);
            for x in couple0.0..couple1.0 {
                s += seq_s.chars().nth(x).unwrap().to_string().as_str();
                t += "-";
            }
        } else {
            println!("{:?} / {:?} Submatrix (to be computed {} elements)", couple0, couple1, (couple1.0 - couple0.0 + 1) * (couple1.1 - couple0.1 + 1));
            s += "[";
            for x in couple0.0..couple1.0 {
                s += seq_s.chars().nth(x).unwrap().to_string().as_str();
            }
            s += "]";
            t += "[";
            for x in couple0.1..couple1.1 {
                t += seq_t.chars().nth(x).unwrap().to_string().as_str();
            }
            t += "]";

        }
    }

    if s.len() < 50 && t.len() < 50 {
        println!("{}", &s);
        println!("{}", &t);
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
