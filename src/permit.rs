//! v30 ขออนุญาตใช้ที่ดิน
//!
//! ชาวบ้านจะไม่ไถไร่ใหม่ / สร้างบ้านของตัวเองเงียบๆ อีกแล้ว — เขาเลือกที่ไว้ แล้วเดินมาหาเราเพื่อถามก่อน
//! (กรอบเหลืองบนพื้น = ที่ที่ขอ) เราตอบได้ 3 แบบ: ให้เลย · ให้ที่อื่น (เลือกเองด้วยโหมดวาง) · ไม่ให้
//! - ไร่: ขอทีละผืน 3x2 = 6 แปลง · อนุญาตแล้วชาวไร่ไถ/ปลูกเฉพาะในผืนที่ได้รับ (ผืนเซฟไว้)
//! - บ้านของชาวบ้าน: ต้องได้ที่ก่อน แล้วค่อยขอวัสดุ/เริ่มสร้าง
//! - ไม่ให้ = จำที่นั้นไว้ 3 วัน จะไม่ขอซ้ำตรงนั้น
//! - ไม่ตอบ (เดินหนี/ยืนรอนาน) = กลับไปทำงานก่อน แล้วมาถามใหม่ทีหลัง · ตอบได้จากหน้าของเขาในแท็บชุมชนด้วย
use super::*;

/// "ชิ้นส่วน" พิเศษในโหมดวาง: เลือกที่ดินให้ชาวบ้าน (ไม่อยู่ในเมนูสร้าง)
pub const OBJ_PERMIT: u8 = 244;
/// ชนิดคำขอ: 0 = ไร่ · อื่นๆ = บ้านสำเร็จรูปชนิดนั้น
pub const AK_FARM: u8 = 0;
pub const FIELD_W: i32 = 3;
pub const FIELD_H: i32 = 2;
/// ผืนไร่ต้องอยู่ไม่ไกลบ้านชาวไร่ (ช่อง)
pub const FIELD_R: i32 = 12;
/// ยืนรอคำตอบตรงหน้าเรานานสุด (วินาที) แล้วกลับไปทำงานก่อน
pub const ASK_WAIT: f32 = 40.;
/// ไม่ได้คำตอบ → อีกนานเท่าไรจึงมาถามใหม่ (วินาทีเกม)
pub const ASK_SNOOZE: f32 = 150.;
/// ระยะที่ถือว่า "มาถึงตัวเราแล้ว" (พิกเซล)
pub const ASK_NEAR: f32 = 64.;
/// ไม่ให้ = ไม่ขอซ้ำที่เดิมกี่วัน
pub const DENY_DAYS: i32 = 3;

#[derive(Clone, Debug)]
pub struct LandAsk { pub by: u32, pub kind: u8, pub at: (i32, i32), pub dir: u8, pub came: bool, pub snooze: f32 }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LandField { pub by: u32, pub x: i32, pub y: i32, pub w: i32, pub h: i32 }
impl LandField {
    pub fn cells(&self) -> Vec<(i32, i32)> { (0..self.h).flat_map(|dy| (0..self.w).map(move |dx| (self.x + dx, self.y + dy))).collect() }
    pub fn center(&self) -> (i32, i32) { (self.x + self.w / 2, self.y + self.h / 2) }
}
#[derive(Clone, Debug, Default)]
pub struct Permits {
    pub asks: Vec<LandAsk>,
    pub fields: Vec<LandField>,
    /// ที่ที่ไม่ให้: (x0,y0,x1,y1, วันที่)
    pub no: Vec<(i32, i32, i32, i32, i32)>,
    /// กำลังเลือกที่ให้ใคร (โหมดวาง)
    pub pick: Option<u32>,
    /// ชาวไร่ที่หาที่ทำไร่ใกล้บ้านไม่ได้ (เวลาที่ลองล่าสุด) — ไม่ค้นซ้ำถี่ๆ + บอกผู้เล่น
    pub nospot: HashMap<u32, f32>,
}

pub fn farm_cells(at: (i32, i32)) -> Vec<(i32, i32)> { LandField { by: 0, x: at.0, y: at.1, w: FIELD_W, h: FIELD_H }.cells() }
pub fn piece_permit() -> usize { PIECES.iter().position(|p| p.obj == OBJ_PERMIT).unwrap() }
fn dir_word(dx: i32, dy: i32) -> &'static str {
    if dx.abs() > dy.abs() * 2 { if dx > 0 { "ทางขวา" } else { "ทางซ้าย" } }
    else if dy.abs() > dx.abs() * 2 { if dy > 0 { "ด้านล่าง" } else { "ด้านบน" } }
    else { match (dx > 0, dy > 0) { (true, true) => "ขวาล่าง", (true, false) => "ขวาบน", (false, true) => "ซ้ายล่าง", _ => "ซ้ายบน" } }
}

impl Game {
    pub fn ask_of(&self, id: u32) -> Option<&LandAsk> { self.rs.pm.asks.iter().find(|a| a.by == id) }
    pub fn has_ask(&self, id: u32) -> bool { self.ask_of(id).is_some() }
    pub fn area_cells(&self, kind: u8, at: (i32, i32), dir: u8) -> Vec<(i32, i32)> {
        if kind == AK_FARM { farm_cells(at) } else { self.prefab_cells(kind, at, dir).into_iter().map(|c| c.0).collect() }
    }
    /// ช่องนี้จองไว้แล้ว: ผืนไร่ที่อนุญาต · ที่ที่คนอื่นขออยู่ · ที่ที่เราเพิ่งไม่ให้ (นับเฉพาะของคนอื่น ถ้าใส่ except)
    pub fn land_taken(&self, cells: &[(i32, i32)], except: Option<u32>) -> Option<&'static str> {
        let day = day_of(self);
        for &(x, y) in cells {
            if self.rs.pm.fields.iter().any(|f| x >= f.x && x < f.x + f.w && y >= f.y && y < f.y + f.h) { return Some("ทับไร่ที่อนุญาตไปแล้ว"); }
            if self.rs.pm.asks.iter().filter(|a| Some(a.by) != except).any(|a| self.area_cells(a.kind, a.at, a.dir).contains(&(x, y))) { return Some("ทับที่ที่คนอื่นขออยู่"); }
            if self.rs.pm.no.iter().any(|n| day - n.4 < DENY_DAYS && x >= n.0 && x <= n.2 && y >= n.1 && y <= n.3) { return Some("ที่นี่เพิ่งไม่อนุญาตไป"); }
        }
        None
    }
    fn field_cell_ok(&self, k: (i32, i32)) -> bool {
        self.can_till(k.0, k.1) && !self.crops.contains_key(&k) && !self.w.placed.contains_key(&k) && self.room_of.get(&k).is_none() && !self.site_reserved(k)
            && self.w.realm.city_at(k.0, k.1).map_or(true, |ci| self.city_owner(ci) == PLAYER_OWN) && self.bed != Some(k)
    }
    /// ทำไร่ 3x2 ตรงนี้ได้ไหม (None = ได้)
    pub fn farm_blocked(&self, at: (i32, i32), except: Option<u32>) -> Option<&'static str> {
        if self.w.dun.is_some() { return Some("ทำไร่ในดันเจี้ยนไม่ได้"); }
        let cells = farm_cells(at);
        for &k in &cells {
            if self.w.realm.city_at(k.0, k.1).map_or(false, |ci| self.city_owner(ci) != PLAYER_OWN) { return Some("ทำไร่ในเมืองของอาณาจักรอื่นไม่ได้"); }
            if SOLIDT[self.w.tile_at(k.0, k.1) as usize] { return Some("ติดน้ำ/ภูเขา — หาที่โล่งกว่านี้"); }
            if !self.tillable_ground(k.0, k.1) && !matches!(self.w.tile_at(k.0, k.1), T_SOIL | T_WET) { return Some("ดินตรงนี้ไถไม่ได้ (ทราย/หิมะ/หิน) — ทรายต้องอยู่ริมน้ำ"); }
            if !self.field_cell_ok(k) { return Some("มีของ/ต้นไม้/บ้านขวางอยู่"); }
        }
        self.land_taken(&cells, except)
    }
    /// หาผืนไร่ว่างใกล้บ้าน (ให้ชาวไร่ขอ)
    pub fn find_field(&self, bed: (i32, i32)) -> Option<(i32, i32)> {
        for rr in 2..=FIELD_R - 3 {
            for dy in -rr..=rr { for dx in -rr..=rr {
                if dx.abs().max(dy.abs()) != rr { continue; }
                let at = (bed.0 + dx, bed.1 + dy);
                // ไม่เอาช่องที่ชิดเตียงเกินไป
                if farm_cells(at).iter().any(|c| (c.0 - bed.0).abs() + (c.1 - bed.1).abs() < 3) { continue; }
                if self.farm_blocked(at, None).is_none() { return Some(at); }
            } }
        }
        None
    }
    /// ช่องว่างในผืนไร่ที่อนุญาตแล้ว (ใกล้บ้านชาวไร่)
    pub fn field_free_cell(&self, bed: (i32, i32), taken: &dyn Fn((i32, i32)) -> bool) -> Option<(i32, i32)> {
        self.rs.pm.fields.iter()
            .filter(|f| { let c = f.center(); (c.0 - bed.0).abs() <= FIELD_R + 2 && (c.1 - bed.1).abs() <= FIELD_R + 2 })
            .flat_map(|f| f.cells())
            .filter(|&k| self.field_cell_ok(k) && !taken(k))
            .min_by_key(|k| (k.0 - bed.0).abs() + (k.1 - bed.1).abs())
    }
    /// ชาวไร่ที่ไร่ไม่พอและไม่มีที่ว่างในผืนเดิม → ขอที่ใหม่ (ทีละคำขอไร่ทั้งเมือง)
    pub fn farm_ask_maybe(&mut self, id: u32) {
        let Some(i) = self.idx(id) else { return };
        if self.npcs[i].role != 1 || self.has_ask(id) || self.rs.pm.asks.iter().any(|a| a.kind == AK_FARM) { return; }
        let bed = self.npcs[i].bed;
        let near = |a: (i32, i32), d: i32| (a.0 - bed.0).abs() <= d && (a.1 - bed.1).abs() <= d;
        let mine = self.crops.keys().filter(|k| near(**k, 14)).count();
        let farmers = self.npcs.iter().filter(|m| m.state == ST_HOME && m.role == 1 && near(m.bed, 14)).count().max(1);
        if mine >= PLOTS_PER_FARMER * farmers { return; }
        if self.field_free_cell(bed, &|_| false).is_some() { return; }
        if self.rs.pm.nospot.get(&id).map_or(false, |&t| self.time - t < 30.) { return; }
        let Some(at) = self.find_field(bed) else { self.rs.pm.nospot.insert(id, self.time); return };
        self.rs.pm.nospot.remove(&id);
        let nm = NPC_NAMES[self.npcs[i].name as usize];
        self.news_add(format!("{}อยากขอที่ทำไร่ใหม่ 6 แปลง", nm));
        self.rs.pm.asks.push(LandAsk { by: id, kind: AK_FARM, at, dir: 0, came: false, snooze: 0. });
    }
    /// บ้านที่ชาวบ้านคิดจะสร้าง → ขอที่ก่อน
    pub fn house_ask(&mut self, id: u32, kind: u8, at: (i32, i32), dir: u8) {
        self.rs.pm.asks.retain(|a| a.by != id);
        self.rs.pm.asks.push(LandAsk { by: id, kind, at, dir, came: false, snooze: 0. });
    }
    pub fn ask_what(&self, a: &LandAsk) -> String {
        if a.kind == AK_FARM { "ขอใช้ที่ทำไร่ 6 แปลง".to_string() } else { format!("ขอที่สร้าง{}ของตัวเอง ({})", prefab_name(a.kind), DIR_NAMES[a.dir as usize & 3]) }
    }
    /// บอกทางจากตัวเราไปที่ที่ขอ
    pub fn ask_where(&self, a: &LandAsk) -> String {
        let cs = self.area_cells(a.kind, a.at, a.dir);
        let n = cs.len().max(1) as i32;
        let c = (cs.iter().map(|c| c.0).sum::<i32>() / n, cs.iter().map(|c| c.1).sum::<i32>() / n);
        let (dx, dy) = (c.0 - tfloor(self.p.x), c.1 - tfloor(self.p.y));
        let d = dx.abs().max(dy.abs());
        if d <= 3 { "ตรงนี้เลย (กรอบเหลือง)".to_string() } else { format!("{} {} ช่อง (กรอบเหลือง)", dir_word(dx, dy), d) }
    }
    fn ask_done(&mut self, id: u32) {
        self.rs.pm.asks.retain(|a| a.by != id);
        if self.rs.work.get(&id).map_or(false, |j| j.kind == J_ASK) { self.rs.work.remove(&id); }
        if self.rs.pm.pick == Some(id) { self.rs.pm.pick = None; if self.build_sel == Some(piece_permit()) { self.build_sel = None; } }
    }
    fn grant(&mut self, a: LandAsk) {
        let id = a.by;
        let Some(i) = self.idx(id) else { self.ask_done(id); return };
        let nm = NPC_NAMES[self.npcs[i].name as usize];
        self.npcs[i].mood = (self.npcs[i].mood + 4).min(100);
        if a.kind == AK_FARM {
            self.rs.pm.fields.push(LandField { by: id, x: a.at.0, y: a.at.1, w: FIELD_W, h: FIELD_H });
            self.npc_bubble(id, "ขอบคุณ! เดี๋ยวไปไถเลย");
            self.toast(format!("อนุญาตแล้ว — {}จะทำไร่ตรงนั้น", nm));
        } else {
            for p in self.rs.projects.iter_mut().filter(|p| p.by == id && !p.started) { p.ok = true; p.at = a.at; p.dir = a.dir; }
            self.npc_bubble(id, "ขอบคุณ! จะสร้างให้สวยเลย");
            self.toast(format!("อนุญาตแล้ว — {}จะสร้าง{}ตรงนั้น (ถ้าวัสดุพอจะเริ่มเลย)", nm, prefab_name(a.kind)));
            self.ask_done(id);
            self.projects_try_start();
            play(&self.sfx.click);
            return;
        }
        self.ask_done(id);
        play(&self.sfx.click);
    }
    /// ให้ตรงที่เขาขอ
    pub fn permit_yes(&mut self, id: u32) {
        let Some(mut a) = self.ask_of(id).cloned() else { return };
        // v34 ประตูถูกบังระหว่างรอ → หมุนบ้านหาทิศที่โล่งให้เอง
        if a.kind != AK_FARM { if let Some(d) = self.door_dir_for(a.kind, a.at, a.dir) { a.dir = d; } }
        // ระหว่างรอ อาจมีของมาขวางแล้ว
        let bad = if a.kind == AK_FARM { self.farm_blocked(a.at, Some(id)) } else { self.prefab_blocked(a.kind, a.at, a.dir).or(self.land_taken(&self.area_cells(a.kind, a.at, a.dir), Some(id))) };
        if let Some(why) = bad { self.toast(format!("ตรงนั้นใช้ไม่ได้แล้ว: {} — กด \"ที่อื่น\" เพื่อเลือกให้ใหม่", why)); return; }
        self.grant(a);
    }
    /// ไม่ให้: จำที่นั้นไว้ไม่ขอซ้ำ · บ้าน = ยกเลิกโครงการ (พรุ่งนี้อาจหาที่ใหม่)
    pub fn permit_no(&mut self, id: u32) {
        let Some(a) = self.ask_of(id).cloned() else { return };
        let cs = self.area_cells(a.kind, a.at, a.dir);
        let (x0, y0) = (cs.iter().map(|c| c.0).min().unwrap_or(0), cs.iter().map(|c| c.1).min().unwrap_or(0));
        let (x1, y1) = (cs.iter().map(|c| c.0).max().unwrap_or(0), cs.iter().map(|c| c.1).max().unwrap_or(0));
        let day = day_of(self);
        self.rs.pm.no.retain(|n| day - n.4 < DENY_DAYS);
        self.rs.pm.no.push((x0, y0, x1, y1, day));
        if a.kind != AK_FARM { self.rs.projects.retain(|p| !(p.by == id && !p.started)); self.rs.help.remove(&id); }
        if let Some(i) = self.idx(id) {
            self.npcs[i].mood = (self.npcs[i].mood - 3).max(0);
            self.npc_bubble(id, "โอเค ไว้หาที่อื่นแล้วกัน");
        }
        self.ask_done(id);
        self.toast("ไม่อนุญาต — เขาจะหาที่อื่นแล้วค่อยมาถามใหม่");
    }
    /// ให้ที่อื่น: เข้าโหมดวางเพื่อเลือกที่เอง
    pub fn permit_pick_start(&mut self, id: u32) {
        let Some(a) = self.ask_of(id).cloned() else { return };
        self.rs.pm.pick = Some(id);
        self.build_sel = Some(piece_permit());
        self.build_cur = None;
        self.build_dir = a.dir;
        self.rs.build_arm = None;
        if self.rs.work.get(&id).map_or(false, |j| j.kind == J_ASK) { self.rs.work.remove(&id); }
        let nm = self.idx(id).map(|i| NPC_NAMES[self.npcs[i].name as usize]).unwrap_or("");
        self.toast(format!("เลือกที่ให้{}: แตะพื้น 2 ครั้ง = ให้ตรงนั้น", nm));
    }
    pub fn permit_pick_cancel(&mut self) -> bool {
        if self.rs.pm.pick.take().is_none() { return false; }
        self.build_sel = None;
        self.rs.build_arm = None;
        self.toast("ยังไม่ได้ให้ที่ — ตอบทีหลังได้ที่หน้าของเขาในแท็บชุมชน");
        true
    }
    /// พรีวิวที่เลือก: (ช่อง, เหตุผลที่ใช้ไม่ได้)
    pub fn permit_preview(&self, t: (i32, i32)) -> Option<(Vec<(i32, i32)>, Option<String>)> {
        let id = self.rs.pm.pick?;
        let a = self.ask_of(id)?;
        let i = self.idx(id)?;
        let bed = self.npcs[i].bed;
        let cells = self.area_cells(a.kind, t, self.build_dir);
        let bad: Option<String> = if a.kind == AK_FARM {
            if farm_cells(t).iter().any(|c| (c.0 - bed.0).abs() > FIELD_R + 2 || (c.1 - bed.1).abs() > FIELD_R + 2) { Some(format!("ไกลบ้านของเขาเกินไป (ไม่เกิน {} ช่องจากเตียง)", FIELD_R)) }
            else { self.farm_blocked(t, Some(id)).map(|s| s.to_string()) }
        } else { self.prefab_blocked(a.kind, t, self.build_dir).or(self.land_taken(&cells, Some(id))).map(|s| s.to_string()) };
        Some((cells, bad))
    }
    /// วางที่ที่เลือก (แตะยืนยัน / ปุ่มสร้าง)
    pub fn permit_place(&mut self, t: (i32, i32)) -> bool {
        let Some(id) = self.rs.pm.pick else { return false };
        let Some((_, bad)) = self.permit_preview(t) else { self.permit_pick_cancel(); return false };
        if let Some(why) = bad { self.toast(why); return false; }
        let Some(mut a) = self.ask_of(id).cloned() else { return false };
        a.at = t;
        a.dir = self.build_dir;
        self.rs.pm.pick = None;
        self.build_sel = None;
        self.grant(a);
        true
    }
    /// คนที่กำลังยืนถามอยู่ตรงหน้า (โชว์การ์ด)
    pub fn permit_card(&self) -> Option<&LandAsk> {
        if self.rs.pm.pick.is_some() || self.build_sel.is_some() || self.big_map || self.w.dun.is_some() || war::in_combat(self) || self.rs.crew.yld.is_some() { return None; }
        self.rs.pm.asks.iter().find(|a| a.came && self.npcs.iter().any(|n| n.id == a.by && (n.x - self.p.x).hypot(n.y - self.p.y) < ASK_NEAR))
    }
    /// ล้างคำขอที่ไม่ใช้แล้ว · เลือกคนที่จะเดินมาถาม
    pub fn permit_tick(&mut self) {
        let alive: Vec<(u32, u8, bool)> = self.npcs.iter().map(|n| (n.id, n.role, n.state == ST_HOME && !n.child)).collect();
        let projects: Vec<u32> = self.rs.projects.iter().filter(|p| !p.started && !p.ok).map(|p| p.by).collect();
        self.rs.pm.asks.retain(|a| alive.iter().any(|&(id, r, home)| id == a.by && home && if a.kind == AK_FARM { r == 1 } else { projects.contains(&a.by) }));
        if let Some(id) = self.rs.pm.pick { if !self.has_ask(id) || self.build_sel != Some(piece_permit()) { self.rs.pm.pick = None; if self.build_sel == Some(piece_permit()) { self.build_sel = None; } } }
        let ids: Vec<u32> = self.rs.pm.asks.iter().map(|a| a.by).collect();
        self.rs.work.retain(|id, j| j.kind != J_ASK || ids.contains(id));
    }
    /// ใครควรเดินมาหาเราตอนนี้ (ทีละคน)
    pub fn permit_next_asker(&self, free: &[u32]) -> Option<u32> {
        if self.rs.pm.pick.is_some() || self.w.dun.is_some() { return None; }
        if self.rs.work.values().any(|j| j.kind == J_ASK) { return None; }
        self.rs.pm.asks.iter().find(|a| a.snooze <= self.time && free.contains(&a.by)).map(|a| a.by)
    }
    /// เดินมาหาเรา → ยืนรอคำตอบ (คืนจุดเดิน + ความเร็ว)
    pub fn ask_step(&mut self, n: &mut npc::Npc, dt: f32) -> Option<((f32, f32), f32)> {
        let (px, py) = (self.p.x, self.p.y);
        let side = if n.x < px { -18. } else { 18. };
        let tgt = (px + side, py + 2.);
        let d = (n.x - tgt.0).hypot(n.y - tgt.1);
        let j = self.rs.work.get_mut(&n.id)?;
        if !j.working && d > 10. { return Some((tgt, 52.)); }
        if j.working && (n.x - px).hypot(n.y - py) > ASK_NEAR - 8. { j.working = false; return Some((tgt, 52.)); }
        j.working = true;
        j.t += dt;
        let give_up = j.t > ASK_WAIT;
        n.moving = false;
        n.left = px < n.x;
        let time = self.time;
        if let Some(a) = self.rs.pm.asks.iter_mut().find(|a| a.by == n.id) {
            if !a.came { a.came = true; }
            if give_up { a.came = false; a.snooze = time + ASK_SNOOZE; }
        }
        if give_up { self.rs.work.remove(&n.id); self.npc_bubble(n.id, "ไว้ว่างๆ ค่อยตอบนะ"); }
        Some(((n.x, n.y), 0.))
    }

    pub fn permit_save(&self) -> String {
        let mut s = "permit".to_string();
        for f in &self.rs.pm.fields { s += &format!(" f{},{},{},{},{}", f.by, f.x, f.y, f.w, f.h); }
        s + "\n"
    }
    pub fn permit_load(&mut self, r: &[&str]) {
        self.rs.pm.fields.clear();
        for x in r.iter().filter(|x| x.starts_with('f')) {
            let v: Vec<i64> = x[1..].split(',').filter_map(|a| a.parse().ok()).collect();
            if v.len() == 5 { self.rs.pm.fields.push(LandField { by: v[0] as u32, x: v[1] as i32, y: v[2] as i32, w: v[3] as i32, h: v[4] as i32 }); }
        }
    }
}

// ---------------------------------------------------------------- วาด
fn dash_rect(v: &View, cells: &[(i32, i32)], c: Color, t: f32) {
    if cells.is_empty() { return; }
    let (x0, y0) = (cells.iter().map(|c| c.0).min().unwrap(), cells.iter().map(|c| c.1).min().unwrap());
    let (x1, y1) = (cells.iter().map(|c| c.0).max().unwrap() + 1, cells.iter().map(|c| c.1).max().unwrap() + 1);
    let (ax, ay, bx, by) = ((x0 as f32 * 16. - v.cx) * v.s, (y0 as f32 * 16. - v.cy) * v.s, (x1 as f32 * 16. - v.cx) * v.s, (y1 as f32 * 16. - v.cy) * v.s);
    let seg = 6. * v.s;
    let off = (t * 12. * v.s) % (seg * 2.);
    let th = (1.5 * v.s).max(2.);
    let mut line = |x0: f32, y0: f32, x1: f32, y1: f32| {
        let l = (x1 - x0).hypot(y1 - y0);
        let (ux, uy) = ((x1 - x0) / l.max(0.01), (y1 - y0) / l.max(0.01));
        let mut s = -off;
        while s < l { let a = s.max(0.); let b = (s + seg).min(l); if b > a { draw_line(x0 + ux * a, y0 + uy * a, x0 + ux * b, y0 + uy * b, th, c); } s += seg * 2.; }
    };
    line(ax, ay, bx, ay); line(bx, ay, bx, by); line(bx, by, ax, by); line(ax, by, ax, ay);
}
/// กรอบที่ที่ขออยู่ (เหลือง) + หมุดมุมผืนไร่ที่อนุญาตแล้วแต่ยังไม่ได้ปลูก
pub fn draw_permit_world(g: &Game, v: &View, font: &Font, t: f32) {
    if g.w.dun.is_some() { return; }
    let pulse = 0.65 + (t * 4.).sin() * 0.25;
    for a in &g.rs.pm.asks {
        if g.rs.pm.pick == Some(a.by) { continue; }
        let cs = g.area_cells(a.kind, a.at, a.dir);
        for c in &cs { draw_rectangle((c.0 as f32 * 16. - v.cx) * v.s, (c.1 as f32 * 16. - v.cy) * v.s, 16. * v.s, 16. * v.s, Color::new(1., 0.9, 0.3, 0.12)); }
        dash_rect(v, &cs, Color::new(1., 0.88, 0.3, pulse), t);
        let nm = g.idx(a.by).map(|i| NPC_NAMES[g.npcs[i].name as usize]).unwrap_or("");
        let x0 = cs.iter().map(|c| c.0).min().unwrap_or(0);
        let y0 = cs.iter().map(|c| c.1).min().unwrap_or(0);
        let lbl = format!("{}ขอ{}", nm, if a.kind == AK_FARM { "ทำไร่" } else { "สร้างบ้าน" });
        let fs = (5.5 * v.s).max(11.);
        let (lx, ly) = ((x0 as f32 * 16. - v.cx) * v.s, (y0 as f32 * 16. - v.cy) * v.s - 4.);
        let d = measure_text(&lbl, Some(font), fs as u16, 1.);
        draw_rectangle(lx, ly - fs, d.width + 8., fs + 4., Color::new(0., 0., 0., 0.6));
        text(font, &lbl, lx + 4., ly - 2., fs, hx(0xffe066));
    }
    for f in &g.rs.pm.fields {
        if f.cells().iter().all(|k| g.crops.contains_key(k)) { continue; }
        let stake = Color::new(0.55, 0.38, 0.2, 0.85);
        for (x, y) in [(f.x, f.y), (f.x + f.w, f.y), (f.x, f.y + f.h), (f.x + f.w, f.y + f.h)] {
            let (sx, sy) = (x as f32 * 16. - v.cx, y as f32 * 16. - v.cy);
            prect(v, sx - 1., sy - 5., 2., 6., stake);
        }
    }
}
/// โหมดเลือกที่ให้: พรีวิวเขียว/แดง ตามจุดที่แตะ
pub fn draw_permit_preview(g: &Game, v: &View, font: &Font, t: f32) {
    let Some((cells, bad)) = g.permit_preview(g.build_target()) else { return };
    let col = if bad.is_none() { Color::new(0.45, 1., 0.5, 0.32) } else { Color::new(1., 0.4, 0.4, 0.32) };
    for c in &cells { draw_rectangle((c.0 as f32 * 16. - v.cx) * v.s, (c.1 as f32 * 16. - v.cy) * v.s, 16. * v.s, 16. * v.s, col); }
    dash_rect(v, &cells, if bad.is_none() { hx(0x7dff8a) } else { hx(0xff6a6a) }, t);
    let _ = (font, bad);
}
pub fn pick_line(g: &Game, touch: bool) -> Option<(String, bool)> {
    let id = g.rs.pm.pick?;
    let a = g.ask_of(id)?;
    let nm = g.idx(id).map(|i| NPC_NAMES[g.npcs[i].name as usize]).unwrap_or("");
    // ตรงที่ชี้อยู่ใช้ไม่ได้ → บอกเหตุผลแทน (บรรทัดเดียว ไม่ทับพื้นที่)
    if let Some(why) = g.permit_preview(g.build_target()).and_then(|p| p.1) { return Some((format!("ตรงนี้ไม่ได้: {} · {}", why, if touch { "ปุ่มแดง = ยังไม่ให้" } else { "T = ยังไม่ให้" }), false)); }
    Some((format!("เลือกที่{}ให้{}: แตะพื้น 2 ครั้ง = ให้ตรงนั้น{} · {}", if a.kind == AK_FARM { "ทำไร่" } else { "สร้างบ้าน" }, nm, if a.kind != AK_FARM { " · หมุน = หันประตู" } else { "" }, if touch { "ปุ่มแดง = ยังไม่ให้" } else { "T = ยังไม่ให้" }), true))
}

/// ตำแหน่งการ์ด: ช่องเดียวกับแถบคำแนะนำ (การ์ดขึ้น = คำแนะนำหลบ) → (กรอบ, [ให้เลย, ที่อื่น, ไม่ให้])
pub fn card_rects_for(l: &hud::Layout, u: f32, top_h: f32) -> (Rect, [Rect; 3]) {
    let w = l.col_w.min(600. * u);
    let x = l.col_x + l.col_w / 2. - w / 2.;
    let y = 8. * u + if top_h > 0. { top_h + 6. * u } else { 0. };
    if l.col_w >= 460. * u {
        let (h, bw, bh, gap) = (60. * u, 70. * u, 36. * u, 6. * u);
        let bx = x + w - 8. * u - bw * 3. - gap * 2.;
        let r = |i: usize| Rect::new(bx + i as f32 * (bw + gap), y + (h - bh) / 2., bw, bh);
        (Rect::new(x, y, w, h), [r(0), r(1), r(2)])
    } else {
        let (h, bh, gap) = (104. * u, 34. * u, 6. * u);
        let bw = (w - 16. * u - gap * 2.) / 3.;
        let r = |i: usize| Rect::new(x + 8. * u + i as f32 * (bw + gap), y + h - bh - 8. * u, bw, bh);
        (Rect::new(x, y, w, h), [r(0), r(1), r(2)])
    }
}
pub fn card_rects(g: &Game) -> Option<(u32, Rect, [Rect; 3])> {
    let a = g.permit_card()?;
    let u = ui_scale();
    let (r, b) = card_rects_for(&hud::layout(), u, hud::top_banner_h(g, u));
    Some((a.by, r, b))
}
/// แตะการ์ด: true = ใช้แตะนี้ไปแล้ว
pub fn card_tap(g: &mut Game, tp: Vec2) -> bool {
    let Some((id, r, b)) = card_rects(g) else { return false };
    if b[0].contains(tp) { g.permit_yes(id); }
    else if b[1].contains(tp) { g.permit_pick_start(id); }
    else if b[2].contains(tp) { g.permit_no(id); }
    else if !r.contains(tp) { return false; }
    true
}
pub fn draw_permit_card(g: &Game, v: &View, font: &Font, t: f32) {
    let Some((id, r, b)) = card_rects(g) else { return };
    let Some(a) = g.ask_of(id) else { return };
    let Some(i) = g.idx(id) else { return };
    let n = &g.npcs[i];
    let u = ui_scale();
    let gold = hx(0xffd84a);
    draw_rectangle(r.x, r.y, r.w, r.h, Color::new(0.06, 0.06, 0.1, 0.92));
    draw_rectangle(r.x, r.y, 5. * u, r.h, gold);
    draw_rectangle_lines(r.x, r.y, r.w, r.h, 2., Color::new(1., 0.85, 0.3, 0.9));
    let wide = b[0].y > r.y + 4. * u && b[0].y + b[0].h < r.y + r.h - 4. * u && b[0].x > r.x + r.w * 0.4;
    let tw = if wide { b[0].x - r.x - 20. * u } else { r.w - 22. * u };
    text(font, &format!("{} ({}) มาขออนุญาต", NPC_NAMES[n.name as usize], ROLE_N[n.role as usize]), r.x + 12. * u, r.y + 16. * u, 12. * u, gold);
    let body = format!("{} — {}", g.ask_what(a), g.ask_where(a));
    let ws = wrap_th(font, &body, 12. * u, tw);
    let maxl = if wide { 2 } else { 3 };
    for (k, l) in ws.iter().take(maxl).enumerate() { text(font, l, r.x + 12. * u, r.y + 33. * u + k as f32 * 14. * u, 12. * u, WHITE); }
    let labels = ["ให้เลย", "ที่อื่น", "ไม่ให้"];
    let cols = [hx(0x2f7a3a), hx(0x2f4f7a), hx(0x7a2f2f)];
    for k in 0..3 {
        let q = b[k];
        draw_rectangle(q.x, q.y, q.w, q.h, cols[k]);
        draw_rectangle_lines(q.x, q.y, q.w, q.h, 2., Color::new(1., 1., 1., 0.55));
        text_c(font, labels[k], q.x + q.w / 2., q.y + q.h * 0.66, 13. * u, WHITE);
    }
    // ลูกศรชี้ไปที่ที่ขอ ถ้าอยู่นอกจอ
    let cs = g.area_cells(a.kind, a.at, a.dir);
    let nn = cs.len().max(1) as f32;
    let (wx, wy) = (cs.iter().map(|c| c.0 as f32 * 16. + 8.).sum::<f32>() / nn, cs.iter().map(|c| c.1 as f32 * 16. + 8.).sum::<f32>() / nn);
    let (sx, sy) = ((wx - v.cx) * v.s, (wy - v.cy) * v.s);
    // พื้นที่ที่มองเห็นจริง = ใต้การ์ดลงไป (ถ้าที่ที่ขอโดนการ์ด/แถบบนบัง ก็ชี้ลูกศรให้ด้วย)
    let (sw, sh) = (screen_width(), screen_height());
    let top = r.y + r.h;
    if sx < 0. || sy < top || sx > sw || sy > sh {
        let (cx, cy) = (sw / 2., (top + sh) / 2.);
        let (dx, dy) = (sx - cx, sy - cy);
        let l = dx.hypot(dy).max(1.);
        let (ux, uy) = (dx / l, dy / l);
        let k = ((sw / 2. - 40. * u) / ux.abs().max(0.001)).min(((sh - top) / 2. - 30. * u) / uy.abs().max(0.001));
        let (ax, ay) = (cx + ux * k, cy + uy * k);
        let s = 20. * u * (1. + (t * 5.).sin() * 0.12);
        draw_triangle(vec2(ax + ux * s, ay + uy * s), vec2(ax - uy * s * 0.6, ay + ux * s * 0.6), vec2(ax + uy * s * 0.6, ay - ux * s * 0.6), gold);
    }
}

#[cfg(test)]
mod tests_v30 {
    use super::*;
    fn mk(id: u32, role: u8, bed: (i32, i32)) -> npc::Npc {
        let mut n = npc::Npc { id, name: (id % 12) as u8, look: 0, x: bed.0 as f32 * 16. + 8., y: bed.1 as f32 * 16. + 18., state: ST_HOME, hearts: 3, mood: 70, married: false, child: false, role, bed, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(id, (id % 12) as u8) };
        n.soc.anger = 0; n.soc.grudge = 0;
        n
    }
    fn setup(g: &mut Game) -> (i32, i32) {
        let (sx, sy) = (tfloor(g.spawn.0) + 80, tfloor(g.spawn.1) + 80);
        for y in sy - 22..=sy + 22 { for x in sx - 22..=sx + 22 { g.w.tmod.insert((x, y), 3); g.w.removed.insert((x, y)); } }
        g.w.placed.insert((sx, sy), 33);
        g.w.placed.insert((sx - 2, sy), 14);
        g.chests.insert((sx - 2, sy), [0; NI]);
        g.bed = Some((sx + 40, sy));
        g.p.x = sx as f32 * 16.; g.p.y = (sy + 6) as f32 * 16.;
        g.tut = TUT_DONE;
        while !work_hr(g.hour()) || g.hour() > 9. { g.time += 3.; }
        g.last_day = day_of(g);
        (sx, sy)
    }
    fn run(g: &mut Game, secs: f32) { let dt = 1. / 30.; let mut t = 0.; while t < secs { g.npc_tick(dt); g.work_tick(dt); g.upd_crops(dt); t += dt; } }
    fn farmer(g: &mut Game) -> (i32, i32) {
        let (sx, sy) = setup(g);
        g.npcs.push(mk(1, 1, (sx, sy)));
        g.rs.manual.insert(1, 1);
        (sx, sy)
    }

    #[test]
    fn farmer_comes_to_ask_before_tilling_then_farms_only_the_granted_field() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let (sx, sy) = farmer(&mut g);
        run(&mut g, 20.);
        assert!(g.crops.is_empty(), "ยังไม่ได้อนุญาต = ยังไม่ไถ");
        let a = g.ask_of(1).cloned().expect("ชาวไร่ขอที่");
        assert_eq!(a.kind, AK_FARM);
        assert!(a.came, "เดินมาถามถึงตัวเรา");
        let n = &g.npcs[0];
        assert!((n.x - g.p.x).hypot(n.y - g.p.y) < ASK_NEAR, "ยืนอยู่ข้างเรา");
        assert!(g.permit_card().is_some(), "การ์ดถามขึ้น");
        g.permit_yes(1);
        assert!(!g.has_ask(1) && g.rs.pm.fields.len() == 1);
        assert!(g.permit_card().is_none());
        run(&mut g, 60.);
        let f = g.rs.pm.fields[0];
        assert!(g.crops.len() >= 4, "ไถในผืนที่ให้ {}", g.crops.len());
        assert!(g.crops.keys().all(|k| f.cells().contains(k)), "ไถเฉพาะในผืนที่อนุญาต");
        let _ = (sx, sy);
    }
    #[test]
    fn deny_remembers_spot_and_next_ask_is_elsewhere() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        farmer(&mut g);
        run(&mut g, 20.);
        let first = g.ask_of(1).cloned().unwrap();
        let m0 = g.npcs[0].mood;
        g.permit_no(1);
        assert!(!g.has_ask(1) && g.crops.is_empty());
        assert!(g.npcs[0].mood < m0);
        run(&mut g, 20.);
        let second = g.ask_of(1).cloned().expect("ขอใหม่");
        let c1 = farm_cells(first.at);
        assert!(!farm_cells(second.at).iter().any(|c| c1.contains(c)), "ไม่ขอที่เดิมซ้ำ");
        assert!(g.crops.is_empty());
    }
    #[test]
    fn give_elsewhere_with_pick_mode() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let (sx, sy) = farmer(&mut g);
        run(&mut g, 20.);
        g.permit_pick_start(1);
        assert_eq!(g.build_sel, Some(piece_permit()));
        assert!(g.permit_card().is_none(), "เลือกที่อยู่ การ์ดหลบ");
        // ทับเตียง = ไม่ได้
        assert!(!g.permit_place((sx - 1, sy)));
        assert!(g.has_ask(1));
        // ไกลบ้านเกินไป = ไม่ได้
        assert!(!g.permit_place((sx + 19, sy)));
        let spot = (sx - 6, sy + 5);
        assert!(g.permit_place(spot), "ที่ที่เลือกเอง");
        assert_eq!((g.rs.pm.fields[0].x, g.rs.pm.fields[0].y), spot);
        assert!(g.build_sel.is_none() && g.rs.pm.pick.is_none());
        // ยกเลิกโหมดเลือกที่ = คำขอยังอยู่
        let mut h = Game::new_world_seed(4242, Sfx::default());
        farmer(&mut h);
        run(&mut h, 20.);
        h.permit_pick_start(1);
        assert!(h.permit_pick_cancel());
        assert!(h.has_ask(1) && h.build_sel.is_none());
    }
    #[test]
    fn ignored_asker_goes_back_to_work_and_asks_later() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        farmer(&mut g);
        run(&mut g, 20. + ASK_WAIT + 2.);
        let a = g.ask_of(1).cloned().unwrap();
        assert!(!a.came && a.snooze > g.time, "ไม่ตอบ = กลับไปก่อน");
        assert!(g.rs.work.get(&1).map_or(true, |j| j.kind != J_ASK));
    }
    #[test]
    fn villager_house_needs_land_permission_before_materials_and_building() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let (sx, sy) = setup(&mut g);
        let keys: Vec<(i32, i32)> = g.w.tmod.keys().copied().collect();
        for k in keys { let t = g.w.gen_tile(k.0, k.1); if !SOLIDT[t as usize] && t != 6 { g.w.tmod.remove(&k); } }
        g.w.chunks.borrow_mut().clear();
        g.npcs.push(mk(3, 0, (sx, sy)));
        for it in [WOOD, STONE, BERRY, GEM] { g.chests.get_mut(&(sx - 2, sy)).unwrap()[it] = 200; }
        g.projects_dawn();
        let a = g.ask_of(3).cloned().expect("ขอที่สร้างบ้าน");
        assert!(is_prefab(a.kind));
        assert!(g.sites.is_empty() && !g.rs.help.contains_key(&3), "ยังไม่ได้ที่ = ยังไม่สร้าง/ยังไม่ขอของ");
        g.permit_yes(3);
        assert!(!g.sites.is_empty(), "ได้ที่ + มีของ = เริ่มสร้าง {:?} {:?} {:?}", g.toasts.iter().map(|t| t.0.clone()).collect::<Vec<_>>(), g.rs.projects.iter().map(|p| (p.ok, p.started)).collect::<Vec<_>>(), g.rs.help);
        assert_eq!(g.sites[0].k, a.at);
        // ไม่ให้ = ยกเลิกโครงการ
        let mut h = Game::new_world_seed(4242, Sfx::default());
        let (sx, sy) = setup(&mut h);
        let keys: Vec<(i32, i32)> = h.w.tmod.keys().copied().collect();
        for k in keys { let t = h.w.gen_tile(k.0, k.1); if !SOLIDT[t as usize] && t != 6 { h.w.tmod.remove(&k); } }
        h.w.chunks.borrow_mut().clear();
        h.npcs.push(mk(3, 0, (sx, sy)));
        h.projects_dawn();
        h.permit_no(3);
        assert!(h.rs.projects.iter().all(|p| p.by != 3) && h.sites.is_empty());
    }
    #[test]
    fn desert_town_can_farm_by_the_water_and_says_so_when_it_cannot() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let (sx, sy) = farmer(&mut g);
        for y in sy - 22..=sy + 22 { for x in sx - 22..=sx + 22 { g.w.tmod.insert((x, y), 8); } }
        g.w.chunks.borrow_mut().clear();
        assert!(!g.can_till(sx + 5, sy + 5), "ทรายไกลน้ำไถไม่ได้");
        run(&mut g, 20.);
        assert!(!g.has_ask(1) && g.rs.pm.nospot.contains_key(&1), "หาที่ไม่ได้ = จำไว้บอกผู้เล่น");
        assert!(g.advice().iter().any(|a| a.id == "nospot1"), "ที่ปรึกษาบอกว่าทำไมไม่มีไร่");
        // ขุดสระ/มีน้ำใกล้ๆ → ทรายริมน้ำไถได้
        for y in sy + 6..=sy + 7 { for x in sx - 2..=sx + 2 { g.w.tmod.insert((x, y), 1); } }
        g.w.chunks.borrow_mut().clear();
        assert!(g.can_till(sx, sy + 4), "ทรายริมน้ำไถได้");
        g.time += 31.;
        while !work_hr(g.hour()) { g.time += 3.; }
        run(&mut g, 20.);
        assert!(g.has_ask(1), "มีที่แล้วมาขอ");
    }
    #[test]
    fn fields_survive_save_load() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        g.rs.pm.fields.push(LandField { by: 4, x: 10, y: -3, w: 3, h: 2 });
        let s = g.permit_save();
        let parts: Vec<&str> = s.split_whitespace().skip(1).collect();
        let mut h = Game::new_world_seed(4242, Sfx::default());
        h.permit_load(&parts);
        assert_eq!(h.rs.pm.fields, g.rs.pm.fields);
    }
    #[test]
    fn ask_card_fits_center_column_on_all_screens() {
        for &(sw, sh) in &[(1280., 720.), (2400., 1080.), (1600., 720.), (2340., 1080.), (1920., 1200.), (1024., 600.), (2800., 1260.), (800., 450.)] {
            for touch in [true, false] {
                let u: f32 = (sh / 450f32).clamp(1., 3.);
                let t = (sh / 450.).clamp(1., 3.2);
                let l = hud::layout_for(sw, u, t, touch);
                let (r, b) = card_rects_for(&l, u, 42. * u);
                assert!(r.x >= l.col_x - 0.5 && r.x + r.w <= l.col_x + l.col_w + 0.5, "อยู่ในโซนกลาง {}x{}", sw, sh);
                assert!(!r.overlaps(&l.mm), "ไม่ทับแผนที่เล็ก");
                for k in 0..3 {
                    assert!(b[k].x >= r.x && b[k].x + b[k].w <= r.x + r.w + 0.5 && b[k].y >= r.y && b[k].y + b[k].h <= r.y + r.h + 0.5, "ปุ่มอยู่ในการ์ด");
                    assert!(b[k].w >= 44. * u.min(1.5) && b[k].h >= 30., "ปุ่มกดง่าย");
                    for q in k + 1..3 { assert!(!b[k].overlaps(&b[q]), "ปุ่มไม่ทับกัน"); }
                }
            }
        }
    }
}
