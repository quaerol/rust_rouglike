use rltk::RandomNumberGenerator;

// 生成表
// 随着leve 改变，物品和怪物的生成也会改变

pub struct RandomEntry {
    name: String,
    weight: i32,
}

impl RandomEntry {
    pub fn new(name: String, weight: i32) -> RandomEntry {
        RandomEntry {
            name: name.to_string(),
            weight,
        }
    }
}

#[derive(Default)]
pub struct RandomTable {
    entries: Vec<RandomEntry>,
    total_weight: i32,
}
impl RandomTable {
    pub fn new() -> RandomTable {
        RandomTable {
            entries: Vec::new(),
            total_weight: 0,
        }
    }

    // ignore entries with 0 or lower spawn chances, 忽略生成机会为0 或者更低的实体
    pub fn add<S: ToString>(mut self, name: S, weight: i32) -> RandomTable {
        self.total_weight += weight;
        self.entries
            .push(RandomEntry::new(name.to_string(), weight));
        self
    }
    // 得到一些随机的物品
    pub fn roll(&self, rng: &mut RandomNumberGenerator) -> String {
        if self.total_weight == 0 {
            return "None".to_string();
        }
        let mut roll = rng.roll_dice(1, self.total_weight) - 1;
        let mut index: usize = 0;

        while roll > 0 {
            if roll < self.entries[index].weight {
                return self.entries[index].name.clone();
            }
            roll -= self.entries[index].weight;
            index += 1;
        }
        "None".to_string()
    }
}
