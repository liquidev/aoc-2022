//! The A* pathfinding algorithm.

use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

pub struct AStar<'a, Node> {
    pub start: Node,
    pub goal: Node,
    pub heuristic: &'a dyn Fn(&Node) -> f32,
    #[allow(clippy::type_complexity)]
    pub visit_neighbors: &'a dyn Fn(&Node, &mut dyn FnMut(&Node, f32)),
}

impl<'a, Node> AStar<'a, Node>
where
    Node: Debug + Clone + Eq + Hash + Ord,
{
    fn reconstruct_path(came_from: &HashMap<Node, Node>, mut current: Node) -> Vec<Node> {
        let mut total_path = vec![];
        while came_from.contains_key(&current) {
            current = came_from[&current].clone();
            total_path.push(current.clone());
        }
        total_path.reverse();
        total_path
    }

    pub fn find_path(self) -> Option<Vec<Node>> {
        let mut open_set = HashSet::new();
        open_set.insert(self.start.clone());
        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();
        g_score.insert(self.start.clone(), 0.0);
        let mut f_score = HashMap::new();
        f_score.insert(self.start.clone(), (self.heuristic)(&self.start));

        while !open_set.is_empty() {
            let current = open_set.iter().min().expect("no nodes in open_set").clone();
            if current == self.goal {
                return Some(Self::reconstruct_path(&came_from, current));
            }

            open_set.remove(&current);
            (self.visit_neighbors)(&current, &mut |neighbor, weight| {
                let tentative_g_score = g_score[&current] + weight;
                if tentative_g_score < g_score.get(neighbor).copied().unwrap_or(f32::INFINITY) {
                    came_from.insert(neighbor.clone(), current.clone());
                    g_score.insert(neighbor.clone(), tentative_g_score);
                    f_score.insert(
                        neighbor.clone(),
                        tentative_g_score + (self.heuristic)(neighbor),
                    );
                    open_set.insert(neighbor.clone());
                }
            });
        }

        None
    }
}
