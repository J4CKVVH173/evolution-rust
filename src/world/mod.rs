mod types;

use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

use crate::cell::types::{Action, CellID, CellType};
use crate::cell::Cell;
use crate::map::types::Move;
use crate::map::Map;

use types::CleanUp;
use uuid::Uuid;

pub struct World {
    gen_idx: usize,
    gens_length: usize,
    // Ячейка хранятся в бинарном дереве в порядке своих ходов в пределах одной итерации
    order: BTreeMap<usize, Rc<Cell>>,
    directory: HashMap<Uuid, Rc<Cell>>,
    iter_count: usize,
    map: Map,
}

impl Default for World {
    fn default() -> Self {
        Self {
            gen_idx: 0,
            gens_length: 8,
            // дерево содержит все ячейки ячейки которые могут двигаться в порядке их движения за цикл
            order: BTreeMap::new(),
            // Внутренний справочник для быстрого получения информации о клетке
            directory: HashMap::new(),
            iter_count: 0,
            map: Map::default(),
        }
    }
}

impl World {
    pub fn populate_world(&mut self) {
        let square = self.map.get_square() as f64;
        let food_count = ((square / 100.0) * 25.0).round() as usize;
        let herbivore_count = ((square / 100.0) * 10.0).round() as usize;
        let predator_count = ((square / 100.0) * 2.0).round() as usize;
        for _ in 0..food_count {
            self.add_generated_cell(CellType::Organic);
        }
        for _ in 0..herbivore_count {
            self.add_generated_cell(CellType::Herbivore);
        }
        for _ in 0..predator_count {
            self.add_generated_cell(CellType::Predator);
        }

        self.clean_inactive_cells();
    }

    fn clean_inactive_cells(&mut self) {
        let mut inactive = vec![];
        for (key, cell) in self.order.iter() {
            if !cell.is_alive() {
                inactive.push(key.clone());
            }
        }
        for key in inactive {
            self.order.remove(&key);
        }
    }

    fn generate_gens(&self) -> Vec<Action> {
        let mut gens = vec![];
        for _ in 0..self.gens_length {
            gens.push(Action::random());
        }
        println!("{:?}", gens);
        gens
    }

    fn generate_herbivore(&self) -> Cell {
        Cell::new(8, self.generate_gens(), CellType::Herbivore)
    }

    fn generate_predator(&self) -> Cell {
        Cell::new(16, self.generate_gens(), CellType::Predator)
    }

    fn generate_food(&self) -> Cell {
        Cell::new(1, vec![], CellType::Organic)
    }

    pub fn add_generated_cell(&mut self, cell_type: CellType) {
        let new_cell = match cell_type {
            CellType::Herbivore => self.generate_herbivore(),
            CellType::Predator => self.generate_predator(),
            CellType::Organic => self.generate_food(),
        };
        self.map.throw_cell_to_map(new_cell.get_id());
        self.add_cell(new_cell);
    }

    fn add_cell(&mut self, new_cell: Cell) {
        let r_cell = Rc::new(new_cell);
        if self.order.is_empty() {
            // Если это первая ячейка во всем пол, то устанавливаем ее в начало
            self.order.insert(0, r_cell.clone());
        } else {
            // Если это не первая ячейка, то просто добавляем ее в конец
            let max = self.order.iter().next_back().unwrap();
            let idx = max.0.clone();
            self.order.insert(idx + 1, r_cell.clone());
        };
        self.directory.insert(r_cell.get_id(), r_cell);
    }

    fn remove_cell(&mut self, clean_up: CleanUp) {
        self.map.remove_cell(&clean_up.0);

        self.directory.remove(&clean_up.0);
        self.order.remove(&clean_up.1);
    }

    pub fn get_map(&self) -> Vec<Vec<Option<CellID>>> {
        let mut map = vec![];
        let raw_map = self.map.get_map().clone();
        for row in raw_map {
            let mut new_row: Vec<Option<CellID>> = vec![];
            for spot in row {
                match spot {
                    Some(uuid) => new_row.push(Some(self.directory[&uuid].as_ref().clone().into())),
                    None => new_row.push(None),
                }
            }
            map.push(new_row);
        }
        map
    }

    // Метод проводит одну итерацию
    pub fn run(&mut self) {
        let mut clean_up = vec![];
        let mut new_cells = vec![];
        for (order_key, cell) in self.order.iter() {
            let get_move = cell.get_move_info(self.gen_idx);

            match get_move {
                Action::MoveBottom => {
                    self.map.move_to(cell.get_id(), Move::Bottom);
                }
                Action::MoveLeft => {
                    self.map.move_to(cell.get_id(), Move::Left);
                }
                Action::MoveRight => {
                    self.map.move_to(cell.get_id(), Move::Right);
                }
                Action::MoveTop => {
                    self.map.move_to(cell.get_id(), Move::Top);
                }
                Action::Bite => {
                    let coordinates = self.map.get_cell_coordinates(&cell.get_id());
                    let around = self.map.look_around(coordinates);
                    let mut a: Vec<CellID> = vec![];
                    for m in around {
                        let b = self.directory.get(&m).unwrap();
                        a.push(b.as_ref().clone().into())
                    }
                    let id: CellID = cell.as_ref().clone().into();
                }
                Action::Reproduce => {
                    let child = cell.make_child();
                    // Если ребенок родился
                    if child.is_some() {
                        let child = child.unwrap();
                        let parent_coordinates = self.map.get_cell_coordinates(&cell.get_id());
                        new_cells.push((child, parent_coordinates));
                    }
                }
                Action::DoNothing => {}
            }

            cell.heat();

            // Записываем ячейки, которые должны быть вычищены с поля
            if cell.is_dead() {
                clean_up.push(CleanUp(cell.get_id(), order_key.clone()));
            }
        }

        for clean in clean_up {
            self.remove_cell(clean);
        }

        for (new_cell, birth_spot) in new_cells {
            if self.map.set_around(new_cell.get_id(), birth_spot) {
                self.add_cell(new_cell);
            }
        }

        self.gen_idx += 1;
        if self.gen_idx >= self.gens_length {
            self.gen_idx = 0;
            self.iter_count += 1;
        };
    }
}
