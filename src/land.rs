// land.rs — v8b: ไบโอมใหม่ + จุดสำคัญในโลก
//  ไบโอม: หนองน้ำ (เดินช้า ต้นไม้หนองน้ำ ต้นกก เห็ด) · สะวันนา (อะคาเซีย หญ้าแห้ง ร้อน) · ภูเขาไฟ (ลาวา เถ้า หินออบซิเดียน)
//  จุดสำคัญ (สุ่มแบบกำหนดแน่นอนจาก seed): ซากปรักหักพังโบราณ (มีผู้พิทักษ์ + หีบโบราณ) · วงหินศักดิ์สิทธิ์ (สวดขอพรวันละครั้ง)
//    · ต้นไม้ยักษ์โบราณ (เก็บสมุนไพร/ผลไม้วันละครั้ง) · ภูเขาไฟ
//  หมู่บ้านหลายแบบ: แคมป์พ่อค้าเปลี่ยนตามไบโอม — ชาวประมง (ริมน้ำ) · คาราวาน (ทะเลทราย/สะวันนา) · กระท่อมนายพราน (หิมะ)
//  ถ้ำพิเศษ: ปากถ้ำในหิมะ = ถ้ำคริสตัล · ปากถ้ำในเขตเถ้าภูเขาไฟ = ถ้ำลาวา
use super::*;

pub const T_SWAMP: u8 = 15;
pub const T_SAVANNA: u8 = 16;
pub const T_ASH: u8 = 17;
pub const T_LAVA: u8 = 18;

pub const O_REED: u8 = 50;
pub const O_SWTREE: u8 = 51;
pub const O_SHROOM: u8 = 52;
pub const O_ACACIA: u8 = 53;
pub const O_DRYGRASS: u8 = 54;
pub const O_RUINWALL: u8 = 55;
pub const O_PILLAR: u8 = 56;
pub const O_ALTAR: u8 = 57;
pub const O_OBSIDIAN: u8 = 58;
pub const O_GIANT: u8 = 59;
pub const O_FILL: u8 = 60;
pub const O_HUT_FISH: u8 = 61;
pub const O_HUT_SAND: u8 = 62;
pub const O_HUT_LOG: u8 = 63;
pub const O_RACK: u8 = 64;
pub const O_JAR: u8 = 65;
pub const O_WOODPILE: u8 = 66;
pub const O_CRYSTAL: u8 = 67;
pub const O_ACHEST: u8 = 68;
pub const O_BONES: u8 = 69;

/// วัตถุที่ทำลายไม่ได้ (โครงสร้าง/สิ่งศักดิ์สิทธิ์)
pub fn land_fixed(o: u8) -> bool { matches!(o, O_ALTAR | O_GIANT | O_FILL | O_HUT_FISH | O_HUT_SAND | O_HUT_LOG | O_RACK | O_JAR | O_WOODPILE) }
/// วาดราบกับพื้น (ไม่บังผู้เล่น)
pub fn land_flat(o: u8) -> bool { matches!(o, O_DRYGRASS | O_BONES | O_FILL) }

pub const MK_RUINS: u8 = 1;
pub const MK_CIRCLE: u8 = 2;
pub const MK_GIANT: u8 = 3;
pub const MK_VOLCANO: u8 = 4;
pub fn mark_name(k: u8) -> &'static str {
    match k { MK_RUINS => "ซากปรักหักพังโบราณ", MK_CIRCLE => "วงหินศักดิ์สิทธิ์", MK_GIANT => "ต้นไม้ยักษ์โบราณ", _ => "ภูเขาไฟ" }
}
pub fn mark_letter(k: u8) -> (&'static str, u32) {
    match k { MK_RUINS => ("ซ", 0xd8c8a0), MK_CIRCLE => ("ศ", 0x9ad8ff), MK_GIANT => ("ต", 0x7ae07a), _ => ("ภ", 0xff6a30) }
}

/// เซลล์จุดสำคัญ (ไทล์) · เซลล์ภูเขาไฟ
pub const LCELL: i32 = 72;
pub const VCELL: i32 = 192;
/// รัศมีอิทธิพลของภูเขาไฟ (ไทล์)
pub const VR: i32 = 12;

pub fn camp_title(kind: u8) -> &'static str {
    match kind { 1 => "หมู่บ้านชาวประมง", 2 => "กองคาราวาน", 3 => "กระท่อมนายพราน", _ => "พ่อค้าเร่" }
}
pub fn camp_greet(kind: u8) -> &'static str {
    match kind {
        1 => "ปลาเพิ่งขึ้นจากน้ำเมื่อเช้า สดๆ เลย!",
        2 => "เดินทางข้ามทะเลทรายต้องมีเสบียงนะ",
        3 => "ข้างนอกหนาวจัด แวะผิงไฟก่อนได้",
        _ => "",
    }
}
pub const KIND_OFFERS: [[Offer; 2]; 3] = [
    [
        Offer { name: "ปลาสด x3", desc: "ปลาของชาวประมง (ทำอาหารได้)", item: FISH, qty: 3, price: 7, once: false },
        Offer { name: "ซุปปลา", desc: "อาหารปรุงสุก อิ่มนาน", item: DISH_FISH, qty: 1, price: 12, once: false },
    ],
    [
        Offer { name: "เนื้อแห้ง x2", desc: "เสบียงคาราวาน (ปรุงก่อนกินปลอดภัยกว่า)", item: MEAT, qty: 2, price: 8, once: false },
        Offer { name: "สมุนไพรทะเลทราย x2", desc: "ใช้ปรุงยา/อาหาร", item: HERB, qty: 2, price: 8, once: false },
    ],
    [
        Offer { name: "เนื้อล่าสัตว์ x3", desc: "เนื้อจากนายพราน", item: MEAT, qty: 3, price: 10, once: false },
        Offer { name: "ลูกธนู x40", desc: "นายพรานขายราคาส่ง", item: ARROW, qty: 40, price: 26, once: false },
    ],
];
/// รายการสินค้าของแคมป์ชนิดนี้ (สินค้าทั่วไป + สินค้าท้องถิ่น)
pub fn camp_offers(kind: u8) -> Vec<&'static Offer> {
    let mut v: Vec<&'static Offer> = OFFERS.iter().collect();
    if (1..=3).contains(&kind) { v.extend(KIND_OFFERS[kind as usize - 1].iter()); }
    v
}

/// ความสูงพื้นดินดิบ (เหมือนใน gen_tile)
pub fn elev(x: i32, y: i32, s: i32) -> f64 { fbm(x as f64 / 48., y as f64 / 48., s) }

impl World {
    // ---------------- ภูเขาไฟ ----------------
    pub fn volcano_in_cell(&self, cx: i32, cy: i32) -> Option<(i32, i32)> {
        if let Some(&r) = self.volc.borrow().get(&(cx, cy)) { return r; }
        let r = self.calc_volcano(cx, cy);
        self.volc.borrow_mut().insert((cx, cy), r);
        r
    }
    fn calc_volcano(&self, cx: i32, cy: i32) -> Option<(i32, i32)> {
        let s = self.seed.wrapping_add(6061);
        if hash(cx, cy, s) > 0.6 { return None; }
        for k in 0..10 {
            let tx = cx * VCELL + 20 + (hash(cx, cy, s.wrapping_add(1 + k * 2)) * (VCELL - 40) as f64) as i32;
            let ty = cy * VCELL + 20 + (hash(cx, cy, s.wrapping_add(2 + k * 2)) * (VCELL - 40) as f64) as i32;
            if elev(tx, ty, self.seed) < 0.6 { continue; }
            // ห้ามมีทะเลในรัศมีภูเขาไฟ
            let dry = [(VR, 0), (-VR, 0), (0, VR), (0, -VR), (8, 8), (-8, 8), (8, -8), (-8, -8)].iter().all(|&(dx, dy)| elev(tx + dx, ty + dy, self.seed) >= 0.46);
            if dry { return Some((tx, ty)); }
        }
        None
    }
    pub fn volcano_near(&self, x: i32, y: i32, r: i32) -> Option<(i32, i32)> {
        for cy in (y - r).div_euclid(VCELL)..=(y + r).div_euclid(VCELL) {
            for cx in (x - r).div_euclid(VCELL)..=(x + r).div_euclid(VCELL) {
                if let Some((vx, vy)) = self.volcano_in_cell(cx, cy) {
                    if (vx - x).abs() <= r && (vy - y).abs() <= r { return Some((vx, vy)); }
                }
            }
        }
        None
    }
    /// ไทล์ของภูเขาไฟ (None = ไม่อยู่ในเขต)
    pub fn volcano_tile(&self, x: i32, y: i32) -> Option<u8> {
        let (vx, vy) = self.volcano_near(x, y, VR)?;
        let (dx, dy) = ((x - vx) as f64, (y - vy) as f64);
        let d = dx.hypot(dy) + (hash(x, y, self.seed.wrapping_add(707)) - 0.5) * 1.4;
        if d < 2.4 { return Some(T_LAVA); }
        if d < 3.4 { return Some(6); }
        // ธารลาวาไหลลงด้านหนึ่ง
        let a0 = hash(vx, vy, self.seed.wrapping_add(708)) * std::f64::consts::TAU;
        let a = dy.atan2(dx);
        let da = ((a - a0 + std::f64::consts::PI).rem_euclid(std::f64::consts::TAU) - std::f64::consts::PI).abs();
        let wig = (d * 0.9).sin() * 0.08;
        if d < 9.5 && (da + wig) * d < 0.85 { return Some(T_LAVA); }
        if d < 10.5 + (hash(x / 3, y / 3, self.seed.wrapping_add(709)) - 0.5) * 2.5 { return Some(T_ASH); }
        None
    }

    // ---------------- จุดสำคัญ ----------------
    pub fn mark_in_cell(&self, cx: i32, cy: i32) -> Option<(i32, i32, u8)> {
        if self.dun.is_some() { return None; }
        if let Some(&r) = self.marks.borrow().get(&(cx, cy)) { return r; }
        let r = self.calc_mark(cx, cy);
        self.marks.borrow_mut().insert((cx, cy), r);
        r
    }
    fn calc_mark(&self, cx: i32, cy: i32) -> Option<(i32, i32, u8)> {
        let s = self.seed.wrapping_add(31337);
        if hash(cx, cy, s) > 0.5 { return None; }
        let r2 = hash(cx, cy, s.wrapping_add(99));
        for k in 0..5 {
            let tx = cx * LCELL + 7 + (hash(cx, cy, s.wrapping_add(1 + k * 2)) * (LCELL - 14) as f64) as i32;
            let ty = cy * LCELL + 7 + (hash(cx, cy, s.wrapping_add(2 + k * 2)) * (LCELL - 14) as f64) as i32;
            if self.near_city(tx, ty, 8) { continue; }
            let t = self.tile_at(tx, ty);
            let kind = match t {
                4 => if r2 < 0.6 { MK_GIANT } else { MK_RUINS },
                8 => MK_RUINS,
                3 | 16 | 15 => if r2 < 0.55 { MK_RUINS } else { MK_CIRCLE },
                5 | 7 => MK_CIRCLE,
                _ => continue,
            };
            let ok = (-4..=4).all(|dy| (-4..=4).all(|dx| !SOLIDT[self.tile_at(tx + dx, ty + dy) as usize]));
            if !ok || self.camp_near(tx, ty, 12).is_some() { continue; }
            return Some((tx, ty, kind));
        }
        None
    }
    /// วัตถุของจุดสำคัญที่ไทล์นี้ (Some(0) = พื้นที่โล่งของจุดสำคัญ)
    pub fn mark_obj(&self, tx: i32, ty: i32) -> Option<u8> {
        let (mx, my, k) = self.mark_in_cell(tx.div_euclid(LCELL), ty.div_euclid(LCELL))?;
        let (dx, dy) = (tx - mx, ty - my);
        if dx.abs() > 4 || dy.abs() > 4 { return None; }
        let h = hash(tx, ty, self.seed.wrapping_add(4040));
        Some(match k {
            MK_RUINS => {
                let edge = dx.abs().max(dy.abs()) == 4;
                if edge {
                    if dx == 0 || (dy == 0 && h < 0.5) { 0 } else if h < 0.28 { 0 } else if h < 0.36 { 4 } else { O_RUINWALL }
                } else if (dx, dy) == (0, 0) {
                    O_ACHEST
                } else if dx.abs() == 2 && dy.abs() == 2 {
                    if h < 0.25 { O_BONES } else { O_PILLAR }
                } else if h < 0.07 { O_BONES } else if h < 0.1 { 3 } else { 0 }
            }
            MK_CIRCLE => {
                let ring = [(0, -3), (2, -2), (3, 0), (2, 2), (0, 3), (-2, 2), (-3, 0), (-2, -2)];
                if (dx, dy) == (0, 0) { O_ALTAR } else if ring.contains(&(dx, dy)) { O_PILLAR } else if dx * dx + dy * dy >= 12 && h < 0.18 { 3 } else { 0 }
            }
            MK_GIANT => {
                if (dx, dy) == (0, 0) { O_GIANT } else if dy == 0 && dx.abs() == 1 { O_FILL } else if dx * dx + dy * dy >= 6 && h < 0.22 { if h < 0.08 { O_SHROOM } else { 3 } } else { 0 }
            }
            _ => 0,
        })
    }
    pub fn marks_near(&self, tx: i32, ty: i32, r: i32) -> Vec<(i32, i32, u8)> {
        let mut v = vec![];
        for cy in (ty - r).div_euclid(LCELL)..=(ty + r).div_euclid(LCELL) {
            for cx in (tx - r).div_euclid(LCELL)..=(tx + r).div_euclid(LCELL) {
                if let Some((mx, my, k)) = self.mark_in_cell(cx, cy) {
                    if (mx - tx).abs() <= r && (my - ty).abs() <= r { v.push((mx, my, k)); }
                }
            }
        }
        v
    }

    // ---------------- แคมป์หลายแบบ ----------------
    /// 0 พ่อค้าเร่ · 1 ชาวประมง · 2 คาราวาน · 3 นายพราน
    pub fn camp_kind(&self, c: (i32, i32)) -> u8 {
        if let Some(&k) = self.camp_kinds.borrow().get(&c) { return k; }
        let t = self.tile_at(c.0, c.1);
        let k = if t == 7 { 3 } else if t == 8 || t == T_SAVANNA { 2 } else if t == 2 || t == T_SWAMP || (-5..=5).any(|dy| (-5..=5).any(|dx| self.tile_at(c.0 + dx, c.1 + dy) < 2)) { 1 } else { 0 };
        self.camp_kinds.borrow_mut().insert(c, k);
        k
    }

    /// วัตถุธรรมชาติของไบโอมใหม่
    pub fn land_nat(&self, t: u8, tx: i32, ty: i32, r: f64) -> u8 {
        match t {
            T_SWAMP => if r < 0.07 { O_SWTREE } else if r < 0.17 { O_REED } else if r < 0.195 { O_SHROOM } else if r < 0.21 { 2 } else if r > 0.9985 { 6 } else { 0 },
            T_SAVANNA => if r < 0.022 { O_ACACIA } else if r < 0.16 { O_DRYGRASS } else if r < 0.17 { 2 } else if r < 0.18 { 4 } else if r > 0.9988 { 6 } else { 0 },
            T_ASH => if r < 0.045 { O_OBSIDIAN } else if r < 0.065 { T_DEAD } else if r < 0.085 { 4 } else if r < 0.095 { O_BONES } else { let _ = (tx, ty); 0 },
            _ => 0,
        }
    }
}

impl Game {
    /// ตัวคูณความเร็วเดินจากพื้น (หนองน้ำเดินช้า)
    pub fn ground_speed_mul(&self) -> f32 {
        if self.w.dun.is_some() { return 1.; }
        match self.w.tile_at(tfloor(self.p.x), tfloor(self.p.y)) { T_SWAMP => 0.7, T_ASH => 0.92, _ => 1. }
    }
    pub fn discover_marks(&mut self) {
        if self.w.dun.is_some() { return; }
        let (tx, ty) = (tfloor(self.p.x), tfloor(self.p.y));
        let mut found = self.w.marks_near(tx, ty, 18);
        if let Some((vx, vy)) = self.w.volcano_near(tx, ty, 26) { found.push((vx, vy, MK_VOLCANO)); }
        for (mx, my, k) in found {
            if self.marks_found.contains_key(&(mx, my)) { continue; }
            self.marks_found.insert((mx, my), k);
            self.toast(format!("ค้นพบ: {}! ดูตำแหน่งบนแผนที่", mark_name(k)));
            self.p.xp += 4;
            if k == MK_RUINS {
                // ผู้พิทักษ์ซากโบราณ
                let mul = 1.1 + (mx as f32 - self.spawn.0 / 16.).hypot(my as f32 - self.spawn.1 / 16.) / 250.;
                for (dx, dy) in [(-1, 2), (1, -2), (2, 1)] {
                    let (x, y) = ((mx + dx) as f32 * 16. + 8., (my + dy) as f32 * 16. + 12.);
                    if !collide(&self.w, x, y) { self.make_enemy(EK::Skel, x, y, mul, 0, false); }
                }
                self.toast("ผู้พิทักษ์ซากโบราณตื่นขึ้นมา!");
            }
            self.check_lv();
            self.save();
        }
    }
    /// ภูเขาไฟ: ใกล้ลาวาโดนความร้อนเผา · ในเขตเถ้ากระหายเร็ว (ดู climate)
    pub fn land_tick(&mut self, dt: f32) {
        if self.w.dun.is_some() && !self.w.dun.as_ref().map_or(false, |d| d.cave) { return; }
        self.lava_t -= dt;
        if self.lava_t > 0. { return; }
        self.lava_t = 0.5;
        let (tx, ty) = (tfloor(self.p.x), tfloor(self.p.y - 4.));
        let mut lv: Option<(i32, i32)> = None;
        for (dx, dy) in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] {
            if self.w.tile_at(tx + dx, ty + dy) == T_LAVA { lv = Some((dx, dy)); break; }
        }
        if let Some((dx, dy)) = lv {
            if self.p.invul <= 0. {
                let (nx, ny) = if (dx, dy) == (0, 0) { (0., 1.) } else { (-dx as f32, -dy as f32) };
                self.hurt(1, nx, ny);
                if !self.lava_told { self.lava_told = true; self.toast("ร้อนจัด! ลาวาเผา — อย่าเข้าใกล้"); }
            }
        }
    }
    /// ศาลหิน: วันละครั้ง ฟื้นเลือดเต็ม หายป่วย หายเพลียครึ่งหนึ่ง
    pub fn shrine_pray(&mut self, k: (i32, i32)) -> bool {
        if self.try_summon_final(k) { return true; }
        let day = day_of(self);
        if self.blessed.get(&k) == Some(&day) { self.toast("ศาลหินเงียบงัน — กลับมาขอพรใหม่พรุ่งนี้"); return false; }
        self.blessed.insert(k, day);
        self.p.hp = self.p.maxhp;
        self.p.sick = 0.;
        self.p.rest = (self.p.rest + 50.).min(100.);
        let (x, y) = (k.0 as f32 * 16. + 8., k.1 as f32 * 16.);
        self.burst(x, y, hx(0x9ad8ff), 18, 60.);
        self.toast("ศาลหินอวยพร: เลือดเต็ม หายป่วย หายเพลีย");
        play(&self.sfx.gem);
        self.save();
        true
    }
    /// ต้นไม้ยักษ์: วันละครั้ง ได้สมุนไพร + ผลไม้
    pub fn giant_gather(&mut self, k: (i32, i32)) -> bool {
        let day = day_of(self);
        if self.blessed.get(&k) == Some(&day) { self.toast("เก็บจากต้นไม้ยักษ์ไปแล้ววันนี้"); return false; }
        self.blessed.insert(k, day);
        let (x, y) = (k.0 as f32 * 16. + 8., k.1 as f32 * 16.);
        self.gain(HERB, rnd(1, 3), x, y - 8.);
        self.gain(BERRY, rnd(2, 4), x, y - 16.);
        self.toast("ต้นไม้ยักษ์โบราณมอบสมุนไพรและผลไม้");
        play(&self.sfx.eat);
        self.save();
        true
    }
    /// หีบโบราณในซากปรักหักพัง
    pub fn open_ancient(&mut self, x: f32, y: f32) {
        let c = rnd(20, 40);
        self.gain(COIN, c, x, y);
        self.gain(GEM, rnd(2, 4), x, y - 8.);
        if gen_range(0., 1.) < 0.5 { self.gain(SHARD, 1, x, y - 16.); }
        self.gain(POTION, 1, x, y - 24.);
        self.p.xp += 12;
        self.check_lv();
        self.toast(format!("เปิดหีบโบราณ! ได้ {} เหรียญ อัญมณี และยา", c));
        play(&self.sfx.chest);
    }
    pub fn marks_save_str(&self) -> String {
        let mut s = String::from("marks");
        for ((x, y), k) in &self.marks_found { s += &format!(" {},{},{}", x, y, k); }
        s += "\nbless";
        for ((x, y), d) in &self.blessed { s += &format!(" {},{},{}", x, y, d); }
        s + "\n"
    }
    pub fn marks_load_line(&mut self, tag: &str, rest: &[&str]) {
        for v in rest {
            let n: Vec<i32> = v.split(',').filter_map(|a| a.parse().ok()).collect();
            if n.len() != 3 { continue; }
            if tag == "marks" { self.marks_found.insert((n[0], n[1]), n[2] as u8); } else { self.blessed.insert((n[0], n[1]), n[2]); }
        }
    }
}

// ======================= ภาพ (วาดในโค้ดทั้งหมด) =======================
fn cv(w: u16, h: u16) -> Image { Image::gen_image_color(w, h, BLANK) }
fn fill(im: &mut Image, x: i32, y: i32, w: i32, h: i32, c: u32) {
    for j in y.max(0)..(y + h).min(im.height as i32) {
        for i in x.max(0)..(x + w).min(im.width as i32) { im.set_pixel(i as u32, j as u32, hx(c)); }
    }
}
fn dot(im: &mut Image, x: i32, y: i32, c: u32) { fill(im, x, y, 1, 1, c); }
fn ellipse(im: &mut Image, cx: f32, cy: f32, rx: f32, ry: f32, c: u32, sh: u32, seed: i32) {
    for j in 0..im.height as i32 {
        for i in 0..im.width as i32 {
            let (dx, dy) = ((i as f32 + 0.5 - cx) / rx, (j as f32 + 0.5 - cy) / ry);
            let d = dx * dx + dy * dy;
            if d <= 1. {
                let n = hash(i, j, seed);
                let col = if dy > 0.35 && n < 0.7 { sh } else if n < 0.12 { sh } else { c };
                im.set_pixel(i as u32, j as u32, hx(col));
            }
        }
    }
}

pub fn swamp_tex(i: i32) -> Image {
    let mut im = mk_tex(0x4f6136, &[0x435430, 0x5c6e3e, 0x3d4a2c], 18, 150 + i, Extra::None);
    // แอ่งน้ำขุ่น
    let (px, py) = ((hash(i, 7, 151) * 9.) as i32 + 2, (hash(7, i, 152) * 9.) as i32 + 3);
    fill(&mut im, px, py, 5, 2, 0x34503f);
    fill(&mut im, px + 1, py - 1, 3, 4, 0x34503f);
    dot(&mut im, px + 1, py, 0x5a7a68);
    im
}
pub fn savanna_tex(i: i32) -> Image { mk_tex(0xc4b25c, &[0xae9c4a, 0xd6c672, 0x9e8c3e], 18, 160 + i, if i < 2 { Extra::Tufts(0x9a883a) } else { Extra::None }) }
pub fn ash_tex(i: i32) -> Image {
    let mut im = mk_tex(0x4a4446, &[0x3a3436, 0x5a5456, 0x433d3f], 22, 170 + i, Extra::None);
    if i % 2 == 0 { dot(&mut im, (hash(i, 3, 171) * 14.) as i32 + 1, (hash(3, i, 172) * 14.) as i32 + 1, 0xc04a20); }
    im
}
/// ลาวา 4 เฟรม: (var&2)|เฟรมกระเพื่อม
pub fn lava_tex(i: i32) -> Image {
    let mut im = Image::gen_image_color(16, 16, hx(0xd8460e));
    let ph = (i & 1) as f32 * 1.7;
    for y in 0..16 {
        for x in 0..16 {
            let v = ((x as f32 * 0.7 + ph).sin() + (y as f32 * 0.9 + x as f32 * 0.3 + (i >> 1) as f32).cos()) * 0.5;
            let c = if v > 0.62 { 0xffd040 } else if v > 0.3 { 0xff8a1c } else if v < -0.75 { 0x7a1c08 } else { 0xd8460e };
            im.set_pixel(x, y, hx(c));
        }
    }
    im
}

fn reeds() -> Image {
    let mut im = cv(16, 18);
    for k in 0..7 {
        let x = 1 + k * 2 + (hash(k, 1, 501) * 2.) as i32;
        let h = 9 + (hash(k, 2, 502) * 7.) as i32;
        let c = if k % 2 == 0 { 0x7aa040 } else { 0x5e8a32 };
        fill(&mut im, x, 18 - h, 1, h, c);
        if k % 3 == 1 { fill(&mut im, x, 18 - h - 3, 1, 3, 0x6a4020); fill(&mut im, x, 18 - h - 4, 1, 1, 0x7aa040); }
        if k % 2 == 1 { dot(&mut im, x + 1, 18 - h / 2, c); dot(&mut im, x + 2, 18 - h / 2 - 1, c); }
    }
    fill(&mut im, 0, 16, 16, 2, 0x3e5a2a);
    im
}
fn swamp_tree() -> Image {
    let mut im = cv(18, 28);
    // ลำต้นบิดงอ + รากโผล่
    for y in 12..26 { let x = 8 + ((y as f32 * 0.5).sin() * 1.5) as i32; fill(&mut im, x, y, 3, 1, 0x4a3a28); dot(&mut im, x, y, 0x382a1c); }
    fill(&mut im, 4, 25, 10, 2, 0x3a2c1e);
    dot(&mut im, 3, 26, 0x3a2c1e); dot(&mut im, 14, 26, 0x3a2c1e);
    ellipse(&mut im, 9., 9., 8.6, 6.5, 0x4a6a32, 0x34502a, 511);
    // ตะไคร่ห้อย
    for k in 0..6 {
        let x = 2 + k * 3;
        let h = 4 + (hash(k, 0, 512) * 6.) as i32;
        fill(&mut im, x, 11, 1, h, 0x7a9a5a);
    }
    im
}
fn shrooms() -> Image {
    let mut im = cv(16, 14);
    for (x, y, w, c) in [(2, 6, 6, 0xd8402a), (9, 3, 6, 0xb87a4a), (6, 9, 4, 0xd8402a)] {
        fill(&mut im, x + w / 2 - 1, y + 2, 2, 14 - y - 2, 0xeee6d8);
        fill(&mut im, x, y, w, 2, c);
        fill(&mut im, x + 1, y - 1, w - 2, 1, c);
        if c == 0xd8402a { dot(&mut im, x + 1, y, 0xffffff); dot(&mut im, x + w - 2, y + 1, 0xffffff); }
    }
    im
}
fn acacia() -> Image {
    let mut im = cv(30, 26);
    fill(&mut im, 14, 10, 2, 16, 0x6a4a2c);
    for (i, y) in (8..13).enumerate() { dot(&mut im, 13 - i as i32, y, 0x6a4a2c); dot(&mut im, 16 + i as i32, y, 0x6a4a2c); }
    // หลังคาแบนกว้าง
    for y in 3..9 {
        let half = 14 - ((y as i32 - 5).abs() * 2);
        for x in 15 - half..15 + half {
            let n = hash(x, y as i32, 521);
            im.set_pixel(x as u32, y as u32, hx(if y >= 7 || n < 0.15 { 0x5a7a2a } else if n > 0.85 { 0x9ab84a } else { 0x7a9a3a }));
        }
    }
    im
}
fn dry_grass() -> Image {
    let mut im = cv(16, 16);
    for k in 0..5 {
        let x = 2 + k * 3;
        let h = 3 + (hash(k, 4, 531) * 4.) as i32;
        let c = if k % 2 == 0 { 0xb89a48 } else { 0x9a7e34 };
        fill(&mut im, x, 14 - h, 1, h, c);
        dot(&mut im, x - 1, 14 - h + 1, c);
        dot(&mut im, x + 1, 14 - h + 2, c);
    }
    im
}
fn ruin_wall() -> Image {
    let mut im = cv(16, 22);
    for row in 0..4 {
        let y = 6 + row * 4;
        let off = if row % 2 == 0 { 0 } else { 4 };
        for b in -1..3 {
            let x = b * 8 + off;
            let c = if hash(b, row, 541) < 0.5 { 0x8e8a80 } else { 0x7e7a70 };
            fill(&mut im, x, y, 7, 3, c);
            fill(&mut im, x, y + 3, 8, 1, 0x5a564e);
        }
    }
    // ยอดหักไม่เรียบ + ตะไคร่
    for x in 0..16 { let top = 6 - (hash(x / 3, 0, 542) * 5.) as i32; fill(&mut im, x, top, 1, 6 - top, 0x8e8a80); dot(&mut im, x, top, 0xa8a49a); }
    for k in 0..5 { dot(&mut im, (hash(k, 1, 543) * 16.) as i32, 8 + (hash(1, k, 544) * 12.) as i32, 0x5e8a40); }
    dot(&mut im, 7, 12, 0x3e3a34); dot(&mut im, 8, 13, 0x3e3a34); dot(&mut im, 8, 14, 0x3e3a34);
    im
}
fn pillar() -> Image {
    let mut im = cv(12, 28);
    fill(&mut im, 1, 0, 10, 3, 0xb0aca2);
    fill(&mut im, 0, 3, 12, 1, 0x7e7a70);
    fill(&mut im, 2, 4, 8, 21, 0x9e9a90);
    fill(&mut im, 2, 4, 2, 21, 0xb8b4aa);
    fill(&mut im, 8, 4, 2, 21, 0x7e7a70);
    fill(&mut im, 0, 25, 12, 3, 0x7e7a70);
    for y in (6..24).step_by(4) { dot(&mut im, 5, y, 0x6e6a62); }
    dot(&mut im, 6, 14, 0x4e4a44); dot(&mut im, 7, 15, 0x4e4a44); dot(&mut im, 6, 16, 0x4e4a44);
    dot(&mut im, 3, 20, 0x5e8a40); dot(&mut im, 4, 21, 0x5e8a40);
    im
}
fn altar() -> Image {
    let mut im = cv(16, 14);
    fill(&mut im, 0, 4, 16, 4, 0xa8a49a);
    fill(&mut im, 0, 8, 16, 1, 0x6e6a62);
    fill(&mut im, 2, 9, 12, 5, 0x8e8a80);
    fill(&mut im, 2, 13, 12, 1, 0x5e5a52);
    // อักขระเรืองแสง
    for (x, y) in [(5, 5), (6, 6), (7, 5), (8, 6), (9, 5), (10, 6)] { dot(&mut im, x, y, 0x6ae8ff); }
    fill(&mut im, 7, 1, 2, 3, 0x9ad8ff);
    dot(&mut im, 7, 0, 0xe0f8ff);
    im
}
fn obsidian() -> Image {
    let mut im = cv(16, 15);
    for (x, y, w, h) in [(2, 7, 5, 8), (6, 2, 5, 13), (10, 6, 4, 9)] {
        fill(&mut im, x, y, w, h, 0x1e1824);
        fill(&mut im, x + 1, y, w - 2, 1, 0x1e1824);
        fill(&mut im, x, y + 1, 1, h - 1, 0x4a3a62);
        dot(&mut im, x + 1, y + 2, 0x8a6ab8);
    }
    fill(&mut im, 1, 14, 14, 1, 0x141018);
    im
}
fn giant() -> Image {
    let mut im = cv(56, 74);
    // ลำต้นใหญ่ + ราก
    fill(&mut im, 20, 40, 16, 30, 0x5a3c22);
    fill(&mut im, 22, 40, 4, 30, 0x6e4c2c);
    fill(&mut im, 32, 40, 3, 30, 0x45301c);
    for k in 0..4 { let x = 14 + k * 8; fill(&mut im, x, 66, 7, 4, 0x4a321e); }
    fill(&mut im, 10, 70, 36, 4, 0x3e2a18);
    fill(&mut im, 25, 52, 5, 7, 0x2a1c10); // โพรง
    // พุ่มใบหลายชั้น
    ellipse(&mut im, 28., 22., 27., 20., 0x2f7a34, 0x23602a, 551);
    ellipse(&mut im, 16., 30., 14., 10., 0x358a3a, 0x23602a, 552);
    ellipse(&mut im, 40., 30., 14., 10., 0x358a3a, 0x23602a, 553);
    ellipse(&mut im, 28., 14., 16., 11., 0x46a048, 0x2f7a34, 554);
    // แสงหิ่งห้อย
    for k in 0..7 { dot(&mut im, (hash(k, 2, 555) * 50.) as i32 + 3, (hash(2, k, 556) * 36.) as i32 + 4, 0xf8f0a0); }
    im
}
/// บ้าน 3 ช่อง (กว้าง 40) — 0 ชาวประมง · 1 ดินเหนียวทะเลทราย · 2 กระท่อมไม้ซุงหิมะ
fn hut(kind: u8) -> Image {
    let mut im = cv(40, 34);
    let (wall, wall2, roof, roof2) = match kind {
        0 => (0x9a6a3c, 0x7e5430, 0xd8b85a, 0xb89a40),
        1 => (0xd8b07a, 0xbe955e, 0xc49a62, 0xa8804c),
        _ => (0x7a5030, 0x5e3c22, 0xf0f4f8, 0xc8d4e0),
    };
    fill(&mut im, 3, 16, 34, 18, wall);
    if kind == 2 {
        for y in (16..34).step_by(3) { fill(&mut im, 3, y + 2, 34, 1, wall2); }
        for y in (16..34).step_by(3) { dot(&mut im, 2, y, wall2); dot(&mut im, 37, y, wall2); }
    } else if kind == 0 {
        for x in (3..37).step_by(4) { fill(&mut im, x, 16, 1, 18, wall2); }
    } else {
        for k in 0..8 { dot(&mut im, 4 + (hash(k, 1, 561) * 32.) as i32, 17 + (hash(1, k, 562) * 15.) as i32, wall2); }
    }
    // ประตู + หน้าต่าง
    fill(&mut im, 17, 23, 7, 11, 0x3a2414);
    dot(&mut im, 22, 28, 0xd8b040);
    fill(&mut im, 7, 21, 6, 5, 0x2a3a4a); fill(&mut im, 28, 21, 6, 5, 0x2a3a4a);
    dot(&mut im, 8, 22, 0xffe08a); dot(&mut im, 29, 22, 0xffe08a);
    if kind == 1 {
        // หลังคาแบน + ขอบ
        fill(&mut im, 1, 12, 38, 4, roof);
        fill(&mut im, 1, 15, 38, 1, roof2);
        for x in (4..38).step_by(6) { fill(&mut im, x, 14, 2, 2, 0x6a4a2c); }
    } else {
        for y in 0..16 {
            let half = 4 + y * 5 / 4;
            let x0 = 20 - half;
            fill(&mut im, x0, y, half * 2, 1, if y % 3 == 2 { roof2 } else { roof });
        }
        if kind == 2 { fill(&mut im, 28, 2, 4, 7, 0x6e6a62); fill(&mut im, 28, 2, 4, 1, 0xf0f4f8); }
    }
    im
}
fn rack() -> Image {
    let mut im = cv(16, 18);
    fill(&mut im, 1, 4, 1, 14, 0x7a5030); fill(&mut im, 14, 4, 1, 14, 0x7a5030);
    fill(&mut im, 0, 4, 16, 1, 0x8a6038);
    for k in 0..4 { let x = 3 + k * 3; fill(&mut im, x, 5, 2, 6, 0x9ab0c0); dot(&mut im, x, 11, 0x6a8090); dot(&mut im, x + 1, 5, 0x404a50); }
    im
}
fn jars() -> Image {
    let mut im = cv(16, 14);
    for (x, w, h, c) in [(1, 6, 10, 0xb8683a), (8, 7, 13, 0xa05a30)] {
        fill(&mut im, x, 14 - h, w, h, c);
        fill(&mut im, x + 1, 14 - h - 1, w - 2, 1, c);
        fill(&mut im, x + 1, 14 - h + 2, w - 2, 1, 0xe0b080);
        fill(&mut im, x + w - 1, 14 - h, 1, h, 0x7a3c20);
    }
    im
}
fn woodpile() -> Image {
    let mut im = cv(16, 12);
    for (x, y) in [(1, 7), (6, 7), (11, 7), (3, 3), (8, 3), (6, 0)] {
        fill(&mut im, x, y, 4, 4, 0x7a5030);
        fill(&mut im, x + 1, y + 1, 2, 2, 0xc8a070);
    }
    fill(&mut im, 3, 0, 8, 1, 0xf0f4f8);
    im
}
fn crystal() -> Image {
    let mut im = cv(16, 20);
    for (x, h, c, c2) in [(3, 12, 0x5ad8ff, 0xc0f4ff), (7, 18, 0x7ab0ff, 0xe0ecff), (11, 10, 0x5ad8ff, 0xc0f4ff)] {
        fill(&mut im, x, 20 - h, 3, h, c);
        dot(&mut im, x + 1, 20 - h - 1, c);
        fill(&mut im, x, 20 - h + 1, 1, h - 2, c2);
    }
    fill(&mut im, 1, 19, 14, 1, 0x2a3a5a);
    im
}
fn ancient_chest() -> Image {
    let mut im = cv(16, 13);
    fill(&mut im, 1, 3, 14, 10, 0x6e6a62);
    fill(&mut im, 1, 1, 14, 3, 0x8e8a80);
    fill(&mut im, 1, 5, 14, 1, 0xd8b040);
    fill(&mut im, 1, 1, 1, 12, 0xd8b040); fill(&mut im, 14, 1, 1, 12, 0xd8b040);
    fill(&mut im, 7, 5, 2, 3, 0xffe080);
    dot(&mut im, 4, 9, 0x5ae8ff); dot(&mut im, 11, 9, 0x5ae8ff);
    im
}
fn bones() -> Image {
    let mut im = cv(16, 12);
    fill(&mut im, 2, 4, 5, 4, 0xe8e2d0); dot(&mut im, 3, 5, 0x3a3428); dot(&mut im, 5, 5, 0x3a3428); fill(&mut im, 3, 8, 3, 1, 0xd0c8b4);
    fill(&mut im, 8, 8, 7, 1, 0xe0dac8); dot(&mut im, 8, 7, 0xe0dac8); dot(&mut im, 14, 9, 0xe0dac8);
    fill(&mut im, 9, 5, 1, 3, 0xd0c8b4);
    im
}

pub fn build_land(at: &mut Atlas, obj: &mut Vec<Rect>) {
    let mut put = |id: u8, im: Image| { obj[id as usize] = at.add(&im); };
    put(O_REED, reeds());
    put(O_SWTREE, swamp_tree());
    put(O_SHROOM, shrooms());
    put(O_ACACIA, acacia());
    put(O_DRYGRASS, dry_grass());
    put(O_RUINWALL, ruin_wall());
    put(O_PILLAR, pillar());
    put(O_ALTAR, altar());
    put(O_OBSIDIAN, obsidian());
    put(O_GIANT, giant());
    put(O_FILL, cv(1, 1));
    put(O_HUT_FISH, hut(0));
    put(O_HUT_SAND, hut(1));
    put(O_HUT_LOG, hut(2));
    put(O_RACK, rack());
    put(O_JAR, jars());
    put(O_WOODPILE, woodpile());
    put(O_CRYSTAL, crystal());
    put(O_ACHEST, ancient_chest());
    put(O_BONES, bones());
}
