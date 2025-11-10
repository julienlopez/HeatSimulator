use std::collections::HashMap;

pub enum Item {
    Heater { power: f64, position: (u64, u64) },
    Pipe { position: (u64, u64) },
}

impl Item {
    pub fn position(&self) -> &(u64, u64) {
        match self {
            Item::Heater { position, .. } => position,
            Item::Pipe { position } => position,
        }
    }
}

pub type Index = u64;

#[derive(Default)]
pub struct Infrastructure {
    pub elements: HashMap<Index, Item>,
}

impl Infrastructure {
    pub fn new(elems: Vec<Item>) -> Self {
        let mut elements = HashMap::new();
        for (i, elem) in elems.into_iter().enumerate() {
            elements.insert(i as Index, elem);
        }
        Self { elements }
    }

    pub fn add_item(&mut self, item: Item) -> Index {
        let id = self.elements.len() as u64;
        self.elements.insert(id, item);
        id
    }

    pub fn adjacent_items(&self, item: Index) -> Vec<Index> {
        match self.elements.iter().find(|(i, _)| **i == item) {
            Some((id, this_item)) => {
                let mut adjacent = vec![];
                for (other_id, other_item) in &self.elements {
                    if other_id != id && other_item.position() == this_item.position() {
                        // Placeholder logic for adjacency
                        adjacent.push(*other_id);
                    }
                }
                adjacent
            }
            None => vec![],
        }
    }
}
