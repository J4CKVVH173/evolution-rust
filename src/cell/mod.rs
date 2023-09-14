pub mod types;

use std::{cell::RefCell, hash::Hash};

use uuid::Uuid;

use types::{Action, CellType};

use self::types::CellID;

#[derive(Debug, PartialEq, Clone)]
pub struct Cell {
    cell_type: CellType,
    health: RefCell<usize>,
    gens: Vec<Action>,
    id: Uuid,
}

impl Hash for Cell {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.cell_type.hash(state);
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            health: RefCell::new(8),
            gens: vec![],
            id: Uuid::new_v4(),
            cell_type: CellType::Herbivore,
        }
    }
}

impl Into<CellID> for Cell {
    fn into(self) -> CellID {
        CellID {
            health: self.health.borrow().clone(),
            kind: self.cell_type.into(),
        }
    }
}

impl Cell {
    pub fn new(health: usize, gens: Vec<Action>, cell_type: CellType) -> Self {
        Self {
            cell_type,
            gens,
            health: RefCell::new(health),
            id: Uuid::new_v4(),
        }
    }
    pub fn set_gens(&mut self, gens: Vec<Action>) {
        self.gens = gens;
    }

    pub fn heat(&self) {
        *self.health.borrow_mut() -= 1;
    }

    pub fn heal(&mut self, hp_point: Option<usize>) {
        let mut health = *self.health.borrow_mut();
        match hp_point {
            Some(hp) => {
                health += hp;
            }
            None => {
                health += 1;
            }
        }
    }

    pub fn is_dead(&self) -> bool {
        *self.health.borrow() < 1
    }

    // Метод определяет клетку как живую или не живую с
    // биологической точки зрения (животное - живое, растение нет)
    pub fn is_alive(&self) -> bool {
        self.cell_type != CellType::Organic
    }

    // В зависимости от текущего тика игрового мира, клетка сообщает какое действие
    // она пытается выполнить
    pub fn get_move_info(&self, gen_idx: usize) -> Action {
        if self.cell_type == CellType::Organic {
            return Action::DoNothing;
        }
        self.gens[gen_idx]
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn make_child(&self) -> Option<Cell> {
        let mut health = *self.health.borrow_mut();
        if health > 1 {
            health /= 2;
            return Some(Cell::new(health.clone(), self.gens.clone(), self.cell_type));
        }
        return None;
    }
}
