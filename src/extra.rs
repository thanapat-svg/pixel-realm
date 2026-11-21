//! เสาวาร์ป (Fast Travel) · หม้อทำอาหาร/ปรุงยา · บัฟชั่วคราว

use super::*;

// ======================= เสาวาร์ป =======================
/// ขนาดเซลล์ของโลกที่ใช้สุ่มเสาวาร์ป (ไทล์)
pub const WPC: i32 = 96;

impl World {
    pub fn wp_in_cell(&self, cx: i32, cy: i32) -> Option<(i32, i32)> {
        if self.dun.is_some() {
            return None;
        }
        if let Some(&r) = self.wps.borrow().get(&(cx, cy)) {
            return r;
        }
        let r = self.calc_wp(cx, cy).filter(|p| !self.near_city(p.0, p.1, 3));
        self.wps.borrow_mut().insert((cx, cy), r);
        r
    }
    fn calc_wp(&self, cx: i32, cy: i32) -> Option<(i32, i32)> {
        let s = self.seed.wrapping_add(6161);
        if hash(cx, cy, s) > 0.6 {
            return None;
        }
        for k in 0..10 {
            let tx = cx * WPC + 6 + (hash(cx, cy, s.wrapping_add(1 + k * 2)) * (WPC - 12) as f64) as i32;
            let ty = cy * WPC + 6 + (hash(cx, cy, s.wrapping_add(2 + k * 2)) * (WPC - 12) as f64) as i32;
            let mut ok = matches!(self.tile_at(tx, ty), 2 | 3 | 4 | 5 | 7 | 8);
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if SOLIDT[self.tile_at(tx + dx, ty + dy) as usize] {
                        ok = false;
                    }
                }
            }
            if ok && self.camp_near(tx, ty, 6).is_none() {
                return Some((tx, ty));
            }
        }
        None
    }
    pub fn wp_near(&self, tx: i32, ty: i32, r: i32) -> Option<(i32, i32)> {
        for cy in (ty - r).div_euclid(WPC)..=(ty + r).div_euclid(WPC) {
            for cx in (tx - r).div_euclid(WPC)..=(tx + r).div_euclid(WPC) {
                if let Some((mx, my)) = self.wp_in_cell(cx, cy) {
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
    pub fn discover_wps(&mut self) {
        if self.w.dun.is_some() {
            return;
        }
        let (tx, ty) = (tfloor(self.p.x), tfloor(self.p.y));
        if let Some(c) = self.w.wp_near(tx, ty, 10) {
            if self.wp_found.insert(c) {
                let b = TN[self.w.tile_at(c.0, c.1) as usize];
                self.toast(format!("ปลดล็อกเสาวาร์ป ({})! เปิดแผนที่แล้วแตะ W เพื่อวาร์ป", b));
                play(&self.sfx.quest);
                self.p.xp += 3;
                self.check_lv();
                self.save();
            }
        }
    }
    /// จุดวาร์ปทั้งหมดที่ใช้ได้: (tx, ty, เป็นจุดเกิดหรือไม่)
    pub fn wp_list(&self) -> Vec<(i32, i32, bool)> {
        let mut v: Vec<(i32, i32, bool)> = self.wp_found.iter().map(|&(x, y)| (x, y, false)).collect();
        v.push((tfloor(self.spawn.0), tfloor(self.spawn.1), true));
        v
    }
    pub fn warp_to(&mut self, tx: i32, ty: i32, spawn: bool) {
        if self.w.dun.is_some() {
            self.toast("วาร์ปในดันเจี้ยนไม่ได้");
            return;
        }
        let (mut nx, mut ny) = if spawn { (self.spawn.0, self.spawn.1) } else { (tx as f32 * 16. + 8., (ty + 1) as f32 * 16. + 8.) };
        if !spawn {
            'f: for (dx, dy) in [(0, 1), (1, 1), (-1, 1), (1, 0), (-1, 0), (0, -1), (2, 1), (-2, 1)] {
                if !self.w.solid_at(tx + dx, ty + dy) {
                    nx = (tx + dx) as f32 * 16. + 8.;
                    ny = (ty + dy) as f32 * 16. + 8.;
                    break 'f;
                }
            }
        }
        let (ox, oy) = (self.p.x, self.p.y - 8.);
        self.burst(ox, oy, hx(0x6ae8ff), 24, 90.);
        self.p.x = nx;
        self.p.y = ny;
        self.en.clear();
        self.proj.clear();
        self.drops.clear();
        self.tp_comp();
        self.p.invul = self.p.invul.max(1.0);
        self.burst(nx, ny - 8., hx(0xc8a0ff), 28, 110.);
        self.shake(0.8, 0.2);
        play(&self.sfx.upgrade);
        self.toast(if spawn { "วาร์ปกลับจุดเกิด".to_string() } else { format!("วาร์ปไปเสา ({})", TN[self.w.tile_at(tx, ty) as usize]) });
    }
}

// ======================= ทำอาหาร / ปรุงยา =======================
pub struct Dish {
    pub name: &'static str,
    pub desc: &'static str,
    pub item: usize,
    pub cost: &'static [(usize, i32)],
}
pub const DISHES: [Dish; 5] = [
    Dish { name: "สเต็กเนื้อย่าง", desc: "ฟื้นเลือด 14 หน่วย", item: DISH_STEW, cost: &[(MEAT, 2), (HERB, 1)] },
    Dish { name: "ซุปเห็ดสายลม", desc: "วิ่งเร็วขึ้น +25% นาน 2 นาที", item: DISH_SPEED, cost: &[(SHROOM, 2), (BERRY, 1)] },
    Dish { name: "ชาสมุนไพรคมเขี้ยว", desc: "คริติคอล +20% นาน 2 นาที", item: DISH_CRIT, cost: &[(HERB, 2), (SHROOM, 1)] },
    Dish { name: "ปลาย่าง", desc: "ฟื้นเลือด 10 หน่วย (ใช้ปลา 2 ตัว)", item: DISH_FISH, cost: &[(FISH, 2)] },
    Dish { name: "ปรุงยาฟื้นฟู", desc: "ยาฟื้นเลือด 8 หน่วย (กด H)", item: POTION, cost: &[(HERB, 2), (BERRY, 1)] },
];
pub const BUFF_T: f32 = 120.;

impl Game {
    pub fn cook(&mut self, i: usize) {
        let d = &DISHES[i];
        if !can_pay(&self.p, d.cost) {
            play(&self.sfx.deny);
            self.toast("วัตถุดิบไม่พอ");
            return;
        }
        for &(it, n) in d.cost {
            self.p.inv[it] -= n;
        }
        self.p.inv[d.item] += 1;
        play(&self.sfx.craft);
        let (x, y) = (self.p.x, self.p.y - 12.);
        self.burst(x, y, hx(0xffc060), 12, 60.);
        self.toast(format!("ทำ {} สำเร็จ", d.name));
        self.save();
    }
    pub fn use_dish(&mut self, item: usize) {
        if !(DISH_STEW..=DISH_FISH).contains(&item) || self.p.inv[item] <= 0 {
            return;
        }
        let (x, y) = (self.p.x, self.p.y - 18.);
        let food = nutrition(item).map_or(0., |n| n.0);
        match item {
            DISH_STEW => {
                if self.p.hp >= self.p.maxhp && self.p.hunger >= 90. {
                    self.toast("เลือดเต็มแล้ว");
                    return;
                }
                self.p.hp = (self.p.hp + 14).min(self.p.maxhp);
                self.add_float(x, y, "+14 HP", hx(0xff6666), None);
            }
            DISH_FISH => {
                if self.p.hp >= self.p.maxhp && self.p.hunger >= 90. {
                    self.toast("เลือดเต็มแล้ว");
                    return;
                }
                self.p.hp = (self.p.hp + 10).min(self.p.maxhp);
                self.add_float(x, y, "+10 HP", hx(0xff6666), None);
            }
            DISH_SPEED => {
                self.p.buff_spd = BUFF_T;
                self.add_float(x, y, "วิ่งเร็ว!", hx(0x88e0ff), None);
            }
            _ => {
                self.p.buff_crit = BUFF_T;
                self.add_float(x, y, "คริ+20%!", hx(0xffd060), None);
            }
        }
        self.p.inv[item] -= 1;
        self.feed(food);
        play(&self.sfx.eat);
    }
    pub fn upd_buffs(&mut self, dt: f32) {
        self.p.buff_spd = (self.p.buff_spd - dt).max(0.);
        self.p.buff_crit = (self.p.buff_crit - dt).max(0.);
    }
}

// ======================= ตกปลา =======================
pub struct Fishing {
    pub tx: i32,
    pub ty: i32,
    pub t: f32,
    pub bite_at: f32,
    /// > 0 = ปลากินเบ็ดอยู่ (เวลาที่เหลือให้ดึง)
    pub bite_t: f32,
    pub spot: bool,
    pub px: f32,
    pub py: f32,
}
pub const FISH_PRICE: i32 = 7;

/// จุดตกปลาพิเศษ (ผิวน้ำเป็นระลอก) — ปลาติดไวและมีโอกาสได้ของดี
pub fn fish_spot(seed: i32, tx: i32, ty: i32) -> bool {
    hash(tx, ty, seed.wrapping_add(777)) > 0.985
}

impl Game {
    pub fn water_ahead(&self) -> Option<(i32, i32)> {
        if self.w.dun.is_some() {
            return None;
        }
        let (dx, dy) = self.p.facing();
        for d in [10., 18., 26., 34., 42.] {
            let (tx, ty) = (tfloor(self.p.x + dx * d), tfloor(self.p.y - 3. + dy * d));
            let t = self.w.tile_at(tx, ty);
            if t < 2 {
                return Some((tx, ty));
            }
            if SOLIDT[t as usize] || OSOLID[self.w.obj_at(tx, ty) as usize] {
                return None;
            }
        }
        None
    }
    pub fn start_fish(&mut self) -> bool {
        let Some((tx, ty)) = self.water_ahead() else { return false };
        if !self.p.rod {
            self.toast("ต้องมีคันเบ็ดก่อน — คราฟต์ได้ที่เมนู (C)");
            return true;
        }
        let spot = fish_spot(self.w.seed, tx, ty);
        let wait = if spot { gen_range(1.2, 2.6) } else { gen_range(2.2, 5.5) };
        self.fish = Some(Fishing { tx, ty, t: 0., bite_at: wait, bite_t: 0., spot, px: self.p.x, py: self.p.y });
        play_vol(&self.sfx.splash, 0.6);
        true
    }
    pub fn fish_press(&mut self) {
        let Some(f) = self.fish.take() else { return };
        if f.bite_t <= 0. {
            self.toast("ดึงเร็วไป ปลายังไม่กินเบ็ด");
            return;
        }
        let (x, y) = (f.tx as f32 * 16. + 8., f.ty as f32 * 16. + 4.);
        let r = gen_range(0., 1.);
        self.burst(x, y, hx(0xbfe8ff), 12, 60.);
        play(&self.sfx.splash);
        if f.spot && r < 0.12 {
            self.gain(GEM, rnd(1, 2), x, y);
            self.toast("ตกได้อัญมณีจากก้นบ่อ!");
        } else if f.spot && r < 0.22 {
            self.gain(COIN, rnd(12, 30), x, y);
            self.toast("ตกได้ถุงเหรียญ!");
        } else if !f.spot && r < 0.07 {
            self.gain(COIN, rnd(4, 10), x, y);
            self.toast("ตกได้เหรียญเก่า");
        } else {
            let n = if f.spot { rnd(1, 2) } else { 1 };
            self.gain(FISH, n, x, y);
            self.toast("ตกปลาได้!");
        }
        self.p.xp += 2;
        self.check_lv();
        play(&self.sfx.upgrade);
    }
    pub fn upd_fish(&mut self, dt: f32) {
        let Some(f) = self.fish.as_mut() else { return };
        f.t += dt;
        let moved = (self.p.x - f.px).hypot(self.p.y - f.py) > 6.;
        if moved || self.p.last_hit < 0.2 || self.w.dun.is_some() {
            self.fish = None;
            self.toast("เลิกตกปลา");
            return;
        }
        if f.bite_t <= 0. && f.t >= f.bite_at {
            f.bite_t = 1.0;
            play(&self.sfx.pickup);
            self.shake(0.5, 0.1);
        } else if f.bite_t > 0. {
            f.bite_t -= dt;
            if f.bite_t <= 0. {
                self.fish = None;
                self.toast("ปลาหลุดไปแล้ว!");
            }
        }
    }
    pub fn sell_fish(&mut self) {
        let n = self.p.inv[FISH];
        if n <= 0 {
            self.toast("ไม่มีปลาจะขาย");
            return;
        }
        self.p.inv[FISH] = 0;
        self.p.inv[COIN] += n * FISH_PRICE;
        play(&self.sfx.buy);
        self.toast(format!("ขายปลา {} ตัว ได้ {} เหรียญ", n, n * FISH_PRICE));
        self.save();
    }
}

// ======================= เขตปลอดภัย (คบเพลิงในฐาน) =======================
/// รัศมีเขตปลอดภัยรอบคบเพลิงที่วางเอง (พิกเซล) — มอนสเตอร์ไม่เกิดและถูกผลักออก
pub const SAFE_R: f32 = 84.;

pub fn torch_near(ts: &[(f32, f32)], x: f32, y: f32) -> Option<(f32, f32)> {
    ts.iter().copied().find(|&(tx, ty)| (tx - x).hypot(ty - y) < SAFE_R)
}

impl Game {
    pub fn refresh_torches(&mut self) {
        self.torches = self.w.placed.iter().filter(|(_, &v)| v == 18 || v == O_LAMP).map(|(&(x, y), _)| (x as f32 * 16. + 8., y as f32 * 16. + 8.)).collect();
    }
    pub fn in_safe(&self, x: f32, y: f32) -> bool {
        self.w.dun.is_none() && (torch_near(&self.torches, x, y).is_some() || self.room_of.contains_key(&(tfloor(x), tfloor(y))))
    }
}
