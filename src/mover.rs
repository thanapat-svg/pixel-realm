// mover.rs — v28 ย้ายของที่สร้างแล้ว (ไม่ต้องรื้อแล้วสร้างใหม่)
//  v31 ใช้ง่ายขึ้น: กดที่ของค้างไว้แล้วลากไปวาง (ปล่อยนิ้ว = วาง) · หรือแตะของ 1 ที แล้วแตะที่ใหม่ 1 ที
//  ระยะย้ายไกลกว่าระยะสร้าง (เกือบทั้งจอ) · ปุ่ม "ย้ายของ" อยู่บนสุดของเมนูสร้าง
//  - ของชิ้นเดียว (เฟอร์นิเจอร์ กล่อง เตียง ตะเกียง บ่อน้ำ รั้ว ฯลฯ): ย้ายไปพร้อมของในกล่อง/เจ้าของเตียง
//  - แตะผนัง/ประตู/หน้าต่าง/พื้นของบ้านที่ปิดเป็นห้อง = ย้าย "ทั้งหลัง" (ผนัง พื้น เฟอร์นิเจอร์ กล่อง เตียง) · หมุนได้ 4 ทิศ
//  - ระหว่างยก ของยังอยู่ที่เดิม (เซฟกลางทางไม่หาย) · ย้ายจริงทีเดียวตอนวาง · ยกเลิก = อยู่ที่เดิม
use super::*;

pub const OBJ_MOVE: u8 = 245;
/// ระยะย้ายของ (ช่อง) — ไกลกว่าระยะสร้าง จะได้ไม่ต้องเดินตามของ
pub const MOVE_REACH: f32 = 14.;

#[derive(Clone, Debug)]
pub struct Carry {
    /// ช่องที่แตะตอนยก (จุดหมุน/จุดอ้างอิง)
    pub src: (i32, i32),
    /// ช่องทั้งหมดที่จะย้าย (พิกัดจริงที่เดิม)
    pub tiles: Vec<(i32, i32)>,
    pub building: bool,
    pub name: String,
}

fn structural(o: u8) -> bool { matches!(o, 10 | 11 | 12 | 13 | O_WINDOW) }
fn is_floor(t: u8) -> bool { t == T_FLOORW || t == T_FLOORS }
/// ของที่ย้ายได้: ทุกอย่างที่ผู้เล่นวางเอง ยกเว้นแปลงผัก
fn movable(o: u8) -> bool { !(27..=32).contains(&o) }

impl Game {
    /// เลือกว่าจะยกอะไร จากช่องที่แตะ (Err = เหตุผลที่ยกไม่ได้)
    pub fn move_select(&self, k: (i32, i32)) -> Result<Carry, &'static str> {
        if self.w.dun.is_some() { return Err("ย้ายของในดันเจี้ยนไม่ได้"); }
        if self.site_reserved(k) { return Err("กำลังก่อสร้างอยู่ — รอสร้างเสร็จก่อน"); }
        let placed = self.w.placed.get(&k).copied();
        let floor = is_floor(self.w.tile_at(k.0, k.1)) && self.w.tmod.contains_key(&k);
        let is_struct = placed.map_or(false, structural) || (floor && placed.is_none());
        if is_struct {
            // ผนัง/ประตู/พื้นของห้องที่ปิดสนิท = ย้ายทั้งหลัง
            // รวมแนวทแยงด้วย: ผนังมุมบ้านติดกับช่องในบ้านแค่ทางทแยง
            let starts = [k, (k.0, k.1 - 1), (k.0, k.1 + 1), (k.0 - 1, k.1), (k.0 + 1, k.1), (k.0 - 1, k.1 - 1), (k.0 + 1, k.1 - 1), (k.0 - 1, k.1 + 1), (k.0 + 1, k.1 + 1)];
            let room = starts.iter().find_map(|s| self.w.room_from(*s));
            if let Some(r) = room {
                let mut tiles: Vec<(i32, i32)> = r.tiles.iter().chain(r.walls.iter()).chain(r.doors.iter()).copied().collect();
                tiles.sort(); tiles.dedup();
                // ผนังต้องเป็นของที่สร้างเองทั้งหมด (บ้านที่พิงหน้าผา/หินธรรมชาติ ย้ายทั้งหลังไม่ได้)
                if r.walls.iter().chain(r.doors.iter()).any(|w| !self.w.placed.contains_key(w)) { return Err("บ้านหลังนี้ใช้หิน/ภูเขาธรรมชาติเป็นผนัง — ย้ายทั้งหลังไม่ได้"); }
                if tiles.iter().any(|t| self.site_reserved(*t)) { return Err("กำลังก่อสร้างอยู่ — รอสร้างเสร็จก่อน"); }
                if tiles.iter().any(|t| self.crops.contains_key(t)) { return Err("มีแปลงผักในบ้าน — เก็บเกี่ยว/รื้อแปลงก่อน"); }
                let n = tiles.iter().filter(|t| self.w.placed.contains_key(*t)).count();
                return Ok(Carry { src: k, tiles, building: true, name: format!("บ้านทั้งหลัง ({} ชิ้น)", n) });
            }
        }
        self.move_single(k)
    }
    fn move_single(&self, k: (i32, i32)) -> Result<Carry, &'static str> {
        match self.w.placed.get(&k) {
            Some(&o) if movable(o) => Ok(Carry { src: k, tiles: vec![k], building: false, name: obj_name(o).to_string() }),
            Some(_) => Err("แปลงผักย้ายไม่ได้ — ใช้ไถดินวางซ้ำเพื่อรื้อ"),
            None if is_floor(self.w.tile_at(k.0, k.1)) && self.w.tmod.contains_key(&k) => Ok(Carry { src: k, tiles: vec![k], building: false, name: "พื้น".to_string() }),
            None => Err("ตรงนี้ไม่มีของที่คุณสร้าง — แตะของที่สร้างแล้ว (ต้นไม้/หินธรรมชาติย้ายไม่ได้)"),
        }
    }
    /// ปลายทางของแต่ละช่อง (หมุนได้เฉพาะทั้งหลัง)
    pub fn move_targets(&self, c: &Carry, anchor: (i32, i32), dir: u8) -> Vec<((i32, i32), (i32, i32))> {
        let d = if c.building { dir } else { 0 };
        c.tiles.iter().map(|&s| { let o = rot_off((s.0 - c.src.0, s.1 - c.src.1), d); (s, (anchor.0 + o.0, anchor.1 + o.1)) }).collect()
    }
    /// วางที่ anchor ได้ไหม (None = ได้)
    pub fn move_blocked(&self, c: &Carry, anchor: (i32, i32), dir: u8) -> Option<&'static str> {
        if !self.move_reach(anchor) { return Some("ไกลเกินไป — เดินเข้าไปใกล้อีกหน่อย"); }
        let pairs = self.move_targets(c, anchor, dir);
        if pairs.iter().all(|(s, d)| s == d) { return Some("ยังอยู่ที่เดิม — เลือกที่ใหม่"); }
        let src: HashSet<(i32, i32)> = c.tiles.iter().copied().collect();
        for &(s, d) in &pairs {
            if src.contains(&d) { continue; }
            let t = self.w.tile_at(d.0, d.1);
            if SOLIDT[t as usize] { return Some("ทับน้ำ/ภูเขา"); }
            if self.w.realm.city_at(d.0, d.1).map_or(false, |ci| self.city_owner(ci) != PLAYER_OWN) { return Some("อยู่ในเมืองคนอื่น"); }
            if self.site_reserved(d) { return Some("ทับที่กำลังก่อสร้าง"); }
            if t == T_SOIL || t == 12 || self.crops.contains_key(&d) { return Some("ทับแปลงผัก"); }
            if self.w.tmod.get(&s).map_or(false, |x| is_floor(*x)) && self.w.tmod.contains_key(&d) && !is_floor(t) { return Some("ปูพื้นทับช่องนี้ไม่ได้"); }
            let o = self.w.obj_at(d.0, d.1);
            if o != 0 && o != 3 { return Some("มีของขวางอยู่"); }
            let so = self.w.placed.get(&s).copied().unwrap_or(0);
            if OSOLID[so as usize] && self.player_in_tile(d.0, d.1) { return Some("ถอยออกจากตรงนั้นก่อน"); }
        }
        None
    }
    /// ยกขึ้น (ยังไม่ขยับของจริง)
    pub fn move_pick(&mut self, k: (i32, i32)) -> bool {
        match self.move_select(k) {
            Ok(c) => {
                self.toast(format!("ยกแล้ว: {} — ลากไปวาง หรือแตะที่ใหม่{}", c.name, if c.building { " · ปุ่มหมุน = หันบ้าน" } else { "" }));
                self.build_dir = 0;
                self.rs.carry = Some(c);
                play(&self.sfx.click);
                true
            }
            Err(e) => { self.toast(e); play(&self.sfx.deny); false }
        }
    }
    /// วางที่ใหม่: ย้ายทุกอย่างทีเดียว (ของ พื้น ของในกล่อง เจ้าของเตียง ประตูอัตโนมัติ)
    pub fn move_drop(&mut self, anchor: (i32, i32)) -> bool {
        let Some(c) = self.rs.carry.clone() else { return false };
        if let Some(why) = self.move_blocked(&c, anchor, self.build_dir) { self.toast(why); play(&self.sfx.deny); return false; }
        let pairs = self.move_targets(&c, anchor, self.build_dir);
        let mut objs = vec![];
        let mut floors = vec![];
        let mut chests = vec![];
        let mut autos = vec![];
        for &(s, d) in &pairs {
            if let Some(o) = self.w.placed.remove(&s) { objs.push((d, o)); }
            if let Some(&t) = self.w.tmod.get(&s) { if is_floor(t) { self.w.tmod.remove(&s); floors.push((d, t)); } }
            if let Some(inv) = self.chests.remove(&s) { chests.push((d, inv)); }
            if self.auto_doors.remove(&s) { autos.push(d); }
            self.obj_dmg.remove(&s);
        }
        let bed_map: HashMap<(i32, i32), (i32, i32)> = pairs.iter().copied().collect();
        for (d, t) in floors {
            if self.w.obj_at(d.0, d.1) == 3 && !self.w.placed.contains_key(&d) { self.w.removed.insert(d); }
            self.w.tmod.insert(d, t);
        }
        for (d, o) in objs {
            if self.w.obj_at(d.0, d.1) == 3 { self.w.removed.insert(d); }
            self.w.placed.insert(d, o);
            self.obj_dmg.remove(&d);
        }
        for (d, inv) in chests { self.chests.insert(d, inv); }
        for d in autos { self.auto_doors.insert(d); }
        // เจ้าของเตียงตามไปด้วย
        for n in self.npcs.iter_mut() {
            if let Some(&d) = bed_map.get(&n.bed) { if self.w.placed.get(&d) == Some(&33) { n.bed = d; self.rs.sleeping.remove(&n.id); } }
        }
        self.rs.work.clear();
        self.rs.carry = None;
        self.rs.build_arm = None;
        self.room_t = 0.;
        self.refresh_torches();
        let (x, y) = (anchor.0 as f32 * 16. + 8., anchor.1 as f32 * 16. + 8.);
        self.burst(x, y, hx(0xc8a070), 12, 50.);
        play(&self.sfx.door);
        self.toast(format!("ย้าย{}เรียบร้อย", c.name));
        self.save();
        true
    }
    pub fn move_cancel(&mut self) -> bool {
        if self.rs.carry.take().is_some() { self.rs.build_arm = None; self.toast("ยกเลิกการย้าย — ของอยู่ที่เดิม"); true } else { false }
    }
    pub fn move_reach(&self, c: (i32, i32)) -> bool {
        (c.0 as f32 * 16. + 8. - self.p.x).hypot(c.1 as f32 * 16. + 8. - (self.p.y - 3.)) <= MOVE_REACH * 16.
    }
    /// กดลงบนจอ (โหมดย้ายของ): ยังไม่ได้ยก = ยกของตรงนั้นทันที · ยกอยู่ = เริ่มเลือกที่วาง
    pub fn move_press(&mut self, t: (i32, i32)) {
        self.build_cur = Some(t);
        if self.rs.carry.is_none() {
            if !self.move_reach(t) { self.toast("ไกลเกินไป — เดินเข้าไปใกล้อีกหน่อย"); play(&self.sfx.deny); self.rs.move_press = None; return; }
            let ok = self.move_pick(t);
            self.rs.move_press = if ok { Some((t, true)) } else { None };
        } else {
            self.rs.move_press = Some((t, false));
        }
    }
    /// ปล่อยนิ้ว/เมาส์: ลากไปที่ใหม่ = วาง · แตะเฉยๆ ตอนยก = ยกค้างไว้ (แตะที่ใหม่อีกทีเพื่อวาง)
    pub fn move_release(&mut self, t: (i32, i32)) {
        let Some((start, fresh)) = self.rs.move_press.take() else { return };
        if self.rs.carry.is_none() { return; }
        if fresh && t == start { return; }
        self.build_cur = Some(t);
        self.move_drop(t);
    }
    pub fn moving_on(&self) -> bool { self.build_sel.map_or(false, |s| PIECES[s].obj == OBJ_MOVE) }
}

fn obj_name(o: u8) -> &'static str {
    if let Some(p) = PIECES.iter().find(|p| p.obj == o) { return p.name; }
    match o { 9 => "กองไฟ", 12 | 13 => "ประตู", 33 => "เตียง", 14 => "กล่องเก็บของ", _ => "ของ" }
}

/// พรีวิว: ช่องเดิมขอบฟ้า · ปลายทางเป็นภาพจางสีเขียว (วางได้) / แดง (มีเหตุผลบอก)
pub fn draw_move_preview(g: &Game, sp: &Sprites, v: &View, font: &Font, t_now: f32) {
    let (tx, ty) = g.build_target();
    let Some(c) = &g.rs.carry else {
        // ยังไม่ได้ยก: ไฮไลต์สิ่งที่จะถูกยกถ้าแตะตรงนี้
        let (lc, list) = match g.move_select((tx, ty)) { Ok(c) => (Color::new(0.4, 0.85, 1., 0.9), c.tiles), Err(_) => (Color::new(1., 0.4, 0.4, 0.7), vec![(tx, ty)]) };
        for &(x, y) in &list { draw_rectangle_lines((x as f32 * 16. - v.cx) * v.s, (y as f32 * 16. - v.cy) * v.s, 16. * v.s, 16. * v.s, 2., lc); }
        return;
    };
    let pul = 0.55 + 0.25 * (t_now * 4.).sin();
    for &(x, y) in &c.tiles { draw_rectangle_lines((x as f32 * 16. - v.cx) * v.s, (y as f32 * 16. - v.cy) * v.s, 16. * v.s, 16. * v.s, 1.5, Color::new(0.4, 0.85, 1., pul)); }
    let bad = g.move_blocked(c, (tx, ty), g.build_dir);
    let tint = if bad.is_none() { Color::new(0.75, 1., 0.75, 0.6) } else { Color::new(1., 0.55, 0.55, 0.55) };
    for (s, d) in g.move_targets(c, (tx, ty), g.build_dir) {
        let (px, py) = (d.0 as f32 * 16. - v.cx, d.1 as f32 * 16. - v.cy);
        if let Some(&t) = g.w.tmod.get(&s) { if is_floor(t) { blit(sp, v, sp.tiles[t as usize][0], px, py, false, tint); } }
        if let Some(&o) = g.w.placed.get(&s) { let r = sp.obj[o as usize]; blit(sp, v, r, px + 8. - r.w / 2., py + 16. - r.h, false, tint); }
        else { prect(v, px, py, 16., 16., Color::new(tint.r, tint.g, tint.b, 0.12)); }
        draw_rectangle_lines(px * v.s, py * v.s, 16. * v.s, 16. * v.s, 1., if bad.is_none() { Color::new(0.3, 1., 0.3, 0.6) } else { Color::new(1., 0.3, 0.3, 0.6) });
    }
    if let Some(why) = bad {
        let (mx, my) = (((tx as f32) * 16. + 8. - v.cx) * v.s, ((ty + 1) as f32 * 16. + 4. - v.cy) * v.s);
        let fs = (6. * v.s) as u16;
        let d = measure_text(why, Some(font), fs, 1.);
        draw_rectangle(mx - d.width / 2. - 6., my, d.width + 12., fs as f32 + 8., Color::new(0., 0., 0., 0.7));
        draw_text_ex(why, mx - d.width / 2., my + fs as f32, TextParams { font: Some(font), font_size: fs, color: hx(0xff9a9a), ..Default::default() });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn town() -> (Game, (i32, i32)) {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let c = (tfloor(g.p.x), tfloor(g.p.y));
        // ล้างของธรรมชาติ · ช่องน้ำ/ภูเขา/ดินปลูกเท่านั้นที่ถมเป็นหญ้า (ช่องอื่นคงพื้นธรรมชาติไว้ ปูพื้นทับได้)
        for y in c.1 - 25..=c.1 + 25 { for x in c.0 - 25..=c.0 + 25 {
            g.w.removed.insert((x, y));
            let t = g.w.tile_at(x, y);
            if SOLIDT[t as usize] || t == T_SOIL { g.w.tmod.insert((x, y), 3); }
        } }
        (g, c)
    }
    #[test]
    fn v31_drag_to_move_and_tap_tap() {
        let (mut g, c) = town();
        g.build_sel = PIECES.iter().position(|p| p.obj == OBJ_MOVE);
        let k = (c.0 + 3, c.1);
        g.w.placed.insert(k, 14);
        // กดค้าง-ลาก-ปล่อย = ย้ายทีเดียว
        g.build_drag(k, true);
        assert!(g.rs.carry.is_some(), "กดที่ของ = ยกทันที");
        let to = (c.0 + 6, c.1 + 2);
        g.build_drag((c.0 + 4, c.1 + 1), false);
        g.build_drag(to, false);
        g.build_release();
        assert_eq!(g.w.placed.get(&to), Some(&14), "ปล่อย = วาง");
        assert!(g.rs.carry.is_none() && g.w.placed.get(&k).is_none());
        // แตะ 1 ที = ยกค้าง · แตะที่ใหม่ 1 ที = วาง
        g.build_drag(to, true);
        g.build_release();
        assert!(g.rs.carry.is_some(), "แตะเฉยๆ = ยกค้างไว้");
        let to2 = (c.0 - 5, c.1 - 3);
        g.build_drag(to2, true);
        g.build_release();
        assert_eq!(g.w.placed.get(&to2), Some(&14));
        // วางไม่ได้ = ยังถืออยู่ เลือกใหม่ได้
        g.w.placed.insert((c.0 + 1, c.1 + 1), 14);
        g.build_drag(to2, true);
        g.build_release();
        g.build_drag((c.0 + 1, c.1 + 1), true);
        g.build_release();
        assert!(g.rs.carry.is_some(), "มีของขวาง = ยังถืออยู่");
        // ไกลกว่าระยะสร้างแต่อยู่ในระยะย้าย
        let far = (c.0 + 12, c.1);
        assert!(!g.build_reach(far) && g.move_reach(far));
        g.build_drag(far, true);
        g.build_release();
        assert_eq!(g.w.placed.get(&far), Some(&14));
    }
    #[test]
    fn move_chest_keeps_contents_and_bed_keeps_owner() {
        let (mut g, c) = town();
        let k = (c.0 + 3, c.1);
        g.w.placed.insert(k, 14);
        let mut inv = [0; NI]; inv[WOOD] = 42; g.chests.insert(k, inv);
        assert!(g.move_pick(k));
        // ระหว่างยก ของยังอยู่ที่เดิม (เซฟกลางทางไม่หาย)
        assert_eq!(g.w.placed.get(&k), Some(&14));
        let to = (c.0 + 5, c.1 + 2);
        assert!(g.move_drop(to));
        assert!(g.w.placed.get(&k).is_none() && g.chests.get(&k).is_none());
        assert_eq!(g.w.placed.get(&to), Some(&14));
        assert_eq!(g.chests[&to][WOOD], 42, "ของในกล่องตามไปด้วย");
        // เตียง: เจ้าของตามไป
        let b = (c.0 - 3, c.1);
        g.w.placed.insert(b, 33);
        g.npcs.push(Npc { id: 9, name: 0, look: 0, x: 0., y: 0., state: ST_HOME, hearts: 3, mood: 60, married: false, child: false, role: 0, bed: b, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(9, 0) });
        assert!(g.move_pick(b));
        let nb = (c.0 - 4, c.1 + 3);
        assert!(g.move_drop(nb));
        assert_eq!(g.npcs.last().unwrap().bed, nb);
    }
    #[test]
    fn move_blocked_reasons_and_cancel() {
        let (mut g, c) = town();
        let k = (c.0 + 3, c.1);
        g.w.placed.insert(k, O_LAMP);
        // ของธรรมชาติ/ที่ว่าง ยกไม่ได้
        assert!(g.move_select((c.0 + 6, c.1 + 6)).is_err());
        assert!(g.move_pick(k));
        // ทับของอื่น
        g.w.placed.insert((c.0 + 4, c.1), 14);
        assert_eq!(g.move_blocked(g.rs.carry.as_ref().unwrap(), (c.0 + 4, c.1), 0), Some("มีของขวางอยู่"));
        // ไกลเกิน
        assert_eq!(g.move_blocked(g.rs.carry.as_ref().unwrap(), (c.0 + 30, c.1), 0), Some("ไกลเกินไป — เดินเข้าไปใกล้อีกหน่อย"));
        // ที่เดิม
        assert!(g.move_blocked(g.rs.carry.as_ref().unwrap(), k, 0).is_some());
        // ยกเลิก = อยู่ที่เดิม
        assert!(g.move_cancel());
        assert!(g.rs.carry.is_none());
        assert_eq!(g.w.placed.get(&k), Some(&O_LAMP));
        // แปลงผักย้ายไม่ได้
        g.w.placed.insert((c.0, c.1 + 5), 27);
        assert!(g.move_select((c.0, c.1 + 5)).is_err());
    }
    #[test]
    fn move_whole_house_with_rotation() {
        let (mut g, c) = town();
        // หาที่ว่างที่สร้างบ้านได้ และมีที่ว่างทางขวาอีก 8 ช่องให้ย้ายไป
        let k = (-12..12).flat_map(|j| (-14..6).map(move |i| (c.0 + i, c.1 + j)))
            .find(|&k| g.prefab_blocked(OBJ_HOUSE_S, k, 0).is_none() && g.prefab_blocked(OBJ_HOUSE_S, (k.0 + 8, k.1), 3).is_none() && g.prefab_blocked(OBJ_HOUSE_S, (k.0 + 4, k.1), 0).is_none())
            .expect("มีที่ว่าง");
        g.p.inv[WOOD] = 80; g.p.inv[STONE] = 20;
        let cost = PIECES.iter().find(|p| p.obj == OBJ_HOUSE_S).unwrap().cost;
        g.p.x = k.0 as f32 * 16. + 8.; g.p.y = (k.1 + 2) as f32 * 16. + 12.;
        assert!(g.place_prefab(OBJ_HOUSE_S, k, 0, cost));
        for _ in 0..(31 * 30) { g.sites_tick(1. / 30.); }
        assert!(g.sites.is_empty());
        let inside = (k.0, k.1 - 2);
        let r0 = g.w.room_from(inside).expect("บ้านเป็นห้อง");
        let bed0 = r0.tiles.iter().copied().find(|t| g.w.placed.get(t) == Some(&33)).unwrap();
        g.npcs.push(Npc { id: 5, name: 1, look: 0, x: 0., y: 0., state: ST_HOME, hearts: 3, mood: 60, married: false, child: false, role: 0, bed: bed0, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(5, 1) });
        let n_before = g.w.placed.len();
        // แตะผนัง = ยกทั้งหลัง
        // ทุกผนัง รวมผนังมุม แตะแล้วได้ทั้งหลัง
        for &wall in &r0.walls { let sel = g.move_select(wall).unwrap(); assert!(sel.building, "แตะผนัง {:?} = ทั้งหลัง", wall); }
        // แตะเฟอร์นิเจอร์ในบ้าน = ชิ้นเดียว
        assert!(!g.move_select(bed0).unwrap().building);
        assert!(g.move_pick(k));
        assert!(g.rs.carry.as_ref().unwrap().building);
        // หมุนให้ประตูหันขวา แล้ววางห่างไป 8 ช่อง
        g.build_dir = 3;
        let to = (k.0 + 8, k.1);
        g.p.x = to.0 as f32 * 16. + 8. + 40.; g.p.y = to.1 as f32 * 16. + 8.;
        assert_eq!(g.move_blocked(g.rs.carry.as_ref().unwrap(), to, 3), None);
        assert!(g.move_drop(to));
        assert_eq!(g.w.placed.len(), n_before, "ของครบทุกชิ้น ไม่หายไม่เพิ่ม");
        assert!(g.w.room_from(inside).is_none(), "ที่เดิมว่างแล้ว");
        assert!(!g.w.tmod.get(&inside).map_or(false, |t| is_floor(*t)), "พื้นเดิมย้ายไปด้วย");
        assert_eq!(g.w.obj_at(to.0, to.1), 12, "ประตูอยู่ที่จุดวาง");
        let (ox, oy) = rot_off((0, -2), 3);
        let r1 = g.w.room_from((to.0 + ox, to.1 + oy)).expect("ที่ใหม่เป็นห้องปิด");
        assert!(r1.has_bed && r1.tiles.len() == r0.tiles.len());
        let nb = g.npcs.last().unwrap().bed;
        assert!(r1.tiles.contains(&nb), "เจ้าของเตียงตามไปบ้านใหม่");
    }
}
