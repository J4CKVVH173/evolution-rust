pub mod types;

use rand::Rng;
use std::collections::HashMap;

use types::Move;
use uuid::Uuid;

pub struct Map {
    // Размер карты NxM
    size: [usize; 2],
    // Карта, из которой можно получить положение любой ячейки
    map: HashMap<Uuid, [usize; 2]>,
    // Карта которая предоставляет доступ к ячейкам по координатам карты
    coordinates_map: HashMap<[usize; 2], Uuid>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            size: [16, 16],
            map: HashMap::new(),
            coordinates_map: HashMap::new(),
        }
    }
}

impl Map {
    pub fn new(size: [usize; 2]) -> Self {
        Self {
            size,
            map: HashMap::new(),
            coordinates_map: HashMap::new(),
        }
    }
    fn is_empty_spot(&self, coordinates: &[usize; 2]) -> bool {
        !self.coordinates_map.contains_key(coordinates)
    }

    fn set_cell(&mut self, cell_id: Uuid, coordinates: [usize; 2]) {
        self.map.insert(cell_id, coordinates.clone());
        self.coordinates_map.insert(coordinates, cell_id);
    }

    fn shift_cell(&mut self, cell_id: Uuid, coordinates: [usize; 2]) {
        let prev_coordinates = self.map.get(&cell_id);
        match prev_coordinates {
            Some(prev) => {
                self.coordinates_map.remove(prev);
            }
            None => {}
        }
        self.set_cell(cell_id, coordinates);
    }

    pub fn remove_cell(&mut self, cell_id: &Uuid) {
        let coordinates = self.map.get(cell_id);
        if coordinates.is_some() {
            let coordinates = coordinates.unwrap().clone();
            self.map.remove(cell_id);
            self.coordinates_map.remove(&coordinates);
        }
    }

    // Метод передвигает клетку в указанном направление
    // передвижение будет осуществленно только в том случа, если
    // место для клетки свободно
    pub fn move_to(&mut self, cell_id: Uuid, direction: Move) {
        let coordinates = self.map.get(&cell_id).unwrap();
        let mut new_coordinates = [0, 0];
        match direction {
            Move::Bottom => {
                let new_y;
                if coordinates[1] == 0 {
                    new_y = self.size[1] - 1;
                } else {
                    new_y = coordinates[1] - 1;
                }
                new_coordinates = [coordinates[0], new_y];
            }
            Move::Top => {
                let new_y;
                if coordinates[1] + 1 == self.size[1] {
                    new_y = 0;
                } else {
                    new_y = coordinates[1] + 1;
                }
                new_coordinates = [coordinates[0], new_y];
            }
            Move::Left => {
                let new_x;
                if coordinates[0] == 0 {
                    new_x = self.size[0] - 1;
                } else {
                    new_x = coordinates[0] - 1;
                }
                new_coordinates = [new_x, coordinates[1]];
            }
            Move::Right => {
                let new_x;
                if coordinates[0] + 1 == self.size[0] {
                    new_x = 0;
                } else {
                    new_x = coordinates[0] + 1;
                }
                new_coordinates = [new_x, coordinates[1]];
            }
        }

        if self.is_empty_spot(&new_coordinates) {
            self.shift_cell(cell_id, new_coordinates);
        }
    }

    pub fn get_cell_coordinates(&self, id: &Uuid) -> [usize; 2] {
        self.map.get(id).unwrap().clone()
    }

    // На карту забрасывается клетка в случайное место
    pub fn throw_cell_to_map(&mut self, cell_id: Uuid) {
        let mut rng = rand::thread_rng();
        let mut x = rng.gen_range(0..self.size[0]);
        let mut y = rng.gen_range(0..self.size[1]);
        let mut coordinates = [x, y];
        while !self.is_empty_spot(&coordinates) {
            x = rng.gen_range(0..self.size[0]);
            y = rng.gen_range(0..self.size[1]);
            coordinates = [x, y];
        }
        self.shift_cell(cell_id, coordinates);
    }

    // Метод пытается установить ячейку вокруг указанной точки
    // Возвращает тру если это удалось
    pub fn set_around(&mut self, uuid: Uuid, dot: [usize; 2]) -> bool {
        let mut try_x: isize = 0;
        let mut try_y: isize = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                try_x = dot[0] as isize + i;
                try_y = dot[1] as isize + j;
                if try_x == dot[0] as isize && try_y == dot[1] as isize {
                    continue;
                }
                if try_x < 0 || try_x >= self.size[0] as isize {
                    continue;
                }
                if try_y < 0 || try_y >= self.size[1] as isize {
                    continue;
                }
                let coordinates = [try_x as usize, try_y as usize];
                if self.is_empty_spot(&coordinates) {
                    self.set_cell(uuid, coordinates);
                    return true;
                }
            }
        }
        false
    }

    // Метод возвращает все клетки, которые окружают указанную точку
    pub fn look_around(&self, sport: [usize; 2]) -> Vec<Uuid> {
        let mut try_x: isize = 0;
        let mut try_y: isize = 0;
        let mut result = vec![];
        for i in -1..=1 {
            for j in -1..=1 {
                try_x = sport[0] as isize + i;
                try_y = sport[1] as isize + j;
                if try_x == sport[0] as isize && try_y == sport[1] as isize {
                    continue;
                }
                if try_x < 0 || try_x >= self.size[0] as isize {
                    continue;
                }
                if try_y < 0 || try_y >= self.size[1] as isize {
                    continue;
                }
                let coordinates = [try_x as usize, try_y as usize];
                if !self.is_empty_spot(&coordinates) {
                    result.push(self.coordinates_map[&coordinates]);
                }
            }
        }
        result
    }

    pub fn get_map(&self) -> Vec<Vec<Option<Uuid>>> {
        let mut map = Vec::new();
        for y in 0..self.size[0] {
            let mut row = Vec::new();
            for x in 0..self.size[1] {
                if self.is_empty_spot(&[x, y]) {
                    row.push(None);
                } else {
                    row.push(Some(self.coordinates_map.get(&[x, y]).unwrap().clone()));
                }
            }
            map.push(row);
        }
        map
    }

    pub fn get_square(&self) -> usize {
        self.size[0] * self.size[1]
    }

    // Метод заполняет карту камнями
    fn fill_rocks(&mut self) {
        todo!()
    }
}
