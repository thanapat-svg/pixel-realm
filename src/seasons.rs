// seasons.rs — v8c: ฤดูกาล + ต้นไม้งอกใหม่ + ปลูกต้นไม้
//  ปีละ 4 ฤดู ฤดูละ 5 วันเกม: ใบไม้ผลิ (ฝนบ่อย พืชโตเร็ว) · ร้อน (ฝนน้อย กระหายเร็ว) · ใบไม้ร่วง (ใบส้ม ฝนบ่อย) · หนาว (หิมะ หิวเร็ว พืชแทบไม่โต)
//  ต้นไม้ธรรมชาติที่โค่นแล้ว: 3 วัน (ไม่นับฤดูหนาว) → ต้นกล้า → อีก 3 วัน → ต้นไม้ชนิดเดิม
//  ปลูกต้นกล้าเองได้จากเมนูสร้าง (ไม้ 1 + ผลไม้ 1) → 4 วัน (ไม่นับฤดูหนาว) โตเป็นต้นไม้ตามไบโอม
use super::*;

pub const SEASON_DAYS: i32 = 5;
pub const REGROW_DAYS: i32 = 3;
pub const PLANT_DAYS: i32 = 4;
pub const SEASON_N: [&str; 4] = ["ฤดูใบไม้ผลิ", "ฤดูร้อน", "ฤดูใบไม้ร่วง", "ฤดูหนาว"];

/// เกมเริ่มที่ฤดูร้อน (วัน 0–4) → ใบไม้ร่วง → หนาว → ใบไม้ผลิ
pub fn season_of_day(day: i32) -> u8 { ((day.max(0) + SEASON_DAYS) / SEASON_DAYS % 4) as u8 }
pub fn season(g: &Game) -> u8 { season_of_day(day_of(g)) }

/// ต้นไม้ผลัดใบ (เปลี่ยนสีตามฤดู)
pub fn deciduous(o: u8) -> bool { matches!(o, 1 | 40 | 41 | 45 | 46 | 47 | 48 | O_SWTREE) }
/// ไบโอมที่ปกคลุมด้วยหิมะในฤดูหนาว
pub fn wintry(t: u8) -> bool { matches!(t, 3 | 4 | 5 | 15) }

/// สีย้อมพื้น (tile) ตามฤดู
pub fn ground_tint(s: u8, t: u8) -> Color {
    if !matches!(t, 3 | 4 | 15 | 16) { return WHITE; }
    match s {
        1 => Color::new(1., 0.98, 0.86, 1.),
        2 => Color::new(1., 0.86, 0.62, 1.),
        3 => Color::new(0.86, 0.9, 0.98, 1.),
        _ => WHITE,
    }
}
/// สีย้อมต้นไม้ตามฤดู
pub fn tree_tint(s: u8, o: u8) -> Color {
    if !deciduous(o) { return WHITE; }
    match (s, o) {
        (2, 45) => WHITE,
        (2, _) => Color::new(1., 0.72, 0.36, 1.),
        (3, _) => Color::new(0.74, 0.7, 0.68, 1.),
        _ => WHITE,
    }
}
/// ชนิดต้นไม้ที่โตจากต้นกล้าที่ปลูก (ไม่ให้เป็นต้นกล้าซ้ำ)
pub fn grow_species(tile: u8, r: f64) -> u8 {
    match tile {
        2 => T_PALM,
        T_SAVANNA => O_ACACIA,
        T_SWAMP => O_SWTREE,
        _ => { let s = pick_species(tile, r); if s == T_SAPLING { 1 } else { s } }
    }
}
/// ปลูกต้นกล้าได้บนดินแบบไหน
pub fn plantable(t: u8) -> bool { matches!(t, 2 | 3 | 4 | 5 | 7 | T_SWAMP | T_SAVANNA) }

impl Game {
    /// ตัวคูณ (ความกระหาย, ความหิว) จากฤดู — ใช้เฉพาะกลางแจ้ง
    pub fn season_climate(&self) -> (f32, f32) {
        if self.w.dun.is_some() { return (1., 1.); }
        let fire = self.w.near_fire(tfloor(self.p.x), tfloor(self.p.y), 3).is_some();
        match season(self) { 1 => (1.15, 1.), 3 => (1., if fire { 1. } else { 1.2 }), _ => (1., 1.) }
    }
    /// ตัวคูณการโตของพืชผล
    pub fn crop_season_mul(&self) -> f32 {
        match season(self) { 0 => 1.3, 1 => 1., 2 => 0.8, _ => 0.25 }
    }
    /// ช่วงฝนตก/ฝนหยุด ตามฤดู (ตัวคูณช่วงเวลา: (ตก, หยุด))
    pub fn rain_len_mul(&self) -> (f32, f32) {
        match season(self) { 0 => (1.2, 0.7), 1 => (0.7, 1.7), 2 => (1.1, 0.8), _ => (1., 1.) }
    }
    /// ชนิดอากาศ: ฤดูหนาวที่ทุ่งหญ้า/ป่า/หินผา/หนองน้ำ = หิมะ
    pub fn season_weather(&self, base: u8, t: u8) -> u8 {
        if base == 1 && season(self) == 3 && wintry(t) { 2 } else { base }
    }

    /// โค่นต้นไม้ธรรมชาติ → จดไว้ให้งอกใหม่ · โค่นต้นที่ปลูกเอง → หายไปเลย
    pub fn on_tree_felled(&mut self, k: (i32, i32)) {
        self.planted.remove(&k);
        if is_tree(self.w.nat_obj(k.0, k.1)) {
            self.w.placed.remove(&k);
            self.w.removed.insert(k);
            self.felled.insert(k, (0, 0));
        } else {
            self.felled.remove(&k);
        }
    }
    pub fn near_build(&self, k: (i32, i32)) -> bool {
        self.w.placed.iter().any(|(p, &o)| (p.0 - k.0).abs() <= 3 && (p.1 - k.1).abs() <= 3 && !(o == T_SAPLING && self.planted.contains_key(p)) && !(*p == k && o == T_SAPLING))
            || self.w.tmod.keys().any(|p| (p.0 - k.0).abs() <= 1 && (p.1 - k.1).abs() <= 1)
    }
    /// ปลูกต้นกล้า (ตรวจพื้นแล้ว)
    pub fn plant_sapling(&mut self, k: (i32, i32)) -> bool {
        if !plantable(self.w.tile_at(k.0, k.1)) { self.toast("ปลูกต้นไม้บนพื้นนี้ไม่ได้"); return false; }
        self.w.placed.insert(k, T_SAPLING);
        self.planted.insert(k, 0);
        self.toast("ปลูกต้นกล้าแล้ว — จะโตเป็นต้นไม้ใน 4 วัน (ฤดูหนาวไม่โต)");
        true
    }
    /// ทุกเช้า: นับวันโตของต้นไม้ + แจ้งเปลี่ยนฤดู
    pub fn season_dawn(&mut self) {
        let day = day_of(self);
        let s = season_of_day(day);
        if day > 0 && day % SEASON_DAYS == 0 {
            self.toast(format!("เข้าสู่{}", SEASON_N[s as usize]) + match s {
                0 => " — พืชโตเร็ว ฝนตกบ่อย",
                1 => " — ร้อน กระหายน้ำเร็ว ฝนน้อย",
                2 => " — ใบไม้เปลี่ยนสี แอปเปิลยังออกผล",
                _ => " — หนาวจัด หิวเร็ว พืชแทบไม่โต หิมะตก",
            });
        }
        // เซฟเก่า: ต้นไม้ที่โค่นไว้ก่อนมีระบบนี้ก็เริ่มงอกใหม่ได้
        let old: Vec<(i32, i32)> = self.w.removed.iter().copied().filter(|k| !self.felled.contains_key(k) && is_tree(self.w.nat_obj(k.0, k.1))).collect();
        for k in old { self.felled.insert(k, (0, 0)); }
        if s == 3 { return; }
        let me = (tfloor(self.p.x), tfloor(self.p.y));
        let keys: Vec<(i32, i32)> = self.felled.keys().copied().collect();
        for k in keys {
            let (d, st) = self.felled[&k];
            let d = d + 1;
            if st == 0 && d >= REGROW_DAYS {
                // ไม่งอกทับของที่สร้าง/ในเขตฐาน (มีสิ่งก่อสร้างในระยะ 3 ช่อง) หรือตรงที่ผู้เล่นยืน
                if k == me || self.near_build(k) { self.felled.insert(k, (d, 0)); continue; }
                self.w.removed.remove(&k);
                self.w.placed.insert(k, T_SAPLING);
                self.felled.insert(k, (0, 1));
            } else if st == 1 && d >= REGROW_DAYS {
                if self.w.placed.get(&k) == Some(&T_SAPLING) { self.w.placed.remove(&k); }
                self.felled.remove(&k);
                self.fruit.remove(&k);
            } else {
                self.felled.insert(k, (d, st));
            }
        }
        let keys: Vec<(i32, i32)> = self.planted.keys().copied().collect();
        for k in keys {
            if self.w.placed.get(&k) != Some(&T_SAPLING) { self.planted.remove(&k); continue; }
            let d = self.planted[&k] + 1;
            if d >= PLANT_DAYS {
                let sp = grow_species(self.w.tile_at(k.0, k.1), hash(k.0, k.1, self.w.seed.wrapping_add(55)));
                self.w.placed.insert(k, sp);
                self.planted.remove(&k);
            } else {
                self.planted.insert(k, d);
            }
        }
    }
    pub fn seasons_save_str(&self) -> String {
        let mut s = String::from("felled");
        for ((x, y), (d, st)) in &self.felled { s += &format!(" {},{},{},{}", x, y, d, st); }
        s += "\nplanted";
        for ((x, y), d) in &self.planted { s += &format!(" {},{},{}", x, y, d); }
        s + "\n"
    }
    pub fn seasons_load_line(&mut self, tag: &str, rest: &[&str]) {
        for v in rest {
            let n: Vec<i32> = v.split(',').filter_map(|a| a.parse().ok()).collect();
            match (tag, n.len()) {
                ("felled", 4) => { self.felled.insert((n[0], n[1]), (n[2], n[3] as u8)); }
                ("planted", 3) => { self.planted.insert((n[0], n[1]), n[2]); }
                _ => {}
            }
        }
    }
}
