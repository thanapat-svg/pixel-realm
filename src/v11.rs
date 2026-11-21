// v11.rs — แตะช่องของใช้ได้ · โหมดสันติ · เป้าหมาย (ความสำเร็จ) · เรือ · สัตว์เลี้ยง · เครื่องมือในมือ
use super::*;

// ======================= 1) แตะช่องของในแถบล่าง =======================
/// ช่องที่ถูกแตะ: 0..8 = ช่องของ · 9 = ช่องอาวุธ
pub fn hotbar_hit(p: Vec2) -> Option<usize> {
    let u = ui_scale();
    let (x0, y0, s) = hotbar_geom(u);
    if p.y < y0 - 2. || p.y > y0 + s + 2. { return None; }
    let wx = x0 - s - 6. * u;
    if p.x >= wx && p.x <= wx + s { return Some(9); }
    if p.x >= x0 && p.x < x0 + s * 9. { return Some(((p.x - x0) / s) as usize); }
    None
}

impl Game {
    pub fn hotbar_use(&mut self, k: usize) {
        if k == 9 { self.swap_weapon(); return; }
        let items = hotbar_items(self);
        let Some(&it) = items.get(k) else { return };
        match it {
            POTION => self.drink(),
            CORN | CARROT => self.eat_crop(it),
            DISH_STEW | DISH_SPEED | DISH_CRIT | DISH_FISH => self.use_dish(it),
            BERRY | MEAT | FISH | SHROOM | HERB => { self.eat_generic(it); }
            FIRE => { if let Some(i) = PIECES.iter().position(|p| p.obj == 9) { self.build_sel = Some(i); self.toast("เลือกวางกองไฟ — กดปุ่มสร้างเพื่อวาง"); } }
            ARROW => { if self.p.bow && self.p.weapon != 1 { self.p.weapon = 1; self.toast("ถือธนู"); } }
            _ => self.toast(format!("{} — ใช้คราฟต์/สร้างของ (เปิดเมนู)", INAME[it])),
        }
    }
}

// ======================= 2) โหมดสันติ =======================
pub fn peaceful() -> bool { diff() == 3 }

// ======================= 3) เป้าหมาย =======================
/// (ชื่อ, วิธีทำ, เหรียญรางวัล, XP)
pub const ADV: [(&str, &str, i32, i32); 16] = [
    ("เก็บไม้ 10 ชิ้น", "ตีต้นไม้ด้วยปุ่มโจมตี", 3, 3),
    ("คราฟต์ดาบไม้", "เมนู → คราฟต์ → ดาบไม้", 5, 5),
    ("วางกองไฟหรือคบเพลิง", "กลางคืนจะปลอดภัยขึ้น", 5, 5),
    ("สร้างเตียง (บ้านหลังแรก)", "เมนู → สร้าง → เตียง แล้ววาง", 8, 8),
    ("คราฟต์ดาบหิน", "ต้องมีดาบไม้ + หิน", 8, 10),
    ("ทำอาหารปรุงสุก", "สร้างหม้อหุงต้ม แล้วกด ใช้", 8, 10),
    ("ล่าสัตว์ตัวแรก", "กวาง/กระต่าย — ใช้ธนูจะง่ายกว่า", 6, 8),
    ("หาแคมป์พ่อค้า", "เดินสำรวจ ดูไอคอนบนแผนที่ (M)", 10, 10),
    ("ชวนชาวบ้านมาอยู่ด้วย", "ช่วยคนหลงทาง แล้วสร้างเตียงให้", 15, 15),
    ("ล่องเรือ", "คราฟต์เรือ แล้วเดินลงน้ำ", 10, 12),
    ("ทำให้สัตว์เชื่อง", "ให้เนื้อหมาป่า / ผลไม้กระต่าย (กด ใช้)", 12, 12),
    ("ค้นพบจุดสำคัญ", "ซากโบราณ วงหิน ต้นไม้ยักษ์ ภูเขาไฟ", 10, 12),
    ("ชนะดันเจี้ยน 1 แห่ง", "เข้าประตูดันเจี้ยน แล้วชนะบอส", 25, 25),
    ("คราฟต์ดาบอัญมณี", "ต้องมีดาบหิน + อัญมณี", 20, 20),
    ("ค้นพบไบโอมครบทุกแบบ", "ดูความคืบหน้าในสมุดบันทึก (สเตตัส)", 50, 40),
    ("ปราบราชันย์เงา (บอสสุดท้าย)", "กลางคืนที่แท่นวงหิน · ชนะดันเจี้ยน 3 แห่ง + แกนโกเลม/เมือกราชา/เศษดาว อย่างละ 3", 100, 100),
];
impl Game {
    pub fn adv_check_one(&self, k: usize) -> bool {
        let p = &self.p;
        match k {
            0 => p.inv[WOOD] >= 10 || p.sword >= 1,
            1 => p.sword >= 1,
            2 => self.w.placed.values().any(|&o| o == 9 || o == 18),
            3 => self.w.placed.values().any(|&o| o == 33),
            4 => p.sword >= 2,
            5 => (DISH_STEW..=DISH_FISH).any(|d| p.inv[d] > 0),
            6 => self.an_hunted != 0,
            7 => !self.camps_found.is_empty(),
            8 => self.npcs.iter().any(|n| n.state == ST_HOME),
            9 => self.boated,
            10 => self.animals.iter().any(|a| a.tame),
            11 => !self.marks_found.is_empty(),
            12 => !self.dun_clear.is_empty(),
            13 => p.sword >= 3,
            14 => self.codex_done & 1 != 0,
            _ => self.final_won,
        }
    }
    pub fn adv_check(&mut self) {
        for k in 0..ADV.len() {
            if self.adv_done & (1 << k) != 0 || !self.adv_check_one(k) { continue; }
            self.adv_done |= 1 << k;
            let (n, _, c, x) = ADV[k];
            self.p.inv[COIN] += c;
            self.p.xp += x;
            self.check_lv();
            self.toast(format!("เป้าหมายสำเร็จ: {}! (+{} เหรียญ)", n, c));
            play(&self.sfx.lvl);
        }
    }
    /// เป้าหมายถัดไปที่ยังไม่สำเร็จ
    pub fn adv_next(&self) -> Option<usize> { (0..ADV.len()).find(|&k| self.adv_done & (1 << k) == 0) }
}
/// แผงเป้าหมายด้านบน (แสดงเมื่อบทเรียนจบแล้ว)
pub fn draw_adv(g: &Game, font: &Font, u: f32) {
    if g.tut_text().is_some() { return; }
    let Some(k) = g.adv_next() else { return };
    let sw = screen_width();
    let w = (420. * u).min(sw * 0.38);
    let h = 42. * u;
    let x = sw / 2. - w / 2.;
    let y = 8. * u;
    draw_rectangle(x, y, w, h, Color::new(0.05, 0.07, 0.12, 0.72));
    draw_rectangle_lines(x, y, w, h, 2., hx(0x7ee83a));
    let n = g.adv_done.count_ones();
    text(font, &format!("เป้าหมาย {}/{}", n + 1, ADV.len()), x + 8. * u, y + 13. * u, 10. * u, hx(0x9cff5a));
    text_c(font, ADV[k].0, sw / 2., y + 26. * u, 14. * u, WHITE);
    text_c(font, ADV[k].1, sw / 2., y + 38. * u, 10. * u, hx(0xaaaaaa));
}

// ======================= 5) เรือ =======================
/// ชนกำแพงสำหรับผู้เล่น: มีเรือ = ลงน้ำได้ (ยกเว้นมีของขวาง)
pub fn collide_p(w: &World, x: f32, y: f32, boat: bool) -> bool {
    if !boat || w.dun.is_some() { return collide(w, x, y); }
    let (l, r, t, b) = (tfloor(x - 4.), tfloor(x + 4.), tfloor(y - 4.), tfloor(y));
    [(l, t), (r, t), (l, b), (r, b)].iter().any(|&(tx, ty)| {
        if w.tile_at(tx, ty) < 2 { w.obj_at(tx, ty) != 0 && OSOLID[w.obj_at(tx, ty) as usize] } else { w.solid_at(tx, ty) }
    })
}
impl Game {
    pub fn on_boat(&self) -> bool { self.p.boat && self.w.dun.is_none() && self.w.tile_at(tfloor(self.p.x), tfloor(self.p.y - 2.)) < 2 }
}
/// เรือไม้ใต้ตัวผู้เล่น (วาดด้วยสี่เหลี่ยม)
pub fn draw_boat(v: &View, sx: f32, sy: f32, t: f32, side: bool) {
    let bob = (t * 3.).sin() * 0.6;
    let (w, h) = if side { (18., 6.) } else { (10., 12.) };
    let (x, y) = (sx - w / 2., sy - h + 3. + bob);
    prect(v, x + 1., y + h, w - 2., 1., Color::new(0.85, 0.95, 1., 0.5));
    prect(v, x, y, w, h, hx(0x6a4024));
    prect(v, x + 1., y + 1., w - 2., h - 2., hx(0x9a6a3a));
    prect(v, x + 2., y + 2., w - 4., h - 4., hx(0x5a3418));
    if side { prect(v, x, y, 2., 2., hx(0x4a2a12)); prect(v, x + w - 2., y, 2., 2., hx(0x4a2a12)); }
}

// ======================= 6) สัตว์เลี้ยง =======================
pub const PET_MAX: usize = 2;
impl Game {
    /// ให้อาหารสัตว์ตรงหน้าเพื่อทำให้เชื่อง (คืน true ถ้ามีสัตว์ให้ป้อน)
    pub fn try_tame(&mut self) -> bool {
        let (fx, fy) = self.p.facing();
        let (cx, cy) = (self.p.x + fx * 10., self.p.y - 4. + fy * 10.);
        let Some(i) = self.animals.iter().position(|a| !a.tame && matches!(a.k, A_WOLF | A_RABBIT) && (a.x - cx).hypot(a.y - 5. - cy) < 22.) else { return false };
        let k = self.animals[i].k;
        let food = if k == A_WOLF { [MEAT, MEAT] } else { [CARROT, BERRY] };
        let Some(&f) = food.iter().find(|&&f| self.p.inv[f] > 0) else {
            self.toast(if k == A_WOLF { "หมาป่าชอบเนื้อ — ถือเนื้อแล้วกด ใช้ เพื่อป้อน" } else { "กระต่ายชอบแครอท/ผลไม้ — มีแล้วกด ใช้ เพื่อป้อน" });
            return true;
        };
        if self.animals.iter().filter(|a| a.tame).count() >= PET_MAX { self.toast("เลี้ยงสัตว์ได้สูงสุด 2 ตัว"); return true; }
        self.p.inv[f] -= 1;
        let (x, y) = (self.animals[i].x, self.animals[i].y);
        let chance = if k == A_WOLF { 0.35 } else { 0.5 };
        if gen_range(0., 1.) < chance {
            let a = &mut self.animals[i];
            a.tame = true;
            a.st = AS::Idle;
            a.hp = AN_STAT[k as usize].0;
            a.fed = 0.;
            self.burst(x, y - 8., hx(0xff6aa0), 14, 50.);
            self.toast(if k == A_WOLF { "หมาป่าเชื่องแล้ว! มันจะตามคุณและช่วยสู้" } else { "กระต่ายเชื่องแล้ว! มันจะตามคุณไปทุกที่" });
            play(&self.sfx.lvl);
            self.save();
        } else {
            self.burst(x, y - 8., hx(0xaaaaaa), 6, 30.);
            self.toast(format!("{}กินแล้ว แต่ยังไม่ไว้ใจ — ลองป้อนอีก", AN_NAME[k as usize]));
        }
        true
    }
    /// สัตว์เลี้ยง: เดินตาม · วาร์ปมาถ้าห่างเกิน · หมาป่ากัดมอนสเตอร์ใกล้ๆ
    pub fn pets_tick(&mut self, dt: f32) {
        let (px, py) = (self.p.x, self.p.y);
        let mut bite: Option<(usize, f32, f32)> = None;
        let n = self.animals.len();
        for i in 0..n {
            if !self.animals[i].tame { continue; }
            let k = self.animals[i].k;
            let (ax, ay) = (self.animals[i].x, self.animals[i].y);
            // หาเป้ามอนสเตอร์ (เฉพาะหมาป่า)
            let mut tgt: Option<(f32, f32, usize)> = None;
            if k == A_WOLF {
                for (j, e) in self.en.iter().enumerate() {
                    let d = (e.x - ax).hypot(e.y - ay);
                    if d < 90. && (e.x - px).hypot(e.y - py) < 140. && tgt.map_or(true, |t| d < (t.0 - ax).hypot(t.1 - ay)) { tgt = Some((e.x, e.y, j)); }
                }
            }
            let a = &mut self.animals[i];
            a.anim += dt;
            if a.cd > 0. { a.cd -= dt; }
            if a.flash > 0. { a.flash -= dt; }
            let dp = (px - ax).hypot(py - ay);
            if dp > 260. {
                a.x = px - 14.; a.y = py + 4.;
                continue;
            }
            let (tx, ty, spd) = if let Some((ex, ey, j)) = tgt {
                if (ex - ax).hypot(ey - ay) < 12. && a.cd <= 0. { a.cd = 0.8; bite = Some((j, ex - ax, ey - ay)); }
                (ex, ey, 80.)
            } else if dp > 26. { (px - if a.flip { -12. } else { 12. }, py + 2., if dp > 80. { 95. } else { 66. }) } else { (ax, ay, 0.) };
            let l = (tx - ax).hypot(ty - ay).max(1.);
            let (vx, vy) = if spd > 0. && l > 3. { ((tx - ax) / l * spd, (ty - ay) / l * spd) } else { (0., 0.) };
            a.st = if vx.abs() + vy.abs() > 1. { AS::Wander } else { AS::Idle };
            a.vx = vx; a.vy = vy;
            if vx.abs() > 1. { a.flip = vx < 0.; }
            let (nx, ny) = (ax + vx * dt, ay + vy * dt);
            if !collide(&self.w, nx, ay) { self.animals[i].x = nx; }
            if !collide(&self.w, self.animals[i].x, ny) { self.animals[i].y = ny; }
        }
        if let Some((j, dx, dy)) = bite {
            let l = dx.hypot(dy).max(1.);
            if j < self.en.len() { self.damage_enemy(j, 2 + self.p.lvl / 4, false, dx / l, dy / l); }
        }
    }
    pub fn pets_save_str(&self) -> String {
        let mut s = String::new();
        for a in self.animals.iter().filter(|a| a.tame) { s += &format!("pet {}\n", a.k); }
        s
    }
    pub fn pets_load_line(&mut self, rest: &[&str]) {
        let k: u8 = rest.first().and_then(|v| v.parse().ok()).unwrap_or(A_WOLF);
        if self.animals.iter().filter(|a| a.tame).count() >= PET_MAX { return; }
        let mut a = self.new_animal(k, self.p.x - 14., self.p.y + 4.);
        a.tame = true;
        self.animals.push(a);
    }
}

// ======================= 7) เครื่องมือในมือ =======================
/// 0 ไม่มี · 1 อีเต้อ (หันหน้าเข้าหิน/แร่) · 2 คันเบ็ด (กำลังตกปลา)
pub fn tool_in_hand(g: &Game) -> u8 {
    if g.fish.is_some() { return 2; }
    if g.p.pick && g.p.atk_t <= 0. {
        let (tx, ty) = g.facing_tile();
        if matches!(g.w.obj_at(tx, ty), 4 | 5 | O_OBSIDIAN | O_CRYSTAL | O_RUINWALL | O_PILLAR) { return 1; }
    }
    0
}
