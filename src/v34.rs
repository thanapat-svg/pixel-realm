// v34.rs — Smart Tool · ประตูบ้านต้องโล่ง · ปุ่มย้ายบ้านบนจอ
//  1) Smart Tool: ปุ่มโจมตีปุ่มเดียว — ใกล้มอนสเตอร์ = ถืออาวุธต่อสู้ · ใกล้ต้นไม้/หิน/ภูเขา = ถือดาบ(หรือมือเปล่า)
//     แล้วหันเข้าหาของนั้นให้เอง · เดินไปไกลจากของแล้วเจอมอนสเตอร์ = กลับไปถืออาวุธเดิม (เช่น ธนู)
//     เมืองที่ห้ามอาวุธ = ใช้มือเปล่าเอง ไม่โดนปรับ · ปุ่มสลับของในมือเดิมยังอยู่ (สลับเองแล้ว Smart ไม่ยุ่งจนกว่าจะเปลี่ยนเป้า)
//  2) ประตูบ้านต้องโล่ง: วาง/ให้ชาวบ้านสร้างบ้านที่ประตูชนกำแพง/บ้านอื่น = ไม่ได้ (เดิมเข้าบ้านไม่ได้)
//     บ้านที่ประตูถูกบังไปแล้ว = ขึ้นเครื่องหมายแดงที่ประตู + แจ้งตอนเช้า + ย้ายบ้านแก้ได้
//  3) ปุ่ม "ย้าย" บนจอ (ปุ่ม Z): แตะของ/ผนังบ้านแล้วลากไปวาง — ย้ายได้ทั้งของเราและบ้านที่ชาวบ้านสร้าง
use super::*;
use std::collections::VecDeque;

// ======================= 1) Smart Tool =======================
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Work { Tree, Rock, Mountain }
/// ระยะจากตัวถึงกลางช่องที่ Smart Tool ยอมหันไปตี (พิกเซล) — ชิดต้นไม้แล้วยืนเยื้องๆ ก็ยังเอา
pub const SMART_REACH: f32 = 26.;
pub const SMART_TOOL_T: f32 = 1.3;
pub const TOOL_AXE: u8 = 1;
pub const TOOL_PICK: u8 = 2;
pub const TOOL_SWORD: u8 = 3;

fn is_rock_obj(o: u8) -> bool { matches!(o, 4 | 5 | O_OBSIDIAN | O_CRYSTAL | O_RUINWALL | O_PILLAR) }

impl Game {
    /// ช่องนี้เป็นของธรรมชาติที่ตีเก็บได้ไหม (ของที่เราสร้างเอง/ของเมืองอื่น ไม่นับ — กันรื้อบ้านตัวเอง/โดนปรับโดยไม่ตั้งใจ)
    pub fn work_kind(&self, tx: i32, ty: i32) -> Option<Work> {
        if self.w.placed.contains_key(&(tx, ty)) { return None; }
        if self.w.realm.city_at(tx, ty).map_or(false, |ci| self.city_owner(ci) != PLAYER_OWN) { return None; }
        let o = self.w.obj_at(tx, ty);
        let in_dun = self.w.dun.is_some();
        if is_tree(o) { return if in_dun { None } else { Some(Work::Tree) }; }
        if is_rock_obj(o) { return if in_dun && !matches!(o, 4 | 5 | O_CRYSTAL) { None } else { Some(Work::Rock) }; }
        if !in_dun && (o == 0 || o == 3) && self.p.pick && self.w.tile_at(tx, ty) == 6 { return Some(Work::Mountain); }
        None
    }
    /// ช่องที่ตีเมื่อไม่ได้ลากเล็ง: หน้าตัว 11 พิกเซล (เหมือน do_attack)
    pub fn swing_tile(&self) -> (i32, i32) {
        let (dx, dy) = self.p.facing();
        (tfloor(self.p.x + dx * 11.), tfloor(self.p.y - 3. + dy * 11.))
    }
    /// เป้าเก็บของที่ใกล้ที่สุด: ช่องหน้าตัวก่อน แล้วค่อยหาช่องรอบๆ (ให้น้ำหนักด้านที่หันอยู่)
    pub fn smart_work(&self) -> Option<((i32, i32), Work)> {
        let ft = self.swing_tile();
        if let Some(w) = self.work_kind(ft.0, ft.1) { return Some((ft, w)); }
        let (px, py) = (self.p.x, self.p.y - 3.);
        let (fx, fy) = self.p.facing();
        let (cx, cy) = (tfloor(px), tfloor(py));
        let mut best: Option<(f32, (i32, i32), Work)> = None;
        for ty in cy - 2..=cy + 2 {
            for tx in cx - 2..=cx + 2 {
                let Some(w) = self.work_kind(tx, ty) else { continue };
                let (dx, dy) = (tx as f32 * 16. + 8. - px, ty as f32 * 16. + 8. - py);
                let d = dx.hypot(dy);
                if d > SMART_REACH { continue; }
                let cos = (dx * fx + dy * fy) / d.max(0.01);
                let s = d * (1.4 - 0.5 * cos);
                if best.map_or(true, |b| s < b.0) { best = Some((s, (tx, ty), w)); }
            }
        }
        best.map(|b| (b.1, b.2))
    }
    /// มีศัตรูในระยะไหม (ranged = ถือธนู/คทา หรือเพิ่งสลับจากธนูมาตีของ — มองไกลกว่า) · รวมสัตว์ป่าที่ล่าได้ตอนถือธนู
    pub fn smart_foe(&self, ranged: bool) -> bool {
        let r = if ranged { 150. } else { 60. };
        let (px, py) = (self.p.x, self.p.y - 6.);
        if self.en.iter().any(|e| !e.dead && (e.x - px).hypot(e.y - 5. - py) < r) { return true; }
        if self.bosses.iter().any(|b| !b.dead && !b.yld && !b.ally && (b.x - px).hypot(b.y - 12. - py) < r + BOSS_R) { return true; }
        if self.animals.iter().any(|a| !a.tame && matches!(a.st, AS::Chase | AS::Charge) && predator(a.k) && (a.x - px).hypot(a.y - 5. - py) < r) { return true; }
        // ถือธนู: กวาง/กระต่ายที่อยู่ในระยะยิง ไม่ใช่เป้าให้สลับไปตัดไม้
        ranged && self.animals.iter().any(|a| !a.tame && (a.x - px).hypot(a.y - 5. - py) < 130.)
    }
    /// เมืองที่เราอยู่ห้ามอาวุธ
    pub fn weapon_ban(&self) -> bool { self.player_city().map_or(false, |ci| self.law_on(ci, L_WEAPON)) }
    /// ถือมือเปล่าอยู่ (ไม่นับเป็นอาวุธตามกฎเมือง)
    pub fn unarmed(&self) -> bool { self.p.weapon == 0 && (self.p.bare || self.p.sword == 0) }

    /// เรียกตอนกดโจมตีโดยไม่ได้ลากเล็งเอง — เลือกของในมือ/หันหน้าให้เหมาะกับสิ่งที่อยู่ใกล้
    pub fn smart_apply(&mut self) {
        self.smart_tile = None;
        if !self.smart || self.build_sel.is_some() || self.fish.is_some() { return; }
        let cur = self.hand_cur();
        let ranged = cur.0 != 0;
        let ban = self.weapon_ban();
        // สลับเองไปแล้ว (ไม่ตรงกับที่ Smart ตั้งไว้) = ลืมของเดิม ไม่ดึงกลับ
        if self.smart_set.map_or(false, |h| h != cur) { self.smart_prev = None; self.smart_set = None; }
        if self.smart_foe(ranged || self.smart_prev.is_some()) {
            self.smart_t = 0.;
            self.smart_tool = TOOL_SWORD;
            if let Some(h) = self.smart_prev.take() {
                self.smart_set = None;
                if self.hand_options().contains(&h) && (h.0 != 1 || self.p.inv[ARROW] > 0) { self.set_hand(h); }
            } else if cur.1 && self.p.sword > 0 {
                // มือเปล่าแต่มีดาบ เจอศัตรู → หยิบดาบ
                self.set_hand((0, false, 0));
            }
            return;
        }
        if let Some((t, w)) = self.smart_work() {
            let melee = if self.p.sword > 0 && !ban { (0u8, false, 0u8) } else { (0u8, true, 0u8) };
            if cur != melee {
                if cur.0 != 0 && self.smart_prev.is_none() { self.smart_prev = Some(cur); }
                self.set_hand(melee);
                self.smart_set = Some(melee);
            }
            if self.swing_tile() != t {
                let (dx, dy) = (t.0 as f32 * 16. + 8. - self.p.x, t.1 as f32 * 16. + 8. - (self.p.y - 3.));
                self.face(vec2(dx, dy));
            }
            self.smart_tile = Some(t);
            let tool = if w == Work::Tree { TOOL_AXE } else { TOOL_PICK };
            self.smart_tool = tool;
            self.smart_t = SMART_TOOL_T;
            if self.smart_shown != tool {
                self.smart_shown = tool;
                let (x, y) = (self.p.x, self.p.y - 24.);
                let (txt, c) = if tool == TOOL_AXE { ("ขวาน", 0xe0b070) } else { ("อีเต้อ", 0xb8c0d0) };
                self.add_float(x, y, txt, hx(c), None);
            }
            return;
        }
        // ไม่มีเป้า: เมืองห้ามอาวุธ + ถืออาวุธ → เก็บอาวุธก่อนโดนปรับ
        if ban && !cur.1 {
            if self.smart_prev.is_none() { self.smart_prev = Some(cur); }
            self.set_hand((0, true, 0));
            self.smart_set = Some((0, true, 0));
        }
    }
    pub fn smart_tick(&mut self, dt: f32) {
        if self.smart_t > 0. { self.smart_t = (self.smart_t - dt).max(0.); if self.smart_t <= 0. { self.smart_shown = 0; } }
    }
}

// ======================= 2) ประตูบ้านต้องโล่ง =======================
/// ช่องหน้าประตู (นอกบ้าน): บ้านหมุน dir ประตูอยู่ที่ k
pub fn door_front(k: (i32, i32), dir: u8) -> (i32, i32) {
    let o = rot_off((0, 1), dir);
    (k.0 + o.0, k.1 + o.1)
}
const NB4: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

impl Game {
    /// ช่องนี้เดินผ่านไม่ได้: น้ำ/ภูเขา · ของแข็ง (ผนัง ต้นไม้ หิน) · ไซต์ก่อสร้างที่จองไว้ · ตัวบ้านที่กำลังจะวาง
    pub(crate) fn walk_blocked(&self, t: (i32, i32), extra: &HashSet<(i32, i32)>) -> bool {
        if extra.contains(&t) { return true; }
        if SOLIDT[self.w.tile_at(t.0, t.1) as usize] { return true; }
        if OSOLID[self.w.obj_at(t.0, t.1) as usize] { return true; }
        self.site_reserved(t)
    }
    /// จากช่องหน้าประตู f เดินออกไปที่โล่งได้ไหม (ต้องหลุดออกไปห่างอย่างน้อย 3 ช่อง ไม่ใช่ซอกตัน)
    fn open_from(&self, f: (i32, i32), extra: &HashSet<(i32, i32)>) -> bool {
        if self.walk_blocked(f, extra) { return false; }
        let mut seen: HashSet<(i32, i32)> = HashSet::new();
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        seen.insert(f);
        q.push_back(f);
        while let Some(c) = q.pop_front() {
            if (c.0 - f.0).abs().max((c.1 - f.1).abs()) >= 3 { return true; }
            if seen.len() > 80 { return true; }
            for (dx, dy) in NB4 {
                let n = (c.0 + dx, c.1 + dy);
                if seen.contains(&n) || self.walk_blocked(n, extra) { continue; }
                seen.insert(n);
                q.push_back(n);
            }
        }
        false
    }
    /// ประตูบ้านสำเร็จรูป (kind หมุน dir ประตูอยู่ที่ k) มีทางออกหน้าประตูไหม (None = โล่ง)
    pub fn door_clear(&self, kind: u8, k: (i32, i32), dir: u8) -> Option<&'static str> {
        if kind == OBJ_FIRERING { return None; }
        let foot: HashSet<(i32, i32)> = self.prefab_cells(kind, k, dir).into_iter().map(|c| c.0).collect();
        if self.open_from(door_front(k, dir), &foot) { None } else { Some("ประตูจะติดกำแพง/ของอื่น — เว้นทางหน้าประตูให้เดินเข้าออกได้ (ลองหมุนบ้าน)") }
    }
    /// ฝั่งในของประตูที่สร้างเสร็จแล้ว (ช่องข้างประตูที่อยู่ในห้องปิด) — None = ไม่ใช่ประตูบ้าน
    pub(crate) fn door_inside(&self, d: (i32, i32), outside: &mut HashSet<(i32, i32)>) -> Option<(i32, i32)> {
        NB4.iter().map(|&(dx, dy)| (d.0 + dx, d.1 + dy)).find(|&n| self.w.room_from_ex(n, outside).is_some())
    }
    /// ประตูบ้านที่สร้างเสร็จแล้ว (12 ปิด/13 เปิด) ที่ด้านนอกถูกบัง: ฝั่งในคือช่องที่อยู่ในห้อง ฝั่งนอกคือช่องตรงข้าม
    pub fn door_blocked_now(&self, d: (i32, i32)) -> bool {
        let mut outside = HashSet::new();
        self.door_blocked_with(d, &mut outside)
    }
    fn door_blocked_with(&self, d: (i32, i32), outside: &mut HashSet<(i32, i32)>) -> bool {
        let Some(i) = self.door_inside(d, outside) else { return false };
        let out = (d.0 * 2 - i.0, d.1 * 2 - i.1);
        // ฝั่งนอกเป็นห้องปิดอีกห้อง (บ้านติดกัน) = ถือว่าไม่บัง
        if self.w.room_from_ex(out, outside).is_some() { return false; }
        !self.open_from(out, &HashSet::new())
    }
    /// ประตูที่ถูกบังทั้งหมด (เฉพาะบ้านที่เป็นห้องปิดแล้ว)
    pub fn blocked_doors(&self) -> Vec<(i32, i32)> {
        if self.w.dun.is_some() { return vec![]; }
        let mut outside: HashSet<(i32, i32)> = HashSet::new();
        let mut v: Vec<(i32, i32)> = vec![];
        let mut doors: Vec<(i32, i32)> = self.w.placed.iter().filter(|(_, &o)| o == 12 || o == 13).map(|(&k, _)| k).collect();
        doors.sort();
        for d in doors { if self.door_blocked_with(d, &mut outside) { v.push(d); } }
        v
    }
    /// ตรวจประตูถูกบัง: แจ้งเมื่อมีใหม่ (เช้าทุกวัน/หลังย้ายของ)
    pub fn doors_check(&mut self, announce: bool) {
        let now = self.blocked_doors();
        let fresh = now.iter().filter(|d| !self.bad_doors.contains(d)).count();
        self.bad_doors = now;
        if announce && fresh > 0 {
            self.toast(format!("ประตูบ้านถูกบัง {} หลัง (เครื่องหมายแดง) — กดปุ่ม \"ย้าย\" แล้วแตะผนังบ้านเพื่อย้ายออก", self.bad_doors.len()));
        }
    }
    /// ตรวจประตูรอบตัว (ในระยะ ~36 ช่อง) ทุก 4 วินาที — เครื่องหมายแดงขึ้นเองแม้เป็นเซฟเก่า/บ้านชาวบ้านเพิ่งสร้างเสร็จ
    pub fn doors_tick(&mut self, dt: f32) {
        self.door_t -= dt;
        if self.door_t > 0. || self.w.dun.is_some() { return; }
        self.door_t = 4.;
        let (px, py) = (tfloor(self.p.x), tfloor(self.p.y));
        let near = |k: &(i32, i32)| (k.0 - px).abs() <= 36 && (k.1 - py).abs() <= 30;
        let mut doors: Vec<(i32, i32)> = self.w.placed.iter().filter(|(k, &o)| (o == 12 || o == 13) && near(k)).map(|(k, _)| *k).collect();
        doors.sort();
        let mut outside: HashSet<(i32, i32)> = HashSet::new();
        let mut now: Vec<(i32, i32)> = vec![];
        for d in doors { if self.door_blocked_with(d, &mut outside) { now.push(d); } }
        let fresh = now.iter().filter(|d| !self.bad_doors.contains(d)).count();
        self.bad_doors.retain(|k| !near(k));
        self.bad_doors.extend(now);
        self.bad_doors.sort();
        if fresh > 0 { self.toast(format!("ประตูบ้านถูกบัง {} หลัง (เครื่องหมายแดง) — กดปุ่ม \"ย้าย\" แล้วแตะผนังบ้านเพื่อย้ายออก", self.bad_doors.len())); }
    }
    /// ย้ายบ้านทั้งหลัง: ประตูที่ปลายทางต้องโล่ง (ดูจากทิศที่หมุนแล้ว) — None = โล่ง
    pub fn move_door_blocked(&self, c: &mover::Carry, anchor: (i32, i32), dir: u8) -> Option<&'static str> {
        if !c.building { return None; }
        let pairs = self.move_targets(c, anchor, dir);
        let src: HashSet<(i32, i32)> = c.tiles.iter().copied().collect();
        let foot: HashSet<(i32, i32)> = pairs.iter().map(|p| p.1).collect();
        for &(s, d) in &pairs {
            if !matches!(self.w.placed.get(&s), Some(12) | Some(13)) { continue; }
            // ฝั่งนอกของประตูเดิม (เทียบกับช่องในห้อง) แล้วหมุนตามบ้าน
            let Some(i) = self.door_inside(s, &mut HashSet::new()) else { continue };
            let (vx, vy) = (s.0 - i.0, s.1 - i.1);
            let o = rot_off((vx, vy), dir);
            let f = (d.0 + o.0, d.1 + o.1);
            // ช่องเดิมที่ยกออกไปแล้วถือว่าว่าง
            if foot.contains(&f) { return Some("ประตูหันชนตัวบ้านเอง"); }
            let blocked = |t: (i32, i32)| -> bool {
                if foot.contains(&t) { return true; }
                if src.contains(&t) { return false; }
                // v37 ต้นไม้/หินธรรมชาติหน้าประตู ระบบย้ายตัดให้เอง ไม่นับว่าบัง
                self.walk_blocked(t, &HashSet::new()) && !self.nat_clear_at(t)
            };
            if blocked(f) { return Some("ประตูจะติดกำแพง/ของอื่น — เลื่อนหรือหมุนบ้านให้ประตูโล่ง"); }
            let mut seen: HashSet<(i32, i32)> = HashSet::new();
            let mut q: VecDeque<(i32, i32)> = VecDeque::new();
            seen.insert(f); q.push_back(f);
            let mut out = false;
            while let Some(cc) = q.pop_front() {
                if (cc.0 - f.0).abs().max((cc.1 - f.1).abs()) >= 3 || seen.len() > 80 { out = true; break; }
                for (dx, dy) in NB4 { let n = (cc.0 + dx, cc.1 + dy); if seen.contains(&n) || blocked(n) { continue; } seen.insert(n); q.push_back(n); }
            }
            if !out { return Some("ประตูจะติดซอกตัน — เลื่อนหรือหมุนบ้านให้ประตูโล่ง"); }
        }
        None
    }
    /// ลองหมุนบ้านจนประตูโล่ง (ลำดับ: ทิศที่ต้องการก่อน) — ใช้ตอนชาวบ้านเลือกที่/เริ่มสร้าง
    pub fn door_dir_for(&self, kind: u8, k: (i32, i32), want: u8) -> Option<u8> {
        (0..4u8).map(|i| (want + i) & 3).find(|&d| self.prefab_blocked(kind, k, d).is_none())
    }
}

/// เครื่องหมายแดงที่ประตูซึ่งถูกบัง (วาดในโลก)
pub fn draw_bad_doors(g: &Game, v: &View, font: &Font, t: f32) {
    if g.bad_doors.is_empty() { return; }
    let pul = 0.6 + 0.4 * (t * 5.).sin().abs();
    for &(x, y) in &g.bad_doors {
        let (px, py) = ((x as f32 * 16. - v.cx) * v.s, (y as f32 * 16. - v.cy) * v.s);
        if px < -64. || py < -64. || px > screen_width() + 64. || py > screen_height() + 64. { continue; }
        draw_rectangle_lines(px, py, 16. * v.s, 16. * v.s, 2.5, Color::new(1., 0.25, 0.2, pul));
        let r = 7. * v.s;
        let (cx, cy) = (px + 8. * v.s, py - 9. * v.s);
        draw_circle(cx, cy, r, Color::new(0.85, 0.12, 0.1, 0.9));
        draw_circle_lines(cx, cy, r, 2., WHITE);
        let fs = (r * 1.7) as u16;
        let d = measure_text("!", Some(font), fs, 1.);
        draw_text_ex("!", cx - d.width / 2., cy + d.height / 2. - 1., TextParams { font: Some(font), font_size: fs, color: WHITE, ..Default::default() });
    }
}

// ======================= 3) ปุ่มย้ายบนจอ =======================
impl Game {
    /// เปิด/ปิดโหมดย้ายของ (ปุ่ม "ย้าย" บนจอ / Z)
    pub fn move_toggle(&mut self) {
        if self.moving_on() { self.move_cancel(); self.build_sel = None; self.rs.carry = None; return; }
        if self.w.dun.is_some() { self.toast("ย้ายของในดันเจี้ยนไม่ได้"); return; }
        let Some(mi) = PIECES.iter().position(|p| p.obj == OBJ_MOVE) else { return };
        self.build_sel = Some(mi);
        self.build_cur = None;
        self.rs.carry = None;
        self.rs.build_arm = None;
        self.doors_check(false);
        self.toast("ย้ายของ: กดที่ของหรือผนังบ้านค้างไว้แล้วลากไปวาง (ปล่อยนิ้ว = วาง) — ย้ายบ้านชาวบ้านได้ด้วย");
    }
}

#[cfg(debug_assertions)]
impl Game {
    /// debug (Ctrl+F5): สร้างบ้านเล็กเสร็จแล้วข้างตัว + กำแพงชิดหน้าประตู เพื่อดูเครื่องหมายแดง/ทดลองย้าย
    pub fn dbg_blocked_house(&mut self) {
        let k = (tfloor(self.p.x) + 5, tfloor(self.p.y) - 2);
        for ((x, y), c) in self.prefab_cells(OBJ_HOUSE_S, k, 0) {
            self.w.removed.insert((x, y));
            match c {
                Cell::Wall(o) => { self.w.placed.insert((x, y), o); }
                Cell::Floor(t, f) => { self.w.tmod.insert((x, y), t); if f != 0 { self.w.placed.insert((x, y), f); } }
                Cell::Ground(f) => { if f != 0 { self.w.placed.insert((x, y), f); } }
            }
        }
        for dx in -3..=3 { self.w.removed.insert((k.0 + dx, k.1 + 1)); self.w.placed.insert((k.0 + dx, k.1 + 1), 11); }
        self.room_t = 0.;
        self.doors_check(true);
    }
}

pub fn draw_pix_axe(x: f32, y: f32, s: f32) {
    const AXE: [&str; 12] = ["...XXXX.....", "..XaaaaX....", ".XaaaaaaX...", ".XaaaXbXaX..", ".XaaXbX.XX..", "..XXbX......", "....bX......", "...XbX......", "..XbX.......", "..XbX.......", ".XbX........", "XXX........."];
    for (j, r) in AXE.iter().enumerate() {
        for (i, ch) in r.chars().enumerate() {
            let c = match ch { 'X' => hx(0x1e1e22), 'a' => hx(0xb8bcc8), 'b' => hx(0x8a5a2c), _ => continue };
            draw_rectangle((x + i as f32 * s).floor(), (y + j as f32 * s).floor(), s.ceil(), s.ceil(), c);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- นิ้วค้าง (ปุ่มโจมตีค้าง)
    #[test]
    fn stale_touch_detection() {
        use TouchPhase::*;
        // ไม่มีนิ้วที่ id นั้นแล้ว = ค้าง
        assert!(touch_stale(Some(3), &[]));
        assert!(touch_stale(Some(3), &[(1, Stationary)]));
        // ยังกดอยู่ = ไม่ค้าง
        assert!(!touch_stale(Some(3), &[(3, Stationary), (1, Moved)]));
        assert!(!touch_stale(Some(3), &[(3, Moved)]));
        // ปล่อยปกติ (Ended/Cancelled ยังเห็นอยู่ 1 เฟรม) = ให้ทางปกติจัดการ ไม่นับค้าง
        assert!(!touch_stale(Some(3), &[(3, Ended)]));
        assert!(!touch_stale(Some(3), &[(3, Cancelled)]));
        // id เดิมแต่เป็นการแตะใหม่ (Started) = ตัวเก่ายกหายไป → ค้าง
        assert!(touch_stale(Some(3), &[(3, Started)]));
        // ไม่ได้ถืออะไรอยู่ = ไม่มีอะไรค้าง
        assert!(!touch_stale(None, &[]));
    }
    #[test]
    fn release_all_clears_every_held_finger() {
        let mut tu = TouchUI { active: true, joy_id: Some(1), joy_c: Vec2::ZERO, joy_v: vec2(1., 0.), atk_id: Some(2), aim: vec2(0., 1.), flash: [0.; 13], place_id: Some(3), atk_start: Vec2::ZERO, build_id: Some(4) };
        tu.release_all();
        assert!(tu.joy_id.is_none() && tu.atk_id.is_none() && tu.place_id.is_none() && tu.build_id.is_none());
        assert_eq!(tu.joy_v, Vec2::ZERO);
        assert_eq!(tu.aim, Vec2::ZERO);
    }

    // ---- Smart Tool
    /// โลกทดสอบ: หาต้นไม้ธรรมชาติจริงต้นหนึ่งใกล้จุดเกิด ยืนข้างขวาของมัน (ต้นไม้อยู่ซ้ายมือ ตัวหันขวา) · เคลียร์ของอื่นรอบๆ
    fn world() -> (Game, (i32, i32)) {
        let mut g = Game::new_world_seed(5150, Sfx::default());
        let c = (tfloor(g.p.x), tfloor(g.p.y));
        let mut found = None;
        'o: for r in 1..=60i32 { for dy in -r..=r { for dx in -r..=r {
            if dx.abs().max(dy.abs()) != r { continue; }
            let t = (c.0 + dx, c.1 + dy);
            let s = (t.0 + 1, t.1);
            if !is_tree(g.w.nat_obj(t.0, t.1)) || g.w.realm.city_at(t.0, t.1).is_some() { continue; }
            if SOLIDT[g.w.tile_at(t.0, t.1) as usize] || SOLIDT[g.w.tile_at(s.0, s.1) as usize] { continue; }
            if (-3..=3).any(|j| (-3..=3).any(|i| g.w.tile_at(s.0 + i, s.1 + j) == 6)) { continue; }
            found = Some((t, s)); break 'o;
        } } }
        let (t, s) = found.expect("มีต้นไม้ธรรมชาติใกล้จุดเกิด");
        for y in s.1 - 4..=s.1 + 4 { for x in s.0 - 4..=s.0 + 4 { if (x, y) != t { g.w.removed.insert((x, y)); } } }
        g.en.clear(); g.animals.clear(); g.bosses.clear();
        g.p.x = s.0 as f32 * 16. + 8.; g.p.y = s.1 as f32 * 16. + 12.;
        g.p.sword = 2; g.p.bow = true; g.p.inv[ARROW] = 20;
        g.p.weapon = 0; g.p.bare = false;
        g.p.dir = 2; g.p.flip = false;
        (g, t)
    }

    #[test]
    fn near_tree_with_bow_switches_to_sword_and_faces_it() {
        let (mut g, t) = world();
        assert_eq!(g.work_kind(t.0, t.1), Some(Work::Tree), "ต้นไม้ธรรมชาติตีเก็บได้");
        g.p.weapon = 1;
        g.smart_apply();
        assert_eq!((g.p.weapon, g.p.bare), (0, false), "ถือธนู → หยิบดาบมาตัดไม้");
        assert!(g.p.dir == 2 && g.p.flip, "หันเข้าหาต้นไม้ (ซ้าย)");
        assert_eq!(g.smart_tile, Some(t));
        assert_eq!(g.smart_tool, TOOL_AXE);
        assert!(g.smart_prev.is_some(), "จำธนูไว้");
        // ตีจริง: โดนต้นไม้ (ผ่านช่องที่ Smart เลือก แม้ช่องหน้าตัวไม่ใช่)
        let h0 = g.obj_dmg.get(&t).copied().unwrap_or(0);
        g.p.atk_cd = 0.;
        g.do_attack();
        assert!(g.obj_dmg.get(&t).copied().unwrap_or(0) > h0 || g.w.removed.contains(&t), "ตีโดนต้นไม้");
    }
    #[test]
    fn monster_near_returns_to_bow_and_beats_tree() {
        let (mut g, _t) = world();
        g.p.weapon = 1;
        g.smart_apply();
        assert_eq!(g.p.weapon, 0);
        // เจอศัตรูระยะยิง → กลับไปถือธนู ไม่ตัดไม้
        let (x, y) = (g.p.x + 90., g.p.y);
        g.make_enemy(EK::Slime, x, y, 1., 0, false);
        g.smart_apply();
        assert_eq!(g.p.weapon, 1, "ศัตรูมาแล้ว → ธนูกลับมา");
        assert!(g.smart_tile.is_none(), "ศัตรูก่อนต้นไม้");
        assert_eq!(g.smart_tool, TOOL_SWORD);
    }
    #[test]
    fn bare_with_sword_grabs_sword_for_monsters_and_manual_swap_is_respected() {
        let (mut g, _t) = world();
        g.p.weapon = 0; g.p.bare = true; g.p.sword = 2;
        let (x, y) = (g.p.x + 30., g.p.y);
        g.make_enemy(EK::Slime, x, y, 1., 0, false);
        g.smart_apply();
        assert_eq!((g.p.weapon, g.p.bare), (0, false), "มือเปล่าเจอศัตรู → ดาบ");
        // สลับเองหลัง Smart ทำงาน = ลืมของเดิม ไม่ดึงกลับ
        g.en.clear();
        g.p.weapon = 1;
        g.smart_apply();
        assert_eq!(g.p.weapon, 0);
        g.swap_weapon();
        assert!(g.smart_prev.is_none(), "สลับเอง = ลืมของเดิม");
    }
    #[test]
    fn setting_off_leaves_hand_alone() {
        let (mut g, _t) = world();
        g.p.weapon = 1;
        g.smart = false;
        g.smart_apply();
        assert_eq!(g.p.weapon, 1, "ปิด Smart = ไม่ยุ่ง");
        assert!(g.smart_tile.is_none());
    }
    #[test]
    fn own_buildings_are_not_work_targets() {
        let (mut g, t) = world();
        g.w.removed.insert(t);
        let k = (t.0 + 1, t.1 + 1);
        g.w.placed.insert(k, 11);
        assert_eq!(g.work_kind(k.0, k.1), None, "กำแพงที่เราสร้างไม่ถูกตี/รื้อโดย Smart");
        g.p.weapon = 1;
        g.smart_apply();
        assert_eq!(g.p.weapon, 1, "ไม่มีเป้า = ไม่สลับ");
    }
    #[test]
    fn mountain_needs_pick() {
        let (mut g, t) = world();
        g.w.removed.insert(t);
        let k = (t.0 + 1, t.1 - 2);
        g.w.tmod.insert(k, 6);
        g.p.pick = false;
        assert_eq!(g.work_kind(k.0, k.1), None);
        g.p.pick = true;
        assert_eq!(g.work_kind(k.0, k.1), Some(Work::Mountain));
    }
    #[test]
    fn weapon_ban_city_goes_bare_and_unarmed_is_not_fined() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        g.realm_tick(0.01);
        let ci = (0..g.w.realm.cities.len()).find(|&ci| g.law_on(ci, L_WEAPON)).expect("มีเมืองห้ามอาวุธ");
        let c = g.w.realm.cities[ci].clone();
        g.p.x = c.pos.0 as f32 * 16. + 8.; g.p.y = (c.pos.1 + 2) as f32 * 16. + 12.;
        g.en.clear(); g.animals.clear();
        g.p.inv[COIN] = 50;
        g.p.sword = 2; g.p.weapon = 0; g.p.bare = false;
        assert!(g.weapon_ban());
        g.smart_apply();
        assert!(g.p.bare, "เมืองห้ามอาวุธ → เก็บดาบ ใช้มือเปล่า");
        let k = g.city_owner(ci) as usize;
        let rep0 = g.rs.ks[k].rep;
        g.rs.crime_t = 0.;
        g.law_on_attack(false);
        assert_eq!((g.p.inv[COIN], g.rs.ks[k].rep), (50, rep0), "มือเปล่า = ไม่ผิดกฎห้ามอาวุธ");
        // ถือดาบฟันในเมือง = ยังโดนปรับ
        g.p.bare = false;
        g.law_on_attack(false);
        assert!(g.p.inv[COIN] < 50 && g.rs.ks[k].rep < rep0);
    }

    // ---- ประตูบ้าน
    fn town() -> (Game, (i32, i32)) {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let c = (tfloor(g.p.x), tfloor(g.p.y));
        for y in c.1 - 25..=c.1 + 25 { for x in c.0 - 25..=c.0 + 25 {
            g.w.removed.insert((x, y));
            let t = g.w.tile_at(x, y);
            if SOLIDT[t as usize] || t == T_SOIL { g.w.tmod.insert((x, y), 3); }
        } }
        g.en.clear(); g.animals.clear();
        (g, c)
    }
    #[test]
    fn house_door_facing_a_wall_is_refused_and_rotation_fixes_it() {
        let (g0, c) = town();
        let mut g = g0;
        // หาที่วางบ้านเล็กที่โล่งรอบด้าน
        let k = (-6..6).flat_map(|j| (-6..6).map(move |i| (c.0 + i, c.1 + j + 8))).find(|&k| (0..4u8).all(|d| g.prefab_blocked(OBJ_HOUSE_S, k, d).is_none())).expect("มีที่โล่ง");
        // กำแพงยาวขวางหน้าประตู (ทิศใต้ = dir 0): แถวกำแพงติดประตู 1 ช่อง
        for dx in -6..=6 { g.w.placed.insert((k.0 + dx, k.1 + 1), 11); }
        assert!(g.prefab_blocked(OBJ_HOUSE_S, k, 0).is_some(), "ประตูชนกำแพง = วางไม่ได้");
        assert!(g.door_clear(OBJ_HOUSE_S, k, 0).is_some());
        // ประตูหันไปทางอื่นที่โล่ง = ได้ (ถ้าตัวบ้านไม่ชนกำแพง) — ใช้ door_dir_for หาทิศ
        let d = g.door_dir_for(OBJ_HOUSE_S, k, 0);
        if let Some(d) = d { assert_ne!(d, 0); assert!(g.prefab_blocked(OBJ_HOUSE_S, k, d).is_none()); }
    }
    #[test]
    fn door_in_a_dead_end_alley_is_refused() {
        let (mut g, c) = town();
        let k = (c.0, c.1 + 8);
        // ซอกกว้าง 1 ช่องยาว 3 ช่องหน้าประตู (dir 0 = ประตูหันใต้)
        for i in 1..=3 { g.w.placed.insert((k.0 - 1, k.1 + i), 11); g.w.placed.insert((k.0 + 1, k.1 + i), 11); }
        g.w.placed.insert((k.0, k.1 + 4), 11);
        assert!(g.door_clear(OBJ_HOUSE_S, k, 0).is_some(), "ซอกตัน = ประตูโล่งไม่จริง");
        g.w.placed.remove(&(k.0, k.1 + 4));
        assert!(g.door_clear(OBJ_HOUSE_S, k, 0).is_none(), "เปิดปลายซอก = เดินออกได้");
    }
    #[test]
    fn finished_house_with_blocked_door_is_detected_and_move_fixes_it() {
        let (mut g, c) = town();
        let k = (-12..12).flat_map(|j| (-14..6).map(move |i| (c.0 + i, c.1 + j)))
            .find(|&k| g.prefab_blocked(OBJ_HOUSE_S, k, 0).is_none() && g.prefab_blocked(OBJ_HOUSE_S, (k.0 + 8, k.1), 0).is_none())
            .expect("มีที่ว่าง");
        g.p.inv[WOOD] = 80; g.p.inv[STONE] = 20;
        let cost = PIECES.iter().find(|p| p.obj == OBJ_HOUSE_S).unwrap().cost;
        g.p.x = k.0 as f32 * 16. + 8.; g.p.y = (k.1 + 2) as f32 * 16. + 12.;
        assert!(g.place_prefab(OBJ_HOUSE_S, k, 0, cost));
        for _ in 0..(31 * 30) { g.sites_tick(1. / 30.); }
        assert!(g.sites.is_empty());
        g.doors_check(false);
        assert!(g.bad_doors.is_empty(), "ประตูโล่งอยู่ ไม่มีเครื่องหมาย");
        // ภายหลังมีบ้านอีกหลัง/กำแพงมาวางชิดหน้าประตู (เหมือนบ้านชาวบ้านสร้างทับ)
        for dx in -3..=3 { g.w.placed.insert((k.0 + dx, k.1 + 1), 11); }
        g.doors_check(true);
        assert_eq!(g.bad_doors, vec![k], "ประตูถูกบัง → ขึ้นเครื่องหมาย");
        // ย้ายบ้านไปที่ใหม่ (ประตูโล่ง) แก้ได้
        g.rs.carry = None;
        assert!(g.move_pick((k.0 - 2, k.1 - 2)), "แตะผนัง = ยกทั้งหลัง");
        let to = (k.0 + 8, k.1);
        g.p.x = to.0 as f32 * 16. + 8. + 40.; g.p.y = to.1 as f32 * 16. + 8.;
        assert!(g.move_drop(to), "ย้ายไปประตูโล่งได้");
        g.doors_check(false);
        assert!(g.bad_doors.is_empty(), "ย้ายแล้วประตูโล่ง");
    }
    #[test]
    fn nearby_scan_marks_blocked_doors_by_itself_and_clears_when_fixed() {
        let (mut g, c) = town();
        let k = (c.0 + 3, c.1 - 3);
        for ((x, y), cell) in g.prefab_cells(OBJ_HOUSE_S, k, 0) {
            match cell {
                Cell::Wall(o) => { g.w.placed.insert((x, y), o); }
                Cell::Floor(t, f) => { g.w.tmod.insert((x, y), t); if f != 0 { g.w.placed.insert((x, y), f); } }
                Cell::Ground(f) => { if f != 0 { g.w.placed.insert((x, y), f); } }
            }
        }
        g.p.x = c.0 as f32 * 16. + 8.; g.p.y = c.1 as f32 * 16. + 12.;
        g.door_t = 0.;
        g.doors_tick(0.1);
        assert!(g.bad_doors.is_empty(), "ประตูโล่ง ไม่มีเครื่องหมาย");
        g.w.placed.insert((k.0, k.1 + 1), 11);
        g.door_t = 0.;
        g.doors_tick(0.1);
        assert_eq!(g.bad_doors, vec![k], "เซฟเก่า/ของวางทีหลังชิดประตู → ขึ้นเครื่องหมายเอง");
        g.w.placed.remove(&(k.0, k.1 + 1));
        g.door_t = 0.;
        g.doors_tick(0.1);
        assert!(g.bad_doors.is_empty(), "เอาของออก → เครื่องหมายหาย");
    }
    #[test]
    fn move_refuses_to_drop_house_with_door_against_a_wall() {
        let (mut g, c) = town();
        let k = (-12..12).flat_map(|j| (-14..6).map(move |i| (c.0 + i, c.1 + j)))
            .find(|&k| g.prefab_blocked(OBJ_HOUSE_S, k, 0).is_none() && g.prefab_blocked(OBJ_HOUSE_S, (k.0 + 8, k.1), 0).is_none())
            .expect("มีที่ว่าง");
        g.p.inv[WOOD] = 80; g.p.inv[STONE] = 20;
        let cost = PIECES.iter().find(|p| p.obj == OBJ_HOUSE_S).unwrap().cost;
        g.p.x = k.0 as f32 * 16. + 8.; g.p.y = (k.1 + 2) as f32 * 16. + 12.;
        assert!(g.place_prefab(OBJ_HOUSE_S, k, 0, cost));
        for _ in 0..(31 * 30) { g.sites_tick(1. / 30.); }
        assert!(g.move_pick(k), "แตะที่ประตู = ยกทั้งหลัง (จุดอ้างอิง = ประตู)");
        let to = (k.0 + 8, k.1);
        g.p.x = to.0 as f32 * 16. + 8. + 40.; g.p.y = to.1 as f32 * 16. + 8.;
        // กำแพงชิดหน้าประตูปลายทาง
        for dx in -4..=4 { g.w.placed.insert((to.0 + dx, to.1 + 1), 11); }
        let c0 = g.rs.carry.clone().unwrap();
        assert!(g.move_door_blocked(&c0, to, 0).is_some());
        assert!(g.move_blocked(&c0, to, 0).is_some(), "ย้ายแล้วประตูติดกำแพง = วางไม่ได้");
        assert!(!g.move_drop(to));
        // หมุนให้ประตูหันขึ้นเหนือ (กำแพงอยู่ใต้) ก็ใช้ได้ถ้าตัวบ้านไม่ชน
        assert!(g.rs.carry.is_some(), "ยังถืออยู่ เลือกใหม่ได้");
    }

    #[test]
    fn villager_picks_a_door_direction_that_is_clear() {
        let (mut g, c) = town();
        let k = (c.0, c.1 + 6);
        // ทิศ 0 ถูกบัง
        for dx in -4..=4 { g.w.placed.insert((k.0 + dx, k.1 + 1), 11); }
        assert!(g.prefab_blocked(OBJ_HOUSE_S, k, 0).is_some());
        if let Some(d) = g.door_dir_for(OBJ_HOUSE_S, k, 0) { assert!(g.prefab_blocked(OBJ_HOUSE_S, k, d).is_none()); }
    }
}
