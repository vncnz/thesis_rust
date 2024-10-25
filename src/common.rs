use std::collections::HashMap;
// use std::fs;
// use serde_json;
use itertools::Itertools;
use colored::*;

static TREE_MODE: bool = false;

#[derive(Debug)]
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
}
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
}

pub fn get_from_map<'a, K: Eq + std::hash::Hash + std::fmt::Debug, V>(map: &'a HashMap<K, V>, key: &K) -> &'a V {
    map.get(key).expect(&format!("Key {:?} not found in map", key))
}
pub fn get_mut_from_map<'a, K: Eq + std::hash::Hash + std::fmt::Debug, V>(map: &'a mut HashMap<K, V>, key: &K) -> &'a mut V {
    map.get_mut(key).expect(&format!("Key {:?} not found in map", key))
}

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
}

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

    if lines_to_keep.contains(&row) {
        // println!("Don't delete {}, This line needs to be kept: {} {:?}", current_node, &row, &lines_to_keep);
        return;
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
            if !dont_skip.contains(&(current_node / m1)) { skip_node(current_node, tree); }
        } else if TREE_MODE {
            // println!("Exited on {} and I can work on this node to skip it?", current_node);
            //* In questo caso c'è un "cambio di direzione". Se eliminiamo questo nodo arriviamo alla versione solo albero, senza percorsi completi.
            // La possiamo chiamare "tree mode"
            if !dont_skip.contains(&(current_node / m1)) { skip_node(current_node, tree); }
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

pub fn print_alignment_DEPRECATED(max_points_pos: usize, map: &HashMap<usize, TreeNode>, seq1: &str, seq2: &str, m1: usize, dependences: &HashMap<usize, Vec<usize>>) {
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
    // println!("print_alignment {:?}", cnode);
    while cnode.parent != cnode.pos {
        // println!("print_alignment {:?}", cnode);
        let mut p = cnode.pos;
        while p > cnode.parent {
            // let diff = &cnode.pos - &cnode.parent;
            // let near = diff == m1 || diff == m1 - 1 || diff == 1;
            let r = nodes_relationship(p, cnode.parent, m1);
            assert!(r.v || r.h || r.d, "Wrong relationship between {} and {}", p, cnode.parent);

            let mut row_shift = 1;
            let j = p / m1;
            if dependences.contains_key(&j) {
                let deps = get_from_map(&dependences, &j);
                println!("La riga {} è speciale: {:?}", j, deps);
                match deps.len() {
                    1 => {
                        row_shift = j-deps[0];
                    },
                    _ => {

                        // row_shift = j-deps[deps.len() - 1];
                    }
                }
            }
            if r.v {
                p = p - m1*row_shift;
                a.insert(0, '-');
                b.insert(0, seq2v[(p / m1) as usize]);
            }
            else if r.h {
                p = p - 1;
                a.insert(0, seq1v[p % m1]);
                b.insert(0, '-');
            }
            else if r.d {
                p = p - m1*row_shift - 1;
                a.insert(0, seq1v[p % m1]);
                b.insert(0, seq2v[(p / m1) as usize]);
            }
        }
        cnode = get_from_map(map, &cnode.parent); // &map[&cnode.parent];
        // println!("{:?}", &cnode);
    }
    println!("{}", a);
    println!("{}", b);
}

pub fn print_alignment(max_points_pos: usize, map: &HashMap<usize, TreeNode>, seq1: &str, seq2: &str, m1: usize, dependences: &HashMap<usize, Vec<usize>>) {
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

    let mut ssafe = seq1.len() * seq2.len() + 3;
    while ssafe > 0 && p > 0 {
        // println!("\n   ssafe={} p={} hmov={} vmov={} x={} y={} cnode={:?}", &ssafe, &p, &hmov, &vmov, p%m1, p/m1, &cnode);
        ssafe -= 1;

        // println!("Inserting x={} ({}) y={} ({})", p%m1, seq1v[p%m1 -1], p/m1, seq2v[p/m1 -1]);
        if vmov { b.insert(0, seq2v[(p / m1 -1) as usize]); }
        else { b.insert(0, '-'); }

        if hmov { a.insert(0, seq1v[p % m1 -1]); }
        else { a.insert(0, '-'); }

        if &p == &parent {
            cnode = get_from_map(map, &p);
            parent = cnode.parent;
            // println!("p={} parent={} node={:?}", &p, &parent, &cnode);
            hmov = p % m1 != parent % m1;
            vmov = p / m1 != parent / m1;
        }

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
        // println!("{}", a);
        // println!("{}", b);
    }

    if ssafe == 0 {
        panic!("{}", "Infinite cycle detected in alignment recostruction".red().bold());
    }

    println!("\n\n{}", "Alignment completed".green());
    println!("{}", a);
    println!("{}", b);
}