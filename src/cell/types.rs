use rand::Rng;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CellType {
    Organic,
    Predator,
    Herbivore,
}

impl Into<String> for CellType {
    fn into(self) -> String {
        match self {
            CellType::Herbivore => "Herbivore".to_string(),
            CellType::Organic => "Organic".to_string(),
            CellType::Predator => "Predator".to_string(),
        }
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Action {
    MoveLeft,
    MoveRight,
    MoveTop,
    MoveBottom,
    Bite,
    Reproduce,
    DoNothing,
}

impl Action {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let v = rng.gen_range(0..6);
        match v {
            0 => Action::MoveRight,
            1 => Action::MoveLeft,
            2 => Action::MoveTop,
            3 => Action::MoveBottom,
            4 => Action::Bite,
            5 => Action::Reproduce,
            _ => Action::MoveRight,
        }
    }
}

#[derive(Debug)]
pub struct CellID {
    pub health: usize,
    pub kind: String
}
