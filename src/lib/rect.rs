// in rust ,when you make a new file in rust ,it automatically becomes a module.
// 2.3 we'll start making a map. Our goal is to randomly place rooms, and join them together with corridors(走廊).
#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct Rect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}
impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect {
            x1: x,
            x2: x + w,
            y1: y,
            y2: y + h,
        }
    }
    // returns true if this overlaps(重叠) with other ， intersect 交集
    // this method can't modify Rect  (it's a "pure" function).纯函数
    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.y1 <= other.y2 && self.x2 >= other.x1 && self.y2 >= other.y1
    }
    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}
