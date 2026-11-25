// psy.rs — v14: บุคลิกภาพชาวบ้านแบบอิงจิตวิทยาจริง + การนั่ง + วงสนทนารอบกองไฟ
//
// โมเดล: HEXACO (Big Five + ความซื่อสัตย์-ถ่อมตน) — โมเดลที่นักจิตวิทยาใช้วัดบุคลิกภาพจริง
//   O เปิดรับสิ่งใหม่ · C มีวินัย · E ชอบเข้าสังคม · A เป็นมิตร · N อารมณ์อ่อนไหว · H ซื่อสัตย์
//   ค่าเป็นสเปกตรัม 0–100 ไม่ใช่กล่อง · "คล้าย INFJ" เป็นแค่ป้ายเทียบเล่นๆ (MBTI/16 personalities ไม่ได้รับการยอมรับทางวิทยาศาสตร์)
//   "ด้านมืด" (แนว Dark Triad) = ซื่อสัตย์ต่ำมาก + เป็นมิตรต่ำ → เจ้าเล่ห์ เห็นแก่ตัว แอบหยิบของ
// นิสัยเปลี่ยนได้ช้าๆ ตามสิ่งที่เจอ (เหมือนคนจริง): มีบ้านดี/อิ่ม/มีเพื่อน → ใจเย็น เป็นมิตรขึ้น ·
//   โดนตี/อดอยาก/นอนกลางแจ้ง → อ่อนไหว ระแวงขึ้น · แต่มี "จุดตั้งต้น" ดึงกลับเล็กน้อย (บุคลิกพื้นฐานไม่หายไปทั้งหมด)
// แบตสังคม: อินโทรเวิร์ตหมดแรงเมื่ออยู่กับคนเยอะ ชาร์จตอนอยู่คนเดียว · เอ็กซ์โทรเวิร์ตกลับกัน
// นิสัยเดิม 5 อย่าง (ใจดี/ใจร้อน/โลภ/กล้า/ช่างพูด) ถูกคำนวณจากโมเดลนี้ทุกเช้า ระบบเก่าทั้งหมดจึงเปลี่ยนตามไปด้วย
use super::*;

pub const PO: usize = 0;
pub const PC: usize = 1;
pub const PE: usize = 2;
pub const PA: usize = 3;
pub const PN: usize = 4;
pub const PH: usize = 5;
pub const PSY_N: [&str; 6] = ["เปิดรับสิ่งใหม่", "มีวินัย", "ชอบเข้าสังคม", "เป็นมิตร", "อารมณ์อ่อนไหว", "ซื่อสัตย์"];
const PSY_UP: [&str; 6] = ["ดูอยากลองอะไรใหม่ๆ มากขึ้น", "ดูขยันและเป็นระเบียบขึ้น", "ดูชอบเข้าสังคมมากขึ้น", "ดูใจดีและเป็นมิตรขึ้น", "ดูเครียดและหงุดหงิดง่ายขึ้น", "ดูจริงใจขึ้น"];
const PSY_DOWN: [&str; 6] = ["ดูติดอยู่กับอะไรเดิมๆ มากขึ้น", "ดูเรื่อยเปื่อยขึ้น", "ดูเก็บตัวมากขึ้น", "ดูเย็นชาลง", "ดูใจเย็นและมั่นคงขึ้น", "ดูเจ้าเล่ห์ขึ้น"];

#[derive(Clone, Debug)]
pub struct Psy {
    pub t: [f32; 6],
    /// บุคลิกตอนเจอกันครั้งแรก (จุดตั้งต้น)
    pub base: [f32; 6],
    /// ค่าที่เคยประกาศข่าวไปแล้ว (เปลี่ยนเกิน 6 จากนี้ = มีข่าว)
    pub noted: [f32; 6],
    pub bat: f32,
    /// ส่วนต่างให้นิสัยเดิม (ใจร้อน/กล้า) ตรงกับค่าตอนสร้าง
    pub off_t: f32,
    pub off_b: f32,
    pub gathered_day: i32,
    pub chat_day: i32,
    // ไม่เซฟ
    pub bubble: Option<(String, f32)>,
    pub sitting: bool,
    pub laugh: f32,
    pub last_d: f32,
    pub stuck: u8,
}

fn fwd_temper(t: &[f32; 6]) -> f32 { t[PN] * 0.6 + (100. - t[PA]) * 0.4 }
fn fwd_brave(t: &[f32; 6]) -> f32 { (100. - t[PN]) * 0.6 + t[PO] * 0.4 }

impl Psy {
    pub fn from_npc(n: &Npc) -> Psy {
        let tr = n.soc.tr.map(|x| x as f32);
        let h = |k: i32| hash(n.id as i32, n.name as i32 * 13 + 5, 6100 + k) as f32;
        let mut t = [0.; 6];
        t[PE] = tr[T_SOCIAL];
        t[PA] = tr[T_KIND];
        t[PH] = 100. - tr[T_GREED];
        t[PN] = ((tr[T_TEMPER] + 100. - tr[T_BRAVE]) / 2.).clamp(5., 95.);
        t[PO] = 15. + h(1) * 75.;
        t[PC] = 15. + h(2) * 75.;
        Psy { t, base: t, noted: t, bat: 50. + h(3) * 40., off_t: tr[T_TEMPER] - fwd_temper(&t), off_b: tr[T_BRAVE] - fwd_brave(&t), gathered_day: -9, chat_day: -9, bubble: None, sitting: false, laugh: 0., last_d: 0., stuck: 0 }
    }
    pub fn intro(&self) -> bool { self.t[PE] < 42. }
    pub fn extra(&self) -> bool { self.t[PE] > 58. }
    pub fn dark(&self) -> bool { self.t[PH] < 30. && self.t[PA] < 40. }
    pub fn social_word(&self) -> &'static str { if self.intro() { "อินโทรเวิร์ต" } else if self.extra() { "เอ็กซ์โทรเวิร์ต" } else { "แอมบิเวิร์ต (กลางๆ)" } }
    /// ป้ายแบบ 16 personalities (เทียบเล่นๆ จากค่า Big Five)
    pub fn mbti(&self) -> String {
        let t = &self.t;
        format!("{}{}{}{}", if t[PE] >= 50. { 'E' } else { 'I' }, if t[PO] >= 50. { 'N' } else { 'S' }, if t[PA] >= 50. { 'F' } else { 'T' }, if t[PC] >= 50. { 'J' } else { 'P' })
    }
    /// นิสัยเดิม 5 อย่าง คำนวณจากโมเดลปัจจุบัน
    pub fn surface(&self) -> [u8; 5] {
        let c = |x: f32| x.round().clamp(1., 99.) as u8;
        let mut r = [0u8; 5];
        r[T_KIND] = c(self.t[PA]);
        r[T_TEMPER] = c(fwd_temper(&self.t) + self.off_t);
        r[T_GREED] = c(100. - self.t[PH]);
        r[T_BRAVE] = c(fwd_brave(&self.t) + self.off_b);
        r[T_SOCIAL] = c(self.t[PE]);
        r
    }
}

/// ประโยคพูดรอบกองไฟตามบุคลิก (เลือกแบบกำหนดจากเวลา+id)
fn fire_line(p: &Psy, seed: i32, player_here: bool) -> &'static str {
    let pick = |v: &[&'static str]| v[(hash(seed, 3, 7700) * v.len() as f64) as usize % v.len()];
    let r = hash(seed, 9, 7701);
    if p.dark() && r < 0.6 { return pick(&["ข้ารู้วิธีหาเหรียญง่ายๆ นะ... แต่ไม่บอกฟรีหรอก", "คนอื่นมันซื่อเกินไป ส่วนข้าฉลาด", "ถ้าเจ้าช่วยข้า ข้าจะ 'ตอบแทน' แน่นอน", "กล่องเก็บของใครไม่ล็อกบ้างนะ หึๆ"]); }
    if player_here && r < 0.25 { return if p.t[PA] > 55. { pick(&["ดีใจที่เจ้ามานั่งด้วยนะ", "เจ้าทำงานหนักมาทั้งวัน พักก่อนเถอะ"]) } else if p.intro() { "...(พยักหน้าทักเจ้า)" } else { "อ้าว มาสิๆ นั่งตรงนี้!" }; }
    if p.t[PN] > 62. && r < 0.55 { return pick(&["คืนนี้มอนจะมาไหมนะ...", "ข้ากังวลเรื่องเสบียงจัง", "ได้ยินเสียงอะไรไหม?", "ถ้าฝนตกหนักบ้านจะรั่วไหมนะ"]); }
    if p.intro() { return pick(&["...", "อืม ไฟสวยดีนะ", "ขอนั่งฟังเงียบๆ นะ", "(จ้องกองไฟเงียบๆ)"]); }
    if p.t[PO] > 62. && r < 0.5 { return pick(&["อยากไปดูภูเขาไฟสักครั้ง", "ข้าลองต้มซุปสูตรใหม่ล่ะ", "ดาวคืนนี้สวยมากเลย", "เคยสงสัยไหมว่าเลยทะเลไปมีอะไร"]); }
    if p.t[PC] > 62. && r < 0.5 { return pick(&["พรุ่งนี้ต้องตื่นเช้าไปทำงาน", "ข้าจัดของในกล่องเรียบร้อยแล้ว", "ใครยังไม่ได้เก็บผักบ้าง"]); }
    if p.t[PA] > 62. && r < 0.5 { return pick(&["ใครหิวบ้าง ข้าแบ่งได้นะ", "ดีใจที่ได้อยู่ด้วยกันแบบนี้", "วันนี้ทุกคนเหนื่อยกันมาก"]); }
    pick(&["เมื่อวานข้าเจอหมาป่าตัวเบ้อเริ่ม!", "ใครอยากฟังเรื่องตลกบ้าง", "ไฟอุ่นดีจัง มานั่งกันเยอะๆ สิ", "ได้ข่าวหรือยัง เรื่องเมื่อวานน่ะ"])
}
fn react_line(p: &Psy, seed: i32) -> &'static str {
    let r = hash(seed, 4, 7702);
    if p.dark() { return if r < 0.5 { "หึ" } else { "แล้วไงต่อ" }; }
    if p.t[PN] > 65. && r < 0.4 { return "น่ากลัวจัง..."; }
    if p.intro() { return if r < 0.6 { "(พยักหน้า)" } else { "...อืม" }; }
    if p.t[PA] > 55. { return if r < 0.5 { "ฮ่าๆ" } else { "จริงเหรอ!" }; }
    if r < 0.5 { "ฮ่าๆ" } else { "แล้วไงต่อ?" }
}

impl Game {
    pub fn psy_of(&mut self, id: u32) -> &mut Psy {
        if !self.psy.contains_key(&id) {
            let p = self.idx(id).map(|i| Psy::from_npc(&self.npcs[i]));
            if let Some(p) = p { self.psy.insert(id, p); }
        }
        self.psy.entry(id).or_insert_with(|| Psy { t: [50.; 6], base: [50.; 6], noted: [50.; 6], bat: 60., off_t: 0., off_b: 0., gathered_day: -9, chat_day: -9, bubble: None, sitting: false, laugh: 0., last_d: 0., stuck: 0 })
    }
    pub fn psy_ensure(&mut self) {
        let ids: Vec<u32> = self.npcs.iter().map(|n| n.id).collect();
        for id in ids { self.psy_of(id); }
        self.psy.retain(|id, _| ids_contains(&self.npcs, *id));
    }

    /// ทุกเฟรม: นั่ง/ลุกของผู้เล่น · ทุก 1 วิ: แบตสังคม แผนนั่งรอบกองไฟ บทสนทนา
    pub fn psy_tick(&mut self, dt: f32) {
        for p in self.psy.values_mut() {
            if let Some((_, t)) = &mut p.bubble { *t -= dt; if *t <= 0. { p.bubble = None; } }
            p.laugh = (p.laugh - dt).max(0.);
        }
        self.sit_tick(dt);
        self.psy_t -= dt;
        if self.psy_t > 0. { return; }
        self.psy_t = 1.;
        if self.w.dun.is_some() { return; }
        self.psy_ensure();
        let h = self.hour();
        let day = day_of(self);
        let evening = (17.5..22.).contains(&h);
        // ---- แผนนั่งรอบกองไฟตอนเย็น
        let mut plan: HashMap<u32, ((f32, f32), (f32, f32))> = HashMap::new();
        if evening {
            let mut used: HashSet<(i32, i32)> = HashSet::new();
            let mut ring_n: HashMap<(i32, i32), usize> = HashMap::new();
            let mut ids: Vec<(u32, (i32, i32))> = self.npcs.iter().filter(|n| n.state == ST_HOME && n.soc.anger < 40).map(|n| (n.id, n.bed)).collect();
            ids.sort();
            // v23 วันเทศกาล: ทุกคน (แม้แต่คนเงียบๆ) มารวมที่ลานเมือง
            let fest = if self.festival_today().is_some() { self.fest_spot() } else { None };
            for (id, bed) in ids {
                let fhere = fest.filter(|f| (f.0 - bed.0).abs().max((f.1 - bed.1).abs()) < 40 && !self.is_sick(id));
                // v42 บ้านไกลกองไฟ แต่มีลานเมือง (ไม่เกิน 45 ช่อง) = ไปนั่งที่ลานเมือง
                let plaza = self.plaza_c().filter(|f| (f.0 - bed.0).abs().max((f.1 - bed.1).abs()) <= 45);
                let Some(f) = fhere.or_else(|| self.w.near_fire(bed.0, bed.1, 14)).or(plaza) else { continue };
                let p = &self.psy[&id];
                // อยากไปไหม: เข้าสังคม + แบตเหลือ + สุ่มรายวัน · อินโทรเวิร์ตแบตหมด = ขอตัวกลับ
                let want = p.t[PE] / 100. * 0.6 + p.bat / 100. * 0.4 + (hash(day, id as i32, 7710) as f32 - 0.5) * 0.35 + if fhere.is_some() { 0.5 } else { 0. };
                if want < 0.45 || (p.bat < 18. && fhere.is_none()) { continue; }
                let fire = (f.0 as f32 * 16. + 8., f.1 as f32 * 16. + 12.);
                // ที่นั่ง: ท่อนไม้/เก้าอี้ใกล้กองไฟก่อน ไม่งั้นนั่งพื้นเป็นวง
                let mut seat = None;
                for dy in -3..=3 { for dx in -3..=3 {
                    let k = (f.0 + dx, f.1 + dy);
                    if seat.is_none() && is_seat(self.w.obj_at(k.0, k.1)) && !used.contains(&k) && self.sit != Some(k) { seat = Some(k); }
                } }
                let pos = if let Some(k) = seat { used.insert(k); (k.0 as f32 * 16. + 8., k.1 as f32 * 16. + 15.) } else {
                    let i = ring_n.entry(f).or_insert(0); *i += 1;
                    let a = *i as f32 * 2.1 + 0.6;
                    let ring = 1. + (*i / 7) as f32 * 0.55;
                    (fire.0 + a.cos() * 26. * ring, fire.1 + a.sin() * 20. * ring)
                };
                plan.insert(id, (pos, fire));
            }
        }
        // ---- v42 ตลาดนัดเย็น: คนอยากเจอผู้คน/คนที่หิว เดินมายืนเลือกของหน้าแผง
        if !evening && self.market_open() {
            let stalls = self.market_stalls();
            let mut n_at: HashMap<(i32, i32), usize> = HashMap::new();
            let mut ids: Vec<u32> = self.npcs.iter().filter(|n| n.state == ST_HOME && n.soc.anger < 50 && !n.child).map(|n| n.id).collect();
            ids.sort();
            for id in ids {
                if self.is_sick(id) { continue; }
                let Some(p) = self.psy.get(&id) else { continue };
                let hungry = self.rs.sh.hungry.get(&id).copied().unwrap_or(0) > 0;
                let want = p.t[PE] / 100. * 0.6 + (hash(day, id as i32, 7720) as f32 - 0.5) * 0.5 + if hungry { 0.6 } else { 0. };
                if want < 0.42 { continue; }
                let k = stalls[(id as usize) % stalls.len()];
                let i = n_at.entry(k).or_insert(0); *i += 1;
                let off = [(0., 1.), (-1., 1.2), (1., 1.2), (-1.6, 0.6), (1.6, 0.6), (0., 2.)][(*i - 1) % 6];
                let pos = (k.0 as f32 * 16. + 8. + off.0 * 14., k.1 as f32 * 16. + 14. + off.1 * 14.);
                plan.insert(id, (pos, (k.0 as f32 * 16. + 8., k.1 as f32 * 16. + 8.)));
            }
        }
        self.gather = plan;
        // ---- แบตสังคม + สถานะนั่ง
        let ppos = (self.p.x, self.p.y);
        let pos: Vec<(u32, f32, f32)> = self.npcs.iter().map(|n| (n.id, n.x, n.y)).collect();
        let mut warp: Vec<(u32, (f32, f32))> = vec![];
        for &(id, x, y) in &pos {
            let mut others = pos.iter().filter(|o| o.0 != id && (o.1 - x).hypot(o.2 - y) < 80.).count();
            if (ppos.0 - x).hypot(ppos.1 - y) < 80. { others += 1; }
            let g = self.gather.get(&id).copied();
            let p = self.psy.get_mut(&id).unwrap();
            let d = if p.extra() { if others > 0 { 1.6 } else { -0.4 } } else if p.intro() { match others { 0 => 1.5, 1 => -0.2, _ => -1.6 } } else if (1..=2).contains(&others) { 0.8 } else { -0.4 };
            p.bat = (p.bat + d).clamp(0., 100.);
            p.sitting = g.map_or(false, |(s, _)| (s.0 - x).hypot(s.1 - y) < 5.);
            if p.sitting { p.gathered_day = day; }
            // เดินติดสิ่งกีดขวางนานเกิน 3 วิ → เดินอ้อมไปถึงที่นั่งเอง (ไม่มีระบบหาทางจริง)
            if let Some((s, _)) = g.filter(|_| !p.sitting) {
                let d = (s.0 - x).hypot(s.1 - y);
                if d > p.last_d - 4. { p.stuck += 1; } else { p.stuck = 0; }
                p.last_d = d;
                if p.stuck >= 3 { p.stuck = 0; warp.push((id, s)); }
            } else { p.stuck = 0; p.last_d = 9999.; }
        }
        for (id, s) in warp { if let Some(i) = self.idx(id) { self.npcs[i].x = s.0; self.npcs[i].y = s.1; } }
        // อินโทรเวิร์ตแบตหมดกลางวง → ขอตัวกลับ
        let leaving: Vec<u32> = self.gather.keys().copied().filter(|id| self.psy[id].sitting && self.psy[id].bat < 18.).collect();
        for id in leaving {
            let p = self.psy.get_mut(&id).unwrap();
            p.bubble = Some(("ขอตัวกลับก่อนนะ... เหนื่อยคนแล้ว".into(), 3.5));
            self.gather.remove(&id);
        }
        // ---- วงสนทนา: ทุก ~5 วิ มีคนพูด คนอื่นตอบ
        let mut circles: HashMap<(i32, i32), Vec<u32>> = HashMap::new();
        for (&id, &(_, fire)) in &self.gather { if self.psy[&id].sitting { circles.entry((tfloor(fire.0), tfloor(fire.1))).or_default().push(id); } }
        let player_fire = self.sit.and_then(|k| self.w.near_fire(k.0, k.1, 3).map(|f| (k, f)));
        let tsec = self.time as i32;
        for (fk, mut ids) in circles {
            ids.sort();
            let player_here = player_fire.map_or(false, |(_, f)| (f.0 - fk.0).abs() <= 1 && (f.1 - fk.1).abs() <= 1);
            if ids.len() + (player_here as usize) < 2 { continue; }
            if (tsec + fk.0 * 3 + fk.1) % 5 != 0 { continue; }
            // คนพูด: เอ็กซ์โทรเวิร์ตมีโอกาสพูดมากกว่า
            let wsum: f32 = ids.iter().map(|id| 10. + self.psy[id].t[PE]).sum();
            let mut r = hash(tsec, fk.0 + fk.1 * 7, 7720) as f32 * wsum;
            let mut spk = ids[0];
            for id in &ids { r -= 10. + self.psy[id].t[PE]; if r <= 0. { spk = *id; break; } }
            let line = fire_line(&self.psy[&spk], tsec + spk as i32, player_here);
            self.psy.get_mut(&spk).unwrap().bubble = Some((line.to_string(), 4.2));
            let spk_dark = self.psy[&spk].dark();
            for &o in ids.iter().filter(|&&o| o != spk) {
                let po = self.psy.get_mut(&o).unwrap();
                if hash(tsec, o as i32, 7721) < 0.55 {
                    let l = react_line(po, tsec + o as i32);
                    if l.starts_with("ฮ่า") { po.laugh = 1.2; }
                    po.bubble = Some((l.to_string(), 2.6));
                }
                // คุยกันดี = สนิทกันขึ้น · คนเจ้าเล่ห์พูดแล้วคนซื่อสัตย์ไม่ชอบ
                let d = if spk_dark && po.t[PH] > 60. { -1 } else { 1 };
                self.add_rel(spk, o, d);
            }
            for &id in &ids {
                if let Some(i) = self.idx(id) {
                    let e = self.psy[&id].t[PE];
                    self.npcs[i].mood = (self.npcs[i].mood + if e > 58. { 2 } else if e < 42. { 0 } else { 1 }).min(100);
                }
            }
            // ผู้เล่นนั่งอยู่ในวง: วันละครั้งต่อคน นับเป็นการคุย (สนิทขึ้นได้)
            if player_here {
                for &id in &ids {
                    let Some(i) = self.idx(id) else { continue };
                    if self.psy[&id].chat_day == day || refuses(&self.npcs[i]) { continue; }
                    self.psy.get_mut(&id).unwrap().chat_day = day;
                    let n = &mut self.npcs[i];
                    n.mood = (n.mood + 4).min(100);
                    let ok = hash(day, id as i32, 7730) < 0.4 + n.soc.tr[T_SOCIAL] as f64 / 250.;
                    if ok && !n.talked { n.hearts = (n.hearts + 1).min(10); }
                    n.talked = true;
                    let nm = NPC_NAMES[n.name as usize];
                    self.toast(if ok { format!("นั่งคุยรอบกองไฟกับ{} — สนิทกันขึ้น", nm) } else { format!("นั่งคุยรอบกองไฟกับ{}", nm) });
                }
            }
        }
    }

    /// รุ่งเช้า: นิสัยค่อยๆ เปลี่ยนตามชีวิตเมื่อวาน · คืนข่าวที่เกิด
    pub fn psy_dawn(&mut self) {
        self.psy_ensure();
        let day = day_of(self);
        let mut news = vec![];
        let mut steal: Vec<u32> = vec![];
        for i in 0..self.npcs.len() {
            let n = self.npcs[i].clone();
            let bed_ok = self.w.placed.get(&n.bed) == Some(&33);
            let room = if bed_ok { self.w.room_from(n.bed) } else { None };
            let p = self.psy.get_mut(&n.id).unwrap();
            let mut d = [0f32; 6];
            if n.mood >= 70 { d[PN] -= 0.5; d[PA] += 0.2; }
            if n.mood < 35 { d[PN] += 0.6; d[PA] -= 0.3; d[PC] -= 0.2; }
            if n.talked { d[PE] += 0.3; d[PA] += 0.2; }
            if n.gifted { d[PA] += 0.4; d[PH] += 0.2; }
            if n.soc.grudge >= 30 || n.soc.anger >= 40 { d[PA] -= 0.5; d[PN] += 0.5; d[PH] -= 0.1; }
            if p.gathered_day >= day - 1 { d[PE] += 0.4; d[PA] += 0.2; if p.intro() && p.bat < 25. { d[PN] += 0.3; } }
            match &room { Some(r) if r.comfort >= 3 => { d[PN] -= 0.3; d[PC] += 0.2; } Some(_) => d[PN] -= 0.1, None if bed_ok => d[PN] += 0.4, None => {} }
            if n.married { d[PN] -= 0.2; d[PA] += 0.1; }
            for k in 0..6 { d[k] += (hash(day, n.id as i32 * 6 + k as i32, 7740) as f32 - 0.5) * 0.6; }
            for k in 0..6 {
                p.t[k] = (p.t[k] + d[k] + (p.base[k] - p.t[k]) * 0.01).clamp(1., 99.);
                if (p.t[k] - p.noted[k]).abs() >= 6. {
                    let up = p.t[k] > p.noted[k];
                    news.push(format!("{} {}", NPC_NAMES[n.name as usize], if up { PSY_UP[k] } else { PSY_DOWN[k] }));
                    p.noted[k] = p.t[k];
                }
            }
            // แบตสังคมกับอารมณ์: เอ็กซ์โทรเวิร์ตแบตหมด = เหงา · อินโทรเวิร์ตแบตหมด = เหนื่อยคน
            let lonely = p.bat < 25.;
            if p.intro() { p.bat = (p.bat + 35.).min(100.); }
            let surf = p.surface();
            let dark = p.dark();
            let nn = &mut self.npcs[i];
            nn.soc.tr = surf;
            if lonely { nn.mood = (nn.mood - 4).max(0); }
            if dark && hash(day, n.id as i32, 7750) < 0.15 { steal.push(n.id); }
        }
        // ด้านมืด: แอบหยิบของกินจากกล่องใกล้บ้าน
        for id in steal {
            let Some(i) = self.idx(id) else { continue };
            let bed = self.npcs[i].bed;
            let foods = [BERRY, CORN, CARROT, FISH, MEAT, DISH_STEW, DISH_FISH];
            let ck: Vec<(i32, i32)> = self.chests.keys().copied().filter(|k| (k.0 - bed.0).abs() <= 12 && (k.1 - bed.1).abs() <= 12).collect();
            'c: for k in ck {
                if let Some(inv) = self.chests.get_mut(&k) {
                    for &f in &foods { if inv[f] > 0 { inv[f] -= 1; news.push(format!("ของกินในกล่องหายไป 1 ชิ้น... มีคนเห็น {} เดินผ่านแถวนั้นตอนดึก", NPC_NAMES[self.npcs[i].name as usize])); break 'c; } }
                }
            }
        }
        for s in news { self.news_add(s); }
    }

    /// ข้อความสรุปนิสัยสำหรับหน้าคุย
    pub fn psy_info(&mut self, id: u32) -> Vec<String> {
        let p = self.psy_of(id).clone();
        let mut v = vec![];
        let t = |k: usize| p.t[k].round() as i32;
        v.push(format!("บุคลิก: เปิดรับ {} · วินัย {} · เข้าสังคม {} · เป็นมิตร {} · อ่อนไหว {} · ซื่อสัตย์ {}", t(PO), t(PC), t(PE), t(PA), t(PN), t(PH)));
        v.push(format!("แนว: {} · คล้าย {} (เทียบเล่นๆ){} · แบตสังคม {}/100", p.social_word(), p.mbti(), if p.dark() { " · มีด้านมืด: เจ้าเล่ห์ เห็นแก่ตัว" } else { "" }, p.bat.round() as i32));
        let ch: Vec<String> = (0..6).filter(|&k| (p.t[k] - p.base[k]).abs() >= 3.).map(|k| format!("{} {:+}", PSY_N[k], (p.t[k] - p.base[k]).round() as i32)).collect();
        if !ch.is_empty() { v.push(format!("เปลี่ยนไปจากตอนรู้จักกันแรกๆ: {}", ch.join(" · "))); }
        v
    }

    // ---------------- นั่ง ----------------
    pub fn sit_on(&mut self, k: (i32, i32)) {
        if self.npcs.iter().any(|n| self.psy.get(&n.id).map_or(false, |p| p.sitting) && (tfloor(n.x), tfloor(n.y - 3.)) == k) { self.toast("มีคนนั่งอยู่แล้ว"); return; }
        self.sit = Some(k);
        self.sit_t = 0.;
        self.p.x = k.0 as f32 * 16. + 8.;
        self.p.y = k.1 as f32 * 16. + 15.;
        if let Some(f) = self.w.near_fire(k.0, k.1, 3) { self.p.dir = 2; self.p.flip = f.0 < k.0; }
        if self.tips_seen & (1 << 9) == 0 {
            self.tips_seen |= 1 << 9;
            self.toast("นั่งพัก: ค่อยๆ ฟื้นเลือด · นั่งกินอาหารอิ่มขึ้น 40% · นั่งใกล้กองไฟตอนเย็น ชาวบ้านจะมาคุยด้วย · เดินเพื่อลุก");
        } else {
            self.toast("นั่งลง (เดินเพื่อลุก)");
        }
    }
    fn sit_tick(&mut self, dt: f32) {
        let Some(k) = self.sit else { return };
        if !is_seat(self.w.obj_at(k.0, k.1)) || (self.p.x - (k.0 as f32 * 16. + 8.)).abs() > 3. || (self.p.y - (k.1 as f32 * 16. + 15.)).abs() > 3. { self.sit = None; return; }
        self.sit_t += dt;
        if self.sit_t >= 3. {
            self.sit_t = 0.;
            if self.p.hp < self.p.maxhp { self.p.hp += 1; }
        }
    }
}

fn ids_contains(npcs: &[Npc], id: u32) -> bool { npcs.iter().any(|n| n.id == id) }

// ---------------- เซฟ ----------------
impl Game {
    pub fn psy_save(&self) -> String {
        let mut s = String::new();
        let mut ids: Vec<&u32> = self.psy.keys().collect();
        ids.sort();
        for id in ids {
            let p = &self.psy[id];
            let f = |a: &[f32; 6]| a.iter().map(|x| format!("{:.1}", x)).collect::<Vec<_>>().join(" ");
            s += &format!("npsy {} {} {} {} {:.1} {:.1} {:.1} {} {}\n", id, f(&p.t), f(&p.base), f(&p.noted), p.bat, p.off_t, p.off_b, p.gathered_day, p.chat_day);
        }
        s
    }
    pub fn psy_load(&mut self, r: &[&str]) {
        let v: Vec<f32> = r.iter().filter_map(|x| x.parse().ok()).collect();
        if v.len() < 24 { return; }
        let a = |o: usize| [v[o], v[o + 1], v[o + 2], v[o + 3], v[o + 4], v[o + 5]];
        self.psy.insert(v[0] as u32, Psy { t: a(1), base: a(7), noted: a(13), bat: v[19], off_t: v[20], off_b: v[21], gathered_day: v[22] as i32, chat_day: v[23] as i32, bubble: None, sitting: false, laugh: 0., last_d: 0., stuck: 0 });
    }
}

/// คำพูดลอยเหนือหัวชาวบ้าน (วาดหลังหลังคา ให้อ่านออกเสมอ)
pub fn draw_bubbles(g: &Game, v: &View, font: &Font) {
    let fs = (5. * v.s).max(12.) as u16;
    for n in &g.npcs {
        let Some(p) = g.psy.get(&n.id) else { continue };
        let Some((s, t)) = &p.bubble else { continue };
        let k = g.npc_body(n).scale().1;
        let (x, y) = ((n.x - v.cx) * v.s, (n.y - v.cy - 24. * k - if p.sitting { -5. } else { 0. }) * v.s);
        if x < -200. || y < -50. || x > screen_width() + 200. || y > screen_height() + 50. { continue; }
        let a = t.min(0.4) / 0.4;
        let d = measure_text(s, Some(font), fs, 1.);
        let (bw, bh) = (d.width + 12., fs as f32 + 8.);
        draw_rectangle(x - bw / 2., y - bh, bw, bh, Color::new(1., 1., 0.96, 0.92 * a));
        draw_triangle(vec2(x - 4., y), vec2(x + 4., y), vec2(x, y + 6.), Color::new(1., 1., 0.96, 0.92 * a));
        draw_text_ex(s, x - d.width / 2., y - 6., TextParams { font: Some(font), font_size: fs, color: Color::new(0.15, 0.12, 0.1, a), ..Default::default() });
    }
}
