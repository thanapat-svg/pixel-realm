//! ฟาร์ม (ไถดิน · ปลูก · รดน้ำ · เก็บเกี่ยว) · เตียงนอน · Grid Snap ของโหมดสร้าง

use super::*;

pub const T_SOIL: u8 = 11;
pub const T_WET: u8 = 12;
pub const OBJ_TILL: u8 = 255; // "ชิ้นส่วน" พิเศษ: ไถดิน (ไม่ใช่วัตถุจริง)
pub const WATER_MAX: i32 = 8;
/// วินาทีที่ดินเปียกอยู่ได้หลังรดน้ำ (พืชโตเฉพาะตอนดินเปียก)
pub const WET_T: f32 = 90.;
/// วินาทีที่ต้อง "เปียก" สะสมจนสุก: [ข้าวโพด, แครอท]
pub const GROW_T: [f32; 2] = [60., 42.];
pub const CROP_N: [&str; 2] = ["ข้าวโพด", "แครอท"];
pub const SEED_OF: [usize; 2] = [SEED_CORN, SEED_CARROT];
pub const HARVEST_OF: [usize; 2] = [CORN, CARROT];
pub const CROP_PRICE: [i32; 2] = [6, 5];
pub const CROP_HEAL: [i32; 2] = [4, 3];
/// ระยะวางสูงสุดเมื่อใช้เคอร์เซอร์ Grid Snap (หน่วยเป็นไทล์)
pub const REACH_T: f32 = 4.5;

#[derive(Clone, Copy, Debug)]
pub struct Crop {
    pub kind: u8,
    pub grown: f32,
    pub wet: f32,
}
impl Crop {
    pub fn stage(&self) -> u8 {
        let r = self.grown / GROW_T[self.kind as usize];
        if r >= 1. { 2 } else if r >= 0.4 { 1 } else { 0 }
    }
    /// id วัตถุที่ใช้วาด: 27..29 ข้าวโพด, 30..32 แครอท
    pub fn obj(&self) -> u8 {
        27 + self.kind * 3 + self.stage()
    }
}

impl Game {
    pub fn hour(&self) -> f32 {
        ((self.time / DAYLEN * 24.) + 6.) % 24.
    }
    pub fn is_night_hr(&self) -> bool {
        let h = self.hour();
        h >= 18. || h < 5.
    }
    /// ไทล์ตรงหน้าตัวผู้เล่น (ใช้กับฟาร์ม และเป็นค่าเริ่มต้นของโหมดสร้าง)
    pub fn facing_tile(&self) -> (i32, i32) {
        let (dx, dy) = self.p.facing();
        (tfloor(self.p.x + dx * 14.), tfloor(self.p.y - 3. + dy * 14.))
    }
    pub fn in_reach(&self, c: (i32, i32)) -> bool {
        (c.0 as f32 * 16. + 8. - self.p.x).hypot(c.1 as f32 * 16. + 8. - (self.p.y - 3.)) <= REACH_T * 16.
    }
    /// ไทล์ที่จะวาง: เคอร์เซอร์ Grid Snap ถ้ามีและอยู่ในระยะ ไม่งั้นไทล์หน้าตัว
    pub fn build_target(&self) -> (i32, i32) {
        if let Some(c) = self.build_cur {
            if self.in_reach(c) {
                return c;
            }
        }
        self.facing_tile()
    }

    fn pick_seed(&self) -> Option<u8> {
        let s = self.seed_sel as usize % 2;
        if self.p.inv[SEED_OF[s]] > 0 {
            Some(s as u8)
        } else if self.p.inv[SEED_OF[1 - s]] > 0 {
            Some((1 - s) as u8)
        } else {
            None
        }
    }

    /// ไถดิน / คืนหญ้า (ชิ้นส่วนพิเศษในเมนูสร้าง) — ไม่เสียวัสดุ
    pub fn till_at(&mut self, k: (i32, i32)) -> bool {
        let t = self.w.tile_at(k.0, k.1);
        let o = self.w.obj_at(k.0, k.1);
        if t == T_SOIL || t == T_WET {
            if (27..=32).contains(&o) {
                self.w.placed.remove(&k);
                self.crops.remove(&k);
            } else if o != 0 {
                self.toast("ไถตรงนี้ไม่ได้");
                return false;
            }
            self.w.tmod.insert(k, 3);
            self.toast("คืนเป็นหญ้า");
        } else if (t == 3 || t == 4) && (o == 0 || o == 3) {
            if o == 3 && self.w.placed.remove(&k).is_none() {
                self.w.removed.insert(k);
            }
            self.w.tmod.insert(k, T_SOIL);
        } else {
            self.toast("ไถได้เฉพาะทุ่งหญ้า/ป่า");
            return false;
        }
        play(&self.sfx.craft);
        self.burst(k.0 as f32 * 16. + 8., k.1 as f32 * 16. + 10., hx(0x7a5230), 8, 40.);
        true
    }

    /// รดน้ำ/ปลูก/เก็บเกี่ยว/เติมน้ำ ที่ไทล์หน้าตัว — คืน true ถ้าจัดการแล้ว
    pub fn try_farm(&mut self) -> bool {
        if self.w.dun.is_some() {
            return false;
        }
        let k = self.facing_tile();
        let o = self.w.obj_at(k.0, k.1);
        let t = self.w.tile_at(k.0, k.1);
        let (px, py) = (self.p.x, self.p.y - 18.);
        if (27..=32).contains(&o) {
            let c = self.crops.get(&k).copied();
            let ripe = c.map_or(true, |c| c.stage() == 2);
            if ripe {
                let kind = ((o - 27) / 3) as usize;
                self.w.placed.remove(&k);
                self.crops.remove(&k);
                let n = rnd(1, 2);
                self.gain(HARVEST_OF[kind], n, px, py);
                if gen_range(0., 1.) < 0.6 {
                    self.gain(SEED_OF[kind], 1, px, py - 10.);
                }
                self.p.xp += 2;
                self.check_lv();
                play(&self.sfx.pickup);
                self.burst(k.0 as f32 * 16. + 8., k.1 as f32 * 16. + 8., hx(0xf0c840), 8, 45.);
                self.save();
            } else if self.p.can && self.p.water > 0 && c.map_or(false, |c| c.wet < WET_T * 0.5) {
                self.water_crop(k);
            } else if !self.p.can {
                self.toast("ยังไม่สุก — คราฟต์บัวรดน้ำ (เมนู C) เพื่อรดน้ำ");
            } else if self.p.water <= 0 {
                self.toast("น้ำหมด — ไปเติมที่แหล่งน้ำ (หันหน้าหาน้ำแล้วกด Q)");
            } else {
                self.toast("ดินยังเปียกอยู่ รอให้โตก่อน");
            }
            return true;
        }
        if o == 0 && (t == T_SOIL || t == T_WET) {
            let Some(kind) = self.pick_seed() else {
                self.toast("ไม่มีเมล็ดพืช — ซื้อที่พ่อค้าหรือเก็บจากสไลม์");
                return true;
            };
            self.p.inv[SEED_OF[kind as usize]] -= 1;
            let wet = if t == T_WET { 40. } else { 0. };
            let c = Crop { kind, grown: 0., wet };
            self.w.placed.insert(k, c.obj());
            self.crops.insert(k, c);
            play(&self.sfx.door);
            self.burst(k.0 as f32 * 16. + 8., k.1 as f32 * 16. + 10., hx(0x7a5230), 6, 30.);
            self.toast(format!("ปลูก{} แล้ว — อย่าลืมรดน้ำ!", CROP_N[kind as usize]));
            self.save();
            return true;
        }
        if self.p.can && t <= 1 && self.p.water < WATER_MAX {
            self.p.water = WATER_MAX;
            play(&self.sfx.pickup);
            self.toast(format!("เติมน้ำเต็ม {}/{}", WATER_MAX, WATER_MAX));
            return true;
        }
        false
    }
    fn water_crop(&mut self, k: (i32, i32)) {
        if let Some(c) = self.crops.get_mut(&k) {
            c.wet = WET_T;
        }
        self.w.tmod.insert(k, T_WET);
        self.p.water -= 1;
        play(&self.sfx.pickup);
        self.burst(k.0 as f32 * 16. + 8., k.1 as f32 * 16. + 6., hx(0x7ab8f0), 8, 35.);
        self.toast(format!("รดน้ำแล้ว (เหลือ {}/{})", self.p.water, WATER_MAX));
    }

    /// ข้อความช่วยเหลือเมื่อหันหน้าหาแปลงปลูก
    pub fn farm_hint(&self, touch: bool) -> Option<String> {
        if self.w.dun.is_some() {
            return None;
        }
        let key = if touch { "ปุ่ม \"ใช้\"" } else { "Q" };
        let k = self.facing_tile();
        let o = self.w.obj_at(k.0, k.1);
        let t = self.w.tile_at(k.0, k.1);
        if (27..=32).contains(&o) {
            return Some(match self.crops.get(&k) {
                Some(c) if c.stage() < 2 => format!("{}: รดน้ำ ({}) น้ำ {}/{}", key, CROP_N[c.kind as usize], self.p.water, WATER_MAX),
                _ => format!("{}: เก็บเกี่ยว", key),
            });
        }
        if o == 0 && (t == T_SOIL || t == T_WET) {
            return Some(format!("{}: ปลูก{}", key, CROP_N[self.pick_seed().unwrap_or(self.seed_sel % 2) as usize]));
        }
        if self.p.can && t <= 1 && self.p.water < WATER_MAX {
            return Some(format!("{}: เติมน้ำ {}/{}", key, self.p.water, WATER_MAX));
        }
        None
    }

    /// พืชโตเฉพาะตอนดินเปียก (ฝนทำให้เปียกเอง) — เรียกทุกเฟรม ทำงานจริงทุก 0.5 วินาที
    pub fn upd_crops(&mut self, dt: f32) {
        self.crop_t += dt;
        if self.crop_t < 0.5 {
            return;
        }
        let step = self.crop_t;
        self.crop_t = 0.;
        let rain = self.rain > 0.4 && self.weather_kind == 1;
        let smul = self.crop_season_mul();
        let keys: Vec<(i32, i32)> = self.crops.keys().copied().collect();
        for k in keys {
            let Some(c) = self.crops.get_mut(&k) else { continue };
            if rain {
                c.wet = c.wet.max(8.);
            }
            let was_wet = c.wet > 0.;
            if was_wet {
                c.wet -= step;
                c.grown += step * smul;
            }
            let (o, wet_now) = (c.obj(), c.wet > 0.);
            if self.w.placed.get(&k) != Some(&o) && self.w.placed.contains_key(&k) {
                self.w.placed.insert(k, o);
            }
            let tn = self.w.tmod.get(&k).copied().unwrap_or(0);
            if wet_now && tn != T_WET {
                self.w.tmod.insert(k, T_WET);
            } else if !wet_now && tn == T_WET {
                self.w.tmod.insert(k, T_SOIL);
            }
        }
        // พืชที่วัตถุถูกรื้อไปแล้ว
        self.crops.retain(|k, _| (27..=32).contains(&self.w.placed.get(k).copied().unwrap_or(0)));
    }

    // ---------------- เตียงนอน ----------------
    pub fn sleep(&mut self, k: (i32, i32)) {
        if !self.is_night_hr() {
            self.toast("ยังไม่ง่วง — นอนได้ตั้งแต่ 18:00 ถึง 05:00 น.");
            return;
        }
        self.bed = Some(k);
        let t0 = self.time;
        self.time = ((self.time / DAYLEN).floor() + 1.) * DAYLEN;
        self.sleep_needs(self.time - t0);
        self.p.hp = self.p.maxhp;
        self.en.clear();
        self.proj.clear();
        self.rain = 0.;
        self.rain_on = false;
        self.thunder = 0.;
        for c in self.crops.values_mut() {
            if c.wet > 0. {
                c.grown += 40.;
                c.wet = 0.;
            }
        }
        self.sleep_fx = 2.;
        self.refresh_torches();
        let day = (self.time / DAYLEN) as i32 + 1;
        let hungry = if self.survival_mode() > 0 && (self.p.thirst < 30. || self.p.hunger < 30.) { " · ตื่นมาหิว/กระหายน้ำ" } else { "" };
        self.toast(format!("หลับสบาย... เช้าวันที่ {} (เลือดเต็ม · แรงเต็ม · ตั้งเตียงเป็นจุดเกิด){}", day, hungry));
        self.save();
    }
    /// จุดเกิดใหม่: เตียงถ้ายังอยู่ ไม่งั้นจุดเกิดเดิม
    pub fn respawn_pt(&self) -> (f32, f32) {
        if let Some(b) = self.bed {
            if self.w.placed.get(&b) == Some(&33) {
                for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let (tx, ty) = (b.0 + dx, b.1 + dy);
                    if !self.w.solid_at(tx, ty) {
                        return (tx as f32 * 16. + 8., ty as f32 * 16. + 12.);
                    }
                }
            }
        }
        self.spawn
    }

    // ---------------- พืชผล ----------------
    pub fn eat_crop(&mut self, item: usize) {
        let Some(kind) = HARVEST_OF.iter().position(|&i| i == item) else { return };
        if self.p.inv[item] <= 0 {
            return;
        }
        if self.p.hp >= self.p.maxhp && self.p.hunger >= 95. {
            self.toast("เลือดเต็มแล้ว");
            return;
        }
        self.p.inv[item] -= 1;
        self.feed(nutrition(item).map_or(15., |n| n.0));
        self.p.hp = (self.p.hp + CROP_HEAL[kind]).min(self.p.maxhp);
        let (x, y) = (self.p.x, self.p.y - 18.);
        self.add_float(x, y, format!("+{} HP", CROP_HEAL[kind]), hx(0xff6666), None);
        play(&self.sfx.eat);
    }
    pub fn sell_crops(&mut self) {
        let mut total = 0;
        let mut n = 0;
        for k in 0..2 {
            let c = self.p.inv[HARVEST_OF[k]];
            total += c * CROP_PRICE[k];
            n += c;
            self.p.inv[HARVEST_OF[k]] = 0;
        }
        if n == 0 {
            self.toast("ไม่มีผลผลิตจะขาย");
            return;
        }
        self.p.inv[COIN] += total;
        play(&self.sfx.buy);
        self.toast(format!("ขายผลผลิต {} ชิ้น ได้ {} เหรียญ", n, total));
        self.save();
    }
}
