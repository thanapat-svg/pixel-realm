// house.rs — v13: บ้านจริง
//  - พื้นไม้/พื้นหิน (เมนูสร้าง · วางซ้ำที่เดิม = รื้อคืนวัสดุ)
//  - ห้อง = พื้นที่ที่ล้อมด้วยกำแพง/ประตู (หรือหน้าผาภูเขา) ครบทุกด้าน ขนาด 1–120 ช่อง
//  - หลังคาอัตโนมัติคลุมห้องเมื่อมองจากข้างนอก · เดินเข้าบ้านแล้วหลังคาจางหาย
//  - อยู่ในบ้าน: ฝน/หิมะไม่โดน · อุ่น (หน้าหนาว/ในหิมะไม่หิวเร็ว · ทะเลทรายร่มเย็น) · มอนสเตอร์ไม่เกิดข้างใน · นอนในบ้านฟื้นดีกว่า
//  - ชาวบ้านที่เตียงอยู่ในห้องมีหลังคามีความสุขขึ้น · นอนกลางแจ้งความสุขลด
use super::*;

pub const T_FLOORW: u8 = 19;
pub const T_FLOORS: u8 = 20;
pub const OBJ_FLOOR_W: u8 = 254;
pub const OBJ_FLOOR_S: u8 = 253;
pub const ROOM_MAX: usize = 120;
// เฟอร์นิเจอร์ (v13.1)
pub const O_TABLE: u8 = 70;
pub const O_CHAIR: u8 = 71;
pub const O_RUG: u8 = 72;
pub const O_SHELF: u8 = 73;
pub const O_PLANT: u8 = 74;
pub const O_LAMP: u8 = 75;
pub const O_WINDOW: u8 = 76;
pub const O_WARDROBE: u8 = 77;
// บ้านสำเร็จรูป (วางทีเดียวได้ทั้งหลัง)
pub const OBJ_HOUSE_S: u8 = 252;
pub const OBJ_HOUSE_L: u8 = 251;

/// ผนังของห้อง: กำแพงไม้ · กำแพงหิน · กำแพงเมือง · ประตู (ปิด/เปิด) · หน้าต่าง
pub fn is_wall_obj(o: u8) -> bool { matches!(o, 10 | 11 | 38 | 12 | 13 | O_WINDOW) }
pub fn is_furniture(o: u8) -> bool { (O_TABLE..=O_WARDROBE).contains(&o) && o != O_WINDOW }
pub fn is_prefab(o: u8) -> bool { o == OBJ_HOUSE_S || o == OBJ_HOUSE_L }

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell { Wall(u8), Floor(u8, u8) }

/// แบบบ้านสำเร็จรูป: ประตูอยู่ที่ (0,0) ตัวบ้านยื่นขึ้นไปด้านบน · Floor(พื้น, ของบนพื้น 0=ว่าง)
pub fn prefab_layout(kind: u8) -> Vec<((i32, i32), Cell)> {
    let big = kind == OBJ_HOUSE_L;
    let (hw, h): (i32, i32) = if big { (3, 5) } else { (2, 4) };
    let wall = if big { 11 } else { 10 };
    let mut v = vec![];
    for dy in -h..=0 {
        for dx in -hw..=hw {
            let edge = dy == -h || dy == 0 || dx.abs() == hw;
            let c = if !edge {
                let furn = if big {
                    match (dx, dy) { (-2, -4) => 33, (2, -4) => O_LAMP, (-1, -4) => O_SHELF, (1, -3) => O_TABLE, (0, -3) => O_CHAIR, (2, -1) => O_PLANT, (-2, -1) => 14, (0, -2) => O_RUG, _ => 0 }
                } else {
                    match (dx, dy) { (-1, -3) => 33, (1, -3) => O_LAMP, _ => 0 }
                };
                Cell::Floor(if big { T_FLOORW } else { T_FLOORW }, furn)
            } else if (dx, dy) == (0, 0) {
                Cell::Wall(12)
            } else if dy == -h && (if big { dx.abs() == 2 } else { dx == 0 }) {
                Cell::Wall(O_WINDOW)
            } else {
                Cell::Wall(wall)
            };
            v.push(((dx, dy), c));
        }
    }
    v
}

/// ความน่าอยู่ของห้อง 0–5 ดาว: เตียง · ปูพื้นเกือบเต็ม · หน้าต่าง · เฟอร์นิเจอร์คนละแบบ (2 ชิ้น = 1 ดาว) · แสงไฟ/กองไฟ
pub fn comfort_of(w: &World, r: &Room) -> u8 {
    let mut pts = 0f32;
    if r.has_bed { pts += 1.; }
    let floored = r.tiles.iter().filter(|&&(x, y)| matches!(w.tile_at(x, y), T_FLOORW | T_FLOORS)).count();
    if floored * 10 >= r.tiles.len() * 7 { pts += 1.; }
    if r.walls.iter().any(|&(x, y)| w.obj_at(x, y) == O_WINDOW) { pts += 0.5; }
    let mut kinds: HashSet<u8> = HashSet::new();
    for &(x, y) in &r.tiles { let o = w.obj_at(x, y); if is_furniture(o) || matches!(o, 14 | 26 | 9 | 18) { kinds.insert(o); } }
    if kinds.iter().any(|&o| matches!(o, O_LAMP | 9 | 18 | 26)) { pts += 0.5; }
    pts += kinds.len() as f32 * 0.5;
    (pts.floor() as u8).min(5)
}
/// ฟอนต์ไทยไม่มีรูปดาว → เขียนเป็นตัวเลข
pub fn stars(n: u8) -> String { format!("{}/5 ดาว", n) }

#[derive(Clone, Debug, Default)]
pub struct Room {
    pub tiles: Vec<(i32, i32)>,
    /// ผนังรอบห้อง (ไม่รวมประตู) — ใช้วาดหลังคาคลุม
    pub walls: Vec<(i32, i32)>,
    pub doors: Vec<(i32, i32)>,
    pub stone: bool,
    pub has_bed: bool,
    pub fire: bool,
    pub alpha: f32,
    pub comfort: u8,
}

impl World {
    /// ไบโอมจริง (พื้นที่ปูทับ = ดูดินเดิมข้างใต้)
    pub fn biome_at(&self, x: i32, y: i32) -> u8 {
        let t = self.tile_at(x, y);
        if t == T_FLOORW || t == T_FLOORS { self.gen_tile(x, y) } else { t }
    }
    fn boundary(&self, tx: i32, ty: i32) -> bool {
        let o = self.obj_at(tx, ty);
        is_wall_obj(o) || self.tile_at(tx, ty) == 6
    }
    /// ห้องที่ช่องนี้อยู่ (None = ไม่ได้อยู่ในห้องปิด)
    pub fn room_from(&self, start: (i32, i32)) -> Option<Room> {
        if self.dun.is_some() || self.boundary(start.0, start.1) { return None; }
        let mut seen: HashSet<(i32, i32)> = HashSet::new();
        let mut stack = vec![start];
        let mut r = Room::default();
        let mut walls: HashSet<(i32, i32)> = HashSet::new();
        seen.insert(start);
        while let Some((x, y)) = stack.pop() {
            r.tiles.push((x, y));
            if r.tiles.len() > ROOM_MAX { return None; }
            let o = self.obj_at(x, y);
            if o == 33 { r.has_bed = true; }
            if o == 9 || o == 26 { r.fire = true; }
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let n = (x + dx, y + dy);
                if seen.contains(&n) { continue; }
                if self.boundary(n.0, n.1) {
                    walls.insert(n);
                    continue;
                }
                // ทะลุออกไปน้ำ/ของธรรมชาติที่ไม่ใช่ผนัง = ห้องไม่ปิด
                if SOLIDT[self.tile_at(n.0, n.1) as usize] { return None; }
                seen.insert(n);
                stack.push(n);
            }
        }
        // มุมห้อง (เฉียง) ให้หลังคาคลุมด้วย
        let inside: HashSet<(i32, i32)> = r.tiles.iter().copied().collect();
        for &(x, y) in &r.tiles {
            for (dx, dy) in [(1, 1), (-1, 1), (1, -1), (-1, -1)] {
                let n = (x + dx, y + dy);
                if !inside.contains(&n) && self.boundary(n.0, n.1) { walls.insert(n); }
            }
        }
        for w in walls {
            let o = self.obj_at(w.0, w.1);
            if o == 12 || o == 13 { r.doors.push(w); } else { r.walls.push(w); }
            if o == 11 || o == 38 { r.stone = true; }
        }
        r.comfort = comfort_of(self, &r);
        Some(r)
    }
}

impl Game {
    /// คำนวณห้องรอบตัวผู้เล่นใหม่ (เรียกทุก ~1 วินาที และทันทีหลังสร้าง/รื้อ)
    pub fn rooms_refresh(&mut self) {
        let old: HashMap<(i32, i32), f32> = self.rooms.iter().filter_map(|r| r.tiles.iter().min().map(|&k| (k, r.alpha))).collect();
        self.rooms.clear();
        self.room_of.clear();
        if self.w.dun.is_some() { return; }
        let (px, py) = (tfloor(self.p.x), tfloor(self.p.y));
        let walls: Vec<(i32, i32)> = self.w.placed.iter().filter(|(k, &o)| is_wall_obj(o) && (k.0 - px).abs() <= 24 && (k.1 - py).abs() <= 18).map(|(k, _)| *k).collect();
        for w in walls {
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let s = (w.0 + dx, w.1 + dy);
                if self.room_of.contains_key(&s) { continue; }
                if let Some(mut r) = self.w.room_from(s) {
                    let id = self.rooms.len();
                    let key = *r.tiles.iter().min().unwrap();
                    r.alpha = old.get(&key).copied().unwrap_or(1.);
                    for &t in &r.tiles { self.room_of.insert(t, id); }
                    self.rooms.push(r);
                }
            }
        }
        if !self.rooms.is_empty() && self.tips_seen & (1 << 8) == 0 {
            self.tips_seen |= 1 << 8;
            self.toast("สร้างบ้านสำเร็จ! ห้องที่ล้อมกำแพงครบมีหลังคาแล้ว — ข้างในฝนไม่โดน อุ่น และมอนสเตอร์ไม่เกิด");
        }
    }
    /// ผู้เล่นอยู่ในบ้าน (ห้องปิดที่มีหลังคา)
    pub fn indoors(&self) -> bool { self.w.dun.is_none() && self.room_of.contains_key(&(tfloor(self.p.x), tfloor(self.p.y - 3.))) }
    pub fn house_tick(&mut self, dt: f32) {
        self.room_t -= dt;
        if self.room_t <= 0. { self.room_t = 1.; self.rooms_refresh(); }
        let me = self.room_of.get(&(tfloor(self.p.x), tfloor(self.p.y - 3.))).copied();
        for (i, r) in self.rooms.iter_mut().enumerate() {
            let target = if Some(i) == me { 0. } else { 1. };
            r.alpha += (target - r.alpha) * (dt * 8.).min(1.);
        }
    }
    /// วาง/รื้อพื้น (คืน true ถ้าทำแล้ว)
    pub fn place_floor(&mut self, k: (i32, i32), obj: u8) -> bool {
        let want = if obj == OBJ_FLOOR_W { T_FLOORW } else { T_FLOORS };
        let (it, n) = if obj == OBJ_FLOOR_W { (WOOD, 1) } else { (STONE, 1) };
        let t = self.w.tile_at(k.0, k.1);
        if t == want {
            self.w.tmod.remove(&k);
            self.p.inv[it] += n;
            self.toast("รื้อพื้นแล้ว (ได้วัสดุคืน)");
            return true;
        }
        if SOLIDT[t as usize] { self.toast("ปูพื้นบนน้ำ/ภูเขาไม่ได้"); return false; }
        if t == T_SOIL || t == 12 || self.crops.contains_key(&k) { self.toast("รื้อแปลงผักก่อน (ใช้ไถดินวางซ้ำ)"); return false; }
        // ช่องที่ถูกดัดแปลงไว้แล้ว (ลาวาที่เย็นเป็นหิน ฯลฯ) ห้ามปูทับ — รื้อพื้นจะได้คืนดินเดิมเสมอ
        if self.w.tmod.contains_key(&k) && t != T_FLOORW && t != T_FLOORS { self.toast("ปูพื้นตรงนี้ไม่ได้"); return false; }
        let o = self.w.obj_at(k.0, k.1);
        if o != 0 && o != 3 && !self.w.placed.contains_key(&k) { self.toast("มีของขวางอยู่"); return false; }
        if self.p.inv[it] < n { self.toast("วัสดุไม่พอ"); return false; }
        if o == 3 && !self.w.placed.contains_key(&k) { self.w.removed.insert(k); }
        if (27..=32).contains(&o) { self.toast("รื้อแปลงผักก่อน"); return false; }
        self.p.inv[it] -= n;
        if t == T_FLOORW || t == T_FLOORS { self.p.inv[if t == T_FLOORW { WOOD } else { STONE }] += 1; }
        self.w.tmod.insert(k, want);
        true
    }
    /// ชาวบ้าน: เตียงอยู่ในห้องที่มีหลังคาไหม (ตรวจตรงๆ ไม่พึ่งแคชรอบตัวผู้เล่น)
    pub fn npc_has_house(&self, bed: (i32, i32)) -> bool { self.w.room_from(bed).is_some() }
}

pub fn floor_tex(stone: bool, i: i32) -> Image {
    let mut im = Image::gen_image_color(16, 16, hx(if stone { 0x8e8a84 } else { 0xb07a44 }));
    if stone {
        for (y0, off) in [(0, 0), (8, 4)] {
            for bx in -1..3 {
                let x0 = bx * 8 + off;
                for y in y0..y0 + 8 { for x in x0..x0 + 8 {
                    if x < 0 || x >= 16 { continue; }
                    let edge = y == y0 || x == x0;
                    let n = hash(x + i * 16, y, 830);
                    let c = if edge { 0x6a6660 } else if n < 0.12 { 0x7e7a74 } else if n > 0.9 { 0xa29e98 } else { 0x8e8a84 };
                    im.set_pixel(x as u32, y as u32, hx(c));
                } }
            }
        }
    } else {
        for y in 0..16 {
            let plank = y / 4;
            for x in 0..16 {
                let seam = y % 4 == 3 || (x + plank * 5 + i * 3) % 16 == 0;
                let n = hash(x + i * 16, y, 831);
                let c = if seam { 0x7a5028 } else if n < 0.1 { 0x9a6838 } else if n > 0.92 { 0xc89058 } else { 0xb07a44 };
                im.set_pixel(x as u32, y as u32, hx(c));
            }
        }
    }
    im
}

/// หลังคา: ปูกระเบื้อง (ไม้ = แดงอิฐ · หิน = เทาน้ำเงิน) มีสันกลางและเงาชายคา
pub fn draw_roofs(g: &Game, v: &View) {
    for r in &g.rooms {
        if r.alpha < 0.03 { continue; }
        let all: Vec<(i32, i32)> = r.tiles.iter().chain(r.walls.iter()).copied().collect();
        let (mut y0, mut y1) = (i32::MAX, i32::MIN);
        for &(_, y) in &all { y0 = y0.min(y); y1 = y1.max(y); }
        let mid = (y0 + y1) as f32 / 2.;
        let (c1, c2, ridge) = if r.stone { (0x5a6a88, 0x4a5874, 0x8a9ab8) } else { (0xb04a32, 0x8e3a26, 0xd87a52) };
        let a = r.alpha;
        for &(tx, ty) in &all {
            let (px, py) = (tx as f32 * 16. - v.cx, ty as f32 * 16. - 8. - v.cy);
            let top = (ty as f32) < mid;
            let base = if top { c1 } else { c2 };
            prect(v, px, py, 16., 16., Color::new(((base >> 16) & 255) as f32 / 255., ((base >> 8) & 255) as f32 / 255., (base & 255) as f32 / 255., a));
            // แถวกระเบื้อง
            for k in 0..4 {
                let yy = py + k as f32 * 4. + 3.;
                prect(v, px, yy, 16., 1., Color::new(0., 0., 0., 0.22 * a));
                let off = if k % 2 == 0 { 0. } else { 4. };
                for s in 0..2 { prect(v, px + off + s as f32 * 8., yy - 3., 1., 3., Color::new(0., 0., 0., 0.15 * a)); }
            }
            if (ty as f32 - mid).abs() < 0.6 {
                prect(v, px, py + 6., 16., 3., Color::new(((ridge >> 16) & 255) as f32 / 255., ((ridge >> 8) & 255) as f32 / 255., (ridge & 255) as f32 / 255., a));
            }
            if ty == y1 { prect(v, px, py + 15., 16., 2., Color::new(0., 0., 0., 0.35 * a)); }
        }
        // ปล่องไฟ + ควัน ถ้าในบ้านมีกองไฟ/หม้อ
        if r.fire {
            if let Some(&(tx, ty)) = r.tiles.iter().min_by_key(|t| (t.1, -t.0)) {
                let (px, py) = (tx as f32 * 16. + 9. - v.cx, ty as f32 * 16. - 18. - v.cy);
                prect(v, px, py, 5., 8., Color::new(0.35, 0.3, 0.28, a));
                prect(v, px - 1., py, 7., 2., Color::new(0.25, 0.2, 0.18, a));
                let t = get_time() as f32;
                for k in 0..3 {
                    let ph = (t * 0.6 + k as f32 / 3.).fract();
                    prect(v, px + 1. + (ph * 6. + k as f32).sin() * 2., py - ph * 14., 3., 3., Color::new(0.85, 0.85, 0.88, 0.5 * (1. - ph) * a));
                }
            }
        }
    }
}

impl Game {
    /// ช่องโลกของบ้านสำเร็จรูป เมื่อประตูอยู่ที่ k
    pub fn prefab_cells(&self, kind: u8, k: (i32, i32)) -> Vec<((i32, i32), Cell)> {
        prefab_layout(kind).into_iter().map(|((dx, dy), c)| ((k.0 + dx, k.1 + dy), c)).collect()
    }
    /// วางบ้านสำเร็จรูปตรงนี้ได้ไหม (None = ได้ · Some(เหตุผล))
    pub fn prefab_blocked(&self, kind: u8, k: (i32, i32)) -> Option<&'static str> {
        if self.w.dun.is_some() { return Some("สร้างในดันเจี้ยนไม่ได้"); }
        let (ptx, pty) = (tfloor(self.p.x), tfloor(self.p.y - 3.));
        for ((x, y), c) in self.prefab_cells(kind, k) {
            let t = self.w.tile_at(x, y);
            if SOLIDT[t as usize] { return Some("พื้นที่ติดน้ำ/ภูเขา — หาที่โล่งกว้างกว่านี้"); }
            if self.w.tmod.contains_key(&(x, y)) && t != T_FLOORW && t != T_FLOORS { return Some("ทับแปลงผัก/ดินที่ขุดไว้ — หาที่อื่น"); }
            if self.crops.contains_key(&(x, y)) { return Some("ทับแปลงผัก — หาที่อื่น"); }
            let o = self.w.obj_at(x, y);
            if !(o == 0 || ((o == 2 || o == 3 || o == O_DRYGRASS) && !self.w.placed.contains_key(&(x, y)))) { return Some("มีต้นไม้/หิน/ของวางอยู่ในพื้นที่ — ตีออกก่อน หรือหาที่โล่ง"); }
            let solid_cell = match c { Cell::Wall(_) => true, Cell::Floor(_, f) => f != 0 && f != O_RUG && f != O_CHAIR };
            if solid_cell && (self.player_in_tile(x, y) || (x, y) == (ptx, pty)) { return Some("ถอยออกมาหน่อย ตัวละครยืนขวางอยู่"); }
        }
        None
    }
    pub fn place_prefab(&mut self, kind: u8, k: (i32, i32), cost: &[(usize, i32)]) -> bool {
        if let Some(why) = self.prefab_blocked(kind, k) { self.toast(why); return false; }
        if !can_pay(&self.p, cost) { self.toast("วัสดุไม่พอ"); return false; }
        for &(it, n) in cost { self.p.inv[it] -= n; }
        for ((x, y), c) in self.prefab_cells(kind, k) {
            if !self.w.placed.contains_key(&(x, y)) && self.w.nat_obj(x, y) != 0 { self.w.removed.insert((x, y)); }
            self.obj_dmg.remove(&(x, y));
            match c {
                Cell::Wall(o) => { self.w.placed.insert((x, y), o); }
                Cell::Floor(t, f) => {
                    self.w.tmod.insert((x, y), t);
                    if f != 0 { self.w.placed.insert((x, y), f); if f == 14 { self.chests.insert((x, y), [0; NI]); } }
                }
            }
            if gen_range(0., 1.) < 0.4 { self.burst(x as f32 * 16. + 8., y as f32 * 16. + 10., hx(0xc8a070), 4, 40.); }
        }
        self.ev[3] += 1;
        self.room_t = 0.;
        self.refresh_torches();
        play(&self.sfx.door);
        self.toast("สร้างบ้านเสร็จ! เดินเข้าประตูได้เลย — เพิ่มเฟอร์นิเจอร์ให้บ้านน่าอยู่ขึ้น (ดาวความน่าอยู่)");
        true
    }
    /// กดค้างปุ่มสร้างแล้วเดิน = วางกำแพงตามทางที่เดินผ่าน (วางที่ช่องที่เพิ่งเดินออกมา) · พื้น = ปูใต้เท้า
    pub fn paint_tick(&mut self, held: bool) {
        let cur = (tfloor(self.p.x), tfloor(self.p.y - 3.));
        let Some(sel) = self.build_sel.filter(|_| held && self.w.dun.is_none()) else { self.paint_last = None; return; };
        let pc = &PIECES[sel];
        let o = pc.obj;
        if o == OBJ_FLOOR_W || o == OBJ_FLOOR_S {
            let want = if o == OBJ_FLOOR_W { T_FLOORW } else { T_FLOORS };
            if self.paint_last != Some(cur) {
                self.paint_last = Some(cur);
                if self.w.tile_at(cur.0, cur.1) != want && self.place_floor_quiet(cur, o) { self.room_t = 0.; }
            }
            return;
        }
        if !matches!(o, 10 | 11 | 38 | O_WINDOW) { return; }
        match self.paint_last {
            Some(prev) if prev != cur => {
                if self.can_place_at(prev.0, prev.1) && can_pay(&self.p, pc.cost) {
                    for &(it, n) in pc.cost { self.p.inv[it] -= n; }
                    self.w.placed.insert(prev, o);
                    self.obj_dmg.remove(&prev);
                    self.room_t = 0.;
                    play(&self.sfx.door);
                    self.burst(prev.0 as f32 * 16. + 8., prev.1 as f32 * 16. + 10., hx(0xc8a070), 4, 30.);
                }
                self.paint_last = Some(cur);
            }
            None => self.paint_last = Some(cur),
            _ => {}
        }
    }
    fn place_floor_quiet(&mut self, k: (i32, i32), obj: u8) -> bool {
        let t = self.w.tile_at(k.0, k.1);
        let (it, want) = if obj == OBJ_FLOOR_W { (WOOD, T_FLOORW) } else { (STONE, T_FLOORS) };
        if SOLIDT[t as usize] || self.p.inv[it] < 1 || self.crops.contains_key(&k) { return false; }
        if self.w.tmod.contains_key(&k) && t != T_FLOORW && t != T_FLOORS { return false; }
        let o = self.w.obj_at(k.0, k.1);
        if o != 0 && o != 3 && !self.w.placed.contains_key(&k) { return false; }
        self.p.inv[it] -= 1;
        if t == T_FLOORW || t == T_FLOORS { self.p.inv[if t == T_FLOORW { WOOD } else { STONE }] += 1; }
        self.w.tmod.insert(k, want);
        true
    }
    /// ดาวความน่าอยู่ของห้องที่ผู้เล่นอยู่
    pub fn home_comfort(&self) -> Option<u8> {
        self.room_of.get(&(tfloor(self.p.x), tfloor(self.p.y - 3.))).map(|&i| self.rooms[i].comfort)
    }
}

/// ตอนอยู่ในบ้าน: ข้างนอกมืดลง (โฟกัสในห้อง) · ผนังด้านในเป็นวอลเปเปอร์ · หน้าต่างเห็นท้องฟ้า+แสงลอดเข้ามา
pub fn cur_room(g: &Game) -> Option<&Room> {
    g.room_of.get(&(tfloor(g.p.x), tfloor(g.p.y - 3.))).map(|&i| &g.rooms[i])
}
/// ผนังห้องที่ต้องซ่อน sprite ปกติ (วาดเป็นผนังด้านในแทน)
pub fn interior_hidden(g: &Game) -> HashSet<(i32, i32)> {
    match cur_room(g) { Some(r) if r.alpha < 0.5 => r.walls.iter().copied().collect(), _ => HashSet::new() }
}
/// phase 0 = ก่อนวาดตัวละคร (ผนังด้านใน แสง) · phase 1 = หลังวาด (ข้างนอกมืด)
pub fn draw_interior(g: &Game, v: &View, t_now: f32, phase: u8) {
    let Some(r) = cur_room(g) else { return };
    let k = (1. - r.alpha).clamp(0., 1.);
    if k < 0.02 { return; }
    let inside: HashSet<(i32, i32)> = r.tiles.iter().chain(r.walls.iter()).chain(r.doors.iter()).copied().collect();
    let room: HashSet<(i32, i32)> = r.tiles.iter().copied().collect();
    if phase == 1 {
    // 1) ข้างนอกมืด
    let (tx0, ty0) = (tfloor(v.cx) - 1, tfloor(v.cy) - 1);
    let (tx1, ty1) = (tfloor(v.cx + v.vw) + 1, tfloor(v.cy + v.vh) + 2);
    for ty in ty0..=ty1 {
        let mut run: Option<i32> = None;
        for tx in tx0..=tx1 + 1 {
            let out = tx <= tx1 && !inside.contains(&(tx, ty));
            match (out, run) {
                (true, None) => run = Some(tx),
                (false, Some(s)) => { prect(v, s as f32 * 16. - v.cx, ty as f32 * 16. - v.cy, (tx - s) as f32 * 16., 16., Color::new(0.04, 0.03, 0.06, 0.9 * k)); run = None; }
                _ => {}
            }
        }
    }
    return;
    }
    // 2) ผนัง: ผนังด้านหลัง (เหนือห้อง) = วอลเปเปอร์ · ผนังข้าง/ล่าง = สันกำแพงเข้ม
    let day = (1. - g.darkv).clamp(0., 1.);
    let (wp1, wp2, base) = if r.stone { (0xb8b4a8, 0xa8a498, 0x6a6660) } else { (0xe8d8b0, 0xd8c498, 0x7a5028) };
    let c = |h: u32, a: f32| Color::new(((h >> 16) & 255) as f32 / 255., ((h >> 8) & 255) as f32 / 255., (h & 255) as f32 / 255., a);
    let kw = if r.alpha < 0.5 { 1. } else { k };
    for &(x, y) in r.walls.iter().chain(r.doors.iter()) {
        let (px, py) = (x as f32 * 16. - v.cx, y as f32 * 16. - v.cy);
        let back = room.contains(&(x, y + 1));
        let o = g.w.obj_at(x, y);
        if back && o != 12 && o != 13 {
            prect(v, px, py, 16., 16., c(wp1, kw));
            for s in 0..4 { prect(v, px + s as f32 * 4. + 1., py, 2., 13., c(wp2, kw)); }
            prect(v, px, py, 16., 2., c(0x00000, 0.35 * kw));
            prect(v, px, py + 13., 16., 3., c(base, kw));
            if o == O_WINDOW {
                // ท้องฟ้าตามเวลา + กรอบหน้าต่าง
                let sky = Color::new(0.05 + 0.45 * day, 0.07 + 0.6 * day, 0.18 + 0.72 * day, kw);
                prect(v, px + 3., py + 3., 10., 8., c(0x5a3418, kw));
                prect(v, px + 4., py + 4., 8., 6., sky);
                prect(v, px + 7.5, py + 4., 1., 6., c(0x5a3418, kw));
                prect(v, px + 4., py + 6.5, 8., 1., c(0x5a3418, kw));
                if day < 0.4 { prect(v, px + 5., py + 5., 1., 1., c(0xffffff, kw * 0.8)); prect(v, px + 10., py + 8., 1., 1., c(0xffffff, kw * 0.6)); }
                // แสงแดดลอดลงพื้น
                if day > 0.3 {
                    let a = 0.16 * day * k;
                    for j in 0..3 { prect(v, px + 2. - j as f32, py + 16. + j as f32 * 8., 12. + j as f32 * 2., 8., Color::new(1., 0.95, 0.7, a * (1. - j as f32 * 0.3))); }
                }
            }
        } else if o != 12 && o != 13 {
            // ผนังข้าง/ล่าง: มองจากบนลงเห็นสันกำแพง (ยังเห็น sprite เดิมจางๆ ตอนกำลังเฟด)
            prect(v, px, py, 16., 16., c(base, kw));
            prect(v, px + 1., py + 1., 14., 14., c(if r.stone { 0x5a5650 } else { 0x6a4220 }, kw));
            prect(v, px + 1., py + 1., 14., 2., c(if r.stone { 0x7a766e } else { 0x8a5a2c }, kw));
        }
    }
    // 3) ไฟอุ่นในห้องเบาๆ (มีโคม/กองไฟ)
    let lit = r.tiles.iter().any(|&(x, y)| matches!(g.w.obj_at(x, y), O_LAMP | 9 | 18 | 26));
    if lit {
        let fl = 0.04 + (t_now * 2.3).sin() * 0.01;
        for &(x, y) in &r.tiles { prect(v, x as f32 * 16. - v.cx, y as f32 * 16. - v.cy, 16., 16., Color::new(1., 0.7, 0.3, fl * k)); }
    }
}

impl Game {
    /// ประตูอัตโนมัติ: เดินชนประตูที่ปิด = เปิดเอง · เดินห่างออกไป = ปิดเอง (กันมอนเข้า) · ประตูที่เปิดเองด้วย Q ไม่ปิดให้
    pub fn auto_door(&mut self, mv: Vec2) {
        if self.w.dun.is_some() { return; }
        if mv.length() > 0.3 {
            let (px, py) = (self.p.x + mv.x.signum() * if mv.x.abs() > 0.3 { 10. } else { 0. }, if mv.y < -0.3 { self.p.y - 10. } else if mv.y > 0.3 { self.p.y + 6. } else { self.p.y - 2. });
            let k = (tfloor(px), tfloor(py));
            if self.w.placed.get(&k) == Some(&12) {
                self.w.placed.insert(k, 13);
                self.auto_doors.insert(k);
                self.room_t = self.room_t.min(0.2);
                play(&self.sfx.door);
            }
        }
        let close: Vec<(i32, i32)> = self.auto_doors.iter().copied().filter(|k| (k.0 as f32 * 16. + 8. - self.p.x).hypot(k.1 as f32 * 16. + 8. - (self.p.y - 3.)) > 30. && !self.player_in_tile(k.0, k.1)).collect();
        for k in close {
            self.auto_doors.remove(&k);
            let blocked = self.en.iter().any(|e| !e.dead && tfloor(e.x) == k.0 && tfloor(e.y) == k.1) || self.npcs.iter().any(|n| tfloor(n.x) == k.0 && tfloor(n.y - 3.) == k.1);
            if self.w.placed.get(&k) == Some(&13) && !blocked { self.w.placed.insert(k, 12); play(&self.sfx.door); }
        }
    }
}
