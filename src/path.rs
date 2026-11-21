// v17.1 — ระบบหาทาง (A*) สำหรับทหาร ชาวเมือง และชาวบ้าน
// ตาราง 8 ทิศ ไม่ตัดมุม (เฉียงได้เมื่อสองช่องข้างว่าง) · จำกัดจำนวนช่องที่ค้นต่อครั้ง
// ถ้าไปไม่ถึง → เดินไปช่องที่ใกล้เป้าที่สุดที่ไปได้ · จำกัดจำนวนการค้นต่อเฟรม (งบ) กันเครื่องหน่วง
use super::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// โหมดการเดิน
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Pass {
    /// เดินทั่วไป: ช่องที่ไม่ทึบ + ประตูได้ (ชาวบ้าน/ชาวเมืองเปิดประตูเอง)
    Door,
    /// เดินทั่วไปแต่เปิดประตูไม่ได้ (ยามเมืองไล่จับ)
    Walk,
    /// ทหารบุก: ผ่านกำแพง/ประตูที่ผู้เล่นสร้างได้ (ต้องทุบ — คิดค่าแพง) · ประตูเมืองเปิดอยู่แล้ว
    Breach,
}

#[derive(Clone, Debug, Default)]
pub struct Path {
    pub pts: Vec<(i32, i32)>,
    pub goal: (i32, i32),
    pub cd: f32,
    pub stuck: f32,
    pub direct: bool,
    pub last: (f32, f32),
    pub los_t: f32,
}

pub const PATH_NODES: usize = 900;
const SPAN: i32 = 48;

/// ค่าเดินผ่านช่อง (None = ผ่านไม่ได้)
pub fn tile_cost(w: &World, tx: i32, ty: i32, mode: Pass) -> Option<i32> {
    if SOLIDT[w.tile_at(tx, ty) as usize] { return None; }
    let o = w.obj_at(tx, ty);
    if o == 0 || !OSOLID[o as usize] { return Some(0); }
    match mode {
        Pass::Door => if o == 12 { Some(2) } else { None },
        Pass::Walk => None,
        // ทหารศัตรูเปิดประตูไม่ได้ ต้องทุบ (เฉพาะของที่ผู้เล่นสร้าง)
        Pass::Breach => if breachable(w, tx, ty) { Some(if o == 12 { 30 } else { 60 }) } else { None },
    }
}
/// ของที่ทหารบุกทุบได้ (กำแพง/ประตู/หน้าต่างที่ผู้เล่นสร้าง)
pub fn breachable(w: &World, tx: i32, ty: i32) -> bool {
    w.placed.get(&(tx, ty)).map_or(false, |&p| is_wall_obj(p))
}
const HARD: i32 = 30;

/// A* จาก a ไป b (ช่อง) — คืนลำดับช่องไม่รวมจุดเริ่ม · ไปไม่ถึง = ไปช่องที่ใกล้ b ที่สุด
pub fn find_path(w: &World, a: (i32, i32), b: (i32, i32), mode: Pass, max_nodes: usize) -> Vec<(i32, i32)> {
    if a == b { return vec![]; }
    let h = |p: (i32, i32)| { let (dx, dy) = ((p.0 - b.0).abs(), (p.1 - b.1).abs()); 10 * dx.max(dy) + 4 * dx.min(dy) };
    let mut open: BinaryHeap<Reverse<(i32, i32, (i32, i32))>> = BinaryHeap::new();
    let mut came: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut gs: HashMap<(i32, i32), i32> = HashMap::new();
    gs.insert(a, 0);
    open.push(Reverse((h(a), 0, a)));
    let mut best = (h(a), a);
    let mut n = 0;
    let mut cache: HashMap<(i32, i32), Option<i32>> = HashMap::new();
    let mut cost = |q: (i32, i32)| *cache.entry(q).or_insert_with(|| tile_cost(w, q.0, q.1, mode));
    // ประเมินช่องปลายทางที่ทึบ (เช่น เป้าหมายยืนชิดกำแพง) ให้หยุดข้างๆ
    while let Some(Reverse((_, g, p))) = open.pop() {
        if gs.get(&p).map_or(false, |&v| v < g) { continue; }
        if p == b { best = (0, p); break; }
        n += 1;
        if n > max_nodes { break; }
        let hp = h(p);
        if hp < best.0 { best = (hp, p); }
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)] {
            let q = (p.0 + dx, p.1 + dy);
            if (q.0 - a.0).abs() > SPAN || (q.1 - a.1).abs() > SPAN { continue; }
            let Some(c) = cost(q) else { continue };
            if dx != 0 && dy != 0 {
                // ไม่ตัดมุม และไม่ทุบกำแพงทางเฉียง
                if c >= HARD { continue; }
                match (cost((p.0 + dx, p.1)), cost((p.0, p.1 + dy))) { (Some(c1), Some(c2)) if c1 < HARD && c2 < HARD => {}, _ => continue }
            }
            let ng = g + if dx != 0 && dy != 0 { 14 } else { 10 } + c;
            if gs.get(&q).map_or(true, |&v| ng < v) {
                gs.insert(q, ng);
                came.insert(q, p);
                open.push(Reverse((ng + h(q), ng, q)));
            }
        }
    }
    let mut out = vec![];
    let mut cur = best.1;
    while cur != a {
        out.push(cur);
        match came.get(&cur) { Some(&p) => cur = p, None => break }
    }
    out.reverse();
    out
}

/// ตำแหน่งเท้า (พิกเซล) ของกลางช่อง
pub fn tile_foot(t: (i32, i32)) -> (f32, f32) { (t.0 as f32 * 16. + 8., t.1 as f32 * 16. + 11.) }
pub fn foot_tile(x: f32, y: f32) -> (i32, i32) { (tfloor(x), tfloor(y - 3.)) }

/// เดินเส้นตรงได้ไหม (ไล่ทีละช่องตามเส้น · กว้างพอสำหรับตัวคน)
pub fn line_clear(w: &World, x0: f32, y0: f32, x1: f32, y1: f32, mode: Pass) -> bool {
    let d = (x1 - x0).hypot(y1 - y0);
    let n = (d / 5.).ceil().max(1.) as i32;
    let (nx, ny) = if d > 0.01 { (-(y1 - y0) / d * 3.5, (x1 - x0) / d * 3.5) } else { (0., 0.) };
    for i in 0..=n {
        let t = i as f32 / n as f32;
        let (x, y) = (x0 + (x1 - x0) * t, y0 + (y1 - y0) * t - 3.);
        for (ox, oy) in [(0., 0.), (nx, ny), (-nx, -ny)] {
            let (tx, ty) = (tfloor(x + ox), tfloor(y + oy));
            match tile_cost(w, tx, ty, mode) { Some(c) if c < HARD => {}, _ => return false }
        }
    }
    true
}

impl Path {
    pub fn clear(&mut self) { self.pts.clear(); self.direct = false; }
    /// จุดที่ควรเดินไปตอนนี้ (พิกเซล) เพื่อไปถึง goal · budget = จำนวน A* ที่ยังค้นได้ในเฟรมนี้
    pub fn steer(&mut self, w: &World, x: f32, y: f32, goal: (f32, f32), mode: Pass, dt: f32, budget: &mut i32) -> (f32, f32) {
        self.cd -= dt;
        self.los_t -= dt;
        // ติดอยู่กับที่ไหม
        let moved = (x - self.last.0).abs() + (y - self.last.1).abs();
        let next_breach = self.pts.first().map_or(false, |&t| breachable(w, t.0, t.1));
        if moved < dt * 4. && !next_breach { self.stuck += dt; } else { self.stuck = (self.stuck - dt).max(0.); }
        self.last = (x, y);
        let gt = foot_tile(goal.0, goal.1);
        let me = foot_tile(x, y);
        let gd = (goal.0 - x).hypot(goal.1 - y);
        // ใกล้มาก หรือเห็นทางตรง → เดินตรง
        if self.los_t <= 0. {
            self.los_t = 0.35;
            self.direct = gd < 300. && line_clear(w, x, y, goal.0, goal.1, mode);
            if self.direct { self.pts.clear(); }
        }
        if gd < 14. || (self.direct && self.stuck < 0.5) { return goal; }
        let goal_moved = (gt.0 - self.goal.0).abs().max((gt.1 - self.goal.1).abs()) > 2;
        let need = self.pts.is_empty() || goal_moved || self.stuck > 0.6;
        if need && self.cd <= 0. && *budget > 0 {
            *budget -= 1;
            self.pts = find_path(w, me, gt, mode, PATH_NODES);
            self.goal = gt;
            self.cd = if self.pts.is_empty() { 1.2 } else { 0.5 };
            self.stuck = 0.;
            self.direct = false;
        }
        // ตัดจุดที่ถึงแล้ว (หรือที่เลยมาแล้ว)
        while let Some(&t) = self.pts.first() {
            let (fx, fy) = tile_foot(t);
            if (fx - x).abs() < 3. && (fy - y).abs() < 3. { self.pts.remove(0); } else { break; }
        }
        match self.pts.first() {
            Some(&t) => tile_foot(t),
            None => goal,
        }
    }
}

/// งบการค้นหาทางต่อเฟรม (ต่อระบบ)
pub const PATH_BUDGET: i32 = 2;
