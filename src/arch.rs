//! v36 ช่างสถาปนิก — จ้างมาวางผังเมืองให้เป็นระเบียบ (สำหรับคนขี้เกียจออกแบบเอง)
//! v37 ผังเมืองจริงจัง: เลือกขนาดเมือง (เล็ก/กลาง/ใหญ่ = ถนน 1/3/5 สาย) · ปูถนนทุกสาย + ถนนตัดผ่านลาน · ลานกลางเมืองปูหิน
//!   (กองไฟ ท่อนไม้ บ่อน้ำ คบเพลิงสี่มุม) · กำแพงรอบเมือง (รั้วไม้/กำแพงหิน/กำแพงเมือง) เว้นประตูเมืองตรงที่ถนนออก + คบเพลิงหน้าประตู
//!   ของไม่พอ = เป็น "แบบร่าง" เงาจางๆ บนพื้นไว้ก่อน ช่างสร้างทีละช่องเท่าที่วัสดุมี · เดินไปยืนที่ลานกลางเมืองพร้อมของ = ช่างรับไปเอง
//!
//! ลำดับ: จ้าง → ช่างสำรวจบ้าน/กำแพง/บ่อน้ำ/ที่นั่งคุยทั้งเมือง → เสนอ "แบบ" (ถนนแนวตะวันออก-ตะวันตก หรือเหนือ-ใต้)
//!   → เราเลือกรับ/ไม่รับ/ปรับหมวด → รับแล้วจ่ายค่าจ้าง ช่างลงมือทีละอย่าง
//! ย้ายบ้านใช้แกนเดียวกับปุ่ม "ย้ายของ" (ของในกล่อง เตียง เจ้าของเตียงตามไป) · ยังสร้าง/ย้ายเองได้เหมือนเดิมตลอด
use super::*;
use mover::Carry;

pub const OPT_NPC: u8 = 1;
pub const OPT_MINE: u8 = 2;
pub const OPT_WALLS: u8 = 4;
pub const OPT_PLAZA: u8 = 8;
pub const OPT_ROAD: u8 = 16;
/// v37 กำแพงรอบเมือง
pub const OPT_CITYWALL: u8 = 32;
pub const OPT_DEFAULT: u8 = OPT_NPC | OPT_MINE | OPT_WALLS | OPT_PLAZA | OPT_ROAD | OPT_CITYWALL;
pub const OPT_LIST: [(u8, &str); 5] = [
    (OPT_NPC, "ย้ายบ้านชาวบ้านเข้าแถวริมถนน"),
    (OPT_MINE, "ย้ายบ้านของเรา/บ้านว่าง/โกดังด้วย"),
    (OPT_WALLS, "รื้อกำแพงสะเปะสะปะ (คืนวัสดุ)"),
    (OPT_PLAZA, "ลานกลางเมือง (กองไฟ ท่อนไม้ บ่อน้ำ คบเพลิง)"),
    (OPT_ROAD, "ปูถนนทุกสาย + ลานปูหิน"),
];
pub const VARIANT_NAMES: [&str; 2] = ["ถนนแนวตะวันออก-ตะวันตก", "ถนนแนวเหนือ-ใต้"];
pub const SIZE_NAMES: [&str; 3] = ["เล็ก", "กลาง", "ใหญ่"];
pub const SIZE_DESC: [&str; 3] = ["ถนนหลัก 1 สาย + ถนนตัด 1 สาย (บ้าน ~8 หลัง)", "ถนน 3 สาย + ถนนตัด 1 สาย (บ้าน ~40 หลัง)", "ถนน 5 สาย + ถนนตัด 3 สาย (บ้าน ~100 หลัง)"];
pub const WALL_OBJ: [u8; 3] = [10, 11, 38];
pub const WALL_NAMES: [&str; 3] = ["รั้วไม้", "กำแพงหิน", "กำแพงเมือง"];

/// v38 งานของชาวบ้านที่ไปหาวัสดุมาช่วยสร้างเมือง (ของที่ได้เอาไปส่งที่ลานกลางเมือง)
pub const ARCH_HELP: u32 = 1 << 28;
pub const ARCH_FEE: i32 = 30;
pub const ARCH_FEE_HOUSE: i32 = 4;
pub const ARCH_FEE_SIZE: i32 = 15;
/// รัศมีที่ช่างสำรวจรอบใจกลางเมือง (ช่อง) — ครอบเมืองขนาดใหญ่ได้
pub const SURVEY_R: i32 = 50;
/// ลานกลางเมือง (พิกัดสัมพัทธ์ u,v): -5..=6 (12×12 ถนน 2 ช่องผ่านกลางพอดี)
pub const PLAZA_LO: i32 = -5;
pub const PLAZA_HI: i32 = 6;
/// v42 ลานตามขนาดเมือง: เล็ก 8×8 · กลาง 10×10 · ใหญ่ 12×12
pub fn plaza_lo(size: u8) -> i32 { -(3 + size.min(2) as i32) }
pub fn plaza_hi(size: u8) -> i32 { 4 + size.min(2) as i32 }
/// ระยะห่างระหว่างถนนขนาน (ช่อง) — บ้านหลังใหญ่หันหลังชนกันได้พอดี
pub const BLOCK: i32 = 18;
/// ช่างลงมือหนึ่งอย่างทุกกี่วินาที
pub const STEP_SECS: f32 = 1.2;
/// ถนน/กำแพง: สร้างได้กี่ช่องต่อครั้ง
const PAVE_BATCH: u32 = 10;
const WALL_BATCH: u32 = 6;

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
    /// ปูถนนเป็นผืน (หิน 1 ก้อน / 2 ช่อง) · at = ช่องถัดไปที่ต้องทำ (ไล่แถวจากซ้ายบน)
    Pave { x0: i32, y0: i32, x1: i32, y1: i32, at: u32 },
    /// กำแพงรอบเมืองหนึ่งช่วง (เส้นตรง) · at = ช่องถัดไปที่ต้องทำ
    Wall { x0: i32, y0: i32, x1: i32, y1: i32, obj: u8, at: u32 },
}

pub fn rect_len(x0: i32, y0: i32, x1: i32, y1: i32) -> u32 { ((x1 - x0 + 1).max(0) * (y1 - y0 + 1).max(0)) as u32 }
pub fn rect_cell(x0: i32, y0: i32, x1: i32, i: u32) -> (i32, i32) { let w = (x1 - x0 + 1) as u32; (x0 + (i % w) as i32, y0 + (i / w) as i32) }

#[derive(Clone, Debug, Default)]
pub struct Plan {
    pub variant: u8,
    pub opts: u8,
    pub size: u8,
    pub wallk: u8,
    /// ใจกลางลาน (ถนนหลักผ่านแถว/คอลัมน์นี้)
    pub center: (i32, i32),
    pub steps: Vec<Step>,
    /// 0 รออยู่ · 1 ทำแล้ว · 2 ข้าม (ทำไม่ได้)
    pub done: Vec<u8>,
    /// วัสดุที่ใส่ไว้ให้ช่าง (รวมของที่ได้คืนจากการรื้อ)
    pub pool: [i32; NI],
    pub houses: usize,
    /// ปูถนนไปแล้วกี่ช่อง (หิน 1 ก้อนทุก 2 ช่อง)
    pub pave_n: u32,
    pub gates: u8,
    /// v41 ช่องถนนที่ผ่านแนวกำแพง (ประตูเมือง)
    pub gate_cells: Vec<(i32, i32)>,
    pub notes: Vec<String>,
    /// ไม่เซฟ: ช่องที่ใช้วาดแผนที่ (x, y, ชนิด: 0 บ้านเดิม · 1 บ้านใหม่ · 2 เคลียร์ · 3 ลาน/บ่อ/กองไฟ)
    pub overlay: Vec<(i32, i32, u8)>,
}

/// ความคืบหน้าแยกหมวด (ทำแล้ว, ทั้งหมด)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Stats { pub houses: (u32, u32), pub road: (u32, u32), pub wall: (u32, u32), pub plaza: (u32, u32), pub clear: (u32, u32) }

#[derive(Clone, Debug, Default)]
pub struct Arch {
    pub opts: u8,
    /// ผู้เล่นเคยปรับหมวดเองแล้ว (ยังไม่ปรับ = ใช้ค่าเริ่มต้น)
    pub set: bool,
    pub variant: u8,
    /// v37 ขนาดเมือง 0 เล็ก · 1 กลาง · 2 ใหญ่
    pub size: u8,
    /// v37 ชนิดกำแพงรอบเมือง (ดัชนี WALL_OBJ)
    pub wallk: u8,
    /// ข้อเสนอที่ยังไม่รับ (ไม่เซฟ)
    pub prop: Option<Plan>,
    pub active: Option<Plan>,
    pub show: bool,
    pub t: f32,
    pub stall: u32,
    pub finished: i32,
    /// ไม่เซฟ: วัสดุที่ยังขาด (อัปเดตทุกครั้งที่ช่างลงมือ — ใช้วาดป้าย/ที่ปรึกษา)
    pub sf: Vec<(usize, i32)>,
    pub dep_cd: f32,
    /// ไม่เซฟ: ขั้นย้ายบ้านที่เคยพักไว้ที่ชั่วคราวแล้ว (พักได้หลังละครั้ง)
    pub unjammed: Vec<usize>,
    /// v38 ปิดการระดมชาวบ้านช่วยหาวัสดุ (ค่าเริ่มต้น = ช่วย)
    pub no_help: bool,
    /// ไม่เซฟ: แผนจากเวอร์ชันเก่า (v36) — ยกเลิกแล้วคืนของตอนเริ่มเล่น
    pub migrate: bool,
    /// ไม่เซฟ: วัสดุที่ชาวบ้านหามาส่งช่างแล้วในแผนนี้
    pub helped: [i32; NI],
    /// v41 ช่องประตูเมือง (ถนนที่ผ่านแนวกำแพง) จากแผนของช่าง — เซฟ
    pub gates_k: Vec<(i32, i32)>,
    /// ไม่เซฟ: ประตูเมืองทั้งหมด (จากแผน + ช่องถนนที่ผ่านกำแพงเมือง) — มอนสเตอร์ผ่านไม่ได้
    pub gate_set: HashSet<(i32, i32)>,
    pub gate_t: f32,
    /// ไม่เซฟ: เวลาสะสมของการหาวัสดุตอนเราไม่อยู่ในเมือง
    pub away_t: f32,
    /// v42 ใจกลางลานเมืองที่ช่างวางไว้ (เซฟ) — ที่นั่งคุยตอนเย็น ตลาดนัด เทศกาล
    pub plaza: Option<(i32, i32)>,
    /// ไม่เซฟ: ตลาดนัดวันนี้ (วัน, คนที่มา, ซื้ออาหารกี่มื้อ, ประกาศแล้ว)
    pub mkt_day: i32,
    pub mkt_seen: HashSet<u32>,
    pub mkt_sold: i32,
    pub mkt_told: bool,
    /// ไม่เซฟ: ป้ายชื่อหน้าบ้าน (ประตู, ชื่อ)
    pub plates: Vec<((i32, i32), String)>,
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

/// ผังเมืองตามขนาด: ถนนขนาน (เลขบล็อก) · ถนนตัด (ออฟเซ็ตตามแนวถนน) · ครึ่งความยาวถนน
pub struct Shape { pub streets: &'static [i32], pub avenues: &'static [i32], pub half: i32 }
pub fn shape(size: u8) -> Shape {
    match size {
        0 => Shape { streets: &[0], avenues: &[0], half: 26 },
        1 => Shape { streets: &[-1, 0, 1], avenues: &[0], half: 32 },
        _ => Shape { streets: &[-2, -1, 0, 1, 2], avenues: &[-22, 0, 22], half: 38 },
    }
}
/// กรอบกำแพง (u0, u1, v0, v1) ในพิกัดสัมพัทธ์ (u = ตามแนวถนน, v = ตั้งฉาก)
pub fn wall_box(size: u8) -> (i32, i32, i32, i32) {
    let s = shape(size);
    let (smin, smax) = (*s.streets.first().unwrap(), *s.streets.last().unwrap());
    (-(s.half + 2), s.half + 3, smin * BLOCK - 10, smax * BLOCK + 11)
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Cs { Free, Clear(u8), Blocked }

/// สถานะช่องของถนน/กำแพง ตอนลงมือ
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cst { Done, Skip, Clear(u8), Todo }

/// ของธรรมชาติที่โค่น/ถางทิ้งได้ตอนสร้างทับ (v40: ทุกต้น ทุกพุ่ม หิน แร่ กระบองเพชร ต้นอ้อ เห็ด — คืนวัสดุ)
pub fn nat_clearable(o: u8) -> bool { is_tree(o) || matches!(o, 2 | 4 | 5 | 7 | O_REED | O_SHROOM) }
fn nat_trivial(o: u8) -> bool { o == 0 || o == 3 || o == O_DRYGRASS }

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
        let mut t = g.w.tile_at(k.0, k.1);
        // พื้นบ้านที่จะถูกย้ายออก: ใต้พื้นเป็นอะไร (ย้ายออกแล้วพื้นเดิมโผล่กลับมา)
        if (t == T_FLOORW || t == T_FLOORS) && self.gone.contains(&k) { t = g.w.gen_tile(k.0, k.1); }
        if SOLIDT[t as usize] || t == T_SOIL || t == T_WET { return Cs::Blocked; }
        if g.crops.contains_key(&k) || self.fields.contains(&k) || g.site_reserved(k) { return Cs::Blocked; }
        if g.w.realm.city_at(k.0, k.1).map_or(false, |ci| g.city_owner(ci) != PLAYER_OWN) { return Cs::Blocked; }
        if self.gone.contains(&k) { return Cs::Free; }
        if g.w.placed.contains_key(&k) { return Cs::Blocked; }
        if g.w.tmod.get(&k).map_or(false, |&x| x == T_FLOORW || x == T_FLOORS) { return Cs::Blocked; }
        match g.w.obj_at(k.0, k.1) {
            o if nat_trivial(o) => Cs::Free,
            o if nat_clearable(o) => Cs::Clear(o),
            _ => Cs::Blocked,
        }
    }
}

pub fn place_cost(obj: u8) -> Vec<(usize, i32)> {
    if obj == 9 { return vec![(WOOD, 3), (STONE, 2)]; }
    if obj == O_STALL { return vec![(WOOD, 6)]; }
    PIECES.iter().find(|p| p.obj == obj).map(|p| p.cost.to_vec()).unwrap_or_default()
}
/// v42 กำแพงรอบเมืองที่ช่างสร้างเป็นแนวยาว (ซื้อวัสดุเหมา ถูกกว่าวางเอง): รั้วไม้ 1 ไม้ · กำแพงหิน 2 หิน · กำแพงเมือง 4 หิน
pub fn wall_cost(obj: u8) -> Vec<(usize, i32)> { match obj { 10 => vec![(WOOD, 1)], 11 => vec![(STONE, 2)], 38 => vec![(STONE, 4)], o => place_cost(o) } }
/// รื้อ/โค่นแล้วได้วัสดุคืน
pub fn clear_refund(o: u8) -> Option<(usize, i32)> {
    match o { 10 => Some((WOOD, 1)), 11 => Some((STONE, 2)), 4 => Some((STONE, 2)), 5 => Some((GEM, 1)), 2 => Some((BERRY, 1)), O_SHROOM => Some((SHROOM, 1)), T_SAPLING => None, o if is_tree(o) => Some((WOOD, 3)), _ => None }
}
fn nb4() -> [(i32, i32); 4] { [(0, -1), (0, 1), (-1, 0), (1, 0)] }
fn face_of(v: (i32, i32)) -> u8 { match v { (0, 1) => 0, (-1, 0) => 1, (0, -1) => 2, _ => 3 } }
fn cheb1(c: (i32, i32)) -> impl Iterator<Item = (i32, i32)> { (-1..=1).flat_map(move |dy| (-1..=1).map(move |dx| (c.0 + dx, c.1 + dy))) }

impl Plan {
    /// วัสดุที่จะได้คืนจากการรื้อที่ยังไม่ทำ
    pub fn refund_pending(&self) -> [i32; NI] {
        let mut n = [0; NI];
        for (i, s) in self.steps.iter().enumerate() {
            if self.done[i] != 0 { continue; }
            if let Step::Clear { obj, .. } = s { if let Some((it, q)) = clear_refund(*obj) { n[it] += q; } }
        }
        n
    }
    pub fn progress(&self) -> (usize, usize) { (self.done.iter().filter(|&&d| d != 0).count(), self.steps.len()) }
    pub fn count_houses(&self) -> usize { self.steps.iter().filter(|s| matches!(s, Step::MoveHouse { .. })).count() }
    pub fn count_clear(&self) -> (usize, usize) {
        let (mut nat, mut wall) = (0, 0);
        for s in &self.steps { if let Step::Clear { obj, .. } = s { if *obj == 10 || *obj == 11 { wall += 1 } else { nat += 1 } } }
        (nat, wall)
    }
    pub fn finished(&self) -> bool { self.done.iter().all(|&d| d != 0) }
    pub fn stats(&self) -> Stats {
        let mut s = Stats::default();
        let add = |t: &mut (u32, u32), d: u32, n: u32| { t.0 += d; t.1 += n; };
        for (i, st) in self.steps.iter().enumerate() {
            let fin = self.done[i] != 0;
            match st {
                Step::Clear { .. } => add(&mut s.clear, fin as u32, 1),
                Step::MoveHouse { .. } => add(&mut s.houses, fin as u32, 1),
                Step::MoveOne { .. } | Step::Place { .. } => add(&mut s.plaza, fin as u32, 1),
                Step::Pave { x0, y0, x1, y1, at } => { let n = rect_len(*x0, *y0, *x1, *y1); add(&mut s.road, if fin { n } else { (*at).min(n) }, n) }
                Step::Wall { x0, y0, x1, y1, at, .. } => { let n = rect_len(*x0, *y0, *x1, *y1); add(&mut s.wall, if fin { n } else { (*at).min(n) }, n) }
            }
        }
        s
    }
    /// 0..1 รวมทุกหมวด (ถ่วงตามปริมาณงาน)
    pub fn frac(&self) -> f32 {
        let s = self.stats();
        let d = s.houses.0 * 8 + s.plaza.0 * 2 + s.clear.0 + s.road.0 / 2 + s.wall.0;
        let n = s.houses.1 * 8 + s.plaza.1 * 2 + s.clear.1 + s.road.1 / 2 + s.wall.1;
        if n == 0 { 1. } else { d as f32 / n as f32 }
    }
}

impl Game {
    pub fn arch_opts(&self) -> u8 { if self.rs.ar.set { self.rs.ar.opts } else { OPT_DEFAULT } }
    pub fn arch_toggle(&mut self, bit: u8) {
        let o = self.arch_opts() ^ bit;
        self.rs.ar.opts = o;
        self.rs.ar.set = true;
    }
    /// v37 ตั้งขนาดเมือง / ชนิดกำแพง (None = ไม่มีกำแพงรอบเมือง)
    pub fn arch_set_size(&mut self, s: u8) { self.rs.ar.size = s.min(2); self.rs.ar.prop = None; }
    pub fn arch_set_wall(&mut self, k: Option<u8>) {
        let o = self.arch_opts();
        self.rs.ar.opts = match k { Some(_) => o | OPT_CITYWALL, None => o & !OPT_CITYWALL };
        if let Some(k) = k { self.rs.ar.wallk = k.min(2); }
        self.rs.ar.set = true;
        self.rs.ar.prop = None;
    }
    /// ค่าจ้าง
    pub fn arch_fee(&self, plan: &Plan) -> i32 { ARCH_FEE + ARCH_FEE_HOUSE * plan.count_houses() as i32 + ARCH_FEE_SIZE * plan.size as i32 }

    // ---------------------------------------------------------------- สำรวจ
    pub fn arch_survey(&self) -> Option<Survey> {
        let c = self.town_center()?;
        let near = |k: &(i32, i32)| (k.0 - c.0).abs() <= SURVEY_R && (k.1 - c.1).abs() <= SURVEY_R;
        // ห้องปิดทั้งหมดรอบเมือง (เพื่อรู้ว่ากำแพง/กองไฟตรงไหนอยู่ในบ้านใคร)
        let mut outside: HashSet<(i32, i32)> = HashSet::new();
        let mut inside: HashSet<(i32, i32)> = HashSet::new();
        let mut room_walls: HashSet<(i32, i32)> = HashSet::new();
        let mut walls_all: Vec<(i32, i32)> = self.w.placed.iter().filter(|(k, &o)| near(k) && is_wall_obj(o) && o != 38).map(|(k, _)| *k).collect();
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
        // v42 อาคารที่ไม่มีเตียง (โกดัง ห้องเก็บของ ครัว) ก็นับเป็นอาคาร — ย้ายเข้าแนวได้ ไม่ค้างกลางถนน
        let mut doors_all: Vec<(i32, i32)> = self.w.placed.iter().filter(|(k, &o)| near(k) && (o == 12 || o == 13)).map(|(k, _)| *k).collect();
        doors_all.sort();
        for door in doors_all {
            if seen.contains(&door) || sv.houses.iter().any(|h| h.carry.tiles.contains(&door)) { continue; }
            let Some((room, inn)) = nb4().iter().map(|d| (door.0 + d.0, door.1 + d.1)).find_map(|n| self.w.room_from(n).map(|r| (r, n))) else { continue };
            if room.tent || room.has_bed { continue; }
            let Ok(carry) = self.move_select(door) else { continue };
            if !carry.building || sv.houses.iter().any(|h| h.carry.tiles.iter().any(|t| carry.tiles.contains(t))) { continue; }
            for d in &room.doors { seen.insert(*d); }
            let face = face_of((door.0 - inn.0, door.1 - inn.1));
            sv.houses.push(Hs { door, face, carry, mine: true, owner: None });
        }
        sv.houses.sort_by_key(|h| h.door);
        let house_tiles: HashSet<(i32, i32)> = sv.houses.iter().flat_map(|h| h.carry.tiles.iter().copied()).collect();
        // กำแพงสะเปะสะปะ: ไม่ได้เป็นส่วนของห้องปิดใดเลย (กำแพงเมือง 38 ไม่นับ — ตั้งใจสร้างเป็นแนวป้องกัน)
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
    /// ข้อเสนอของช่างตามแบบ (variant) ขนาดเมือง และหมวดที่เลือก (None = ยังไม่มีบ้านในเมือง/ไม่มีอะไรให้ทำ)
    pub fn arch_plan(&self, variant: u8, opts: u8) -> Option<Plan> { self.arch_plan_ex(variant, opts, self.rs.ar.size, self.rs.ar.wallk) }
    pub fn arch_plan_ex(&self, variant: u8, opts: u8, size: u8, wallk: u8) -> Option<Plan> {
        let sv = self.arch_survey()?;
        if sv.houses.is_empty() { return None; }
        let size = size.min(2);
        let wobj = WALL_OBJ[wallk.min(2) as usize];
        let sh = shape(size);
        let (bu0, bu1, bv0, bv1) = wall_box(size);
        let ew = variant == 0;
        let (pl, ph) = (plaza_lo(size), plaza_hi(size));
        let mut lay = Lay::new(self);
        let sel: Vec<&Hs> = sv.houses.iter().filter(|h| if h.mine { opts & OPT_MINE != 0 } else { opts & OPT_NPC != 0 }).collect();
        for h in &sel { lay.gone.extend(h.carry.tiles.iter().copied()); }
        if opts & OPT_WALLS != 0 { lay.gone.extend(sv.walls.iter().copied()); }

        // ---- ใจกลางลาน: ใกล้ใจกลางเมือง ที่ลาน 12×12 โดนของขวางน้อยที่สุด
        let mut best: Option<(i32, (i32, i32))> = None;
        for oy in -16..=16 { for ox in -16..=16 {
            let pc = (sv.c.0 + ox, sv.c.1 + oy);
            let mut bad = 0;
            for dv in pl - 1..=ph + 1 { for du in pl - 1..=ph + 1 {
                if lay.cs((pc.0 + du, pc.1 + dv)) == Cs::Blocked { bad += 1; }
            } }
            let sc = bad * 60 + ox.abs() + oy.abs();
            if best.map_or(true, |b| sc < b.0) { best = Some((sc, pc)); }
        } }
        let pc = best.map(|b| b.1).unwrap_or(sv.c);
        let at = |u: i32, v: i32| if ew { (pc.0 + u, pc.1 + v) } else { (pc.0 + v, pc.1 + u) };

        let mut plan = Plan { variant, opts, size, wallk: wallk.min(2), center: pc, ..Default::default() };
        let mut clears: Vec<Step> = vec![];
        let mut cleared: HashSet<(i32, i32)> = HashSet::new();
        let mut rsv: HashSet<(i32, i32)> = HashSet::new();
        let mut fp_all: HashSet<(i32, i32)> = HashSet::new();
        let mut overlay: Vec<(i32, i32, u8)> = vec![];
        let mut plaza_steps: Vec<Step> = vec![];

        // ---- ถนน (จองก่อน บ้านจะได้ไม่ลงทับ)
        let road_on = opts & OPT_ROAD != 0;
        let wall_on = opts & OPT_CITYWALL != 0;
        let ext = if wall_on { 2 } else { 0 };
        let mut road_rects: Vec<(i32, i32, i32, i32)> = vec![]; // (u0, v0, u1, v1)
        road_rects.push((pl, pl, ph, ph));
        for &s in sh.streets { road_rects.push((bu0 - ext, s * BLOCK, bu1 + ext, s * BLOCK + 1)); }
        for &a in sh.avenues { road_rects.push((a, bv0 - ext, a + 1, bv1 + ext)); }
        let mut road_cells: HashSet<(i32, i32)> = HashSet::new();
        for &(u0, v0, u1, v1) in &road_rects { for v in v0..=v1 { for u in u0..=u1 { road_cells.insert(at(u, v)); } } }
        rsv.extend(road_cells.iter().copied());
        for dv in pl - 1..=ph + 1 { for du in pl - 1..=ph + 1 { rsv.insert(at(du, dv)); } }
        let inside_box = |k: (i32, i32)| -> bool {
            let (u, v) = if ew { (k.0 - pc.0, k.1 - pc.1) } else { (k.1 - pc.1, k.0 - pc.0) };
            u >= bu0 + 2 && u <= bu1 - 2 && v >= bv0 + 2 && v <= bv1 - 2
        };

        // ---- ลานกลางเมือง: วงกองไฟกลางลาน · บ่อน้ำ 4 มุม · คบเพลิงขอบลาน
        if opts & OPT_PLAZA != 0 {
            let ring_off = [(0, 0), (-2, 0), (2, 0), (0, -2), (0, 2)];
            let spots: Vec<(i32, i32)> = ring_off.iter().map(|r| at(r.0, r.1)).collect();
            if sv.ring.is_empty() {
                plaza_steps.push(Step::Place { obj: 9, at: spots[0] });
                for &k in &spots[1..] { plaza_steps.push(Step::Place { obj: O_LOG, at: k }); }
            } else {
                let mut free: Vec<(i32, i32)> = spots[1..].to_vec();
                let mut moves: Vec<((i32, i32), (i32, i32))> = vec![(sv.ring[0], spots[0])];
                for &l in &sv.ring[1..] {
                    let (i, _) = free.iter().enumerate().min_by_key(|(_, s)| (s.0 - l.0).abs() + (s.1 - l.1).abs()).unwrap();
                    moves.push((l, free.remove(i)));
                }
                for (f, t) in moves { if f != t { plaza_steps.push(Step::MoveOne { from: f, to: t }); lay.gone.insert(f); } }
                for k in free { plaza_steps.push(Step::Place { obj: O_LOG, at: k }); }
            }
            let corners = [(pl + 1, pl + 1), (ph - 1, pl + 1), (pl + 1, ph - 1), (ph - 1, ph - 1)].map(|o| at(o.0, o.1));
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
            for o in [(pl, pl), (ph, pl), (pl, ph), (ph, ph)] { plaza_steps.push(Step::Place { obj: 18, at: at(o.0, o.1) }); }
            // v42 แผงตลาดนัด 2 แผง ขนาบถนนตัดตรงขอบลาน (ตอนเย็นชาวบ้านมาแลกของกัน)
            for u in [-1, 2] { plaza_steps.push(Step::Place { obj: O_STALL, at: at(u, ph) }); }
            for s in &plaza_steps {
                let k = match s { Step::MoveOne { to, .. } => *to, Step::Place { at, .. } => *at, _ => continue };
                if let Cs::Clear(o) = lay.cs(k) { if cleared.insert(k) { clears.push(Step::Clear { at: k, obj: o }); } }
                overlay.push((k.0, k.1, 3));
            }
        }
        if opts & OPT_WALLS != 0 { for &w in &sv.walls { clears.push(Step::Clear { at: w, obj: *self.w.placed.get(&w).unwrap_or(&10) }); cleared.insert(w); overlay.push((w.0, w.1, 2)); } }

        // ---- คบเพลิงหน้าประตูเมือง (จองไว้ก่อนวางบ้าน)
        let mut gate_torches: Vec<(i32, i32)> = vec![];
        let mut gates = 0u8;
        if wall_on {
            for &s in sh.streets { for u in [bu0 + 1, bu1 - 1] { gates += 1; for v in [s * BLOCK - 1, s * BLOCK + 2] { gate_torches.push(at(u, v)); } } }
            for &a in sh.avenues { for v in [bv0 + 1, bv1 - 1] { gates += 1; for u in [a - 1, a + 2] { gate_torches.push(at(u, v)); } } }
            gate_torches.retain(|k| !road_cells.contains(k));
            rsv.extend(gate_torches.iter().copied());
        }

        // ---- บ้านเรียงสองฝั่งถนน ไล่จากลานออกไป (หลังที่อยู่ใกล้ลานเดิมได้ล็อตใกล้ลาน)
        let mut order: Vec<&Hs> = sel.clone();
        order.sort_by_key(|h| ((h.door.0 - pc.0).abs() + (h.door.1 - pc.1).abs(), h.door));
        let mut moves: Vec<(Step, Vec<(i32, i32)>, Vec<(i32, i32)>)> = vec![]; // (step, ช่องใหม่, ช่องเดิม)
        let mut unplaced = 0;
        for h in order {
            let mut best: Option<(i32, (i32, i32), u8, Vec<(i32, i32)>, Vec<(i32, i32)>)> = None;
            for &s in sh.streets {
                for side in 0..2u8 {
                    for dir in [1i32, -1] {
                        let mut u = if dir > 0 { 0 } else { -1 };
                        while u >= bu0 && u <= bu1 {
                            let (door, face) = if ew {
                                if side == 0 { (at(u, s * BLOCK - 1), 0u8) } else { (at(u, s * BLOCK + 2), 2u8) }
                            } else if side == 0 { (at(u, s * BLOCK - 1), 3u8) } else { (at(u, s * BLOCK + 2), 1u8) };
                            if let Some((fp, cl)) = Self::try_lot(&mut lay, &rsv, &fp_all, h, door, face, &inside_box) {
                                // ใกล้ลานก่อน (เมืองกระชับ) · เสมอกัน = ใกล้ที่เดิม
                                let d0 = (door.0 - h.door.0).abs() + (door.1 - h.door.1).abs();
                                let cost = (u.abs() + s.abs() * 14) * 16 + d0;
                                if best.as_ref().map_or(true, |b| cost < b.0) { best = Some((cost, door, face, fp, cl)); }
                                break;
                            }
                            u += dir;
                        }
                    }
                }
            }
            let Some((_, door, face, fp, cl)) = best else { unplaced += 1; continue };
            let rot = (face + 4 - h.face) % 4;
            for &c in &fp { for n in cheb1(c) { rsv.insert(n); } fp_all.insert(c); overlay.push((c.0, c.1, 1)); }
            for &t in &h.carry.tiles { overlay.push((t.0, t.1, 0)); }
            for at in cl { if let Cs::Clear(obj) = lay.cs(at) { if cleared.insert(at) { clears.push(Step::Clear { at, obj }); overlay.push((at.0, at.1, 2)); } } }
            moves.push((Step::MoveHouse { door: h.door, to: door, rot }, fp, h.carry.tiles.clone()));
        }
        if unplaced > 0 { plan.notes.push(format!("ล็อตริมถนนไม่พอ/มีของขวาง {} หลัง — ปล่อยไว้ที่เดิม (ลองเมืองขนาดใหญ่ขึ้น หรืออีกแนวถนน)", unplaced)); }
        // เรียงลำดับย้ายบ้าน: หลังที่ปลายทางไม่ทับบ้านที่ยังไม่ย้าย ไปก่อน
        let mut ordered: Vec<(Step, Vec<(i32, i32)>, Vec<(i32, i32)>)> = vec![];
        while !moves.is_empty() {
            let olds: HashSet<(i32, i32)> = moves.iter().flat_map(|m| m.2.iter().copied()).collect();
            let i = moves.iter().position(|m| m.1.iter().all(|c| !olds.contains(c) || m.2.contains(c))).unwrap_or(0);
            ordered.push(moves.remove(i));
        }
        plan.houses = ordered.len();

        // ---- ถนน: ลานก่อน แล้วถนนหลัก ถนนตัด
        let mut roads: Vec<Step> = vec![];
        if road_on {
            for &(u0, v0, u1, v1) in &road_rects {
                let (a, b) = (at(u0, v0), at(u1, v1));
                roads.push(Step::Pave { x0: a.0.min(b.0), y0: a.1.min(b.1), x1: a.0.max(b.0), y1: a.1.max(b.1), at: 0 });
            }
        }
        // ---- กำแพงรอบเมือง: 4 ด้าน เว้นช่องตรงถนน = ประตูเมือง
        let mut walls: Vec<Step> = vec![];
        if wall_on {
            let mut sides: Vec<Vec<(i32, i32)>> = vec![];
            sides.push((bu0..=bu1).map(|u| (u, bv0)).collect());
            sides.push((bu0..=bu1).map(|u| (u, bv1)).collect());
            sides.push((bv0 + 1..bv1).map(|v| (bu0, v)).collect());
            sides.push((bv0 + 1..bv1).map(|v| (bu1, v)).collect());
            for side in sides {
                let mut run: Vec<(i32, i32)> = vec![];
                let flush = |run: &mut Vec<(i32, i32)>, walls: &mut Vec<Step>| {
                    if run.is_empty() { return; }
                    let (a, b) = (run[0], *run.last().unwrap());
                    walls.push(Step::Wall { x0: a.0.min(b.0), y0: a.1.min(b.1), x1: a.0.max(b.0), y1: a.1.max(b.1), obj: wobj, at: 0 });
                    run.clear();
                };
                for (u, v) in side {
                    let k = at(u, v);
                    if road_cells.contains(&k) { flush(&mut run, &mut walls); plan.gate_cells.push(k); } else { run.push(k); }
                }
                flush(&mut run, &mut walls);
            }
            for k in &gate_torches { plaza_steps.push(Step::Place { obj: 18, at: *k }); }
            plan.gates = gates;
        }

        plan.steps = clears.into_iter().chain(ordered.into_iter().map(|m| m.0)).chain(plaza_steps).chain(roads).chain(walls).collect();
        plan.done = vec![0; plan.steps.len()];
        plan.overlay = overlay;
        if plan.steps.is_empty() { return None; }
        Some(plan)
    }

    /// ล็อตนี้ลงบ้านหลังนี้ได้ไหม: (ช่องตัวบ้านใหม่, ช่องที่ต้องเคลียร์) — None = ไม่ได้
    fn try_lot(lay: &mut Lay, rsv: &HashSet<(i32, i32)>, fp_all: &HashSet<(i32, i32)>, h: &Hs, door: (i32, i32), face: u8, inside: &dyn Fn((i32, i32)) -> bool) -> Option<(Vec<(i32, i32)>, Vec<(i32, i32)>)> {
        let k = (face + 4 - h.face) % 4;
        let mut fp: Vec<(i32, i32)> = vec![];
        let mut cl: Vec<(i32, i32)> = vec![];
        let has_floor = h.carry.tiles.iter().any(|t| lay.g.w.tmod.get(t).map_or(false, |&x| x == T_FLOORW || x == T_FLOORS));
        for t in &h.carry.tiles {
            let o = rot_off((t.0 - h.door.0, t.1 - h.door.1), k);
            let c = (door.0 + o.0, door.1 + o.1);
            if rsv.contains(&c) || !inside(c) { return None; }
            // ช่องที่ถูกดัดแปลงพื้น (ถมน้ำ/ปูถนน) ปูพื้นบ้านทับไม่ได้ (กติกาเดียวกับตอนย้ายของ)
            if has_floor && lay.g.w.tmod.get(&c).map_or(false, |&x| x != T_FLOORW && x != T_FLOORS) && !h.carry.tiles.contains(&c) { return None; }
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

    // ---------------------------------------------------------------- สถานะช่องถนน/กำแพง
    fn arch_fields(&self) -> HashSet<(i32, i32)> { self.rs.pm.fields.iter().flat_map(|f| f.cells()).collect() }
    fn cell_hard(&self, k: (i32, i32), fields: &HashSet<(i32, i32)>) -> bool {
        let t = self.w.tile_at(k.0, k.1);
        SOLIDT[t as usize] || t == T_SOIL || t == T_WET || t == T_FLOORW || t == T_FLOORS
            || self.crops.contains_key(&k) || fields.contains(&k) || self.site_reserved(k)
            || self.w.realm.city_at(k.0, k.1).map_or(false, |ci| self.city_owner(ci) != PLAYER_OWN)
    }
    pub fn cell_road(&self, k: (i32, i32), fields: &HashSet<(i32, i32)>) -> Cst {
        if self.w.tile_at(k.0, k.1) == T_ROAD { return Cst::Done; }
        if self.cell_hard(k, fields) { return Cst::Skip; }
        // ของที่วางไว้ (กองไฟ ท่อนไม้ บ่อ คบเพลิง) ปูถนนใต้ของได้ ลานจะได้ไม่เป็นรูๆ · ประตูไม่ปู (ไม่ให้พื้นหน้าบ้านเพี้ยน)
        if let Some(&o) = self.w.placed.get(&k) { return if matches!(o, 12 | 13) { Cst::Skip } else { Cst::Todo }; }
        match self.w.obj_at(k.0, k.1) { o if nat_trivial(o) => Cst::Todo, o if nat_clearable(o) => Cst::Clear(o), _ => Cst::Skip }
    }
    pub fn cell_wall(&self, k: (i32, i32), obj: u8, fields: &HashSet<(i32, i32)>) -> Cst {
        if self.w.placed.get(&k) == Some(&obj) { return Cst::Done; }
        // ถนนเดิมที่ตัดผ่านแนวกำแพง = ประตูเมืองโดยธรรมชาติ
        if self.w.tile_at(k.0, k.1) == T_ROAD || self.cell_hard(k, fields) || self.w.placed.contains_key(&k) { return Cst::Skip; }
        match self.w.obj_at(k.0, k.1) { o if nat_trivial(o) => Cst::Todo, o if nat_clearable(o) => Cst::Clear(o), _ => Cst::Skip }
    }
    /// วัสดุที่ยังต้องใช้ (นับจากของในโลกจริง: ช่องที่ทำแล้ว/ทำไม่ได้ ไม่นับ)
    pub fn arch_need(&self, p: &Plan) -> [i32; NI] {
        let mut n = [0; NI];
        let fields = self.arch_fields();
        let mut road: HashSet<(i32, i32)> = HashSet::new();
        for (i, s) in p.steps.iter().enumerate() {
            if p.done[i] != 0 { continue; }
            match s {
                Step::Place { obj, at } => if self.w.placed.get(at) != Some(obj) { for (it, q) in place_cost(*obj) { n[it] += q; } },
                Step::Pave { x0, y0, x1, y1, at } => for j in *at..rect_len(*x0, *y0, *x1, *y1) {
                    let k = rect_cell(*x0, *y0, *x1, j);
                    if matches!(self.cell_road(k, &fields), Cst::Todo | Cst::Clear(_)) { road.insert(k); }
                },
                Step::Wall { x0, y0, x1, y1, obj, at } => {
                    let c = wall_cost(*obj);
                    for j in *at..rect_len(*x0, *y0, *x1, *y1) {
                        if matches!(self.cell_wall(rect_cell(*x0, *y0, *x1, j), *obj, &fields), Cst::Todo | Cst::Clear(_)) { for &(it, q) in &c { n[it] += q; } }
                    }
                }
                _ => {}
            }
        }
        let rn = road.len() as i32 + (p.pave_n % 2) as i32;
        n[STONE] += rn / 2 + rn % 2 - (p.pave_n % 2) as i32;
        n
    }
    /// ขาดอะไรอีกเท่าไร (หักของในกองกลาง + ที่จะได้คืนจากการรื้อแล้ว)
    pub fn arch_shortfall(&self, p: &Plan) -> Vec<(usize, i32)> {
        let (need, back) = (self.arch_need(p), p.refund_pending());
        (0..NI).filter_map(|i| { let s = need[i] - p.pool[i] - back[i]; if s > 0 { Some((i, s)) } else { None } }).collect()
    }
    fn arch_refresh_sf(&mut self) {
        self.rs.ar.sf = match &self.rs.ar.active { Some(p) => self.arch_shortfall(p), None => vec![] };
    }

    // ---------------------------------------------------------------- ข้อเสนอ → รับ/ปฏิเสธ
    pub fn arch_hire(&mut self, variant: u8) -> bool {
        if self.w.dun.is_some() { self.toast("จ้างช่างในดันเจี้ยนไม่ได้"); return false; }
        if self.rs.ar.active.is_some() { self.toast("ช่างกำลังทำตามแผนเดิมอยู่ — ยกเลิกแผนเดิมก่อนถ้าจะวางผังใหม่"); return false; }
        let opts = self.arch_opts();
        match self.arch_plan(variant, opts) {
            Some(p) => {
                self.rs.ar.variant = variant;
                let st = p.stats();
                self.toast(format!("สถาปนิกเสนอผังเมือง{} \"{}\": ย้ายบ้าน {} หลัง · ถนน {} ช่อง · กำแพง {} ช่อง", SIZE_NAMES[p.size as usize], VARIANT_NAMES[variant as usize & 1], st.houses.1, st.road.1, st.wall.1));
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
    pub fn arch_reject(&mut self) { if self.rs.ar.prop.take().is_some() { self.rs.ar.show = false; self.toast("ปฏิเสธแบบนี้ — ไม่เสียค่าจ้าง"); } }
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
        self.rs.ar.unjammed.clear();
        self.rs.ar.helped = [0; NI];
        for k in &p.gate_cells { if !self.rs.ar.gates_k.contains(k) { self.rs.ar.gates_k.push(*k); } }
        if p.opts & OPT_PLAZA != 0 { self.rs.ar.plaza = Some(p.center); }
        self.rs.ar.gate_t = 0.;
        self.rs.ar.active = Some(p);
        self.arch_refresh_sf();
        self.news_add(format!("จ้างสถาปนิกวางผังเมือง (ค่าจ้าง {} เหรียญ)", fee));
        self.toast(if self.rs.ar.sf.is_empty() { "ช่างเริ่มงานแล้ว!".to_string() } else { "ช่างเริ่มงาน — ส่วนที่ของยังไม่พอเป็นแบบร่างจางๆ บนพื้นไว้ก่อน เอาของมาที่ลานกลางเมืองแล้วช่างสร้างต่อเอง".to_string() });
        play(&self.sfx.click);
        true
    }
    /// ใส่วัสดุที่ช่างยังขาด จากกระเป๋าเรา (คืนรายการที่ใส่)
    pub fn arch_deposit(&mut self) -> bool {
        let Some(p) = self.rs.ar.active.as_ref() else { return false };
        let sf = self.arch_shortfall(p);
        let mut got: Vec<(usize, i32)> = vec![];
        for (it, q) in sf {
            let give = q.min(self.p.inv[it]);
            if give > 0 { self.p.inv[it] -= give; got.push((it, give)); }
        }
        if let Some(p) = self.rs.ar.active.as_mut() { for &(it, q) in &got { p.pool[it] += q; } }
        self.arch_refresh_sf();
        if got.is_empty() { self.toast("ในกระเป๋าไม่มีของที่ช่างขาด"); return false; }
        self.toast(format!("ส่งวัสดุให้ช่าง: {}", got.iter().map(|&(i, q)| format!("{} {}", INAME[i], q)).collect::<Vec<_>>().join(" · ")));
        play(&self.sfx.click);
        true
    }
    /// v37 ขนวัสดุที่ช่างขาดจากกล่องเก็บของในเมือง (ไม้/หินที่ชาวบ้านตัด-ขุดมาเก็บไว้)
    pub fn arch_deposit_chests(&mut self) -> bool {
        let Some(p) = self.rs.ar.active.as_ref() else { return false };
        let c = p.center;
        let mut sf = self.arch_shortfall(p);
        let mut keys: Vec<(i32, i32)> = self.chests.keys().copied().filter(|k| (k.0 - c.0).abs() <= SURVEY_R && (k.1 - c.1).abs() <= SURVEY_R).collect();
        keys.sort_by_key(|k| ((k.0 - c.0).abs() + (k.1 - c.1).abs(), *k));
        let mut got = [0; NI];
        for k in keys {
            let Some(inv) = self.chests.get_mut(&k) else { continue };
            for (it, q) in sf.iter_mut() {
                let t = (*q).min(inv[*it]);
                if t > 0 { inv[*it] -= t; *q -= t; got[*it] += t; }
            }
        }
        if let Some(p) = self.rs.ar.active.as_mut() { for i in 0..NI { p.pool[i] += got[i]; } }
        self.arch_refresh_sf();
        let l: Vec<String> = (0..NI).filter(|&i| got[i] > 0).map(|i| format!("{} {}", INAME[i], got[i])).collect();
        if l.is_empty() { self.toast("กล่องในเมืองไม่มีของที่ช่างขาด"); return false; }
        self.toast(format!("ขนจากกล่องในเมืองให้ช่าง: {}", l.join(" · ")));
        play(&self.sfx.click);
        true
    }
    /// ยกเลิกแผนที่กำลังทำ: ของที่ทำไปแล้วอยู่อย่างนั้น วัสดุที่เหลือคืนกระเป๋า
    pub fn arch_cancel(&mut self) {
        if let Some(p) = self.rs.ar.active.take() {
            for i in 0..NI { self.p.inv[i] += p.pool[i]; }
            self.rs.ar.sf.clear();
            self.toast(format!("ยกเลิกแผนวางผัง (ทำไปแล้ว {}%) — คืนวัสดุที่เหลือให้แล้ว", (p.frac() * 100.) as i32));
        }
    }

    // ---------------------------------------------------------------- ช่างลงมือ
    fn arch_clear_nat(&mut self, k: (i32, i32), o: u8) {
        if let Some((it, q)) = clear_refund(o) { if let Some(p) = self.rs.ar.active.as_mut() { p.pool[it] += q; } }
        self.w.removed.insert(k);
        self.obj_dmg.remove(&k);
        self.burst(k.0 as f32 * 16. + 8., k.1 as f32 * 16. + 8., hx(0xc8a070), 5, 30.);
    }
    fn someone_on(&self, k: (i32, i32)) -> bool {
        self.player_in_tile(k.0, k.1) || self.npcs.iter().any(|n| tfloor(n.x) == k.0 && tfloor(n.y - 3.) == k.1)
    }
    /// ยังมีงานย้าย/รื้อค้างอยู่ไหม (ของใหม่/ถนน/กำแพงรอให้ย้ายเสร็จก่อน ไม่งั้นช่องที่บ้านเดิมตั้งอยู่จะกลายเป็นรู)
    fn arch_moves_pending(&self) -> bool {
        self.rs.ar.active.as_ref().map_or(false, |p| p.steps.iter().zip(&p.done).any(|(s, &d)| d == 0 && matches!(s, Step::Clear { .. } | Step::MoveHouse { .. } | Step::MoveOne { .. })))
    }
    /// ย้ายบ้านติดกันเอง (บ้าน A จะไปลงที่ B ตั้งอยู่ และ B จะไปที่ A): พักบ้านหลังหนึ่งไว้ที่ว่างชั่วคราวก่อน แล้วค่อยย้ายเข้าที่จริง
    fn arch_unjam(&mut self) -> bool {
        let Some(p) = self.rs.ar.active.clone() else { return false };
        let mut taken: HashSet<(i32, i32)> = HashSet::new();
        let mut cands: Vec<(usize, Carry, (i32, i32))> = vec![];
        for (i, s) in p.steps.iter().enumerate() {
            if p.done[i] != 0 { continue; }
            if let Step::MoveHouse { door, to, rot } = s {
                if let Ok(c) = self.move_select(*door) {
                    for (_, d) in self.move_targets(&c, *to, *rot) { taken.insert(d); }
                    cands.push((i, c, *door));
                }
            }
        }
        for (i, c, door) in cands {
            if self.rs.ar.unjammed.contains(&i) { continue; }
            self.rs.ar.unjammed.push(i);
            for r in 3..=40i32 {
                for k in 0..8 * r {
                    let (dx, dy) = match k / (2 * r) { 0 => (-r + k % (2 * r), -r), 1 => (r, -r + k % (2 * r)), 2 => (r - k % (2 * r), r), _ => (-r, r - k % (2 * r)) };
                    let a = (door.0 + dx, door.1 + dy);
                    if self.move_targets(&c, a, 0).iter().any(|(_, d)| taken.contains(d)) { continue; }
                    if self.move_blocked_ex(&c, a, 0, false).is_some() { continue; }
                    let back = self.move_apply(&c, a, 0);
                    if let Some(p) = self.rs.ar.active.as_mut() {
                        for q in 0..NI { p.pool[q] += back[q]; }
                        if let Step::MoveHouse { door, .. } = &mut p.steps[i] { *door = a; }
                        p.overlay.clear();
                    }
                    self.burst(a.0 as f32 * 16. + 8., a.1 as f32 * 16. + 8., hx(0xc8a070), 10, 45.);
                    return true;
                }
            }
        }
        false
    }
    /// ลงมือหนึ่งขั้น: 1 ทำเสร็จ · 2 ข้าม · 3 ทำไปบางส่วน · 0 ยังทำไม่ได้ (รอ)
    fn arch_run_step(&mut self, i: usize) -> u8 {
        let Some(step) = self.rs.ar.active.as_ref().map(|p| p.steps[i].clone()) else { return 0 };
        match step {
            Step::Clear { at, .. } => {
                let o = self.w.obj_at(at.0, at.1);
                if o == 0 { return 1; }
                if let Some((it, q)) = clear_refund(o) { if let Some(p) = self.rs.ar.active.as_mut() { p.pool[it] += q; } }
                if self.w.placed.remove(&at).is_none() { self.w.removed.insert(at); }
                self.obj_dmg.remove(&at);
                self.burst(at.0 as f32 * 16. + 8., at.1 as f32 * 16. + 8., hx(0xc8a070), 6, 30.);
                1
            }
            Step::MoveHouse { door, to, rot } => {
                let Ok(c) = self.move_select(door) else { return 0 };
                if !c.building { return 2; }
                if self.move_blocked_ex(&c, to, rot, false).is_some() { return 0; }
                let back = self.move_apply(&c, to, rot);
                if let Some(p) = self.rs.ar.active.as_mut() { for k in 0..NI { p.pool[k] += back[k]; } }
                self.burst(to.0 as f32 * 16. + 8., to.1 as f32 * 16. + 8., hx(0xc8a070), 14, 55.);
                1
            }
            Step::MoveOne { from, to } => {
                let Ok(c) = self.move_select(from) else { return 2 };
                if self.move_blocked_ex(&c, to, 0, false).is_some() { return 0; }
                let back = self.move_apply(&c, to, 0);
                if let Some(p) = self.rs.ar.active.as_mut() { for k in 0..NI { p.pool[k] += back[k]; } }
                self.burst(to.0 as f32 * 16. + 8., to.1 as f32 * 16. + 8., hx(0xc8a070), 8, 40.);
                1
            }
            Step::Place { obj, at } => {
                if self.arch_moves_pending() { return 0; }
                match self.w.placed.get(&at) { Some(&o) if o == obj => return 1, Some(_) => return 2, None => {} }
                let t = self.w.tile_at(at.0, at.1);
                if SOLIDT[t as usize] || self.site_reserved(at) { return 2; }
                let o = self.w.obj_at(at.0, at.1);
                if !nat_trivial(o) && !nat_clearable(o) { return 2; }
                if self.someone_on(at) { return 0; }
                let cost = place_cost(obj);
                if let Some(p) = self.rs.ar.active.as_ref() { if cost.iter().any(|&(it, q)| p.pool[it] < q) { return 0; } }
                if nat_clearable(o) { self.arch_clear_nat(at, o); } else if o != 0 { self.w.removed.insert(at); }
                self.w.placed.insert(at, obj);
                if let Some(p) = self.rs.ar.active.as_mut() { for (it, q) in cost { p.pool[it] -= q; } }
                self.burst(at.0 as f32 * 16. + 8., at.1 as f32 * 16. + 8., hx(0xc8a070), 8, 40.);
                if obj == 18 { self.refresh_torches(); }
                1
            }
            Step::Pave { x0, y0, x1, y1, at } | Step::Wall { x0, y0, x1, y1, at, .. } => {
                // ถนน/กำแพงรอให้ย้ายบ้าน/เคลียร์ของเสร็จก่อน (ไม่งั้นช่องที่บ้านเดิมตั้งอยู่จะกลายเป็นรู)
                if self.arch_moves_pending() { return 0; }
                let wall = if let Step::Wall { obj, .. } = step { Some(obj) } else { None };
                let fields = self.arch_fields();
                let len = rect_len(x0, y0, x1, y1);
                let (mut j, mut built, mut did) = (at, 0u32, false);
                while j < len {
                    let k = rect_cell(x0, y0, x1, j);
                    let st = match wall { Some(o) => self.cell_wall(k, o, &fields), None => self.cell_road(k, &fields) };
                    match st {
                        Cst::Done | Cst::Skip => { j += 1; continue; }
                        _ => {}
                    }
                    if built >= if wall.is_some() { WALL_BATCH } else { PAVE_BATCH } { break; }
                    // ค่าวัสดุ
                    let cost: Vec<(usize, i32)> = match wall { Some(o) => wall_cost(o), None => if self.rs.ar.active.as_ref().map_or(0, |p| p.pave_n) % 2 == 0 { vec![(STONE, 1)] } else { vec![] } };
                    if wall.is_some() && self.someone_on(k) { break; }
                    let pool_ok = self.rs.ar.active.as_ref().map_or(false, |p| cost.iter().all(|&(it, q)| p.pool[it] >= q));
                    if !pool_ok { break; }
                    if let Cst::Clear(o) = st { self.arch_clear_nat(k, o); }
                    let o = self.w.obj_at(k.0, k.1);
                    if o != 0 && !self.w.placed.contains_key(&k) { self.w.removed.insert(k); }
                    match wall { Some(ob) => { self.w.placed.insert(k, ob); self.obj_dmg.remove(&k); } None => { self.w.tmod.insert(k, T_ROAD); } }
                    if let Some(p) = self.rs.ar.active.as_mut() { for (it, q) in cost { p.pool[it] -= q; } if wall.is_none() { p.pave_n += 1; } }
                    built += 1;
                    did = true;
                    j += 1;
                }
                if j != at { did = true; }
                if let Some(p) = self.rs.ar.active.as_mut() { match &mut p.steps[i] { Step::Pave { at, .. } | Step::Wall { at, .. } => *at = j, _ => {} } }
                if built > 0 {
                    let k = rect_cell(x0, y0, x1, j.saturating_sub(1));
                    self.burst(k.0 as f32 * 16. + 8., k.1 as f32 * 16. + 8., hx(0xb0a080), 6, 30.);
                    if wall.is_some() { self.room_t = 0.; }
                }
                if j >= len { 1 } else if did { 3 } else { 0 }
            }
        }
    }
    /// ทำหนึ่งอย่างจากแผน: ได้ทำอะไรไหม
    pub fn arch_step_once(&mut self) -> bool {
        let Some(plan) = self.rs.ar.active.as_ref() else { return false };
        let pending: Vec<usize> = (0..plan.steps.len()).filter(|&i| plan.done[i] == 0).collect();
        for i in pending {
            let is_clear = matches!(self.rs.ar.active.as_ref().unwrap().steps[i], Step::Clear { .. });
            match self.arch_run_step(i) {
                0 => continue,
                r => {
                    if r != 3 { if let Some(p) = self.rs.ar.active.as_mut() { p.done[i] = r; } }
                    // เคลียร์ต้นไม้/รื้อกำแพงทำรวดเดียวทั้งชุด (ไม่ต้องรอทีละชิ้น)
                    if is_clear {
                        for j in i + 1..self.rs.ar.active.as_ref().map_or(0, |p| p.steps.len()) {
                            let (c, d) = { let p = self.rs.ar.active.as_ref().unwrap(); (matches!(p.steps[j], Step::Clear { .. }), p.done[j]) };
                            if !c { break; }
                            if d != 0 { continue; }
                            let r = self.arch_run_step(j);
                            if r == 1 || r == 2 { if let Some(p) = self.rs.ar.active.as_mut() { p.done[j] = r; } }
                        }
                    }
                    return true;
                }
            }
        }
        false
    }
    pub fn arch_tick(&mut self, dt: f32) {
        if self.rs.ar.active.is_none() || self.w.dun.is_some() { return; }
        if self.rs.ar.migrate {
            self.rs.ar.migrate = false;
            self.arch_cancel();
            self.news_add("แผนผังเมืองแบบเก่า (ก่อนอัปเดต) ถูกยกเลิก คืนวัสดุแล้ว — ขอแบบใหม่ได้ที่หน้าสถาปนิก (มีถนน กำแพงเมือง เลือกขนาดเมือง)".to_string());
            return;
        }
        if self.rs.ar.active.as_ref().map_or(false, |p| p.overlay.is_empty() && !p.steps.is_empty()) { self.arch_rebuild_overlay(); self.arch_refresh_sf(); }
        // ยืนที่ลานกลางเมืองพร้อมของที่ช่างขาด = ช่างรับไปเอง
        self.rs.ar.dep_cd -= dt;
        if self.rs.ar.dep_cd <= 0. {
            self.rs.ar.dep_cd = 1.5;
            let c = self.rs.ar.active.as_ref().unwrap().center;
            let (px, py) = (tfloor(self.p.x), tfloor(self.p.y - 3.));
            if (px - c.0).abs() <= 6 && (py - c.1).abs() <= 6 && self.rs.ar.sf.iter().any(|&(it, _)| self.p.inv[it] > 0) { self.arch_deposit(); }
        }
        self.arch_away_help(dt);
        self.rs.ar.t -= dt;
        if self.rs.ar.t > 0. { return; }
        self.rs.ar.t = STEP_SECS;
        if self.arch_step_once() {
            self.rs.ar.stall = 0;
            self.arch_refresh_sf();
            if self.rs.ar.active.as_ref().map_or(false, |p| p.finished()) { self.arch_finish(); }
            return;
        }
        self.arch_refresh_sf();
        // ทำอะไรไม่ได้เลย: เหลือแต่งานที่ต้องใช้วัสดุ = แบบร่างรอของ (รอได้เรื่อยๆ) · ถ้ามีงานย้าย/รื้อค้าง = ติดขัด
        let stuck = {
            let p = self.rs.ar.active.as_ref().unwrap();
            (0..p.steps.len()).any(|i| p.done[i] == 0 && matches!(p.steps[i], Step::Clear { .. } | Step::MoveHouse { .. } | Step::MoveOne { .. }))
        };
        if !stuck { return; }
        self.rs.ar.stall += 1;
        if self.rs.ar.stall >= 2 && self.arch_unjam() { self.rs.ar.stall = 0; return; }
        if self.rs.ar.stall >= 6 {
            let mut skipped = 0;
            if let Some(p) = self.rs.ar.active.as_mut() {
                for i in 0..p.steps.len() { if p.done[i] == 0 && matches!(p.steps[i], Step::Clear { .. } | Step::MoveHouse { .. } | Step::MoveOne { .. }) { p.done[i] = 2; skipped += 1; } }
            }
            self.rs.ar.stall = 0;
            if skipped > 0 { self.news_add(format!("ช่างย้าย/รื้อไม่ได้ {} อย่าง (มีของขวาง/ติดกัน) — ข้ามไปทำถนน กำแพง ลานต่อ", skipped)); }
            if self.rs.ar.active.as_ref().map_or(false, |p| p.finished()) { self.arch_finish(); }
        }
    }
    fn arch_finish(&mut self) {
        let Some(p) = self.rs.ar.active.take() else { return };
        let moved = p.steps.iter().zip(&p.done).filter(|(s, d)| matches!(s, Step::MoveHouse { .. }) && **d == 1).count();
        let skipped = p.steps.iter().zip(&p.done).filter(|(s, d)| matches!(s, Step::MoveHouse { .. } | Step::MoveOne { .. } | Step::Place { .. }) && **d == 2).count();
        let st = p.stats();
        for i in 0..NI { self.p.inv[i] += p.pool[i]; }
        self.rs.ar.finished += 1;
        self.rs.ar.show = false;
        self.rs.ar.sf.clear();
        for n in self.npcs.iter_mut().filter(|n| n.state == ST_HOME) { n.mood = (n.mood + 3).min(100); }
        self.news_add(format!("ผังเมือง{}เสร็จแล้ว: ย้ายบ้าน {} หลัง · ถนน {} ช่อง · กำแพง {} ช่อง{} — ชาวบ้านชอบเมืองที่เป็นระเบียบ", SIZE_NAMES[p.size as usize], moved, st.road.1, st.wall.1, if skipped > 0 { format!(" (ข้าม {} อย่าง)", skipped) } else { String::new() }));
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
                _ => {}
            }
        }
        if ov.is_empty() { ov.push((i32::MIN, i32::MIN, 9)); }
        if let Some(a) = self.rs.ar.active.as_mut() { a.overlay = ov; }
    }

    // ---------------------------------------------------------------- v40 ซ่อมกำแพง/ถนนที่แหว่ง
    /// หาช่องโหว่ในแนวกำแพง (ช่องว่าง 1–4 ช่องระหว่างกำแพงชนิดเดียวกันในแนวตรง ไม่ใช่ถนน/ประตู)
    /// และในแนวถนน (ช่องที่ไม่ได้ปูระหว่างถนนสองฝั่ง) รอบเมือง → เป็นแผนให้ช่างเติม
    pub fn arch_repair_plan(&self) -> Option<Plan> {
        let c = self.town_center()?;
        let near = |k: &(i32, i32)| (k.0 - c.0).abs() <= SURVEY_R && (k.1 - c.1).abs() <= SURVEY_R;
        let fields = self.arch_fields();
        let mut walls: Vec<((i32, i32), u8)> = self.w.placed.iter().filter(|(k, &o)| near(k) && matches!(o, 10 | 11 | 38)).map(|(k, &o)| (*k, o)).collect();
        walls.sort();
        let mut fill: HashMap<(i32, i32), u8> = HashMap::new();
        for &(w, o) in &walls {
            for d in [(1, 0), (0, 1)] {
                let mut gap = vec![];
                let mut k = (w.0 + d.0, w.1 + d.1);
                while gap.len() <= 4 {
                    if self.w.placed.get(&k) == Some(&o) { break; }
                    if matches!(self.cell_wall(k, o, &fields), Cst::Todo | Cst::Clear(_)) { gap.push(k); } else { gap.clear(); break; }
                    k = (k.0 + d.0, k.1 + d.1);
                }
                // ช่องในบ้าน (ห้องปิด) ไม่ใช่รูของกำแพง
                if !gap.is_empty() && gap.len() <= 4 && self.w.placed.get(&k) == Some(&o) && gap.iter().all(|g| self.w.room_from(*g).is_none()) { for g in gap { fill.insert(g, o); } }
            }
        }
        let road = |k: (i32, i32)| self.w.tile_at(k.0, k.1) == T_ROAD;
        let mut roads: Vec<(i32, i32)> = vec![];
        for y in c.1 - SURVEY_R..=c.1 + SURVEY_R { for x in c.0 - SURVEY_R..=c.0 + SURVEY_R {
            let k = (x, y);
            if road(k) || fill.contains_key(&k) { continue; }
            let h = road((x - 1, y)) && (road((x + 1, y)) || road((x + 2, y)));
            let v = road((x, y - 1)) && (road((x, y + 1)) || road((x, y + 2)));
            if (h || v) && matches!(self.cell_road(k, &fields), Cst::Todo | Cst::Clear(_)) && !self.w.placed.contains_key(&k) { roads.push(k); }
        } }
        if fill.is_empty() && roads.is_empty() { return None; }
        let mut ks: Vec<((i32, i32), u8)> = fill.into_iter().collect();
        ks.sort();
        let mut p = Plan { center: c, size: self.rs.ar.size, wallk: self.rs.ar.wallk, ..Default::default() };
        p.notes.push(format!("ซ่อมแซม: เติมกำแพง {} ช่อง · ปูถนน {} ช่อง", ks.len(), roads.len()));
        for (k, o) in ks { p.steps.push(Step::Place { obj: o, at: k }); }
        for k in roads { p.steps.push(Step::Pave { x0: k.0, y0: k.1, x1: k.0, y1: k.1, at: 0 }); }
        p.done = vec![0; p.steps.len()];
        Some(p)
    }
    /// จ้างช่างซ่อม (ค่าจ้าง 10 เหรียญ) — ของไม่พอเป็นแบบร่างเหมือนแผนใหญ่
    pub fn arch_repair(&mut self) -> bool {
        if self.rs.ar.active.is_some() { self.toast("ช่างกำลังทำแผนอื่นอยู่"); return false; }
        let Some(p) = self.arch_repair_plan() else { self.toast("ไม่เจอกำแพง/ถนนที่แหว่งรอบเมือง"); return false; };
        let fee = 10;
        if self.p.inv[COIN] + self.tax_pool < fee { self.toast("ค่าซ่อม 10 เหรียญ — เงินไม่พอ"); return false; }
        let b = self.p.inv[COIN].min(fee); self.p.inv[COIN] -= b; self.tax_pool -= fee - b;
        let n = p.steps.len();
        self.rs.ar.active = Some(p);
        self.rs.ar.stall = 0; self.rs.ar.t = 0.; self.rs.ar.unjammed.clear(); self.rs.ar.helped = [0; NI];
        self.arch_refresh_sf();
        self.toast(format!("ช่างเริ่มเติมช่องที่แหว่ง {} ช่อง (ถางต้นไม้/พุ่มไม้ที่ขวางให้)", n));
        true
    }

    // ---------------------------------------------------------------- v42 ลานเมืองมีชีวิต: ตลาดนัด + ป้ายชื่อบ้าน
    /// ใจกลางลานเมือง: ที่ช่างวางไว้ หรือกองไฟใกล้กลางเมือง (เมืองที่วางเอง)
    pub fn plaza_c(&self) -> Option<(i32, i32)> {
        if let Some(p) = self.rs.ar.plaza { if self.w.placed.get(&p) == Some(&9) { return Some(p); } }
        let c = self.town_center()?;
        self.w.near_fire(c.0, c.1, 10)
    }
    /// แผงตลาดนัดรอบลาน (แผงที่เราวาง/ช่างวาง ไม่ใช่ของเมืองอาณาจักรอื่น)
    pub fn market_stalls(&self) -> Vec<(i32, i32)> {
        let Some(c) = self.plaza_c() else { return vec![] };
        let mut v: Vec<(i32, i32)> = self.w.placed.iter().filter(|(k, &o)| o == O_STALL && (k.0 - c.0).abs() <= 9 && (k.1 - c.1).abs() <= 9 && self.w.realm.city_at(k.0, k.1).is_none()).map(|(k, _)| *k).collect();
        v.sort();
        v
    }
    /// ตลาดนัดเย็น 15:30–17:30 (ต้องมีแผง และชาวบ้าน ≥ 2)
    pub fn market_open(&self) -> bool {
        let h = self.hour();
        self.w.dun.is_none() && (15.5..17.5).contains(&h) && self.pop() >= 2 && !self.market_stalls().is_empty()
    }
    /// ทุก 5 วิ: คนที่มาถึงแผง = ได้คุยกัน (อารมณ์ดีขึ้น) · คนที่หิว ซื้ออาหารจากกล่องในเมือง (จ่ายค่าแผงเข้าคลังเมือง)
    fn market_tick(&mut self) {
        let day = day_of(self);
        if self.rs.ar.mkt_day != day { self.rs.ar.mkt_day = day; self.rs.ar.mkt_seen.clear(); self.rs.ar.mkt_sold = 0; self.rs.ar.mkt_told = false; }
        if !self.market_open() {
            if !self.rs.ar.mkt_told && !self.rs.ar.mkt_seen.is_empty() && self.hour() >= 17.5 {
                self.rs.ar.mkt_told = true;
                let (n, m) = (self.rs.ar.mkt_seen.len(), self.rs.ar.mkt_sold);
                self.news_add(format!("ตลาดนัดเย็นนี้: ชาวบ้านมา {} คน{}", n, if m > 0 { format!(" · ซื้ออาหารกัน {} มื้อ (ค่าแผงเข้าคลังเมือง {} เหรียญ)", m, m * 2) } else { String::new() }));
            }
            return;
        }
        let stalls = self.market_stalls();
        let Some(pc) = self.plaza_c() else { return };
        let here: Vec<(usize, u32)> = self.npcs.iter().enumerate().filter(|(_, n)| n.state == ST_HOME && stalls.iter().any(|k| (k.0 as f32 * 16. + 8. - n.x).hypot(k.1 as f32 * 16. + 16. - n.y) < 40.)).map(|(i, n)| (i, n.id)).collect();
        for (i, id) in here {
            if !self.rs.ar.mkt_seen.insert(id) { continue; }
            self.npcs[i].mood = (self.npcs[i].mood + 1).min(100);
            let hungry = self.rs.sh.hungry.get(&id).copied().unwrap_or(0) > 0 && !self.rs.sh.snack.contains(&id);
            if !hungry { continue; }
            let keys: Vec<(i32, i32)> = self.chests.keys().copied().filter(|k| (k.0 - pc.0).abs() <= 40 && (k.1 - pc.1).abs() <= 40).collect();
            let got = keys.iter().find_map(|k| FOODS.iter().copied().find(|&f| self.chests[k][f] > 0).map(|f| (*k, f)));
            if let Some((k, f)) = got {
                if let Some(c) = self.chests.get_mut(&k) { c[f] -= 1; }
                self.rs.sh.snack.insert(id);
                self.tax_pool += 2;
                self.rs.ar.mkt_sold += 1;
                let (x, y) = (self.npcs[i].x, self.npcs[i].y);
                self.add_float(x, y - 22., format!("ซื้อ{}", INAME[f]), hx(0xffd27a), Some(f));
            }
        }
    }
    /// ป้ายชื่อหน้าบ้านชาวบ้าน (อัปเดตทุก 5 วิ รอบตัวเรา)
    fn plates_refresh(&mut self) {
        let (px, py) = (tfloor(self.p.x), tfloor(self.p.y));
        let mut v = vec![];
        for n in self.npcs.iter().filter(|n| n.state == ST_HOME && (n.bed.0 - px).abs() <= 24 && (n.bed.1 - py).abs() <= 18) {
            let Some(r) = self.w.room_from(n.bed) else { continue };
            let Some(&d) = r.doors.iter().min() else { continue };
            v.push((d, format!("บ้าน{}", NPC_NAMES[n.name as usize])));
        }
        self.rs.ar.plates = v;
    }

    // ---------------------------------------------------------------- v41 ประตูเมือง + หาวัสดุตอนเราไม่อยู่
    /// ช่องประตูเมือง: จากแผนช่าง + ช่องถนนที่ผ่านแนวกำแพงเมือง (กำแพงเมืองทั้งสองฝั่ง ห่างไม่เกิน 3 ช่อง)
    pub fn gates_refresh(&mut self) {
        let mut set: HashSet<(i32, i32)> = self.rs.ar.gates_k.iter().copied().filter(|k| self.w.tile_at(k.0, k.1) == T_ROAD && !self.w.placed.contains_key(k)).collect();
        let roads: Vec<(i32, i32)> = self.w.tmod.iter().filter(|(_, &t)| t == T_ROAD).map(|(k, _)| *k).collect();
        let is_cw = |k: (i32, i32)| self.w.placed.get(&k) == Some(&38);
        let is_road = |k: (i32, i32)| self.w.tile_at(k.0, k.1) == T_ROAD && !self.w.placed.contains_key(&k);
        for r in roads {
            for d in [(1, 0), (0, 1)] {
                let walk = |sgn: i32| -> bool {
                    let mut k = r;
                    for _ in 0..3 { k = (k.0 + d.0 * sgn, k.1 + d.1 * sgn); if is_cw(k) { return true; } if !is_road(k) { return false; } }
                    false
                };
                if walk(1) && walk(-1) { set.insert(r); }
            }
        }
        self.rs.ar.gate_set = set;
    }
    /// มอนสเตอร์ผ่านประตูเมืองไม่ได้ (เราและชาวบ้านผ่านได้)
    pub fn mob_block(&self, x: f32, y: f32) -> bool {
        !self.rs.ar.gate_set.is_empty() && self.w.dun.is_none() && self.rs.ar.gate_set.contains(&(tfloor(x), tfloor(y - 3.)))
    }
    pub fn gates_tick(&mut self, dt: f32) {
        self.rs.ar.gate_t -= dt;
        if self.rs.ar.gate_t > 0. { return; }
        self.rs.ar.gate_t = 5.;
        if self.w.dun.is_some() { return; }
        self.gates_refresh();
        self.market_tick();
        self.plates_refresh();
    }
    /// เราไม่อยู่ในเมือง (ชาวบ้านไม่ได้ทำงานให้เห็น): ช่วงกลางวัน ชาวบ้านยังหาไม้/หินมาส่งช่างเรื่อยๆ (ทุก 20 วินาที คนละ 2)
    fn arch_away_help(&mut self, dt: f32) {
        if self.rs.ar.no_help || self.work_active() || !work::work_hr(self.hour()) || self.rest_day() { return; }
        let (w, s) = (self.arch_wants(WOOD), self.arch_wants(STONE));
        if !w && !s { return; }
        self.rs.ar.away_t += dt;
        if self.rs.ar.away_t < 20. { return; }
        self.rs.ar.away_t = 0.;
        let pop = self.npcs.iter().filter(|n| n.state == ST_HOME && !n.child).count();
        let n = (pop * 2 / 3).max(if pop > 0 { 1 } else { 0 }) as i32;
        if n == 0 { return; }
        for (it, on) in [(WOOD, w), (STONE, s)] {
            if !on { continue; }
            let need = self.rs.ar.sf.iter().find(|x| x.0 == it).map_or(0, |x| x.1);
            let k = (n * 2 / if w && s { 2 } else { 1 }).max(1).min(need);
            if k > 0 { self.arch_receive(it, k); }
        }
    }

    // ---------------------------------------------------------------- v38 ชาวบ้านช่วยหาวัสดุ
    /// ช่างยังขาดของชิ้นนี้ไหม (และเปิดให้ชาวบ้านช่วย)
    pub fn arch_wants(&self, it: usize) -> bool {
        !self.rs.ar.no_help && self.rs.ar.active.is_some() && self.w.dun.is_none() && self.rs.ar.sf.iter().any(|&(i, q)| i == it && q > 0)
    }
    /// จุดส่งของ: ลานกลางเมือง (กระจายคนละช่อง)
    pub fn arch_drop_stand(&self, id: u32) -> Option<(f32, f32)> {
        let c = self.rs.ar.active.as_ref()?.center;
        let k = (c.0 - 3 + (id % 7) as i32, c.1 + 3);
        Some((k.0 as f32 * 16. + 8., k.1 as f32 * 16. + 12.))
    }
    pub fn arch_helpers(&self) -> usize { self.rs.work.values().filter(|j| j.other & ARCH_HELP != 0).count() }
    /// คนว่างงาน: ไปตัดไม้/ขุดหินมาส่งช่าง (ไม่เกินราว 2 ใน 3 ของเมือง ที่เหลือดูแลบ้านตัวเอง)
    pub fn arch_help_job(&self, id: u32) -> Option<work::Job> {
        let (wood, stone) = (self.arch_wants(WOOD), self.arch_wants(STONE));
        if !wood && !stone { return None; }
        let pop = self.npcs.iter().filter(|n| n.state == ST_HOME && !n.child).count();
        if self.arch_helpers() >= (pop * 2 / 3).max(2) { return None; }
        let c = self.rs.ar.active.as_ref()?.center;
        let chop = if wood && stone { id % 2 == 0 } else { wood };
        let t = if chop {
            self.find_obj(c, 34, |g, k, o| is_tree(o) && !is_fruit_tree(o) && o != T_SAPLING && o != 48 && o != 49 && !g.w.placed.contains_key(&k) && g.w.realm.city_at(k.0, k.1).is_none())
        } else {
            self.find_obj(c, 34, |g, k, o| matches!(o, 4 | 5) && !g.w.placed.contains_key(&k))
        }?;
        let stand = [(0, 1), (-1, 0), (1, 0), (0, -1)].iter().map(|d| (t.0 + d.0, t.1 + d.1)).find(|s| !self.w.solid_at(s.0, s.1)).map(|s| (s.0 as f32 * 16. + 8., s.1 as f32 * 16. + 12.))?;
        Some(work::Job { kind: if chop { work::J_CHOP } else { work::J_MINE }, tile: t, stand, t: 0., dur: 6., stage: 0, carry: None, other: ARCH_HELP, working: false })
    }
    /// ชาวบ้านเอาวัสดุมาส่งถึงลาน
    pub fn arch_receive(&mut self, it: usize, k: i32) -> bool {
        let Some(p) = self.rs.ar.active.as_mut() else { return false };
        p.pool[it] += k;
        self.rs.ar.helped[it] += k;
        self.arch_refresh_sf();
        true
    }

    // ---------------------------------------------------------------- เซฟ
    pub fn arch_save(&self) -> String {
        let a = &self.rs.ar;
        let mut s = String::new();
        if a.set || a.finished > 0 || a.size != 0 || a.wallk != 0 || a.no_help || a.plaza.is_some() {
            let (px, py, hp) = a.plaza.map_or((0, 0, 0), |p| (p.0, p.1, 1));
            s += &format!("arch {} {} {} {} {} {} {} {} {}\n", a.opts, a.variant, a.finished, a.size, a.wallk, a.no_help as u8, hp, px, py);
        }
        if !a.gates_k.is_empty() { s += "archgate"; for k in &a.gates_k { s += &format!(" {},{}", k.0, k.1); } s += "\n"; }
        if let Some(p) = &a.active {
            s += &format!("archplan {},{},{},{},{},{},{},{},{}", p.variant, p.opts, p.center.0, p.center.1, p.houses, p.size, p.wallk, p.pave_n, p.gates);
            for (i, &q) in p.pool.iter().enumerate() { if q != 0 { s += &format!(" p{}:{}", i, q); } }
            for (i, st) in p.steps.iter().enumerate() {
                let d = p.done[i];
                s += &match st {
                    Step::Clear { at, obj } => format!(" c,{},{},{},{}", at.0, at.1, obj, d),
                    Step::MoveHouse { door, to, rot } => format!(" h,{},{},{},{},{},{}", door.0, door.1, to.0, to.1, rot, d),
                    Step::MoveOne { from, to } => format!(" m,{},{},{},{},{}", from.0, from.1, to.0, to.1, d),
                    Step::Place { obj, at } => format!(" a,{},{},{},{}", obj, at.0, at.1, d),
                    Step::Pave { x0, y0, x1, y1, at } => format!(" r,{},{},{},{},{},{}", x0, y0, x1, y1, at, d),
                    Step::Wall { x0, y0, x1, y1, obj, at } => format!(" w,{},{},{},{},{},{},{}", x0, y0, x1, y1, obj, at, d),
                };
            }
            s += "\n";
        }
        s
    }
    pub fn arch_load(&mut self, tag: &str, r: &[&str]) {
        if tag == "arch" {
            let n = |i: usize| r.get(i).and_then(|v| v.parse::<i32>().ok()).unwrap_or(0);
            self.rs.ar.opts = (n(0) as u8) & 63;
            // เซฟจาก v36 (ยังไม่มีขนาดเมือง): เปิดถนน+กำแพงเมืองให้ตามค่าเริ่มต้นใหม่
            if r.len() < 4 { self.rs.ar.opts |= OPT_ROAD | OPT_CITYWALL; }
            self.rs.ar.set = true;
            self.rs.ar.variant = (n(1) as u8).min(1);
            self.rs.ar.finished = n(2);
            self.rs.ar.size = (n(3) as u8).min(2);
            self.rs.ar.wallk = (n(4) as u8).min(2);
            self.rs.ar.no_help = n(5) != 0;
            if n(6) == 1 { self.rs.ar.plaza = Some((n(7), n(8))); }
            return;
        }
        if tag == "archgate" {
            for t in r { let v: Vec<i32> = t.split(',').filter_map(|x| x.parse().ok()).collect(); if v.len() == 2 { self.rs.ar.gates_k.push((v[0], v[1])); } }
            return;
        }
        let Some(head) = r.first() else { return };
        let h: Vec<i32> = head.split(',').filter_map(|v| v.parse().ok()).collect();
        if h.len() < 5 { return; }
        let g = |i: usize| h.get(i).copied().unwrap_or(0);
        let mut p = Plan { variant: h[0] as u8, opts: h[1] as u8, center: (h[2], h[3]), houses: h[4] as usize, size: (g(5) as u8).min(2), wallk: (g(6) as u8).min(2), pave_n: g(7).max(0) as u32, gates: g(8) as u8, ..Default::default() };
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
                ('r', 5) => (Step::Pave { x0: v[0], y0: v[1], x1: v[2], y1: v[3], at: 0 }, v[4]),
                ('r', 6) => (Step::Pave { x0: v[0], y0: v[1], x1: v[2], y1: v[3], at: v[4].max(0) as u32 }, v[5]),
                ('w', 7) => (Step::Wall { x0: v[0], y0: v[1], x1: v[2], y1: v[3], obj: v[4] as u8, at: v[5].max(0) as u32 }, v[6]),
                _ => continue,
            };
            p.steps.push(st);
            p.done.push(d.clamp(0, 2) as u8);
        }
        // แผนจาก v36 (ไม่มีกำแพง/ขนาดเมือง และถนนปิดอยู่): ยกเลิกให้ แล้วให้ขอแบบใหม่
        if h.len() < 9 { self.rs.ar.migrate = true; }
        if !p.steps.is_empty() && !p.finished() { self.rs.ar.active = Some(p); }
    }
}

// ---------------------------------------------------------------- วาด
fn view_tiles(v: &View) -> (i32, i32, i32, i32) {
    ((v.cx / 16.).floor() as i32 - 1, (v.cy / 16.).floor() as i32 - 1, ((v.cx + screen_width() / v.s) / 16.).ceil() as i32 + 1, ((v.cy + screen_height() / v.s) / 16.).ceil() as i32 + 2)
}
/// แผนที่ที่ต้องวาดตอนนี้: แผนที่กำลังทำ (เห็นตลอด = แบบร่าง) หรือข้อเสนอ (เฉพาะตอนกดดู)
fn plan_to_draw(g: &Game) -> Option<(&Plan, bool)> {
    if g.w.dun.is_some() { return None; }
    if let Some(p) = g.rs.ar.active.as_ref() { return Some((p, true)); }
    if g.rs.ar.show { if let Some(p) = g.rs.ar.prop.as_ref() { return Some((p, false)); } }
    None
}
/// แบบร่างบนพื้น (วาดใต้ตัวละคร/ของ): ถนนที่ยังไม่ได้ปูเป็นลายถนนจางๆ
pub fn draw_arch_ground(g: &Game, sp: &Sprites, v: &View, t: f32) {
    let Some((p, _)) = plan_to_draw(g) else { return };
    let (vx0, vy0, vx1, vy1) = view_tiles(v);
    let a = 0.32 + 0.06 * (t * 2.).sin();
    let road = sp.tiles[T_ROAD as usize][0];
    for (i, s) in p.steps.iter().enumerate() {
        if p.done[i] != 0 { continue; }
        let Step::Pave { x0, y0, x1, y1, at } = *s else { continue };
        for y in y0.max(vy0)..=y1.min(vy1) { for x in x0.max(vx0)..=x1.min(vx1) {
            let idx = ((y - y0) * (x1 - x0 + 1) + (x - x0)) as u32;
            if idx < at || g.w.tile_at(x, y) == T_ROAD { continue; }
            blit(sp, v, road, x as f32 * 16. - v.cx, y as f32 * 16. - v.cy, false, Color::new(1., 0.95, 0.85, a));
        } }
    }
}

/// v42 ประตูเมือง (ซุ้มเหล็กยกขึ้น: มอนผ่านไม่ได้) + ป้ายชื่อหน้าบ้านเมื่อเดินผ่านใกล้ๆ
fn draw_town_marks(g: &Game, v: &View, font: &Font) {
    if g.w.dun.is_some() { return; }
    let (vx0, vy0, vx1, vy1) = view_tiles(v);
    for &(x, y) in &g.rs.ar.gate_set {
        if x < vx0 || x > vx1 || y < vy0 || y > vy1 { continue; }
        let (px, py, s) = ((x as f32 * 16. - v.cx) * v.s, (y as f32 * 16. - v.cy) * v.s, v.s);
        // คานบน + ซี่เหล็กสั้นๆ ห้อยลงมา (ประตูชักยกขึ้นแล้ว)
        draw_rectangle(px, py - 3. * s, 16. * s, 3. * s, Color::new(0.22, 0.2, 0.24, 0.9));
        for i in 0..4 { draw_rectangle(px + (2. + i as f32 * 4.) * s, py, 1.5 * s, 3. * s, Color::new(0.35, 0.33, 0.38, 0.85)); }
        draw_rectangle(px, py - 3. * s, 16. * s, 1. * s, Color::new(0.55, 0.52, 0.6, 0.9));
    }
    let (pxp, pyp) = (g.p.x, g.p.y);
    let fs = (5. * v.s).max(10.);
    for (d, name) in &g.rs.ar.plates {
        let (wx, wy) = (d.0 as f32 * 16. + 8., d.1 as f32 * 16.);
        if (wx - pxp).hypot(wy - pyp) > 7. * 16. { continue; }
        let (sx, sy) = ((wx - v.cx) * v.s, (wy - v.cy) * v.s - 4. * v.s);
        let m = measure_text(name, Some(font), fs as u16, 1.);
        draw_rectangle(sx - m.width / 2. - 4., sy - fs - 2., m.width + 8., fs + 6., Color::new(0.32, 0.2, 0.1, 0.85));
        draw_rectangle_lines(sx - m.width / 2. - 4., sy - fs - 2., m.width + 8., fs + 6., 1., hx(0xc8a070));
        draw_text_ex(name, sx - m.width / 2., sy, TextParams { font: Some(font), font_size: fs as u16, color: hx(0xffe9c0), ..Default::default() });
    }
}
/// แผนที่ซ้อนในโลก + เงาสิ่งก่อสร้างที่ยังไม่ได้สร้าง (แบบร่าง) + ป้ายกลางลาน
pub fn draw_arch_overlay(g: &Game, sp: &Sprites, v: &View, font: &Font, t: f32) {
    draw_town_marks(g, v, font);
    let Some((p, active)) = plan_to_draw(g) else { return };
    let (vx0, vy0, vx1, vy1) = view_tiles(v);
    let pul = 0.5 + 0.2 * (t * 3.).sin();
    if g.rs.ar.show {
        for &(x, y, k) in &p.overlay {
            if x == i32::MIN || x < vx0 || x > vx1 || y < vy0 || y > vy1 { continue; }
            let (px, py) = ((x as f32 * 16. - v.cx) * v.s, (y as f32 * 16. - v.cy) * v.s);
            let s = 16. * v.s;
            let c = match k {
                0 => Color::new(1., 0.85, 0.2, 0.22),
                1 => Color::new(0.3, 1., 0.4, 0.28 * pul + 0.1),
                2 => Color::new(1., 0.25, 0.25, 0.35),
                _ => Color::new(0.4, 0.8, 1., 0.35),
            };
            draw_rectangle(px, py, s, s, c);
            if k == 0 || k == 1 { draw_rectangle_lines(px, py, s, s, 1., Color::new(c.r, c.g, c.b, 0.6)); }
        }
    }
    // เงาของที่ยังไม่ได้สร้าง: กำแพง/คบเพลิง/บ่อ/กองไฟ
    let ghost = Color::new(0.65, 0.85, 1., 0.38 + 0.08 * (t * 2.).sin());
    let draw_obj = |o: u8, x: i32, y: i32| {
        let r = sp.obj[o as usize];
        let (px, py) = (x as f32 * 16. - v.cx, y as f32 * 16. - v.cy);
        blit(sp, v, r, px + 8. - r.w / 2., py + 16. - r.h, false, ghost);
    };
    for (i, s) in p.steps.iter().enumerate() {
        if p.done[i] != 0 { continue; }
        match *s {
            Step::Wall { x0, y0, x1, y1, obj, at } => {
                let vert = x0 == x1 && y1 > y0;
                for y in y0.max(vy0)..=y1.min(vy1) { for x in x0.max(vx0)..=x1.min(vx1) {
                    let idx = ((y - y0) * (x1 - x0 + 1) + (x - x0)) as u32;
                    if idx < at || g.w.placed.contains_key(&(x, y)) { continue; }
                    if obj == 38 && vert { blit(sp, v, sp.ex.cwall_v, x as f32 * 16. - v.cx, y as f32 * 16. - v.cy, false, ghost); } else { draw_obj(obj, x, y); }
                } }
            }
            Step::Place { obj, at } => {
                if at.0 < vx0 || at.0 > vx1 || at.1 < vy0 || at.1 > vy1 || g.w.placed.contains_key(&at) { continue; }
                draw_obj(obj, at.0, at.1);
            }
            _ => {}
        }
    }
    // ป้ายกลางลาน
    if active {
        let c = p.center;
        let (sx, sy) = (((c.0 as f32 + 0.5) * 16. - v.cx) * v.s, ((c.1 as f32 - 2.6) * 16. - v.cy) * v.s);
        if sx > -300. && sx < screen_width() + 300. && sy > -60. && sy < screen_height() + 60. {
            let sf = &g.rs.ar.sf;
            let l1 = format!("แบบร่างผังเมือง{} · เสร็จ {}%", SIZE_NAMES[p.size as usize], (p.frac() * 100.) as i32);
            let l2 = if sf.is_empty() { "ช่างกำลังสร้างตามแบบ".to_string() } else { format!("ขาด {} — ยืนตรงนี้พร้อมของ ช่างรับไปเอง", sf.iter().map(|&(i, q)| format!("{} {}", INAME[i], q)).collect::<Vec<_>>().join(" · ")) };
            let fs = (6. * v.s).max(11.) as u16;
            let w1 = measure_text(&l1, Some(font), fs, 1.).width;
            let w2 = measure_text(&l2, Some(font), fs, 1.).width;
            let w = w1.max(w2) + 14.;
            let h = fs as f32 * 2. + 16.;
            draw_rectangle(sx - w / 2., sy - h, w, h, Color::new(0.08, 0.1, 0.16, 0.78));
            draw_rectangle_lines(sx - w / 2., sy - h, w, h, 1.5, if sf.is_empty() { hx(0x88ee99) } else { hx(0xffcc66) });
            draw_text_ex(&l1, sx - w1 / 2., sy - h + fs as f32 + 4., TextParams { font: Some(font), font_size: fs, color: hx(0xffe9a8), ..Default::default() });
            draw_text_ex(&l2, sx - w2 / 2., sy - 7., TextParams { font: Some(font), font_size: fs, color: if sf.is_empty() { hx(0xbff5c8) } else { hx(0xffd27a) }, ..Default::default() });
        }
    }
}

mod tests {
    use super::*;
    fn town() -> (Game, (i32, i32)) {
        // โลกที่รอบจุดเกิดเป็นพื้นดินเกือบทั้งหมด (ไม่ต้องถมน้ำ — บ้านที่มีพื้นลงบนช่องถมไม่ได้)
        let mut g = Game::new_world_seed(278, Sfx::default());
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
    fn run_all(g: &mut Game) { for _ in 0..1500 { g.arch_tick(STEP_SECS + 0.01); if g.rs.ar.active.is_none() { break; } } }
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
        g.p.inv[WOOD] = 3000; g.p.inv[STONE] = 3000;
        assert!(g.arch_deposit());
        run_all(&mut g);
        assert!(g.rs.ar.active.is_none(), "แผนเสร็จ");
        assert_eq!(g.w.placed.values().filter(|&&o| o == 33).count(), placed0, "เตียงไม่หายไม่เพิ่ม");
        assert_eq!(rooms_ok(&g), 5, "ทุกบ้านยังเป็นห้องปิดที่มีเตียง");
        assert!(beds_of(&g).iter().zip(&beds0).filter(|(a, b)| a != b).count() >= 4, "เจ้าของเตียงตามไปบ้านใหม่");
        // บ้านเป็นระเบียบ: ประตูทุกบานโล่ง
        assert!(g.blocked_doors().is_empty(), "ประตูต้องไม่ถูกบัง");
        assert!(g.w.placed.len() >= n_obj, "ของครบ (และมีวงกองไฟ/บ่อน้ำเพิ่ม)");
        assert!(g.news.iter().any(|n| n.1.contains("ผังเมือง") && n.1.contains("เสร็จแล้ว")));
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
        let sf = g.arch_shortfall(&p);
        assert!(sf.iter().any(|&(i, _)| i == WOOD), "ต้องใช้ไม้ทำวงกองไฟ/บ่อ — ขาดของ: {:?}", sf);
        g.rs.ar.active = Some(p);
        g.p.inv[WOOD] = 3000; g.p.inv[STONE] = 3000;
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
        assert!(!g.arch_shortfall(a).is_empty());
        // แบบร่าง: กำแพง/ถนนยังไม่ได้เริ่ม (ไม่มีหิน/ไม้)
        assert!(a.steps.iter().any(|s| matches!(s, Step::Wall { at: 0, .. })), "กำแพงรอของ");
        assert!(a.steps.iter().any(|s| matches!(s, Step::Pave { at: 0, .. })), "ถนนรอของ");
        // ใส่ของนิดหน่อย = สร้างไปบางส่วน แบบร่างยังอยู่
        g.p.inv[WOOD] = 30; g.p.inv[STONE] = 30;
        assert!(g.arch_deposit());
        for _ in 0..200 { g.arch_tick(STEP_SECS + 0.01); }
        let a = g.rs.ar.active.as_ref().expect("ของไม่พอ = ยังเป็นแบบร่างต่อ");
        let st = a.stats();
        assert!(st.road.0 > 0 || st.wall.0 > 0, "สร้างไปบางส่วนตามของที่มี {:?}", st);
        assert!(!g.rs.ar.sf.is_empty());
        g.p.inv[WOOD] = 3000; g.p.inv[STONE] = 3000;
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
        assert!(w > 0 && w <= 30, "{}", w);
        g.arch_cancel();
        assert!(g.rs.ar.active.is_none());
        assert_eq!(g.p.inv[WOOD], 30, "ยกเลิก = คืนวัสดุที่ใส่ไว้");
        // ยังย้ายบ้านเองได้ตามเดิม
        let beds = beds_of(&g);
        let sel = g.move_select(g.w.room_from(beds[0]).unwrap().doors[0]).unwrap();
        assert!(sel.building);
    }
    #[test]
    fn v40_every_natural_plant_and_rock_is_clearable_but_water_is_not() {
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
        // v40 สร้างทับได้ทุกอย่างที่เป็นธรรมชาติ (ไม่ให้กำแพง/พื้นแหว่ง)
        if let Some(b) = bush { assert_eq!(lay.cs(b), Cs::Clear(2)); assert_eq!(clear_refund(2), Some((BERRY, 1))); }
        if let Some(f) = fruit { assert!(matches!(lay.cs(f), Cs::Clear(_))); }
        let w = (c.1 - 150..c.1 + 150).flat_map(|y| (c.0 - 150..c.0 + 150).map(move |x| (x, y))).find(|k| g.w.tile_at(k.0, k.1) == 1);
        if let Some(w) = w { assert_eq!(lay.cs(w), Cs::Blocked, "น้ำ"); }
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
        // ถนนทำไปบางส่วน (at ไม่ใช่ 0) ต้องเซฟ/โหลดกลับมาได้
        if let Some(p) = g.rs.ar.active.as_mut() { for s in p.steps.iter_mut() { if let Step::Pave { at, .. } | Step::Wall { at, .. } = s { *at = 3; } } p.pave_n = 5; }
        let a = g.rs.ar.active.clone().unwrap();
        g.rs.ar.active.as_mut().unwrap().pool[STONE] = 7;
        let s = g.arch_save();
        let mut h = Game::new_world_seed(278, Sfx::default());
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
        assert_eq!((b.pave_n, b.size, b.wallk), (a.pave_n, a.size, a.wallk));
        assert_eq!((h.rs.ar.set, h.rs.ar.opts, h.rs.ar.finished), (true, OPT_DEFAULT | OPT_ROAD, 2));
        assert!(Game::new_world_seed(1, Sfx::default()).arch_save().is_empty(), "ไม่เคยใช้ = ไม่เขียนอะไรลงเซฟ");
    }

    fn box_world(p: &Plan) -> impl Fn(i32, i32) -> (i32, i32) + '_ {
        move |u, v| if p.variant == 0 { (p.center.0 + u, p.center.1 + v) } else { (p.center.0 + v, p.center.1 + u) }
    }
    #[test]
    fn v37_bigger_town_more_streets_walls_with_gates_and_everything_built() {
        let (mut g, c) = town();
        scattered(&mut g, c);
        let mut walls_n = vec![];
        for size in 0..3u8 {
            g.rs.ar.size = size;
            let p = g.arch_plan(0, OPT_DEFAULT).unwrap();
            assert_eq!(p.size, size);
            let st = p.stats();
            assert!(st.road.1 > 0 && st.wall.1 > 0, "มีถนน+กำแพงเสมอ (ค่าเริ่มต้น)");
            walls_n.push(st.wall.1);
            let n_pave = p.steps.iter().filter(|s| matches!(s, Step::Pave { .. })).count();
            let sh = shape(size);
            assert_eq!(n_pave, 1 + sh.streets.len() + sh.avenues.len(), "ลาน + ถนนหลัก + ถนนตัด");
        }
        assert!(walls_n[0] < walls_n[1] && walls_n[1] < walls_n[2], "เมืองใหญ่ขึ้น กำแพงยาวขึ้น {:?}", walls_n);
        // สร้างจริงเมืองขนาดกลาง แนวเหนือ-ใต้ ด้วยกำแพงหิน
        g.rs.ar.size = 1; g.rs.ar.wallk = 1;
        let p = g.arch_plan(1, OPT_DEFAULT).unwrap();
        assert_eq!(p.count_houses(), 5);
        g.rs.ar.active = Some(p.clone());
        g.p.inv[WOOD] = 5000; g.p.inv[STONE] = 5000;
        assert!(g.arch_deposit());
        run_all(&mut g);
        assert!(g.rs.ar.active.is_none(), "เสร็จทั้งแผน");
        let w = box_world(&p);
        let (u0, u1, v0, v1) = wall_box(1);
        // ถนนหลักปูจริงตลอดแนว · ทะลุประตูเมืองออกไปข้างนอก
        for &s in shape(1).streets { for u in [u0 - 1, u0, -12, 3, 15, u1, u1 + 1] { let k = w(u, s * BLOCK); assert_eq!(g.w.tile_at(k.0, k.1), T_ROAD, "ถนน s={} u={}", s, u); } }
        // ลานปูหิน มีกองไฟกลางลาน
        let k = w(plaza_lo(1), plaza_lo(1) + 1); assert_eq!(g.w.tile_at(k.0, k.1), T_ROAD, "ลานปูหิน");
        let k = w(0, 0); assert_eq!(g.w.placed.get(&k), Some(&9));
        // กำแพงหินรอบเมือง · เว้นช่องประตูตรงถนน · คบเพลิงหน้าประตู
        let k = w(u0, 6); assert_eq!(g.w.placed.get(&k), Some(&11), "กำแพงหินด้านข้าง");
        let k = w(-7, v0); assert_eq!(g.w.placed.get(&k), Some(&11), "กำแพงด้านบน");
        let k = w(u1, v1); assert_eq!(g.w.placed.get(&k), Some(&11), "มุมกำแพง");
        for &s in shape(1).streets { for v in [s * BLOCK, s * BLOCK + 1] { let k = w(u0, v); assert!(!g.w.placed.contains_key(&k), "ประตูเมืองโล่ง"); } }
        let k = w(0, v0); assert!(!g.w.placed.contains_key(&k), "ประตูเมืองตรงถนนตัด");
        let k = w(u0 + 1, -1); assert_eq!(g.w.placed.get(&k), Some(&18), "คบเพลิงหน้าประตู");
        // บ้านทุกหลังอยู่ในกำแพง ประตูหันเข้าถนนที่ปูแล้ว
        assert_eq!(rooms_ok(&g), 5);
        for n in &g.npcs {
            let (u, v) = (n.bed.1 - p.center.1, n.bed.0 - p.center.0);
            assert!(u > u0 && u < u1 && v > v0 && v < v1, "บ้านอยู่ในกำแพง {:?}", n.bed);
        }
        assert!(g.blocked_doors().is_empty());
    }
    #[test]
    fn v37_blueprint_waits_and_standing_at_plaza_hands_over_materials() {
        let (mut g, c) = town();
        scattered(&mut g, c);
        let p = g.arch_plan(0, OPT_DEFAULT).unwrap();
        g.rs.ar.prop = Some(p); g.p.inv[COIN] = 999;
        assert!(g.arch_accept());
        for _ in 0..60 { g.arch_tick(STEP_SECS + 0.01); }
        assert!(g.rs.ar.active.is_some(), "ไม่มีของ = เป็นแบบร่างค้างไว้ ไม่ถูกข้าม/ยกเลิก");
        let sf = g.rs.ar.sf.clone();
        assert!(sf.iter().any(|&(i, _)| i == STONE), "ขาดหิน {:?}", sf);
        let need_stone = sf.iter().find(|x| x.0 == STONE).unwrap().1;
        // เดินไปยืนกลางลานพร้อมหิน 40 → ช่างรับไปเอง
        let pc = g.rs.ar.active.as_ref().unwrap().center;
        g.p.x = pc.0 as f32 * 16. + 8.; g.p.y = (pc.1 + 3) as f32 * 16. + 12.;
        g.p.inv[STONE] = 40;
        g.rs.ar.dep_cd = 0.;
        g.arch_tick(0.01);
        assert_eq!(g.p.inv[STONE], 0, "ส่งหินให้ช่างแล้ว");
        assert_eq!(g.rs.ar.active.as_ref().unwrap().pool[STONE], 40);
        assert!(g.rs.ar.sf.iter().find(|x| x.0 == STONE).map_or(0, |x| x.1) <= need_stone - 40 + 2);
        // ขนจากกล่องในเมืองได้ด้วย
        let mut inv = [0; NI]; inv[WOOD] = 500; inv[STONE] = 500;
        g.chests.insert((pc.0 + 9, pc.1 + 9), inv);
        assert!(g.arch_deposit_chests());
        assert!(g.chests[&(pc.0 + 9, pc.1 + 9)][STONE] < 500);
    }
    #[test]
    fn v37_old_v36_settings_turn_on_roads_and_walls() {
        let mut g = Game::new_world_seed(3, Sfx::default());
        g.arch_load("arch", &["13", "1", "0"]);
        assert!(g.arch_opts() & OPT_ROAD != 0 && g.arch_opts() & OPT_CITYWALL != 0);
        g.arch_set_wall(None);
        assert_eq!(g.arch_opts() & OPT_CITYWALL, 0);
        g.arch_set_size(2); g.arch_set_wall(Some(2));
        let s = g.arch_save();
        let mut h = Game::new_world_seed(3, Sfx::default());
        let parts: Vec<&str> = s.trim().split(' ').collect();
        h.arch_load(parts[0], &parts[1..]);
        assert_eq!((h.rs.ar.size, h.rs.ar.wallk, h.arch_opts() & OPT_CITYWALL), (2, 2, OPT_CITYWALL));
    }
    #[test]
    fn v38_villagers_gather_wood_and_stone_for_the_plan() {
        let (mut g, c) = town();
        scattered(&mut g, c);
        let p = g.arch_plan(0, OPT_DEFAULT).unwrap();
        g.rs.ar.prop = Some(p); g.p.inv[COIN] = 999;
        assert!(g.arch_accept());
        // ย้ายบ้านให้เสร็จก่อน (ไม่มีวัสดุ = ถนน/กำแพงเป็นแบบร่าง)
        for _ in 0..80 { g.arch_tick(STEP_SECS + 0.01); }
        assert!(g.arch_wants(WOOD) || g.arch_wants(STONE));
        // ป่ารอบเมือง: คืนต้นไม้/หินธรรมชาติในวงแหวน 18..34 ช่อง
        for y in c.1 - 34..=c.1 + 34 { for x in c.0 - 34..=c.0 + 34 {
            let d = (x - c.0).abs().max((y - c.1).abs());
            if d >= 18 && !g.w.placed.contains_key(&(x, y)) { g.w.removed.remove(&(x, y)); }
        } }
        let n_res = (c.1 - 34..=c.1 + 34).flat_map(|y| (c.0 - 34..=c.0 + 34).map(move |x| (x, y))).filter(|k| { let o = g.w.obj_at(k.0, k.1); (is_tree(o) && !is_fruit_tree(o)) || matches!(o, 4 | 5) }).count();
        assert!(n_res > 5, "มีต้นไม้/หินรอบเมือง {}", n_res);
        let pc = g.rs.ar.active.as_ref().unwrap().center;
        g.p.x = pc.0 as f32 * 16. + 8.; g.p.y = (pc.1 + 8) as f32 * 16.;
        while !work::work_hr(g.hour()) || g.hour() > 9. { g.time += 3.; }
        g.last_day = day_of(&g);
        for n in g.npcs.iter_mut() { n.mood = 80; }
        let dt = 1. / 30.;
        let mut seen_help = false;
        for _ in 0..(150 * 30) {
            g.npc_tick(dt); g.work_tick(dt);
            if g.arch_helpers() > 0 { seen_help = true; }
            if g.rs.ar.helped[WOOD] + g.rs.ar.helped[STONE] > 0 { break; }
        }
        assert!(seen_help, "ชาวบ้านออกไปช่วยหาวัสดุ");
        let got = g.rs.ar.helped[WOOD] + g.rs.ar.helped[STONE];
        assert!(got > 0, "ขนไม้/หินมาส่งช่างที่ลาน");
        let a = g.rs.ar.active.as_ref().unwrap();
        assert!(a.pool[WOOD] + a.pool[STONE] >= got);
        // ปิดการระดม = ไม่มีใครรับงานใหม่
        g.rs.ar.no_help = true;
        assert!(g.arch_help_job(g.npcs[0].id).is_none());
    }
    #[test]
    fn v38_old_v36_plan_is_cancelled_and_refunded() {
        let mut g = Game::new_world_seed(3, Sfx::default());
        g.arch_load("archplan", &["0,13,5,5,1", "p4:9", "h,1,1,5,5,0,0", "r,0,0,3,0,0"]);
        assert!(g.rs.ar.active.is_some() && g.rs.ar.migrate);
        let s0 = g.p.inv[4];
        g.arch_tick(0.1);
        assert!(g.rs.ar.active.is_none());
        assert_eq!(g.p.inv[4], s0 + 9, "คืนวัสดุ");
    }
    #[test]
    fn v40_repair_fills_gaps_left_by_trees_in_walls_and_roads() {
        let (mut g, c) = town();
        scattered(&mut g, c);
        let y = c.1 + 30;
        for x in c.0 - 10..=c.0 + 10 { if x != c.0 && x != c.0 + 1 { g.w.placed.insert((x, y), 38); } }
        // ช่องว่าง 2 ช่องกลางแนว (ต้นไม้เคยขวาง) — ประตูเมือง (ถนน) ไม่เติม
        g.w.placed.remove(&(c.0 - 5, y)); g.w.placed.remove(&(c.0 - 4, y));
        g.w.tmod.insert((c.0, y), T_ROAD); g.w.tmod.insert((c.0 + 1, y), T_ROAD);
        for x in c.0 - 8..=c.0 + 8 { if x != c.0 + 3 { g.w.tmod.insert((x, y + 4), T_ROAD); } }
        let p = g.arch_repair_plan().expect("เจอช่องแหว่ง");
        let places: Vec<(i32, i32)> = p.steps.iter().filter_map(|s| if let Step::Place { at, obj: 38 } = s { Some(*at) } else { None }).collect();
        assert!(places.contains(&(c.0 - 5, y)) && places.contains(&(c.0 - 4, y)));
        assert!(!places.contains(&(c.0, y)), "ประตูเมืองไม่ปิด");
        assert!(p.steps.iter().any(|s| matches!(s, Step::Pave { x0, y0, .. } if (*x0, *y0) == (c.0 + 3, y + 4))));
        g.p.inv[COIN] = 50; g.p.inv[STONE] = 100;
        assert!(g.arch_repair());
        g.arch_deposit();
        for _ in 0..60 { g.arch_tick(STEP_SECS + 0.01); }
        assert_eq!(g.w.placed.get(&(c.0 - 5, y)), Some(&38));
        assert_eq!(g.w.tile_at(c.0 + 3, y + 4), T_ROAD);
        assert!(g.rs.ar.active.is_none());
    }
    #[test]
    fn v41_city_gate_blocks_monsters_and_is_found_in_old_towns() {
        let (mut g, c) = town();
        let y = c.1 + 20;
        for x in c.0 - 8..=c.0 + 8 { if x != c.0 && x != c.0 + 1 { g.w.placed.insert((x, y), 38); } }
        for yy in y - 3..=y + 3 { g.w.tmod.insert((c.0, yy), T_ROAD); g.w.tmod.insert((c.0 + 1, yy), T_ROAD); }
        g.gates_refresh();
        assert!(g.rs.ar.gate_set.contains(&(c.0, y)) && g.rs.ar.gate_set.contains(&(c.0 + 1, y)), "เจอประตูเมืองจากกำแพงเมืองเดิม");
        assert!(!g.rs.ar.gate_set.contains(&(c.0, y - 2)), "ถนนนอกแนวกำแพงไม่ใช่ประตู");
        assert!(g.mob_block(c.0 as f32 * 16. + 8., y as f32 * 16. + 10.));
        assert!(!g.mob_block(c.0 as f32 * 16. + 8., (y - 2) as f32 * 16. + 10.));
        // แผนใหม่: ประตูเมืองเซฟไว้
        scattered(&mut g, c);
        let p = g.arch_plan(0, OPT_DEFAULT).unwrap();
        assert!(!p.gate_cells.is_empty());
        g.rs.ar.prop = Some(p); g.p.inv[COIN] = 999;
        assert!(g.arch_accept());
        assert!(!g.rs.ar.gates_k.is_empty());
        let s = g.arch_save();
        assert!(s.contains("archgate "));
    }
    #[test]
    fn v41_villagers_keep_gathering_while_player_is_away() {
        let (mut g, c) = town();
        scattered(&mut g, c);
        let p = g.arch_plan(0, OPT_DEFAULT).unwrap();
        g.rs.ar.prop = Some(p); g.p.inv[COIN] = 999;
        assert!(g.arch_accept());
        for _ in 0..80 { g.arch_tick(STEP_SECS + 0.01); }
        // เราออกไปไกล ช่วงกลางวัน
        g.p.x += 200. * 16.;
        while !work::work_hr(g.hour()) { g.time += 3.; }
        assert!(!g.work_active());
        for _ in 0..200 { g.arch_tick(0.5); }
        assert!(g.rs.ar.helped[WOOD] + g.rs.ar.helped[STONE] > 0, "ชาวบ้านยังหาวัสดุมาส่ง");
    }
    #[test]
    fn v42_small_plaza_market_stalls_cheap_walls_and_bedless_buildings() {
        let (mut g, c) = town();
        scattered(&mut g, c);
        // โกดังไม่มีเตียง
        let store = (c.0 + 16, c.1 - 14);
        for (p, st) in prefab_steps(OBJ_HUT, store, 0) {
            match st { crate::house::Step::Tile(t) => { g.w.tmod.insert(p, t); } crate::house::Step::Obj(o) => { if o != 33 { g.w.placed.insert(p, o); } } }
        }
        let sv = g.arch_survey().unwrap();
        assert!(sv.houses.iter().any(|h| h.owner.is_none() && h.carry.tiles.contains(&store)), "โกดังนับเป็นอาคาร");
        g.rs.ar.size = 0;
        let p = g.arch_plan(0, OPT_DEFAULT).unwrap();
        let stalls: Vec<(i32, i32)> = p.steps.iter().filter_map(|s| if let Step::Place { obj: O_STALL, at } = s { Some(*at) } else { None }).collect();
        assert_eq!(stalls.len(), 2, "แผงตลาด 2 แผง");
        let pave = p.steps.iter().find_map(|s| if let Step::Pave { x0, x1, .. } = s { Some(x1 - x0 + 1) } else { None }).unwrap();
        assert_eq!(pave, plaza_hi(0) - plaza_lo(0) + 1, "ลานเมืองเล็ก 8×8");
        assert_eq!(wall_cost(38), vec![(STONE, 4)]);
        g.rs.ar.prop = Some(p); g.p.inv[COIN] = 999;
        assert!(g.arch_accept());
        g.p.inv[WOOD] = 5000; g.p.inv[STONE] = 5000;
        g.arch_deposit();
        for _ in 0..1500 { g.arch_tick(STEP_SECS + 0.01); if g.rs.ar.active.is_none() { break; } }
        assert!(g.rs.ar.active.is_none());
        assert!(g.rs.ar.plaza.is_some() && g.plaza_c() == g.rs.ar.plaza);
        assert_eq!(g.market_stalls().len(), 2);
        // ตลาดนัด: คนหิวยืนหน้าแผง → ซื้ออาหารจากกล่อง
        while !(15.5..17.0).contains(&g.hour()) { g.time += 1.; }
        assert!(g.market_open());
        let k = g.market_stalls()[0];
        let pc = g.plaza_c().unwrap();
        let mut inv = [0; NI]; inv[BERRY] = 5;
        g.chests.insert((pc.0 + 3, pc.1 - 9), inv);
        let id = g.npcs[0].id;
        g.rs.sh.hungry.insert(id, 1);
        g.npcs[0].x = k.0 as f32 * 16. + 8.; g.npcs[0].y = k.1 as f32 * 16. + 26.;
        let t0 = g.tax_pool;
        g.rs.ar.gate_t = 0.; g.gates_tick(0.1);
        assert!(g.rs.ar.mkt_seen.contains(&id));
        assert!(g.rs.sh.snack.contains(&id), "ซื้ออาหารกินแล้ว");
        assert_eq!(g.tax_pool, t0 + 2);
        assert_eq!(g.chests[&(pc.0 + 3, pc.1 - 9)][BERRY], 4);
        // ป้ายชื่อหน้าบ้าน
        g.p.x = g.npcs[0].bed.0 as f32 * 16.; g.p.y = g.npcs[0].bed.1 as f32 * 16.;
        g.rs.ar.gate_t = 0.; g.gates_tick(0.1);
        assert!(g.rs.ar.plates.iter().any(|(_, n)| n.starts_with("บ้าน")));
        // เซฟ/โหลดลาน
        let s = g.arch_save();
        let mut h = Game::new_world_seed(278, Sfx::default());
        for line in s.lines() { let v: Vec<&str> = line.split(' ').collect(); h.arch_load(v[0], &v[1..]); }
        assert_eq!(h.rs.ar.plaza, g.rs.ar.plaza);
    }
}
