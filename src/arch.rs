//! v36 ช่างสถาปนิก — จ้างมาวางผังเมืองให้เป็นระเบียบ (สำหรับคนขี้เกียจออกแบบเอง)
//!
//! ลำดับ: จ้าง → ช่างสำรวจบ้าน/กำแพง/บ่อน้ำ/ที่นั่งคุยทั้งเมือง → เสนอ "แบบ" (ถนนแนวตะวันออก-ตะวันตก หรือเหนือ-ใต้)
//!   แต่ละแบบ: ลานกลางเมือง (วงกองไฟ + บ่อน้ำ 4 มุม) · บ้านทุกหลังย้ายไปเรียงสองฝั่งถนน หันประตูเข้าถนน · กำแพงที่สร้างสะเปะสะปะถูกรื้อ
//!   (คืนวัสดุ) · ปูถนน (เลือกได้) → เราเลือกรับ/ไม่รับ/ปรับหมวดที่จะทำ → รับแล้วจ่ายค่าจ้าง ช่างลงมือทีละอย่าง
//!   ของไม่พอ (บ่อน้ำใหม่ วงกองไฟ ถนน) = ไปหามาใส่ให้ครบ ช่างจะทำส่วนที่ทำได้ไปก่อน ที่เหลือรอวัสดุ
//! ย้ายบ้านใช้แกนเดียวกับปุ่ม "ย้ายของ" (ของในกล่อง เตียง เจ้าของเตียงตามไป) · ยังสร้าง/ย้ายเองได้เหมือนเดิมตลอด
use super::*;
use mover::Carry;

pub const OPT_NPC: u8 = 1;
pub const OPT_MINE: u8 = 2;
pub const OPT_WALLS: u8 = 4;
pub const OPT_PLAZA: u8 = 8;
pub const OPT_ROAD: u8 = 16;
pub const OPT_DEFAULT: u8 = OPT_NPC | OPT_WALLS | OPT_PLAZA;
pub const OPT_NAMES: [&str; 5] = ["ย้ายบ้านชาวบ้าน", "ย้ายบ้านของเรา/บ้านว่าง", "รื้อกำแพงสะเปะสะปะ", "ลานกลางเมือง (กองไฟ+บ่อน้ำ)", "ปูถนน"];
pub const VARIANT_NAMES: [&str; 2] = ["ถนนแนวตะวันออก-ตะวันตก", "ถนนแนวเหนือ-ใต้"];

pub const ARCH_FEE: i32 = 30;
pub const ARCH_FEE_HOUSE: i32 = 4;
/// รัศมีที่ช่างสำรวจรอบใจกลางเมือง (ช่อง)
pub const SURVEY_R: i32 = 36;
/// ลานกลางเมือง: ครึ่งความกว้าง (9×9)
pub const PLAZA_R: i32 = 4;
/// ระยะห่างระหว่างถนนขนาน (ช่อง)
pub const BLOCK: i32 = 16;
/// ช่างลงมือหนึ่งอย่างทุกกี่วินาที
pub const STEP_SECS: f32 = 1.2;

#[derive(Clone, Debug, PartialEq)]
pub enum Step {
    /// เคลียร์ต้นไม้/หินเล็ก/กำแพงสะเปะสะปะ ที่ช่องนี้ (obj = ของที่เจอตอนวางแผน)
    Clear { at: (i32, i32), obj: u8 },
    /// ย้ายบ้านทั้งหลัง: ประตูเดิม → ประตูใหม่ หมุน rot
    MoveHouse { door: (i32, i32), to: (i32, i32), rot: u8 },
    /// ย้ายของชิ้นเดียว (บ่อน้ำ วงกองไฟ ท่อนไม้)
    MoveOne { from: (i32, i32), to: (i32, i32) },
    /// วางของใหม่ (ใช้วัสดุจากกองกลาง)
    Place { obj: u8, at: (i32, i32) },
    /// ปูถนนเป็นผืน
    Pave { x0: i32, y0: i32, x1: i32, y1: i32 },
}

#[derive(Clone, Debug, Default)]
pub struct Plan {
    pub variant: u8,
    pub opts: u8,
    pub center: (i32, i32),
    pub steps: Vec<Step>,
    /// 0 รออยู่ · 1 ทำแล้ว · 2 ข้าม (ทำไม่ได้)
    pub done: Vec<u8>,
    /// วัสดุที่ใส่ไว้ให้ช่าง (รวมของที่ได้คืนจากการรื้อ)
    pub pool: [i32; NI],
    pub houses: usize,
    pub notes: Vec<String>,
    /// ไม่เซฟ: ช่องที่ใช้วาดแผนที่ (x, y, ชนิด: 0 บ้านเดิม · 1 บ้านใหม่ · 2 เคลียร์ · 3 ลาน/บ่อ/กองไฟ · 4 ถนน)
    pub overlay: Vec<(i32, i32, u8)>,
}

#[derive(Clone, Debug, Default)]
pub struct Arch {
    pub opts: u8,
    /// ผู้เล่นเคยปรับหมวดเองแล้ว (ยังไม่ปรับ = ใช้ค่าเริ่มต้น)
    pub set: bool,
    pub variant: u8,
    /// ข้อเสนอที่ยังไม่รับ (ไม่เซฟ)
    pub prop: Option<Plan>,
    pub active: Option<Plan>,
    pub show: bool,
    pub t: f32,
    pub stall: u32,
    pub finished: i32,
}

#[derive(Clone, Debug)]
pub struct Hs {
    pub door: (i32, i32),
    pub face: u8,
    pub carry: Carry,
    pub owner: Option<u32>,
    pub mine: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Survey {
    pub c: (i32, i32),
    pub houses: Vec<Hs>,
    /// กำแพงที่ไม่ใช่ส่วนของห้องปิดไหนเลย
    pub walls: Vec<(i32, i32)>,
    pub wells: Vec<(i32, i32)>,
    /// วงกองไฟที่มีอยู่ (ช่องกองไฟก่อน แล้วท่อนไม้)
    pub ring: Vec<(i32, i32)>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Cs { Free, Clear(u8), Blocked }

struct Lay<'a> {
    g: &'a Game,
    /// ช่องที่ของเดิมจะถูกย้าย/รื้อออก → ถือว่าว่าง
    gone: HashSet<(i32, i32)>,
    fields: HashSet<(i32, i32)>,
    cache: HashMap<(i32, i32), Cs>,
}
impl<'a> Lay<'a> {
    fn new(g: &'a Game) -> Self {
        let fields = g.rs.pm.fields.iter().flat_map(|f| f.cells()).collect();
        Lay { g, gone: HashSet::new(), fields, cache: HashMap::new() }
    }
    fn cs(&mut self, k: (i32, i32)) -> Cs {
        if let Some(&c) = self.cache.get(&k) { return c; }
        let c = self.calc(k);
        self.cache.insert(k, c);
        c
    }
    fn calc(&self, k: (i32, i32)) -> Cs {
        let g = self.g;
        let t = g.w.tile_at(k.0, k.1);
        if SOLIDT[t as usize] || t == T_SOIL || t == T_WET { return Cs::Blocked; }
        if g.crops.contains_key(&k) || self.fields.contains(&k) || g.site_reserved(k) { return Cs::Blocked; }
        if g.w.realm.city_at(k.0, k.1).map_or(false, |ci| g.city_owner(ci) != PLAYER_OWN) { return Cs::Blocked; }
        if self.gone.contains(&k) { return Cs::Free; }
        if g.w.placed.contains_key(&k) { return Cs::Blocked; }
        if g.w.tmod.get(&k).map_or(false, |&x| x == T_FLOORW || x == T_FLOORS) { return Cs::Blocked; }
        match g.w.obj_at(k.0, k.1) {
            0 | 3 => Cs::Free,
            o if o == 4 || o == T_DEAD || (is_tree(o) && !is_fruit_tree(o) && o != T_SAPLING) => Cs::Clear(o),
            _ => Cs::Blocked,
        }
    }
}

pub fn place_cost(obj: u8) -> Vec<(usize, i32)> {
    if obj == 9 { return vec![(WOOD, 3), (STONE, 2)]; }
    PIECES.iter().find(|p| p.obj == obj).map(|p| p.cost.to_vec()).unwrap_or_default()
}
/// รื้อ/โค่นแล้วได้วัสดุคืน
pub fn clear_refund(o: u8) -> Option<(usize, i32)> {
    match o { 10 => Some((WOOD, 1)), 11 => Some((STONE, 2)), 4 => Some((STONE, 2)), o if is_tree(o) => Some((WOOD, 3)), _ => None }
}
fn nb4() -> [(i32, i32); 4] { [(0, -1), (0, 1), (-1, 0), (1, 0)] }
fn face_of(v: (i32, i32)) -> u8 { match v { (0, 1) => 0, (-1, 0) => 1, (0, -1) => 2, _ => 3 } }
fn cheb1(c: (i32, i32)) -> impl Iterator<Item = (i32, i32)> { (-1..=1).flat_map(move |dy| (-1..=1).map(move |dx| (c.0 + dx, c.1 + dy))) }

impl Plan {
    pub fn need(&self) -> [i32; NI] {
        let mut n = [0; NI];
        for (i, s) in self.steps.iter().enumerate() {
            if self.done[i] != 0 { continue; }
            match s {
                Step::Place { obj, .. } => for (it, q) in place_cost(*obj) { n[it] += q; },
                Step::Pave { x0, y0, x1, y1 } => n[STONE] += (x1 - x0 + 1) * (y1 - y0 + 1),
                _ => {}
            }
        }
        n
    }
    /// วัสดุที่จะได้คืนจากการรื้อที่ยังไม่ทำ
    pub fn refund_pending(&self) -> [i32; NI] {
        let mut n = [0; NI];
        for (i, s) in self.steps.iter().enumerate() {
            if self.done[i] != 0 { continue; }
            if let Step::Clear { obj, .. } = s { if let Some((it, q)) = clear_refund(*obj) { n[it] += q; } }
        }
        n
    }
    /// ขาดอะไรอีกเท่าไร (รวมของที่จะได้คืนจากการรื้อแล้ว)
    pub fn shortfall(&self) -> Vec<(usize, i32)> {
        let (need, back) = (self.need(), self.refund_pending());
        (0..NI).filter_map(|i| { let s = need[i] - self.pool[i] - back[i]; if s > 0 { Some((i, s)) } else { None } }).collect()
    }
    pub fn progress(&self) -> (usize, usize) { (self.done.iter().filter(|&&d| d != 0).count(), self.steps.len()) }
    pub fn count_houses(&self) -> usize { self.steps.iter().filter(|s| matches!(s, Step::MoveHouse { .. })).count() }
    pub fn count_clear(&self) -> (usize, usize) {
        let (mut nat, mut wall) = (0, 0);
        for s in &self.steps { if let Step::Clear { obj, .. } = s { if *obj == 10 || *obj == 11 { wall += 1 } else { nat += 1 } } }
        (nat, wall)
    }
    pub fn finished(&self) -> bool { self.done.iter().all(|&d| d != 0) }
}

impl Game {
    pub fn arch_opts(&self) -> u8 { if self.rs.ar.set { self.rs.ar.opts } else { OPT_DEFAULT } }
    pub fn arch_toggle(&mut self, bit: u8) {
        let o = self.arch_opts() ^ bit;
        self.rs.ar.opts = o;
        self.rs.ar.set = true;
    }
    /// ค่าจ้าง
    pub fn arch_fee(&self, plan: &Plan) -> i32 { ARCH_FEE + ARCH_FEE_HOUSE * plan.count_houses() as i32 }

    // ---------------------------------------------------------------- สำรวจ
    pub fn arch_survey(&self) -> Option<Survey> {
        let c = self.town_center()?;
        let near = |k: &(i32, i32)| (k.0 - c.0).abs() <= SURVEY_R && (k.1 - c.1).abs() <= SURVEY_R;
        // ห้องปิดทั้งหมดรอบเมือง (เพื่อรู้ว่ากำแพง/กองไฟตรงไหนอยู่ในบ้านใคร)
        let mut outside: HashSet<(i32, i32)> = HashSet::new();
        let mut inside: HashSet<(i32, i32)> = HashSet::new();
        let mut room_walls: HashSet<(i32, i32)> = HashSet::new();
        let mut walls_all: Vec<(i32, i32)> = self.w.placed.iter().filter(|(k, &o)| near(k) && is_wall_obj(o)).map(|(k, _)| *k).collect();
        walls_all.sort();
        for &w in &walls_all {
            for dy in -1..=1 { for dx in -1..=1 {
                let n = (w.0 + dx, w.1 + dy);
                if inside.contains(&n) || outside.contains(&n) || (dx == 0 && dy == 0) { continue; }
                if let Some(r) = self.w.room_from_ex(n, &mut outside) {
                    inside.extend(r.tiles.iter().copied());
                    room_walls.extend(r.walls.iter().copied());
                    room_walls.extend(r.doors.iter().copied());
                }
            } }
        }
        let mut sv = Survey { c, ..Default::default() };
        // บ้าน = ห้องปิดที่มีเตียง
        let mut beds: Vec<(i32, i32)> = self.w.placed.iter().filter(|(k, &o)| o == 33 && near(k)).map(|(k, _)| *k).collect();
        beds.sort();
        let mut seen: HashSet<(i32, i32)> = HashSet::new();
        for bed in beds {
            let Some(room) = self.w.room_from(bed) else { continue };
            if room.tent { continue; }
            let mut doors = room.doors.clone();
            doors.sort();
            let Some(&door) = doors.first() else { continue };
            if seen.contains(&door) { continue; }
            let Ok(carry) = self.move_select(door) else { continue };
            if !carry.building || !carry.tiles.contains(&bed) { continue; }
            let Some(inn) = nb4().iter().map(|d| (door.0 + d.0, door.1 + d.1)).find(|n| room.tiles.contains(n)) else { continue };
            seen.insert(door);
            let face = face_of((door.0 - inn.0, door.1 - inn.1));
            let owner = self.npcs.iter().find(|n| n.state == ST_HOME && room.tiles.contains(&n.bed)).map(|n| n.id);
            sv.houses.push(Hs { door, face, carry, mine: owner.is_none(), owner });
        }
        sv.houses.sort_by_key(|h| h.door);
        let house_tiles: HashSet<(i32, i32)> = sv.houses.iter().flat_map(|h| h.carry.tiles.iter().copied()).collect();
        // กำแพงสะเปะสะปะ: ไม่ได้เป็นส่วนของห้องปิดใดเลย
        sv.walls = walls_all.iter().copied().filter(|k| matches!(self.w.placed.get(k), Some(10) | Some(11)) && !room_walls.contains(k) && !house_tiles.contains(k) && !self.site_reserved(*k)).collect();
        // บ่อน้ำกลางแจ้ง
        let mut wells: Vec<(i32, i32)> = self.w.placed.iter().filter(|(k, &o)| near(k) && (o == 39 || o == O_BIGWELL) && !inside.contains(k)).map(|(k, _)| *k).collect();
        wells.sort();
        sv.wells = wells;
        // วงกองไฟ: กองไฟนอกบ้านที่มีท่อนไม้ข้างๆ (ใกล้ใจกลางเมืองสุด)
        let mut fires: Vec<(i32, i32)> = self.w.placed.iter().filter(|(k, &o)| near(k) && o == 9 && !inside.contains(k)).map(|(k, _)| *k).collect();
        fires.sort_by_key(|f| ((f.0 - c.0).abs() + (f.1 - c.1).abs(), *f));
        for f in fires {
            let logs: Vec<(i32, i32)> = (-2..=2).flat_map(|dy| (-2..=2).map(move |dx| (f.0 + dx, f.1 + dy))).filter(|k| self.w.placed.get(k) == Some(&O_LOG)).collect();
            if !logs.is_empty() { sv.ring = std::iter::once(f).chain(logs.into_iter().take(4)).collect(); break; }
        }
        Some(sv)
    }

    // ---------------------------------------------------------------- วางแผน
    /// ข้อเสนอของช่างตามแบบ (variant) และหมวดที่เลือก (None = ยังไม่มีบ้านในเมือง)
    pub fn arch_plan(&self, variant: u8, opts: u8) -> Option<Plan> {
        let sv = self.arch_survey()?;
        if sv.houses.is_empty() { return None; }
        let mut lay = Lay::new(self);
        let sel: Vec<&Hs> = sv.houses.iter().filter(|h| if h.mine { opts & OPT_MINE != 0 } else { opts & OPT_NPC != 0 }).collect();
        for h in &sel { lay.gone.extend(h.carry.tiles.iter().copied()); }
        if opts & OPT_WALLS != 0 { lay.gone.extend(sv.walls.iter().copied()); }
        let mut plan = Plan { variant, opts, center: sv.c, ..Default::default() };
        let mut clears: Vec<Step> = vec![];
        let mut cleared: HashSet<(i32, i32)> = HashSet::new();
        let mut rsv: HashSet<(i32, i32)> = HashSet::new();
        let mut fp_all: HashSet<(i32, i32)> = HashSet::new();
        let mut overlay: Vec<(i32, i32, u8)> = vec![];
        let mut plaza_steps: Vec<Step> = vec![];
        let mut pc = sv.c;
        let mut note = |p: &mut Plan, s: String| p.notes.push(s);

        // ---- ลานกลางเมือง
        let mut ring_cells: Vec<(i32, i32)> = vec![];
        if opts & OPT_PLAZA != 0 {
            let ring_off = [(0, 0), (-2, 0), (2, 0), (0, -2), (0, 2)];
            let mut sites: Vec<(i32, i32)> = (-6..=6).flat_map(|dy| (-6..=6).map(move |dx| (dx, dy))).collect();
            sites.sort_by_key(|o| (o.0.abs() + o.1.abs(), *o));
            // ถ้ามีวงกองไฟเดิม ก็ให้ลานอยู่ที่วงเดิมได้ถ้าอยู่ใกล้ใจกลาง (ไม่ต้องขนไปไกล)
            let mut found = None;
            for o in sites {
                let s = (sv.c.0 + o.0, sv.c.1 + o.1);
                let ok = ring_off.iter().all(|r| { let k = (s.0 + r.0, s.1 + r.1); !matches!(lay.cs(k), Cs::Blocked) || sv.ring.contains(&k) });
                if ok { found = Some(s); break; }
            }
            if let Some(s) = found {
                pc = s;
                for dy in -PLAZA_R..=PLAZA_R { for dx in -PLAZA_R..=PLAZA_R { rsv.insert((pc.0 + dx, pc.1 + dy)); } }
                // วงกองไฟ
                let spots: Vec<(i32, i32)> = ring_off.iter().map(|r| (pc.0 + r.0, pc.1 + r.1)).collect();
                if sv.ring.is_empty() {
                    plaza_steps.push(Step::Place { obj: 9, at: spots[0] });
                    for &k in &spots[1..] { plaza_steps.push(Step::Place { obj: O_LOG, at: k }); }
                } else {
                    // จับคู่ท่อนไม้เดิมกับตำแหน่งใหม่ที่ใกล้ที่สุด
                    let mut free: Vec<(i32, i32)> = spots[1..].to_vec();
                    let mut moves: Vec<((i32, i32), (i32, i32))> = vec![(sv.ring[0], spots[0])];
                    for &l in &sv.ring[1..] {
                        let (i, _) = free.iter().enumerate().min_by_key(|(_, s)| (s.0 - l.0).abs() + (s.1 - l.1).abs()).unwrap();
                        moves.push((l, free.remove(i)));
                    }
                    for (f, t) in moves { if f != t { plaza_steps.push(Step::MoveOne { from: f, to: t }); lay.gone.insert(f); } }
                    for k in free { plaza_steps.push(Step::Place { obj: O_LOG, at: k }); }
                }
                ring_cells = spots;
                // บ่อน้ำ 4 มุมลาน: ของเดิมย้ายมา ไม่พอก็วางใหม่ (อย่างน้อย 1 บ่อ และ 1 บ่อต่อ 5 หลัง)
                let corners = [(-3, -3), (3, -3), (-3, 3), (3, 3)].map(|o| (pc.0 + o.0, pc.1 + o.1));
                let want = (sv.wells.len().max((sv.houses.len() + 4) / 5).max(1)).min(4);
                let mut left: Vec<(i32, i32)> = corners.to_vec();
                let mut wells = sv.wells.clone();
                wells.sort_by_key(|w| (w.0 - pc.0).abs() + (w.1 - pc.1).abs());
                let mut n_done = 0;
                for w in wells {
                    if n_done >= want || left.is_empty() { break; }
                    let (i, _) = left.iter().enumerate().min_by_key(|(_, s)| (s.0 - w.0).abs() + (s.1 - w.1).abs()).unwrap();
                    let t = left.remove(i);
                    if w != t { plaza_steps.push(Step::MoveOne { from: w, to: t }); lay.gone.insert(w); }
                    n_done += 1;
                }
                while n_done < want && !left.is_empty() { plaza_steps.push(Step::Place { obj: 39, at: left.remove(0) }); n_done += 1; }
                // ช่องที่ต้องเคลียร์ก่อน (ต้นไม้ในลาน)
                for s in &plaza_steps {
                    let at = match s { Step::MoveOne { to, .. } => *to, Step::Place { at, .. } => *at, _ => continue };
                    if let Cs::Clear(o) = lay.cs(at) { if cleared.insert(at) { clears.push(Step::Clear { at, obj: o }); } }
                    overlay.push((at.0, at.1, 3));
                }
            } else {
                note(&mut plan, "ไม่มีที่โล่งพอทำลานกลางเมือง — ข้ามส่วนลาน".to_string());
            }
        }
        if opts & OPT_WALLS != 0 { for &w in &sv.walls { clears.push(Step::Clear { at: w, obj: *self.w.placed.get(&w).unwrap_or(&10) }); cleared.insert(w); overlay.push((w.0, w.1, 2)); } }

        // ---- แปลงบ้านเป็นล็อตริมถนน
        let mut order: Vec<&Hs> = sel.clone();
        order.sort_by_key(|h| ((h.door.0 - pc.0).abs() + (h.door.1 - pc.1).abs(), h.door));
        let ew = variant == 0;
        let mut moves: Vec<(Step, Vec<(i32, i32)>, Vec<(i32, i32)>)> = vec![]; // (step, ช่องใหม่, ช่องเดิม)
        let mut street_ext: HashMap<i32, (i32, i32)> = HashMap::new();
        let mut unplaced = 0;
        for h in order {
            let mut best: Option<(i32, (i32, i32), u8, Vec<(i32, i32)>, Vec<(i32, i32)>, i32)> = None;
            for s in [0i32, -1, 1, -2, 2] {
                for side in 0..2u8 {
                    for dir in [1i32, -1] {
                        let base = if ew { pc.0 } else { pc.1 };
                        let mut u = base + if s == 0 { 2 * dir } else if dir > 0 { 0 } else { -1 };
                        for _ in 0..40 {
                            if (u - base).abs() > 38 { break; }
                            let (door, face) = if ew {
                                let sy = pc.1 + s * BLOCK;
                                if side == 0 { ((u, sy - 1), 0u8) } else { ((u, sy + 2), 2u8) }
                            } else {
                                let sx = pc.0 + s * BLOCK;
                                if side == 0 { ((sx - 1, u), 3u8) } else { ((sx + 2, u), 1u8) }
                            };
                            if let Some((fp, cl)) = Self::try_lot(&mut lay, &rsv, &fp_all, h, door, face) {
                                let cost = (door.0 - h.door.0).abs() + (door.1 - h.door.1).abs() + s.abs() * 4;
                                if best.as_ref().map_or(true, |b| cost < b.0) { best = Some((cost, door, face, fp, cl, s)); }
                                break;
                            }
                            u += dir;
                        }
                    }
                }
            }
            let Some((_, door, face, fp, cl, s)) = best else { unplaced += 1; continue };
            let rot = (face + 4 - h.face) % 4;
            for &c in &fp { for n in cheb1(c) { rsv.insert(n); } fp_all.insert(c); overlay.push((c.0, c.1, 1)); }
            for &t in &h.carry.tiles { overlay.push((t.0, t.1, 0)); }
            for at in cl { if let Cs::Clear(obj) = lay.cs(at) { if cleared.insert(at) { clears.push(Step::Clear { at, obj }); overlay.push((at.0, at.1, 2)); } } }
            let e = street_ext.entry(s).or_insert((i32::MAX, i32::MIN));
            let u = if ew { door.0 } else { door.1 };
            e.0 = e.0.min(u); e.1 = e.1.max(u);
            moves.push((Step::MoveHouse { door: h.door, to: door, rot }, fp, h.carry.tiles.clone()));
        }
        if unplaced > 0 { note(&mut plan, format!("หาที่ลงให้บ้านไม่ได้ {} หลัง (ที่ไม่พอ/มีของขวาง) — ปล่อยไว้ที่เดิม", unplaced)); }
        // เรียงลำดับย้ายบ้าน: หลังที่ปลายทางไม่ทับบ้านที่ยังไม่ย้าย ไปก่อน
        let mut ordered: Vec<(Step, Vec<(i32, i32)>, Vec<(i32, i32)>)> = vec![];
        while !moves.is_empty() {
            let olds: HashSet<(i32, i32)> = moves.iter().flat_map(|m| m.2.iter().copied()).collect();
            let i = moves.iter().position(|m| m.1.iter().all(|c| !olds.contains(c) || m.2.contains(c))).unwrap_or(0);
            ordered.push(moves.remove(i));
        }
        plan.houses = ordered.len();

        // ---- ถนน
        let mut roads: Vec<Step> = vec![];
        if opts & OPT_ROAD != 0 {
            let mut keys: Vec<i32> = street_ext.keys().copied().collect();
            keys.sort();
            for s in keys {
                let (a, b) = street_ext[&s];
                let cu = if ew { pc.0 } else { pc.1 };
                let (mut a, mut b) = (a - 2, b + 2);
                if s == 0 { a = a.min(cu - PLAZA_R); b = b.max(cu + PLAZA_R); }
                let (x0, y0, x1, y1) = if ew { let sy = pc.1 + s * BLOCK; (a, sy, b, sy + 1) } else { let sx = pc.0 + s * BLOCK; (sx, a, sx + 1, b) };
                roads.push(Step::Pave { x0, y0, x1, y1 });
                for y in y0..=y1 { for x in x0..=x1 { overlay.push((x, y, 4)); } }
            }
        }

        plan.steps = clears.into_iter().chain(ordered.into_iter().map(|m| m.0)).chain(plaza_steps).chain(roads).collect();
        plan.done = vec![0; plan.steps.len()];
        plan.overlay = overlay;
        let _ = ring_cells;
        if plan.steps.is_empty() { return None; }
        Some(plan)
    }

    /// ล็อตนี้ลงบ้านหลังนี้ได้ไหม: (ช่องตัวบ้านใหม่, ช่องที่ต้องเคลียร์) — None = ไม่ได้
    fn try_lot(lay: &mut Lay, rsv: &HashSet<(i32, i32)>, fp_all: &HashSet<(i32, i32)>, h: &Hs, door: (i32, i32), face: u8) -> Option<(Vec<(i32, i32)>, Vec<(i32, i32)>)> {
        let k = (face + 4 - h.face) % 4;
        let mut fp: Vec<(i32, i32)> = vec![];
        let mut cl: Vec<(i32, i32)> = vec![];
        for t in &h.carry.tiles {
            let o = rot_off((t.0 - h.door.0, t.1 - h.door.1), k);
            let c = (door.0 + o.0, door.1 + o.1);
            if rsv.contains(&c) { return None; }
            // ช่องที่ถูกดัดแปลงพื้น (ถมน้ำ/ปูถนน) ปูพื้นบ้านทับไม่ได้ (กติกาเดียวกับตอนย้ายของ)
            if lay.g.w.tmod.get(&c).map_or(false, |&x| x != T_FLOORW && x != T_FLOORS) && h.carry.tiles.iter().any(|t| lay.g.w.tmod.get(t).map_or(false, |&x| x == T_FLOORW || x == T_FLOORS)) { return None; }
            match lay.cs(c) { Cs::Blocked => return None, Cs::Clear(_) => cl.push(c), Cs::Free => {} }
            fp.push(c);
        }
        // หน้าประตู 2 ช่อง + ข้างๆ ต้องโล่ง (เป็นถนน) และไม่ทับตัวบ้านอื่น
        let fd = rot_off((0, 1), face);
        let pd = rot_off((1, 0), face);
        let f1 = (door.0 + fd.0, door.1 + fd.1);
        let f2 = (door.0 + 2 * fd.0, door.1 + 2 * fd.1);
        for c in [f1, f2, (f1.0 + pd.0, f1.1 + pd.1), (f1.0 - pd.0, f1.1 - pd.1)] {
            if fp_all.contains(&c) || fp.contains(&c) { return None; }
            match lay.cs(c) { Cs::Blocked => return None, Cs::Clear(_) => cl.push(c), Cs::Free => {} }
        }
        Some((fp, cl))
    }

    // ---------------------------------------------------------------- ข้อเสนอ → รับ/ปฏิเสธ
    pub fn arch_hire(&mut self, variant: u8) -> bool {
        if self.w.dun.is_some() { self.toast("จ้างช่างในดันเจี้ยนไม่ได้"); return false; }
        if self.rs.ar.active.is_some() { self.toast("ช่างกำลังทำตามแผนเดิมอยู่ — ยกเลิกแผนเดิมก่อนถ้าจะวางผังใหม่"); return false; }
        let opts = self.arch_opts();
        match self.arch_plan(variant, opts) {
            Some(p) => {
                self.rs.ar.variant = variant;
                let (h, (nat, wall)) = (p.count_houses(), p.count_clear());
                self.toast(format!("สถาปนิกเสนอแบบ \"{}\": ย้ายบ้าน {} หลัง เคลียร์ {} · รื้อกำแพง {}", VARIANT_NAMES[variant as usize & 1], h, nat, wall));
                self.rs.ar.prop = Some(p);
                self.rs.ar.show = true;
                true
            }
            None => {
                self.rs.ar.prop = None;
                self.toast(if self.town_center().is_none() { "ยังไม่มีบ้านในเมืองให้วางผัง" } else { "ไม่มีอะไรให้ปรับตามหมวดที่เลือก (หรือหาที่ลงไม่ได้เลย) — ลองเปลี่ยนแบบหรือหมวด" });
                false
            }
        }
    }
    pub fn arch_reject(&mut self) { if self.rs.ar.prop.take().is_some() { self.toast("ปฏิเสธแบบนี้ — ไม่เสียค่าจ้าง"); } }
    pub fn arch_accept(&mut self) -> bool {
        let Some(p) = self.rs.ar.prop.clone() else { return false };
        let fee = self.arch_fee(&p);
        if self.p.inv[COIN] + self.tax_pool < fee { self.toast(format!("ค่าจ้างสถาปนิก {} เหรียญ — เงินไม่พอ", fee)); play(&self.sfx.deny); return false; }
        let from_bag = self.p.inv[COIN].min(fee);
        self.p.inv[COIN] -= from_bag;
        self.tax_pool -= fee - from_bag;
        self.rs.ar.prop = None;
        self.rs.ar.stall = 0;
        self.rs.ar.t = 0.;
        self.rs.ar.finished += 0;
        let sf = p.shortfall();
        self.rs.ar.active = Some(p);
        self.news_add(format!("จ้างสถาปนิกวางผังเมือง (ค่าจ้าง {} เหรียญ)", fee));
        self.toast(if sf.is_empty() { "ช่างเริ่มงานแล้ว!".to_string() } else { "ช่างเริ่มงาน — บางอย่างรอวัสดุ (ดูรายการที่ขาดในแท็บชุมชน)".to_string() });
        play(&self.sfx.click);
        true
    }
    /// ใส่วัสดุที่ช่างยังขาด จากกระเป๋าเรา
    pub fn arch_deposit(&mut self) -> bool {
        let Some(p) = self.rs.ar.active.as_mut() else { return false };
        let mut any = false;
        for (it, q) in p.shortfall() {
            let give = q.min(self.p.inv[it]);
            if give > 0 { self.p.inv[it] -= give; p.pool[it] += give; any = true; }
        }
        if any { play(&self.sfx.click); } else { self.toast("ในกระเป๋าไม่มีของที่ช่างขาด"); }
        any
    }
    /// ยกเลิกแผนที่กำลังทำ: ของที่ทำไปแล้วอยู่อย่างนั้น วัสดุที่เหลือคืนกระเป๋า
    pub fn arch_cancel(&mut self) {
        if let Some(p) = self.rs.ar.active.take() {
            for i in 0..NI { self.p.inv[i] += p.pool[i]; }
            let (d, n) = p.progress();
            self.toast(format!("ยกเลิกแผนวางผัง (ทำไปแล้ว {}/{}) — คืนวัสดุที่เหลือให้แล้ว", d, n));
        }
    }

    // ---------------------------------------------------------------- ช่างลงมือ
    fn arch_run_step(&mut self, step: &Step) -> Option<bool> {
        // คืน: Some(true) ทำสำเร็จ · Some(false) ทำไม่ได้แน่ๆ (ข้าม) · None = ยังทำไม่ได้ตอนนี้ (รอ)
        match step {
            Step::Clear { at, .. } => {
                let o = self.w.obj_at(at.0, at.1);
                if o == 0 { return Some(true); }
                if let Some((it, q)) = clear_refund(o) { if let Some(p) = self.rs.ar.active.as_mut() { p.pool[it] += q; } }
                if self.w.placed.remove(at).is_none() { self.w.removed.insert(*at); }
                self.obj_dmg.remove(at);
                self.burst(at.0 as f32 * 16. + 8., at.1 as f32 * 16. + 8., hx(0xc8a070), 6, 30.);
                Some(true)
            }
            Step::MoveHouse { door, to, rot } => {
                let Ok(c) = self.move_select(*door) else { return None };
                if !c.building { return Some(false); }
                if self.move_blocked_ex(&c, *to, *rot, false).is_some() { return None; }
                self.move_apply(&c, *to, *rot);
                self.burst(to.0 as f32 * 16. + 8., to.1 as f32 * 16. + 8., hx(0xc8a070), 14, 55.);
                Some(true)
            }
            Step::MoveOne { from, to } => {
                let Ok(c) = self.move_select(*from) else { return Some(false) };
                if self.move_blocked_ex(&c, *to, 0, false).is_some() { return None; }
                self.move_apply(&c, *to, 0);
                self.burst(to.0 as f32 * 16. + 8., to.1 as f32 * 16. + 8., hx(0xc8a070), 8, 40.);
                Some(true)
            }
            Step::Place { obj, at } => {
                let cost = place_cost(*obj);
                if let Some(p) = self.rs.ar.active.as_ref() { if cost.iter().any(|&(it, q)| p.pool[it] < q) { return None; } }
                if !self.can_place_at(at.0, at.1) { return None; }
                if self.w.obj_at(at.0, at.1) == 3 { self.w.removed.insert(*at); }
                self.w.placed.insert(*at, *obj);
                if let Some(p) = self.rs.ar.active.as_mut() { for (it, q) in cost { p.pool[it] -= q; } }
                self.burst(at.0 as f32 * 16. + 8., at.1 as f32 * 16. + 8., hx(0xc8a070), 8, 40.);
                self.refresh_torches();
                Some(true)
            }
            Step::Pave { x0, y0, x1, y1 } => {
                let cells: Vec<(i32, i32)> = (*y0..=*y1).flat_map(|y| (*x0..=*x1).map(move |x| (x, y)))
                    .filter(|&(x, y)| { let t = self.w.tile_at(x, y); !SOLIDT[t as usize] && t != T_ROAD && t != T_SOIL && t != T_WET && t != T_FLOORW && t != T_FLOORS && !self.w.placed.contains_key(&(x, y)) && !self.crops.contains_key(&(x, y)) && !self.site_reserved((x, y)) && self.w.realm.city_at(x, y).is_none() })
                    .collect();
                let n = cells.len() as i32;
                if let Some(p) = self.rs.ar.active.as_ref() { if p.pool[STONE] < n { return None; } }
                for c in cells { self.w.tmod.insert(c, T_ROAD); }
                if let Some(p) = self.rs.ar.active.as_mut() { p.pool[STONE] -= n; }
                Some(true)
            }
        }
    }
    /// ทำหนึ่งอย่างจากแผน: ได้ทำอะไรไหม
    pub fn arch_step_once(&mut self) -> bool {
        let Some(plan) = self.rs.ar.active.as_ref() else { return false };
        let pending: Vec<usize> = (0..plan.steps.len()).filter(|&i| plan.done[i] == 0).collect();
        for i in pending {
            let step = self.rs.ar.active.as_ref().unwrap().steps[i].clone();
            match self.arch_run_step(&step) {
                Some(ok) => {
                    if let Some(p) = self.rs.ar.active.as_mut() { p.done[i] = if ok { 1 } else { 2 }; }
                    // เคลียร์ต้นไม้/รื้อกำแพงทำรวดเดียวทั้งชุด (ไม่ต้องรอทีละชิ้น)
                    if matches!(step, Step::Clear { .. }) {
                        for j in i + 1..self.rs.ar.active.as_ref().map_or(0, |p| p.steps.len()) {
                            let (is_clear, d) = { let p = self.rs.ar.active.as_ref().unwrap(); (matches!(p.steps[j], Step::Clear { .. }), p.done[j]) };
                            if !is_clear { break; }
                            if d != 0 { continue; }
                            let st = self.rs.ar.active.as_ref().unwrap().steps[j].clone();
                            if let Some(ok) = self.arch_run_step(&st) { if let Some(p) = self.rs.ar.active.as_mut() { p.done[j] = if ok { 1 } else { 2 }; } }
                        }
                    }
                    return true;
                }
                None => continue,
            }
        }
        false
    }
    pub fn arch_tick(&mut self, dt: f32) {
        if self.rs.ar.active.is_none() || self.w.dun.is_some() { return; }
        if self.rs.ar.active.as_ref().map_or(false, |p| p.overlay.is_empty() && !p.steps.is_empty()) { self.arch_rebuild_overlay(); }
        self.rs.ar.t -= dt;
        if self.rs.ar.t > 0. { return; }
        self.rs.ar.t = STEP_SECS;
        if self.arch_step_once() {
            self.rs.ar.stall = 0;
            if self.rs.ar.active.as_ref().map_or(false, |p| p.finished()) { self.arch_finish(); }
            return;
        }
        // ทำอะไรไม่ได้เลย: รอวัสดุ หรือ ติดขัด
        let waiting = {
            let p = self.rs.ar.active.as_ref().unwrap();
            (0..p.steps.len()).any(|i| p.done[i] == 0 && matches!(p.steps[i], Step::Place { .. } | Step::Pave { .. }) && !p.shortfall().is_empty())
        };
        let only_mat = {
            let p = self.rs.ar.active.as_ref().unwrap();
            (0..p.steps.len()).all(|i| p.done[i] != 0 || matches!(p.steps[i], Step::Place { .. } | Step::Pave { .. }))
        };
        if waiting && only_mat { return; }
        self.rs.ar.stall += 1;
        if self.rs.ar.stall >= 4 {
            // ติดกันเอง/ของขวาง: ข้ามงานที่ยังย้ายไม่ได้ ที่เหลือทำต่อ
            let mut skipped = 0;
            if let Some(p) = self.rs.ar.active.as_mut() {
                for i in 0..p.steps.len() { if p.done[i] == 0 && !matches!(p.steps[i], Step::Place { .. } | Step::Pave { .. }) { p.done[i] = 2; skipped += 1; } }
                if skipped == 0 { for i in 0..p.steps.len() { if p.done[i] == 0 { p.done[i] = 2; skipped += 1; } } }
            }
            self.rs.ar.stall = 0;
            if skipped > 0 { self.news_add(format!("ช่างทำต่อไม่ได้ {} อย่าง (มีของขวาง/ติดกัน) — ลองให้ช่างวางผังใหม่อีกรอบ", skipped)); }
            if self.rs.ar.active.as_ref().map_or(false, |p| p.finished()) { self.arch_finish(); }
        }
    }
    fn arch_finish(&mut self) {
        let Some(p) = self.rs.ar.active.take() else { return };
        let moved = p.steps.iter().zip(&p.done).filter(|(s, d)| matches!(s, Step::MoveHouse { .. }) && **d == 1).count();
        let skipped = p.done.iter().filter(|&&d| d == 2).count();
        for i in 0..NI { self.p.inv[i] += p.pool[i]; }
        self.rs.ar.finished += 1;
        self.rs.ar.show = false;
        for n in self.npcs.iter_mut().filter(|n| n.state == ST_HOME) { n.mood = (n.mood + 3).min(100); }
        self.news_add(format!("ช่างสถาปนิกวางผังเมืองเสร็จแล้ว: ย้ายบ้าน {} หลัง{} — ชาวบ้านชอบเมืองที่เป็นระเบียบ", moved, if skipped > 0 { format!(" (ข้าม {} อย่าง)", skipped) } else { String::new() }));
        self.toast("ผังเมืองใหม่เสร็จแล้ว! วัสดุที่เหลือคืนกระเป๋าแล้ว");
        play(&self.sfx.lvl);
    }
    /// ภาพของแผนที่รอทำ (หลังโหลดเซฟ): ช่องบ้านเดิม/ใหม่
    pub fn arch_rebuild_overlay(&mut self) {
        let Some(p) = self.rs.ar.active.clone() else { return };
        let mut ov: Vec<(i32, i32, u8)> = vec![];
        for (i, s) in p.steps.iter().enumerate() {
            if p.done[i] != 0 { continue; }
            match s {
                Step::Clear { at, .. } => ov.push((at.0, at.1, 2)),
                Step::MoveHouse { door, to, rot } => {
                    if let Ok(c) = self.move_select(*door) {
                        for &t in &c.tiles { ov.push((t.0, t.1, 0)); }
                        for (_, d) in self.move_targets(&c, *to, *rot) { ov.push((d.0, d.1, 1)); }
                    }
                }
                Step::MoveOne { to, .. } | Step::Place { at: to, .. } => ov.push((to.0, to.1, 3)),
                Step::Pave { x0, y0, x1, y1 } => { for y in *y0..=*y1 { for x in *x0..=*x1 { ov.push((x, y, 4)); } } }
            }
        }
        if ov.is_empty() { ov.push((i32::MIN, i32::MIN, 9)); }
        if let Some(a) = self.rs.ar.active.as_mut() { a.overlay = ov; }
    }

    // ---------------------------------------------------------------- เซฟ
    pub fn arch_save(&self) -> String {
        let a = &self.rs.ar;
        let mut s = String::new();
        if a.set || a.finished > 0 { s += &format!("arch {} {} {}\n", a.opts, a.variant, a.finished); }
        if let Some(p) = &a.active {
            s += &format!("archplan {},{},{},{},{}", p.variant, p.opts, p.center.0, p.center.1, p.houses);
            for (i, &q) in p.pool.iter().enumerate() { if q != 0 { s += &format!(" p{}:{}", i, q); } }
            for (i, st) in p.steps.iter().enumerate() {
                let d = p.done[i];
                s += &match st {
                    Step::Clear { at, obj } => format!(" c,{},{},{},{}", at.0, at.1, obj, d),
                    Step::MoveHouse { door, to, rot } => format!(" h,{},{},{},{},{},{}", door.0, door.1, to.0, to.1, rot, d),
                    Step::MoveOne { from, to } => format!(" m,{},{},{},{},{}", from.0, from.1, to.0, to.1, d),
                    Step::Place { obj, at } => format!(" a,{},{},{},{}", obj, at.0, at.1, d),
                    Step::Pave { x0, y0, x1, y1 } => format!(" r,{},{},{},{},{}", x0, y0, x1, y1, d),
                };
            }
            s += "\n";
        }
        s
    }
    pub fn arch_load(&mut self, tag: &str, r: &[&str]) {
        if tag == "arch" {
            let n = |i: usize| r.get(i).and_then(|v| v.parse::<i32>().ok()).unwrap_or(0);
            self.rs.ar.opts = (n(0) as u8) & 31;
            self.rs.ar.set = true;
            self.rs.ar.variant = (n(1) as u8).min(1);
            self.rs.ar.finished = n(2);
            return;
        }
        let Some(head) = r.first() else { return };
        let h: Vec<i32> = head.split(',').filter_map(|v| v.parse().ok()).collect();
        if h.len() < 5 { return; }
        let mut p = Plan { variant: h[0] as u8, opts: h[1] as u8, center: (h[2], h[3]), houses: h[4] as usize, ..Default::default() };
        for t in &r[1..] {
            if let Some(v) = t.strip_prefix('p') {
                let mut it = v.split(':');
                if let (Some(a), Some(b)) = (it.next().and_then(|x| x.parse::<usize>().ok()), it.next().and_then(|x| x.parse::<i32>().ok())) { if a < NI { p.pool[a] = b; } }
                continue;
            }
            let kind = t.chars().next().unwrap_or(' ');
            let v: Vec<i32> = t.get(2..).unwrap_or("").split(',').filter_map(|x| x.parse().ok()).collect();
            let (st, d) = match (kind, v.len()) {
                ('c', 4) => (Step::Clear { at: (v[0], v[1]), obj: v[2] as u8 }, v[3]),
                ('h', 6) => (Step::MoveHouse { door: (v[0], v[1]), to: (v[2], v[3]), rot: v[4] as u8 }, v[5]),
                ('m', 5) => (Step::MoveOne { from: (v[0], v[1]), to: (v[2], v[3]) }, v[4]),
                ('a', 4) => (Step::Place { obj: v[0] as u8, at: (v[1], v[2]) }, v[3]),
                ('r', 5) => (Step::Pave { x0: v[0], y0: v[1], x1: v[2], y1: v[3] }, v[4]),
                _ => continue,
            };
            p.steps.push(st);
            p.done.push(d.clamp(0, 2) as u8);
        }
        if !p.steps.is_empty() && !p.finished() { self.rs.ar.active = Some(p); }
    }
}

/// แผนที่ซ้อนในโลก: เหลือง = บ้านที่จะย้าย · เขียว = ที่ใหม่ · แดง = เคลียร์/รื้อ · ฟ้า = ลาน/บ่อ/กองไฟ · น้ำตาล = ถนน
pub fn draw_arch_overlay(g: &Game, v: &View, _font: &Font, t: f32) {
    if !g.rs.ar.show || g.w.dun.is_some() { return; }
    let Some(p) = g.rs.ar.active.as_ref().or(g.rs.ar.prop.as_ref()) else { return };
    let pul = 0.5 + 0.2 * (t * 3.).sin();
    for &(x, y, k) in &p.overlay {
        if x == i32::MIN { continue; }
        let (px, py) = ((x as f32 * 16. - v.cx) * v.s, (y as f32 * 16. - v.cy) * v.s);
        if px < -20. || py < -20. || px > screen_width() || py > screen_height() { continue; }
        let s = 16. * v.s;
        let c = match k {
            0 => Color::new(1., 0.85, 0.2, 0.22),
            1 => Color::new(0.3, 1., 0.4, 0.28 * pul + 0.1),
            2 => Color::new(1., 0.25, 0.25, 0.35),
            3 => Color::new(0.4, 0.8, 1., 0.35),
            _ => Color::new(0.8, 0.6, 0.35, 0.22),
        };
        draw_rectangle(px, py, s, s, c);
        if k == 0 || k == 1 { draw_rectangle_lines(px, py, s, s, 1., Color::new(c.r, c.g, c.b, 0.6)); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn town() -> (Game, (i32, i32)) {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        g.npcs.clear();
        let c = (tfloor(g.p.x), tfloor(g.p.y));
        for y in c.1 - 50..=c.1 + 50 { for x in c.0 - 50..=c.0 + 50 {
            g.w.removed.insert((x, y));
            let t = g.w.tile_at(x, y);
            if SOLIDT[t as usize] || t == T_SOIL { g.w.tmod.insert((x, y), 3); }
        } }
        g.p.x = c.0 as f32 * 16. + 8.; g.p.y = c.1 as f32 * 16. + 8.;
        (g, c)
    }
    /// สร้างบ้านสำเร็จรูปทันที (ไม่ผ่านไซต์ก่อสร้าง) + ผู้อยู่อาศัย
    // prefab_steps ใช้ชื่อ Step ของ house.rs — ที่นี่ชนกับ Step ของ arch จึงอ้างชื่อเต็ม
    use crate::house::Step as HStep;
    fn scattered(g: &mut Game, c: (i32, i32)) -> Vec<(i32, i32)> {
        // บ้านสุ่มตำแหน่งทิศมั่วๆ 5 หลัง + กำแพงเกะกะ + บ่อน้ำกระจัดกระจาย
        let spots = [((-12, -3), 0u8, OBJ_HOUSE_S), ((-3, 9), 1, OBJ_HUT), ((6, -9), 2, OBJ_HOUSE_S), ((14, 6), 3, OBJ_STONEHOUSE), ((-15, 14), 0, OBJ_HOUSE_L)];
        let mut beds = vec![];
        for (i, (o, d, k)) in spots.iter().enumerate() {
            let door = (c.0 + o.0, c.1 + o.1);
            let mut bed = (0, 0);
            for (p, s) in prefab_steps(*k, door, *d) {
                match s { HStep::Tile(t) => { g.w.tmod.insert(p, t); } HStep::Obj(ob) => { g.w.placed.insert(p, ob); if ob == 33 { bed = p; } } }
            }
            let id = i as u32 + 1;
            g.npcs.push(Npc { id, name: (id % 8) as u8, look: 0, x: bed.0 as f32 * 16. + 8., y: bed.1 as f32 * 16. + 18., state: ST_HOME, hearts: 3, mood: 60, married: false, child: false, role: 0, bed, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(id, 0) });
            beds.push(bed);
        }
        beds
    }
    fn run_all(g: &mut Game) { for _ in 0..400 { g.arch_tick(STEP_SECS + 0.01); if g.rs.ar.active.is_none() { break; } } }
    fn beds_of(g: &Game) -> Vec<(i32, i32)> { g.npcs.iter().map(|n| n.bed).collect() }
    fn rooms_ok(g: &Game) -> usize { g.npcs.iter().filter(|n| g.w.room_from(n.bed).map_or(false, |r| r.has_bed)).count() }

    #[test]
    fn survey_finds_houses_walls_wells_and_ring() {
        let (mut g, c) = town();
        scattered(&mut g, c);
        for k in [(c.0 + 1, c.1 + 20), (c.0 + 2, c.1 + 20), (c.0 + 3, c.1 + 20)] { g.w.placed.insert(k, 10); }
        g.w.placed.insert((c.0 - 20, c.1 + 3), 39);
        g.w.placed.insert((c.0 + 20, c.1 - 8), O_BIGWELL);
        g.w.placed.insert((c.0 + 2, c.1 - 18), 9); g.w.placed.insert((c.0 + 4, c.1 - 18), O_LOG); g.w.placed.insert((c.0, c.1 - 18), O_LOG);
        let sv = g.arch_survey().unwrap();
        assert_eq!(sv.houses.len(), 5, "บ้านครบ 5 หลัง");
        assert!(sv.houses.iter().all(|h| h.owner.is_some() && !h.mine));
        assert_eq!(sv.walls.len(), 3, "กำแพงสะเปะสะปะ 3 ชิ้น (ผนังบ้านไม่นับ)");
        assert_eq!(sv.wells.len(), 2);
        assert_eq!(sv.ring.len(), 3, "กองไฟ + ท่อนไม้ 2");
        // ทิศประตูอ่านถูก: บ้านที่สร้างด้วย dir d ประตูหัน d
        let mut faces: Vec<u8> = sv.houses.iter().map(|h| h.face).collect(); faces.sort();
        assert_eq!(faces, vec![0, 0, 1, 2, 3]);
    }
    #[test]
    fn plan_lines_houses_up_on_streets_facing_the_road() {
        let (mut g, c) = town();
        scattered(&mut g, c);
        let p = g.arch_plan(0, OPT_DEFAULT).expect("มีแผน");
        assert_eq!(p.count_houses(), 5, "ทุกหลังมีที่ลง · {:?}", p.notes);
        // ทุกหลังหันประตูเข้าถนน (แนวตะวันออก-ตะวันตก ประตูหันลง/ขึ้น) และอยู่คนละล็อต ไม่ทับกัน
        let sv = g.arch_survey().unwrap();
        let mut cells: HashSet<(i32, i32)> = HashSet::new();
        for s in &p.steps {
            if let Step::MoveHouse { door, to, rot } = s {
                let h = sv.houses.iter().find(|h| h.door == *door).unwrap();
                let nf = (h.face + rot) % 4;
                assert!(nf == 0 || nf == 2, "ถนนแนว E-W → ประตูหันลง/ขึ้น (ได้ {})", nf);
                for (_, d) in g.move_targets(&h.carry, *to, *rot) { assert!(cells.insert(d), "ล็อตต้องไม่ทับกัน {:?}", d); }
            }
        }
        // ประตูใหม่ทุกบานอยู่ริมถนน: แถวประตู = (กลางลาน + ทวีคูณของ 16) -1 (ฝั่งเหนือ) หรือ +2 (ฝั่งใต้)
        let pc = p.steps.iter().find_map(|s| if let Step::Place { obj: 9, at } = s { Some(*at) } else { None }).expect("กองไฟกลางลาน");
        for s in &p.steps { if let Step::MoveHouse { to, .. } = s { let r = (to.1 - pc.1).rem_euclid(BLOCK); assert!(r == BLOCK - 1 || r == 2, "ประตู {:?} ไม่ได้อยู่ริมถนน (เศษ {})", to, r); } }
        let pn = g.arch_plan(1, OPT_DEFAULT).unwrap();
        assert_eq!(pn.count_houses(), 5);
        let pcn = pn.steps.iter().find_map(|s| if let Step::Place { obj: 9, at } = s { Some(*at) } else { None }).unwrap();
        for s in &pn.steps { if let Step::MoveHouse { to, door, rot } = s {
            let r = (to.0 - pcn.0).rem_euclid(BLOCK); assert!(r == BLOCK - 1 || r == 2, "แนวเหนือ-ใต้: ประตู {:?} ไม่อยู่ริมถนน", to);
            let h = g.arch_survey().unwrap().houses.into_iter().find(|h| h.door == *door).unwrap();
            let nf = (h.face + rot) % 4; assert!(nf == 1 || nf == 3, "ถนนแนว N-S → ประตูหันซ้าย/ขวา");
        } }
    }
    #[test]
    fn executing_plan_moves_every_house_intact_and_owners_follow() {
        let (mut g, c) = town();
        let beds0 = scattered(&mut g, c);
        let placed0 = g.w.placed.values().filter(|&&o| o == 33).count();
        let n_obj = g.w.placed.len();
        let p = g.arch_plan(0, OPT_DEFAULT).unwrap();
        g.rs.ar.active = Some(p.clone());
        g.p.inv[WOOD] = 200; g.p.inv[STONE] = 200;
        assert!(g.arch_deposit());
        run_all(&mut g);
        assert!(g.rs.ar.active.is_none(), "แผนเสร็จ");
        assert_eq!(g.w.placed.values().filter(|&&o| o == 33).count(), placed0, "เตียงไม่หายไม่เพิ่ม");
        assert_eq!(rooms_ok(&g), 5, "ทุกบ้านยังเป็นห้องปิดที่มีเตียง");
        assert!(beds_of(&g).iter().zip(&beds0).filter(|(a, b)| a != b).count() >= 4, "เจ้าของเตียงตามไปบ้านใหม่");
        // บ้านเป็นระเบียบ: ประตูทุกบานโล่ง
        assert!(g.blocked_doors().is_empty(), "ประตูต้องไม่ถูกบัง");
        assert!(g.w.placed.len() >= n_obj, "ของครบ (และมีวงกองไฟ/บ่อน้ำเพิ่ม)");
        assert!(g.news.iter().any(|n| n.1.contains("วางผังเมืองเสร็จ")));
        let _ = p;
    }
    #[test]
    fn plaza_gets_fire_ring_and_wells_and_stray_walls_are_cleared_with_refund() {
        let (mut g, c) = town();
        scattered(&mut g, c);
        for x in 0..6 { g.w.placed.insert((c.0 + 22 + x, c.1 + 24), 11); }
        let p = g.arch_plan(0, OPT_DEFAULT).unwrap();
        let (_, walls) = p.count_clear();
        assert_eq!(walls, 6, "รื้อกำแพงหิน 6 ชิ้น");
        let sf = p.shortfall();
        assert!(sf.iter().any(|&(i, _)| i == WOOD), "ต้องใช้ไม้ทำวงกองไฟ/บ่อ — ขาดของ: {:?}", sf);
        g.rs.ar.active = Some(p);
        g.p.inv[WOOD] = 100; g.p.inv[STONE] = 100;
        assert!(g.arch_deposit());
        run_all(&mut g);
        assert!(g.rs.ar.active.is_none());
        let fires: Vec<_> = g.w.placed.iter().filter(|(_, &o)| o == 9).collect();
        assert_eq!(fires.len(), 1, "มีวงกองไฟหนึ่งวง");
        let f = *fires[0].0;
        let logs = g.w.placed.iter().filter(|(k, &o)| o == O_LOG && (k.0 - f.0).abs() <= 2 && (k.1 - f.1).abs() <= 2).count();
        assert_eq!(logs, 4, "ท่อนไม้นั่ง 4 ท่อนรอบกองไฟ");
        let wells = g.w.placed.values().filter(|&&o| o == 39).count();
        assert!(wells >= 1, "มีบ่อน้ำ");
        assert!(!g.w.placed.contains_key(&(c.0 + 22, c.1 + 24)), "กำแพงสะเปะสะปะหายไป");
        assert!(g.p.inv[STONE] > 0 && g.p.inv[WOOD] > 0, "วัสดุที่เหลือ+ของที่รื้อได้คืนกระเป๋า");
    }
    #[test]
    fn missing_materials_wait_then_continue_after_deposit() {
        let (mut g, c) = town();
        scattered(&mut g, c);
        let p = g.arch_plan(0, OPT_DEFAULT).unwrap();
        g.rs.ar.prop = Some(p.clone());
        g.p.inv[COIN] = 1;
        assert!(!g.arch_accept(), "เงินไม่พอจ่ายค่าจ้าง");
        assert!(g.rs.ar.prop.is_some() && g.p.inv[COIN] == 1);
        g.p.inv[COIN] = 500;
        let fee = g.arch_fee(&p);
        assert!(g.arch_accept());
        assert_eq!(g.p.inv[COIN], 500 - fee, "จ่ายค่าจ้างตอนรับแบบ");
        for _ in 0..200 { g.arch_tick(STEP_SECS + 0.01); }
        let a = g.rs.ar.active.as_ref().expect("ยังไม่เสร็จ รอวัสดุ");
        assert!(a.steps.iter().zip(&a.done).any(|(s, d)| matches!(s, Step::Place { .. }) && *d == 0), "ของใหม่ยังไม่ได้วาง");
        assert!(a.steps.iter().zip(&a.done).filter(|(s, _)| matches!(s, Step::MoveHouse { .. })).all(|(_, d)| *d == 1), "ย้ายบ้านเสร็จแล้ว ไม่ต้องรอวัสดุ");
        assert!(!a.shortfall().is_empty());
        g.p.inv[WOOD] = 60; g.p.inv[STONE] = 60;
        assert!(g.arch_deposit());
        run_all(&mut g);
        assert!(g.rs.ar.active.is_none(), "ใส่วัสดุแล้วเสร็จ");
        assert_eq!(g.w.placed.values().filter(|&&o| o == 9).count(), 1);
    }
    #[test]
    fn rejecting_costs_nothing_and_cancel_returns_materials_and_manual_build_still_works() {
        let (mut g, c) = town();
        scattered(&mut g, c);
        g.p.inv[COIN] = 100;
        assert!(g.arch_hire(0));
        g.arch_reject();
        assert!(g.rs.ar.prop.is_none() && g.p.inv[COIN] == 100, "ปฏิเสธ = ไม่เสียเงิน");
        assert!(g.arch_hire(1));
        assert!(g.arch_accept());
        g.p.inv[WOOD] = 30;
        assert!(g.arch_deposit());
        let w = g.rs.ar.active.as_ref().unwrap().pool[WOOD];
        assert!(w > 0 && w <= 30);
        g.arch_cancel();
        assert!(g.rs.ar.active.is_none());
        assert_eq!(g.p.inv[WOOD], 30, "ยกเลิก = คืนวัสดุที่ใส่ไว้");
        // ยังย้ายบ้านเองได้ตามเดิม
        let beds = beds_of(&g);
        let sel = g.move_select(g.w.room_from(beds[0]).unwrap().doors[0]).unwrap();
        assert!(sel.building);
    }
    #[test]
    fn natural_trees_and_rocks_are_clearable_but_fruit_trees_bushes_and_water_are_not() {
        let g = Game::new_world_seed(4242, Sfx::default());
        let c = (tfloor(g.p.x), tfloor(g.p.y));
        let mut lay = Lay::new(&g);
        let (mut tree, mut rock, mut bush, mut fruit) = (None, None, None, None);
        for y in c.1 - 150..c.1 + 150 { for x in c.0 - 150..c.0 + 150 {
            let o = g.w.obj_at(x, y);
            if SOLIDT[g.w.tile_at(x, y) as usize] || g.w.realm.city_at(x, y).is_some() { continue; }
            if is_tree(o) && !is_fruit_tree(o) && o != T_SAPLING && o != T_DEAD && tree.is_none() { tree = Some(((x, y), o)); }
            if o == 4 && rock.is_none() { rock = Some((x, y)); }
            if o == 2 && bush.is_none() { bush = Some((x, y)); }
            if is_fruit_tree(o) && fruit.is_none() { fruit = Some((x, y)); }
        } }
        let (t, o) = tree.expect("มีต้นไม้ในโลก");
        assert_eq!(lay.cs(t), Cs::Clear(o));
        if let Some(r) = rock { assert_eq!(lay.cs(r), Cs::Clear(4)); }
        if let Some(b) = bush { assert_eq!(lay.cs(b), Cs::Blocked, "พุ่มไม้ (เบอร์รี่) เป็นอาหาร ไม่เคลียร์"); }
        if let Some(f) = fruit { assert_eq!(lay.cs(f), Cs::Blocked, "ต้นผลไม้ไม่เคลียร์"); }
        assert_eq!(clear_refund(1), Some((WOOD, 3)));
        assert_eq!(clear_refund(10), Some((WOOD, 1)));
        assert_eq!(clear_refund(11), Some((STONE, 2)));
    }
    #[test]
    fn plan_survives_save_load_midway() {
        let (mut g, c) = town();
        scattered(&mut g, c);
        let p = g.arch_plan(0, OPT_DEFAULT | OPT_ROAD).unwrap();
        g.rs.ar.active = Some(p);
        g.rs.ar.set = true; g.rs.ar.opts = OPT_DEFAULT | OPT_ROAD; g.rs.ar.finished = 2;
        for _ in 0..6 { g.arch_tick(STEP_SECS + 0.01); }
        let a = g.rs.ar.active.clone().unwrap();
        g.rs.ar.active.as_mut().unwrap().pool[STONE] = 7;
        let s = g.arch_save();
        let mut h = Game::new_world_seed(4242, Sfx::default());
        for line in s.lines() {
            let mut it = line.split(' ');
            let tag = it.next().unwrap();
            let rest: Vec<&str> = it.collect();
            h.arch_load(tag, &rest);
        }
        let b = h.rs.ar.active.clone().expect("โหลดแผนกลับมา");
        assert_eq!(b.steps, a.steps);
        assert_eq!(b.done, a.done);
        assert_eq!(b.pool[STONE], 7);
        assert_eq!((h.rs.ar.set, h.rs.ar.opts, h.rs.ar.finished), (true, OPT_DEFAULT | OPT_ROAD, 2));
        assert!(Game::new_world_seed(1, Sfx::default()).arch_save().is_empty(), "ไม่เคยใช้ = ไม่เขียนอะไรลงเซฟ");
    }
}
