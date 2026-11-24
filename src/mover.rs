// mover.rs — v28 ย้ายของที่สร้างแล้ว (ไม่ต้องรื้อแล้วสร้างใหม่)
//  v31 ใช้ง่ายขึ้น: กดที่ของค้างไว้แล้วลากไปวาง (ปล่อยนิ้ว = วาง) · หรือแตะของ 1 ที แล้วแตะที่ใหม่ 1 ที
//  ระยะย้ายไกลกว่าระยะสร้าง (เกือบทั้งจอ) · ปุ่ม "ย้ายของ" อยู่บนสุดของเมนูสร้าง
//  - ของชิ้นเดียว (เฟอร์นิเจอร์ กล่อง เตียง ตะเกียง บ่อน้ำ รั้ว ฯลฯ): ย้ายไปพร้อมของในกล่อง/เจ้าของเตียง
//  - แตะผนัง/ประตู/หน้าต่าง/พื้นของบ้านที่ปิดเป็นห้อง = ย้าย "ทั้งหลัง" (ผนัง พื้น เฟอร์นิเจอร์ กล่อง เตียง) · หมุนได้ 4 ทิศ
//  - ระหว่างยก ของยังอยู่ที่เดิม (เซฟกลางทางไม่หาย) · ย้ายจริงทีเดียวตอนวาง · ยกเลิก = อยู่ที่เดิม
//  v37 ใช้ง่าย/เป็นระเบียบขึ้น: ต้นไม้ หิน หญ้าที่ขวางที่ใหม่ ตัดให้อัตโนมัติ (ได้วัสดุคืน) · ช่องที่ติดขัดขึ้นกากบาทแดงทีละช่อง
//    · ลูกศรบอกทางเข้าประตู · เส้นไกด์เมื่อแนวตรงกับบ้านข้างๆ + ดูดเข้าแนวให้เอง (ห่าง 1 ช่องพอดี / ประตูแถวเดียวกัน)
//    · กรอบที่เดิมจางๆ + เส้นประจากที่เดิมไปที่ใหม่ · ป้ายบอกขนาด/ทิศประตู
use super::*;

pub const OBJ_MOVE: u8 = 245;
const NB4: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
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
    /// v37 กรอบบ้านหลังอื่นรอบๆ (x0, y0, x1, y1, ประตู x, ประตู y) — ใช้ทำเส้นไกด์/ดูดเข้าแนว (คำนวณตอนยก)
    pub guides: Vec<[i32; 6]>,
}

/// v37 สถานะช่องปลายทาง
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CellIs { Ok, Clear(u8), Bad(&'static str) }

/// เส้นไกด์ที่ตรงแนว: (แนวตั้ง?, พิกัด, ช่วงจาก, ช่วงถึง)
pub type Guide = (bool, i32, i32, i32);

/// ของธรรมชาติที่ย้ายบ้าน/ของทับแล้วตัดออกให้ได้ (ได้วัสดุคืน)
pub fn nat_cuttable(o: u8) -> bool { arch::nat_clearable(o) || o == 2 || o == 3 || o == O_DRYGRASS }
fn cut_refund(o: u8) -> Option<(usize, i32)> { if o == 2 { Some((BERRY, 1)) } else if o == 3 || o == O_DRYGRASS { None } else { arch::clear_refund(o) } }

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
                return Ok(Carry { src: k, tiles, building: true, name: format!("บ้านทั้งหลัง ({} ชิ้น)", n), guides: vec![] });
            }
        }
        self.move_single(k)
    }
    fn move_single(&self, k: (i32, i32)) -> Result<Carry, &'static str> {
        match self.w.placed.get(&k) {
            Some(&o) if movable(o) => Ok(Carry { src: k, tiles: vec![k], building: false, name: obj_name(o).to_string(), guides: vec![] }),
            Some(_) => Err("แปลงผักย้ายไม่ได้ — ใช้ไถดินวางซ้ำเพื่อรื้อ"),
            None if is_floor(self.w.tile_at(k.0, k.1)) && self.w.tmod.contains_key(&k) => Ok(Carry { src: k, tiles: vec![k], building: false, name: "พื้น".to_string(), guides: vec![] }),
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
        self.move_blocked_ex(c, anchor, dir, true)
    }
    /// v36 เหมือน move_blocked แต่เลือกได้ว่าจะเช็กระยะเอื้อมไหม (ช่างสถาปนิกย้ายบ้านได้ทั้งเมืองโดยไม่ต้องยืนใกล้)
    pub fn move_blocked_ex(&self, c: &Carry, anchor: (i32, i32), dir: u8, reach: bool) -> Option<&'static str> {
        if reach && !self.move_reach(anchor) { return Some("ไกลเกินไป — เดินเข้าไปใกล้อีกหน่อย"); }
        let pairs = self.move_targets(c, anchor, dir);
        if pairs.iter().all(|(s, d)| s == d) { return Some("ยังอยู่ที่เดิม — เลือกที่ใหม่"); }
        let src: HashSet<(i32, i32)> = c.tiles.iter().copied().collect();
        for &(s, d) in &pairs {
            if let CellIs::Bad(why) = self.move_cell(&src, s, d) { return Some(why); }
        }
        // v34 ย้ายบ้านแล้วประตูต้องไม่ชนกำแพง/บ้านอื่น
        self.move_door_blocked(c, anchor, dir)
    }
    /// v37 ช่องปลายทางหนึ่งช่อง: วางได้ / ต้องตัดของธรรมชาติออก / ติดอะไร
    pub fn move_cell(&self, src: &HashSet<(i32, i32)>, s: (i32, i32), d: (i32, i32)) -> CellIs {
        if src.contains(&d) { return CellIs::Ok; }
        let t = self.w.tile_at(d.0, d.1);
        if SOLIDT[t as usize] { return CellIs::Bad("ทับน้ำ/ภูเขา"); }
        if self.w.realm.city_at(d.0, d.1).map_or(false, |ci| self.city_owner(ci) != PLAYER_OWN) { return CellIs::Bad("อยู่ในเมืองคนอื่น"); }
        if self.site_reserved(d) { return CellIs::Bad("ทับที่กำลังก่อสร้าง"); }
        if t == T_SOIL || t == 12 || self.crops.contains_key(&d) { return CellIs::Bad("ทับแปลงผัก"); }
        if self.w.tmod.get(&s).map_or(false, |x| is_floor(*x)) && self.w.tmod.contains_key(&d) && !is_floor(t) { return CellIs::Bad("ปูพื้นทับช่องนี้ไม่ได้ (ถนน/ที่ถม)"); }
        let so = self.w.placed.get(&s).copied().unwrap_or(0);
        if OSOLID[so as usize] && self.player_in_tile(d.0, d.1) { return CellIs::Bad("ถอยออกจากตรงนั้นก่อน"); }
        if self.w.placed.contains_key(&d) { return CellIs::Bad("มีของขวางอยู่"); }
        match self.w.obj_at(d.0, d.1) {
            0 => CellIs::Ok,
            o if nat_cuttable(o) => if o == 3 || o == O_DRYGRASS { CellIs::Ok } else { CellIs::Clear(o) },
            _ => CellIs::Bad("มีของขวางอยู่"),
        }
    }
    pub fn nat_clear_at(&self, t: (i32, i32)) -> bool { !self.w.placed.contains_key(&t) && nat_cuttable(self.w.obj_at(t.0, t.1)) }
    /// v37 ประตูของบ้านที่ยก ณ ที่ใหม่: (ช่องประตู, ทิศออกนอกบ้าน)
    pub fn move_doors(&self, c: &Carry, anchor: (i32, i32), dir: u8) -> Vec<((i32, i32), (i32, i32))> {
        if !c.building { return vec![]; }
        let src: HashSet<(i32, i32)> = c.tiles.iter().copied().collect();
        let mut v = vec![];
        for (s, d) in self.move_targets(c, anchor, dir) {
            if !matches!(self.w.placed.get(&s), Some(12) | Some(13)) { continue; }
            let Some(&o) = NB4.iter().find(|o| !src.contains(&(s.0 + o.0, s.1 + o.1))) else { continue };
            v.push((d, rot_off(o, dir)));
        }
        v.sort();
        v
    }
    /// v37 กรอบบ้านหลังอื่นรอบๆ (ไว้ทำเส้นไกด์)
    pub fn move_guides_for(&self, c: &Carry) -> Vec<[i32; 6]> {
        if !c.building { return vec![]; }
        let own: HashSet<(i32, i32)> = c.tiles.iter().copied().collect();
        let mut doors: Vec<(i32, i32)> = self.w.placed.iter().filter(|(k, &o)| (o == 12 || o == 13) && (k.0 - c.src.0).abs() <= 32 && (k.1 - c.src.1).abs() <= 32 && !own.contains(k)).map(|(k, _)| *k).collect();
        doors.sort();
        let mut seen: HashSet<(i32, i32)> = HashSet::new();
        let mut out = vec![];
        for d in doors {
            if seen.contains(&d) { continue; }
            let Some(r) = NB4.iter().find_map(|o| self.w.room_from((d.0 + o.0, d.1 + o.1))) else { continue };
            let all: Vec<(i32, i32)> = r.tiles.iter().chain(r.walls.iter()).chain(r.doors.iter()).copied().collect();
            for k in r.doors.iter() { seen.insert(*k); }
            if all.iter().any(|k| own.contains(k)) { continue; }
            let (x0, y0) = (all.iter().map(|k| k.0).min().unwrap(), all.iter().map(|k| k.1).min().unwrap());
            let (x1, y1) = (all.iter().map(|k| k.0).max().unwrap(), all.iter().map(|k| k.1).max().unwrap());
            out.push([x0, y0, x1, y1, d.0, d.1]);
        }
        out
    }
    /// v37 ดูดเข้าแนว: เลื่อนไม่เกิน 1 ช่องให้ขอบ/ประตูตรงกับบ้านข้างๆ หรือห่างกัน 1 ช่องพอดี · คืน (จุดวาง, เส้นไกด์ที่ตรง)
    pub fn move_snap(&self, c: &Carry, anchor: (i32, i32), dir: u8) -> ((i32, i32), Vec<Guide>) {
        if !c.building || c.guides.is_empty() { return (anchor, vec![]); }
        let tg: Vec<(i32, i32)> = self.move_targets(c, anchor, dir).into_iter().map(|p| p.1).collect();
        let (x0, y0) = (tg.iter().map(|k| k.0).min().unwrap(), tg.iter().map(|k| k.1).min().unwrap());
        let (x1, y1) = (tg.iter().map(|k| k.0).max().unwrap(), tg.iter().map(|k| k.1).max().unwrap());
        let door = self.move_doors(c, anchor, dir).first().map(|d| d.0).unwrap_or(anchor);
        let near = |g: &[i32; 6]| g[0] - x1 <= 18 && x0 - g[2] <= 18 && g[1] - y1 <= 18 && y0 - g[3] <= 18;
        // แกน x: (ค่าของเรา, ค่าเป้า) · แกน y เหมือนกัน
        let fit = |vert: bool, d: i32| -> Vec<Guide> {
            let mut v = vec![];
            for g in c.guides.iter().filter(|g| near(g)) {
                let (a0, a1, ad, b0, b1, bd) = if vert { (x0 + d, x1 + d, door.0 + d, g[0], g[2], g[4]) } else { (y0 + d, y1 + d, door.1 + d, g[1], g[3], g[5]) };
                // ช่วงของอีกแกน (ไว้ลากเส้นให้ครอบทั้งสองหลัง)
                let (o0, o1) = if vert { (y0.min(g[1]), y1.max(g[3])) } else { (x0.min(g[0]), x1.max(g[2])) };
                let side = if vert { (y0 <= g[3] && y1 >= g[1]) } else { (x0 <= g[2] && x1 >= g[0]) };
                for (ours, theirs) in [(a0, b0), (a1, b1), (ad, bd)] { if ours == theirs { v.push((vert, ours, o0, o1)); } }
                // วางเรียงข้างกัน ห่าง 1 ช่อง
                if side { if a1 + 2 == b0 { v.push((vert, a1 + 1, o0, o1)); } if a0 - 2 == b1 { v.push((vert, a0 - 1, o0, o1)); } }
            }
            v
        };
        let mut best = (anchor, vec![]);
        let pick = |vert: bool| -> i32 { for d in [0, -1, 1] { if !fit(vert, d).is_empty() { return d; } } 0 };
        let (dx, dy) = (pick(true), pick(false));
        let snapped = (anchor.0 + dx, anchor.1 + dy);
        if (dx, dy) != (0, 0) && self.move_blocked(c, snapped, dir).is_some() && self.move_blocked(c, anchor, dir).is_none() {
            best.1 = fit(true, 0).into_iter().chain(fit(false, 0)).collect();
            return best;
        }
        best.0 = snapped;
        best.1 = fit(true, dx).into_iter().chain(fit(false, dy)).collect();
        best
    }
    /// ยกขึ้น (ยังไม่ขยับของจริง)
    pub fn move_pick(&mut self, k: (i32, i32)) -> bool {
        match self.move_select(k) {
            Ok(mut c) => {
                c.guides = self.move_guides_for(&c);
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
        let anchor = self.move_snap(&c, anchor, self.build_dir).0;
        if let Some(why) = self.move_blocked(&c, anchor, self.build_dir) { self.toast(why); play(&self.sfx.deny); return false; }
        let back = self.move_apply(&c, anchor, self.build_dir);
        self.rs.carry = None;
        self.rs.build_arm = None;
        let (x, y) = (anchor.0 as f32 * 16. + 8., anchor.1 as f32 * 16. + 8.);
        self.burst(x, y, hx(0xc8a070), 12, 50.);
        play(&self.sfx.door);
        for i in 0..NI { if back[i] > 0 { self.gain(i, back[i], x, y - 10.); } }
        let cut: i32 = back.iter().sum();
        self.toast(if cut > 0 { format!("ย้าย{}เรียบร้อย · ตัดของที่ขวางออกให้แล้ว", c.name) } else { format!("ย้าย{}เรียบร้อย", c.name) });
        self.save();
        true
    }
    /// v36 ย้ายจริง (ไม่เช็กอะไรแล้ว — เรียกหลัง move_blocked/move_blocked_ex ผ่าน): ของ พื้น ของในกล่อง ประตูอัตโนมัติ เจ้าของเตียง
    /// v37 คืนวัสดุที่ได้จากต้นไม้/หินที่ตัดออกให้ตรงที่ใหม่
    pub fn move_apply(&mut self, c: &Carry, anchor: (i32, i32), dir: u8) -> [i32; NI] {
        let pairs = self.move_targets(c, anchor, dir);
        // ตัดของธรรมชาติที่ขวางที่ใหม่ (รวมช่องหน้าประตู) ก่อน
        let mut back = [0; NI];
        let srcs: HashSet<(i32, i32)> = pairs.iter().map(|p| p.0).collect();
        let mut cut: Vec<(i32, i32)> = pairs.iter().map(|p| p.1).filter(|d| !srcs.contains(d)).collect();
        for (d, o) in self.move_doors(c, anchor, dir) { cut.push((d.0 + o.0, d.1 + o.1)); }
        for d in cut {
            if self.w.placed.contains_key(&d) { continue; }
            let o = self.w.obj_at(d.0, d.1);
            if o == 0 || !nat_cuttable(o) { continue; }
            if let Some((it, q)) = cut_refund(o) { back[it] += q; }
            self.w.removed.insert(d);
            self.obj_dmg.remove(&d);
        }
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
        let old: HashSet<(i32, i32)> = pairs.iter().map(|p| p.0).collect();
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
        // เจ้าของเตียงตามไปด้วย (ถ้ายืนอยู่ในบ้านเดิมตอนนั้น ก็ย้ายตัวไปที่เตียงใหม่ด้วย ไม่ให้ยืนค้างกลางที่โล่ง/ในผนังใหม่)
        for n in self.npcs.iter_mut() {
            if let Some(&d) = bed_map.get(&n.bed) {
                if self.w.placed.get(&d) == Some(&33) {
                    if old.contains(&(tfloor(n.x), tfloor(n.y))) { n.x = d.0 as f32 * 16. + 8.; n.y = d.1 as f32 * 16. + 18.; }
                    n.bed = d;
                    self.rs.sleeping.remove(&n.id);
                }
            }
        }
        // เตียงของเราเอง (จุดเกิด/บ้านของผู้เล่น) ตามไปด้วย
        if let Some(b) = self.bed { if let Some(&d) = bed_map.get(&b) { if self.w.placed.get(&d) == Some(&33) { self.bed = Some(d); } } }
        self.rs.work.clear();
        self.room_t = 0.;
        self.refresh_torches();
        self.doors_check(false);
        back
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

/// พรีวิว v37: ที่เดิมเป็นกรอบประจาง · ที่ใหม่เป็นภาพจริงโปร่งๆ · ช่องติดขัด = กากบาทแดง · ต้นไม้/หินที่จะตัดให้ = เหลือง
///   ลูกศรทางเข้าประตู · เส้นไกด์เมื่อตรงแนวบ้านข้างๆ · ป้ายบอกขนาด ทิศประตู และเหตุผลถ้าวางไม่ได้
pub fn draw_move_preview(g: &Game, sp: &Sprites, v: &View, font: &Font, t_now: f32) {
    let (tx, ty) = g.build_target();
    let tile = |x: i32, y: i32| ((x as f32 * 16. - v.cx) * v.s, (y as f32 * 16. - v.cy) * v.s, 16. * v.s);
    let Some(c) = &g.rs.carry else {
        // ยังไม่ได้ยก: ไฮไลต์สิ่งที่จะถูกยกถ้าแตะตรงนี้
        let (lc, list) = match g.move_select((tx, ty)) { Ok(c) => (Color::new(0.4, 0.85, 1., 0.9), c.tiles), Err(_) => (Color::new(1., 0.4, 0.4, 0.7), vec![(tx, ty)]) };
        for &(x, y) in &list { let (px, py, s) = tile(x, y); draw_rectangle_lines(px, py, s, s, 2., lc); }
        return;
    };
    let (anchor, guides) = g.move_snap(c, (tx, ty), g.build_dir);
    let bbox = |ks: &mut dyn Iterator<Item = (i32, i32)>| -> (i32, i32, i32, i32) {
        let mut b = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
        for k in ks { b.0 = b.0.min(k.0); b.1 = b.1.min(k.1); b.2 = b.2.max(k.0); b.3 = b.3.max(k.1); }
        b
    };
    let dash = |x0: f32, y0: f32, x1: f32, y1: f32, col: Color, w: f32| {
        let (dx, dy) = (x1 - x0, y1 - y0);
        let len = (dx * dx + dy * dy).sqrt().max(1.);
        let step = 6. * v.s;
        let n = (len / step) as i32;
        for i in (0..n).step_by(2) {
            let (a, b) = (i as f32 * step / len, ((i + 1) as f32 * step / len).min(1.));
            draw_line(x0 + dx * a, y0 + dy * a, x0 + dx * b, y0 + dy * b, w, col);
        }
    };
    let rect_dash = |b: (i32, i32, i32, i32), col: Color, w: f32| {
        let (x0, y0, _) = tile(b.0, b.1);
        let (x1, y1, s) = tile(b.2, b.3);
        let (x1, y1) = (x1 + s, y1 + s);
        dash(x0, y0, x1, y0, col, w); dash(x0, y1, x1, y1, col, w); dash(x0, y0, x0, y1, col, w); dash(x1, y0, x1, y1, col, w);
    };
    // ที่เดิม
    let ob = bbox(&mut c.tiles.iter().copied());
    rect_dash(ob, Color::new(0.45, 0.85, 1., 0.75), 2.);
    // ตารางจางๆ รอบที่ใหม่ (ช่วยกะระยะ)
    let pairs = g.move_targets(c, anchor, g.build_dir);
    let nb = bbox(&mut pairs.iter().map(|p| p.1));
    for y in nb.1 - 3..=nb.3 + 4 { let (x0, py, _) = tile(nb.0 - 3, y); let (x1, _, s) = tile(nb.2 + 3, y); draw_line(x0, py, x1 + s, py, 1., Color::new(1., 1., 1., 0.07)); }
    for x in nb.0 - 3..=nb.2 + 4 { let (px, y0, _) = tile(x, nb.1 - 3); let (_, y1, s) = tile(x, nb.3 + 3); draw_line(px, y0, px, y1 + s, 1., Color::new(1., 1., 1., 0.07)); }
    // เส้นประจากที่เดิมไปที่ใหม่
    let cen = |b: (i32, i32, i32, i32)| { let (a, bb, s) = tile(b.0, b.1); let (c2, d, _) = tile(b.2, b.3); ((a + c2 + s) / 2., (bb + d + s) / 2.) };
    let (oc, nc) = (cen(ob), cen(nb));
    if (oc.0 - nc.0).abs() + (oc.1 - nc.1).abs() > 20. { dash(oc.0, oc.1, nc.0, nc.1, Color::new(1., 1., 1., 0.35), 1.5); }
    // ช่องปลายทาง
    let src: HashSet<(i32, i32)> = c.tiles.iter().copied().collect();
    let reason = g.move_blocked(c, anchor, g.build_dir);
    let ok = reason.is_none();
    let tint = if ok { Color::new(1., 1., 1., 0.72) } else { Color::new(1., 0.7, 0.7, 0.55) };
    let mut n_cut = 0;
    for &(s, d) in &pairs {
        let (px, py) = (d.0 as f32 * 16. - v.cx, d.1 as f32 * 16. - v.cy);
        if let Some(&t) = g.w.tmod.get(&s) { if is_floor(t) { blit(sp, v, sp.tiles[t as usize][0], px, py, false, tint); } }
        if let Some(&o) = g.w.placed.get(&s) { let r = sp.obj[o as usize]; blit(sp, v, r, px + 8. - r.w / 2., py + 16. - r.h, false, tint); }
        let (qx, qy, sz) = tile(d.0, d.1);
        match g.move_cell(&src, s, d) {
            CellIs::Ok => {}
            CellIs::Clear(_) => { n_cut += 1; draw_rectangle(qx, qy, sz, sz, Color::new(1., 0.85, 0.2, 0.28)); draw_rectangle_lines(qx + 1., qy + 1., sz - 2., sz - 2., 1.5, Color::new(1., 0.85, 0.2, 0.8)); }
            CellIs::Bad(_) => {
                draw_rectangle(qx, qy, sz, sz, Color::new(1., 0.15, 0.15, 0.35));
                draw_line(qx + 3., qy + 3., qx + sz - 3., qy + sz - 3., 2.5, Color::new(1., 0.25, 0.25, 0.95));
                draw_line(qx + sz - 3., qy + 3., qx + 3., qy + sz - 3., 2.5, Color::new(1., 0.25, 0.25, 0.95));
            }
        }
    }
    // กรอบที่ใหม่
    let (bx0, by0, _) = tile(nb.0, nb.1);
    let (bx1, by1, s) = tile(nb.2, nb.3);
    let frame = if ok { Color::new(0.35, 1., 0.45, 0.9) } else { Color::new(1., 0.35, 0.35, 0.9) };
    draw_rectangle_lines(bx0 - 1., by0 - 1., bx1 + s - bx0 + 2., by1 + s - by0 + 2., 2., frame);
    // ประตู: ลูกศรชี้เข้าบ้าน + ช่องหน้าประตู
    for (d, o) in g.move_doors(c, anchor, g.build_dir) {
        let f = (d.0 + o.0, d.1 + o.1);
        let (fx, fy, sz) = tile(f.0, f.1);
        let clear = !g.w.placed.contains_key(&f) && !SOLIDT[g.w.tile_at(f.0, f.1) as usize];
        draw_rectangle(fx, fy, sz, sz, if clear { Color::new(0.4, 1., 0.5, 0.18) } else { Color::new(1., 0.3, 0.3, 0.3) });
        let (cx, cy) = (fx + sz / 2., fy + sz / 2.);
        let (ux, uy) = (-(o.0 as f32), -(o.1 as f32));
        let a = sz * 0.36;
        let tip = vec2(cx + ux * a, cy + uy * a);
        let back = vec2(cx - ux * a * 0.4, cy - uy * a * 0.4);
        let (pxv, pyv) = (-uy * a * 0.7, ux * a * 0.7);
        let col = Color::new(1., 1., 1., 0.55 + 0.35 * (t_now * 4.).sin().abs());
        draw_triangle(tip, vec2(back.x + pxv, back.y + pyv), vec2(back.x - pxv, back.y - pyv), col);
    }
    // เส้นไกด์ (ตรงแนวบ้านข้างๆ)
    for &(vert, at, a, b) in &guides {
        let col = Color::new(1., 0.9, 0.3, 0.8);
        if vert { let (px, y0, _) = tile(at, a); let (_, y1, s) = tile(at, b); let x = px + s / 2.; dash(x, y0 - s, x, y1 + 2. * s, col, 1.5); }
        else { let (x0, py, _) = tile(a, at); let (x1, _, s) = tile(b, at); let y = py + s / 2.; dash(x0 - s, y, x1 + 2. * s, y, col, 1.5); }
    }
    // ป้าย
    let w = nb.2 - nb.0 + 1;
    let h = nb.3 - nb.1 + 1;
    let face = g.move_doors(c, anchor, g.build_dir).first().map(|&(_, o)| match o { (0, 1) => "ประตูหันลง", (-1, 0) => "ประตูหันซ้าย", (0, -1) => "ประตูหันขึ้น", _ => "ประตูหันขวา" });
    let mut info = if c.building { format!("{} · {}×{} ช่อง{}", c.name, w, h, face.map(|f| format!(" · {}", f)).unwrap_or_default()) } else { c.name.clone() };
    if ok && n_cut > 0 { info += &format!(" · ตัดต้นไม้/หินให้ {} จุด", n_cut); }
    if ok && !guides.is_empty() { info += " · ตรงแนวบ้านข้างๆ"; }
    let fs = (6. * v.s).max(11.) as u16;
    let mx = (bx0 + bx1 + s) / 2.;
    let mut ly = by0 - 6.;
    for (line, col) in [(Some(info), hx(0xeef4ff)), (reason.map(|r| r.to_string()), hx(0xff9a9a))] {
        let Some(line) = line else { continue };
        let d = measure_text(&line, Some(font), fs, 1.);
        let bh = fs as f32 + 8.;
        let y0 = ly - bh;
        draw_rectangle(mx - d.width / 2. - 6., y0, d.width + 12., bh, Color::new(0., 0., 0., 0.7));
        draw_text_ex(&line, mx - d.width / 2., y0 + fs as f32 + 1., TextParams { font: Some(font), font_size: fs, color: col, ..Default::default() });
        ly = y0 - 3.;
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
    /// v37 โลกที่เป็นพื้นดิน (ไม่ต้องถมน้ำ) ยังมีต้นไม้/หินธรรมชาติอยู่
    fn land() -> (Game, (i32, i32)) {
        let g = Game::new_world_seed(278, Sfx::default());
        let c = (tfloor(g.p.x), tfloor(g.p.y));
        (g, c)
    }
    fn built_house(g: &mut Game, k: (i32, i32), dir: u8) {
        for y in k.1 - 7..=k.1 + 3 { for x in k.0 - 4..=k.0 + 4 { g.w.removed.insert((x, y)); } }
        g.p.inv[WOOD] += 80; g.p.inv[STONE] += 20;
        let cost = PIECES.iter().find(|p| p.obj == OBJ_HOUSE_S).unwrap().cost;
        let (px, py) = (g.p.x, g.p.y);
        g.p.x = k.0 as f32 * 16. + 8.; g.p.y = (k.1 + 2) as f32 * 16. + 12.;
        assert!(g.place_prefab(OBJ_HOUSE_S, k, dir, cost), "{:?}", g.prefab_blocked(OBJ_HOUSE_S, k, dir));
        for _ in 0..(31 * 30) { g.sites_tick(1. / 30.); }
        assert!(g.sites.is_empty());
        g.p.x = px; g.p.y = py;
    }
    #[test]
    fn v37_moving_onto_trees_cuts_them_and_gives_wood() {
        let (mut g, c) = land();
        let k = (c.0, c.1 + 3);
        built_house(&mut g, k, 0);
        let sel = g.move_select(k).unwrap();
        let src: HashSet<(i32, i32)> = sel.tiles.iter().copied().collect();
        // หาที่ใหม่ที่มีต้นไม้/หินธรรมชาติขวางอยู่ แต่วางได้
        let mut found = None;
        'o: for r in 6..40 { for dy in -r..=r { for dx in [-r, r] {
            let to = (k.0 + dx, k.1 + dy);
            let tg: Vec<((i32, i32), (i32, i32))> = g.move_targets(&sel, to, 0);
            let cut = tg.iter().filter(|&&(s, d)| matches!(g.move_cell(&src, s, d), CellIs::Clear(_))).count();
            if cut >= 2 && g.move_blocked_ex(&sel, to, 0, false).is_none() { found = Some(to); break 'o; }
        } } }
        let to = found.expect("มีที่ที่มีต้นไม้ขวาง");
        let tg: Vec<(i32, i32)> = g.move_targets(&sel, to, 0).into_iter().map(|p| p.1).collect();
        // ของที่ผู้เล่นวางเองยังบังอยู่เหมือนเดิม
        let free = *tg.iter().find(|t| !g.w.placed.contains_key(t) && g.w.obj_at(t.0, t.1) == 0).unwrap_or(&tg[0]);
        g.w.placed.insert(free, 14);
        assert!(matches!(g.move_cell(&src, sel.tiles[0], free), CellIs::Bad(_)), "กล่องที่วางเองไม่ตัดทิ้ง");
        g.w.placed.remove(&free);
        assert!(g.move_pick(k));
        g.p.x = to.0 as f32 * 16. + 8. + 90.; g.p.y = to.1 as f32 * 16. + 8. + 60.;
        let w0 = g.p.inv[WOOD] + g.p.inv[STONE] + g.p.inv[GEM] + g.p.inv[BERRY];
        assert!(g.move_drop(to), "ต้นไม้ไม่บังการย้ายแล้ว");
        assert!(g.p.inv[WOOD] + g.p.inv[STONE] + g.p.inv[GEM] + g.p.inv[BERRY] > w0, "ได้วัสดุคืนจากของที่ตัด");
        assert!(tg.iter().all(|t| g.w.placed.contains_key(t) || !nat_cuttable(g.w.obj_at(t.0, t.1)) || g.w.obj_at(t.0, t.1) == 3));
        assert!(g.w.room_from((to.0, to.1 - 2)).map_or(false, |r| r.has_bed), "ที่ใหม่เป็นบ้านปิดมีเตียง");
    }
    #[test]
    fn v37_snap_lines_up_with_neighbour_house() {
        let (mut g, c) = land();
        for y in c.1 - 20..=c.1 + 20 { for x in c.0 - 30..=c.0 + 30 { g.w.removed.insert((x, y)); } }
        let a = (c.0 - 10, c.1 + 2);
        built_house(&mut g, a, 0);
        let b = (c.0 + 8, c.1 + 12);
        built_house(&mut g, b, 0);
        assert!(g.move_pick(b));
        let car = g.rs.carry.clone().unwrap();
        assert!(!car.guides.is_empty(), "เจอบ้านข้างๆ เป็นไกด์");
        let off = car.src.0 - car.tiles.iter().map(|t| t.0).min().unwrap();
        let x1a = car.guides.iter().map(|g| g[2]).max().unwrap();
        // วางเยื้องจากแถวประตูบ้าน a ไป 1 ช่อง (ไม่ชิด) → ดูดให้ประตูแถวเดียวกัน
        let (snap, gl) = g.move_snap(&car, (x1a + 6 + off, a.1 + 1), 0);
        assert_eq!(snap.1, a.1, "ประตูแถวเดียวกับบ้านข้างๆ");
        assert!(!gl.is_empty(), "มีเส้นไกด์");
        // วางเกือบชิด (ห่าง 2) → ดูดให้ห่าง 1 ช่องพอดี
        let near = (x1a + 3 + off, a.1);
        let (sn, _) = g.move_snap(&car, near, 0);
        assert_eq!(sn.0 - off, x1a + 2, "ห่างจากบ้านข้างๆ 1 ช่องพอดี");
        g.p.x = sn.0 as f32 * 16. + 8.; g.p.y = (sn.1 + 3) as f32 * 16. + 8.;
        assert!(g.move_drop(near));
        assert_eq!(g.w.placed.get(&(sn.0 - (car.src.0 - b.0), sn.1)), Some(&12), "วางตามจุดที่ดูดเข้าแนว (ประตู)");
    }
}
