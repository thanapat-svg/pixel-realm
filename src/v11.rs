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
        if self.quick_gift_item(k) { return; }
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
    ("สร้างบ้านหลังแรก", "วางเตียง แล้วล้อมด้วยกำแพง+ประตู ให้ครบ — หลังคาจะขึ้นเอง", 10, 10),
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
            3 => self.w.placed.iter().any(|(&k, &o)| o == 33 && self.w.room_from(k).is_some()),
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
/// แถบด้านบน: เป้าหมายผจญภัย 4 ข้อแรก (สอนพื้นฐาน) → จากนั้นภารกิจ "เส้นทาง" (ค่าย→รวมแผ่นดิน) → เป้าหมายผจญภัยที่เหลือ
pub enum TopGoal { Adv(usize), Saga(usize) }
pub fn top_goal(g: &Game) -> Option<TopGoal> {
    if g.tut_text().is_some() { return None; }
    if let Some(k) = g.adv_next() { if k < 4 || !g.rs.inited { return Some(TopGoal::Adv(k)); } }
    if let Some(i) = g.saga_next() { return Some(TopGoal::Saga(i)); }
    g.adv_next().map(TopGoal::Adv)
}
pub fn draw_adv(g: &Game, font: &Font, u: f32) {
    if war::in_combat(g) || g.tut_text().is_some() { return; }
    let Some(goal) = top_goal(g) else { return };
    let cx = hud::col_cx();
    let w = hud::col_wd(420. * u);
    let h = 42. * u;
    let x = cx - w / 2.;
    let y = 8. * u;
    let (col, label, name, hint) = match goal {
        TopGoal::Adv(k) => (hx(0x7ee83a), format!("เป้าหมาย {}/{}", g.adv_done.count_ones() + 1, ADV.len()), ADV[k].0.to_string(), ADV[k].1.to_string()),
        TopGoal::Saga(i) => (hx(0xffcc66), CHAPTERS[MISSIONS[i].0 as usize].0.to_string(), g.mission_name(i), MISSIONS[i].2.to_string()),
    };
    // v41 เป้าหมายใหม่โชว์แผงเต็ม 8 วินาที แล้วพับเหลือป้ายเล็กบรรทัดเดียว (ไม่บังกลางจอด้านบน)
    thread_local! { static SEEN: std::cell::RefCell<(String, f64)> = std::cell::RefCell::new((String::new(), 0.)); }
    let now = get_time();
    let fresh = SEEN.with(|c| { let mut c = c.borrow_mut(); if c.0 != name { *c = (name.clone(), now); } now - c.1 < 8. });
    if !fresh {
        let short = format!("{} · {}", label.split(" · ").next().unwrap_or(&label), name);
        let fs = 11. * u;
        let d = measure_text(&short, Some(font), fs as u16, 1.);
        let pw = (d.width + 16. * u).min(w);
        draw_rectangle(cx - pw / 2., y, pw, 20. * u, Color::new(0.05, 0.07, 0.12, 0.6));
        draw_rectangle(cx - pw / 2., y + 18. * u, pw, 2. * u, col);
        text_c(font, &short, cx, y + 14. * u, fs, Color::new(1., 1., 1., 0.85));
        return;
    }
    draw_rectangle(x, y, w, h, Color::new(0.05, 0.07, 0.12, 0.72));
    draw_rectangle_lines(x, y, w, h, 2., col);
    text(font, &label, x + 8. * u, y + 13. * u, 10. * u, col);
    text_c(font, &name, cx, y + 26. * u, 14. * u, WHITE);
    text_c(font, &hint, cx, y + 38. * u, 10. * u, hx(0xaaaaaa));
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
pub const PET_FOLLOW: u8 = 0;
pub const PET_GUARD: u8 = 1;
pub const PET_ROAM: u8 = 2;
/// ต่ำกว่านี้ = หิวจนอ่อนแอ (เดินช้า ไม่ช่วยสู้)
pub const PET_WEAK: f32 = 25.;
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
        if self.animals.iter().filter(|a| a.tame).count() >= PET_MAX { self.toast("เลี้ยงสัตว์ได้สูงสุด 2 ตัว — ปล่อยตัวเก่าที่แท็บบริวาร (กด N)"); return true; }
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
    /// สัตว์เลี้ยง (v33): ตามตัว/เฝ้าจุด/พักเดินเล่น · หิว→อ่อนแอไม่ช่วยสู้ · หิวนาน/เหงา→หนีกลับป่า · หมาป่ากัดมอนสเตอร์ใกล้ๆ
    pub fn pets_tick(&mut self, dt: f32) {
        let (px, py) = (self.p.x, self.p.y);
        let mut bite: Option<(usize, f32, f32, i32)> = None;
        let mut leave: Vec<usize> = vec![];
        let n = self.animals.len();
        for i in 0..n {
            if !self.animals[i].tame { continue; }
            let k = self.animals[i].k;
            let (ax, ay) = (self.animals[i].x, self.animals[i].y);
            let (role, ppx, ppy, weak) = { let a = &self.animals[i]; (a.role, a.px, a.py, a.sat < PET_WEAK) };
            let (cx, cy, reach) = if role == PET_GUARD { (ppx, ppy, 90.) } else if role == PET_ROAM { (ppx, ppy, 40.) } else { (px, py, 140.) };
            // หาเป้ามอนสเตอร์ (เฉพาะหมาป่าที่ไม่หิวจัด)
            let mut tgt: Option<(f32, f32, usize)> = None;
            if k == A_WOLF && !weak {
                for (j, e) in self.en.iter().enumerate() {
                    if e.dead { continue; }
                    let d = (e.x - ax).hypot(e.y - ay);
                    if d < 90. && (e.x - cx).hypot(e.y - cy) < reach && tgt.map_or(true, |t| d < (t.0 - ax).hypot(t.1 - ay)) { tgt = Some((e.x, e.y, j)); }
                }
            }
            let a = &mut self.animals[i];
            a.anim += dt;
            if a.cd > 0. { a.cd -= dt; }
            if a.flash > 0. { a.flash -= dt; }
            if a.pat > 0. { a.pat -= dt; }
            // ความต้องการ: อิ่ม 100→0 ใน 2.5 วัน · อิ่มพอ ภักดีค่อยๆ เพิ่ม (พักเดินเล่นเพิ่มเร็วกว่า) · อดอยาก ภักดีร่วง
            a.sat = (a.sat - dt * 100. / (DAYLEN * 2.5)).max(0.);
            if a.sat <= 0. { a.loy -= dt * 100. / (DAYLEN * 1.5); }
            else if a.sat >= 60. { a.loy = (a.loy + dt * 100. / (DAYLEN * if role == PET_ROAM { 4. } else { 8. })).min(100.); }
            if role == PET_ROAM && a.hp < AN_STAT[k as usize].0 { a.hp = (a.hp + dt * 0.5).min(AN_STAT[k as usize].0); }
            if a.loy <= 0. { leave.push(i); continue; }
            let dp = (px - ax).hypot(py - ay);
            if role == PET_FOLLOW && dp > 260. {
                a.x = px - 14.; a.y = py + 4.;
                continue;
            }
            let mul = if weak { 0.6 } else { 1. };
            let (tx, ty, spd) = if let Some((ex, ey, j)) = tgt {
                if (ex - ax).hypot(ey - ay) < 12. && a.cd <= 0. { a.cd = 0.8; bite = Some((j, ex - ax, ey - ay, if a.loy >= 70. { 1 } else { 0 })); }
                (ex, ey, 80. * mul)
            } else if role == PET_FOLLOW {
                if dp > 26. { (px - if a.flip { -12. } else { 12. }, py + 2., if dp > 80. { 95. * mul } else { 66. * mul }) } else { (ax, ay, 0.) }
            } else if role == PET_GUARD {
                if (ppx - ax).hypot(ppy - ay) > 6. { (ppx, ppy, 60. * mul) } else { (ax, ay, 0.) }
            } else {
                a.wt -= dt;
                if a.wt <= 0. {
                    a.wt = gen_range(2., 4.);
                    let ang = gen_range(0., std::f32::consts::TAU);
                    let r = gen_range(0., 44.);
                    a.wx = ppx + ang.cos() * r; a.wy = ppy + ang.sin() * r;
                }
                if (a.wx - ax).hypot(a.wy - ay) > 3. { (a.wx, a.wy, 34. * mul) } else { (ax, ay, 0.) }
            };
            let l = (tx - ax).hypot(ty - ay).max(1.);
            let (vx, vy) = if spd > 0. && l > 3. { ((tx - ax) / l * spd, (ty - ay) / l * spd) } else { (0., 0.) };
            a.st = if vx.abs() + vy.abs() > 1. { AS::Wander } else { AS::Idle };
            a.vx = vx; a.vy = vy;
            if vx.abs() > 1. { a.flip = vx < 0.; }
            let (nx, ny) = (ax + vx * dt, ay + vy * dt);
            if !collide(&self.w, nx, ay) { self.animals[i].x = nx; }
            if !collide(&self.w, self.animals[i].x, ny) { self.animals[i].y = ny; }
        }
        if let Some((j, dx, dy, plus)) = bite {
            let l = dx.hypot(dy).max(1.);
            if j < self.en.len() { self.damage_enemy(j, 2 + self.p.lvl / 4 + plus, false, dx / l, dy / l); }
        }
        for i in leave.into_iter().rev() {
            let k = self.animals[i].k;
            let a = &mut self.animals[i];
            a.tame = false; a.st = AS::Flee; a.fed = 150.; a.t = 5.;
            self.toast(format!("{} หิวและเหงาเกินไป หนีกลับป่าไปแล้ว… (ให้อาหาร/ลูบที่แท็บบริวารก่อนจะสายเกินไป)", AN_NAME[k as usize]));
        }
    }
    pub fn pets_save_str(&self) -> String {
        let mut s = String::new();
        for a in self.animals.iter().filter(|a| a.tame) { s += &format!("pet {} {:.0} {:.0} {} {:.1} {:.1} {:.0}\n", a.k, a.sat, a.loy, a.role, a.px, a.py, a.bold * 100.); }
        s
    }
    pub fn pets_load_line(&mut self, rest: &[&str]) {
        let k: u8 = rest.first().and_then(|v| v.parse().ok()).unwrap_or(A_WOLF);
        if self.animals.iter().filter(|a| a.tame).count() >= PET_MAX { return; }
        let f = |i: usize, d: f32| rest.get(i).and_then(|v| v.parse::<f32>().ok()).unwrap_or(d);
        let mut a = self.new_animal(k, self.p.x - 14., self.p.y + 4.);
        a.tame = true;
        a.sat = f(1, 70.).clamp(0., 100.);
        a.loy = f(2, 50.).clamp(1., 100.);
        a.role = (f(3, 0.) as u8).min(2);
        a.px = f(4, a.x); a.py = f(5, a.y);
        if rest.len() > 6 { a.bold = (f(6, 50.) / 100.).clamp(0., 1.); }
        if a.role != PET_FOLLOW { a.x = a.px; a.y = a.py; }
        self.animals.push(a);
    }
}

// ======================= 7) เครื่องมือในมือ =======================
/// 0 ไม่มี · 1 อีเต้อ (หันหน้าเข้าหิน/แร่) · 2 คันเบ็ด (กำลังตกปลา)
pub fn tool_in_hand(g: &Game) -> u8 {
    if g.fish.is_some() { return 2; }
    // v34 Smart Tool: เพิ่งเลือกเครื่องมือให้ตามเป้า (ต้นไม้ = ขวาน · หิน/ภูเขา = อีเต้อ ถ้ามี)
    if g.smart_t > 0. && g.p.weapon == 0 {
        if g.smart_tool == TOOL_AXE { return 3; }
        if g.smart_tool == TOOL_PICK && g.p.pick { return 1; }
    }
    if g.p.pick && g.p.atk_t <= 0. {
        let (tx, ty) = g.facing_tile();
        if matches!(g.w.obj_at(tx, ty), 4 | 5 | O_OBSIDIAN | O_CRYSTAL | O_RUINWALL | O_PILLAR) { return 1; }
    }
    0
}
