//! แคมป์พ่อค้า: สุ่มสร้างแบบกำหนดแน่นอนจาก seed (ไม่ต้องเก็บลงเซฟ) + รายการสินค้า

use super::*;

/// ขนาดเซลล์ของโลกที่ใช้สุ่มแคมป์ (ไทล์)
pub const CELL: i32 = 64;

pub struct Offer {
    pub name: &'static str,
    pub desc: &'static str,
    pub item: usize,
    pub qty: i32,
    pub price: i32,
    /// พิมพ์เขียวซื้อได้ครั้งเดียว
    pub once: bool,
}
pub const OFFERS: [Offer; 6] = [
    Offer { name: "ยาฟื้นฟู", desc: "ฟื้นเลือด 8 หน่วย", item: POTION, qty: 1, price: 10, once: false },
    Offer { name: "ยาฟื้นฟู x3", desc: "ฟื้นเลือด 8 หน่วย (ถูกกว่า)", item: POTION, qty: 3, price: 27, once: false },
    Offer { name: "ลูกธนู x10", desc: "กระสุนสำหรับธนู", item: ARROW, qty: 10, price: 8, once: false },
    Offer { name: "ลูกธนู x30", desc: "กระสุนสำหรับธนู (คุ้มกว่า)", item: ARROW, qty: 30, price: 22, once: false },
    Offer { name: "พิมพ์เขียวเกราะหายาก", desc: "ปลดล็อกเกราะระดับสูงสุด (ใช้คู่กับของบอส)", item: BP_ARM, qty: 1, price: 70, once: true },
    Offer { name: "พิมพ์เขียวอาวุธหายาก", desc: "ปลดล็อกอาวุธระดับสูงสุด (ใช้คู่กับของบอส)", item: BP_WPN, qty: 1, price: 90, once: true },
];
pub const GREETS: [&str; 4] = [
    "ยินดีต้อนรับนักเดินทาง! ดูของก่อนได้เลย",
    "เหรียญจากมอนสเตอร์ก็ใช้ได้นะ ราคามิตรภาพ!",
    "ทางข้างหน้าอันตราย เตรียมยาไว้ก็ดี",
    "ถ้าเจอบอส อย่าลืมมาหาพิมพ์เขียวที่นี่",
];

impl World {
    /// ศูนย์กลางแคมป์ในเซลล์ (จำผลไว้ใน memo)
    pub fn camp_in_cell(&self, cx: i32, cy: i32) -> Option<(i32, i32)> {
        if let Some(&r) = self.camps.borrow().get(&(cx, cy)) {
            return r;
        }
        let r = self.calc_camp(cx, cy);
        self.camps.borrow_mut().insert((cx, cy), r);
        r
    }
    fn calc_camp(&self, cx: i32, cy: i32) -> Option<(i32, i32)> {
        let s = self.seed.wrapping_add(4242);
        if hash(cx, cy, s) > 0.45 {
            return None;
        }
        let tx = cx * CELL + 8 + (hash(cx, cy, s.wrapping_add(1)) * (CELL - 16) as f64) as i32;
        let ty = cy * CELL + 8 + (hash(cx, cy, s.wrapping_add(2)) * (CELL - 16) as f64) as i32;
        // พื้นที่ 7x5 ต้องเป็นพื้นดินเดินได้ทั้งหมด
        for dy in -2..=2 {
            for dx in -3..=3 {
                if SOLIDT[self.tile_at(tx + dx, ty + dy) as usize] {
                    return None;
                }
            }
        }
        Some((tx, ty))
    }
    /// วัตถุของแคมป์ที่ไทล์นี้ (Some(0) = พื้นที่โล่งของแคมป์)
    pub fn camp_obj(&self, tx: i32, ty: i32) -> Option<u8> {
        let (mx, my) = self.camp_in_cell(tx.div_euclid(CELL), ty.div_euclid(CELL))?;
        let (dx, dy) = (tx - mx, ty - my);
        if dx.abs() > 3 || dy.abs() > 2 {
            return None;
        }
        Some(match (dx, dy) {
            (0, 0) => 15,
            (2, -1) | (-2, -1) => 16,
            _ => 0,
        })
    }
    /// แคมป์ที่อยู่ในระยะ r ไทล์ (สี่เหลี่ยม)
    pub fn camp_near(&self, tx: i32, ty: i32, r: i32) -> Option<(i32, i32)> {
        for cy in (ty - r).div_euclid(CELL)..=(ty + r).div_euclid(CELL) {
            for cx in (tx - r).div_euclid(CELL)..=(tx + r).div_euclid(CELL) {
                if let Some((mx, my)) = self.camp_in_cell(cx, cy) {
                    if (mx - tx).abs() <= r && (my - ty).abs() <= r {
                        return Some((mx, my));
                    }
                }
            }
        }
        None
    }
}

impl Game {
    /// ค้นพบแคมป์เมื่อเดินเข้าใกล้ -> แสดงสัญลักษณ์ M บนแผนที่
    pub fn discover_camps(&mut self) {
        let (tx, ty) = (tfloor(self.p.x), tfloor(self.p.y));
        if let Some(c) = self.w.camp_near(tx, ty, 34) {
            if self.camps_found.insert(c) {
                self.toast("พบแคมป์พ่อค้า! ดูตำแหน่ง (M) บนแผนที่ใหญ่");
                self.p.xp += 2;
                self.check_lv();
                self.save();
            }
        }
    }
    pub fn buy(&mut self, o: usize) {
        let of = &OFFERS[o];
        if of.once && self.p.inv[of.item] > 0 {
            self.toast("มีพิมพ์เขียวนี้แล้ว");
            return;
        }
        if self.p.inv[COIN] < of.price {
            self.toast("เหรียญไม่พอ");
            return;
        }
        self.p.inv[COIN] -= of.price;
        self.p.inv[of.item] += of.qty;
        play(&self.sfx.buy);
        self.toast(format!("ซื้อ {} สำเร็จ", of.name));
        self.save();
    }
}
