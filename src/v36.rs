//! v36 กฎบัตรเมือง — ตั้งกฎหมายให้เมืองเหมือนสังคมจริง
//!
//! ต่อยอดจาก "กฎหมายเมืองเรา" เดิม (ภาษี/เกณฑ์ทหาร/พรมแดน/เคอร์ฟิว/ห้ามอาวุธ) ด้วยกฎที่เป็นเรื่องของสังคม 6 ข้อ แต่ละข้อมีหลายระดับ:
//!   ที่ดินและการสร้าง · ความยุติธรรม · ทรัพย์สินส่วนรวม · สวัสดิการ · แรงงานส่วนรวม · วันพักของเมือง
//! หลักคิด: กฎทุกข้อมี "ผู้ได้-ผู้เสีย" — ชาวบ้านแต่ละคนมีความเห็นตามนิสัยของตัวเอง (ใจดี/โลภ/ใจร้อน/กล้า/ช่างพูด)
//!   ประกาศกฎแล้วเห็นปฏิกิริยาทันที (พอใจ/เฉย/ไม่พอใจ) · ทุกเช้าความเห็นสะสมกลายเป็นอารมณ์ · ไม่พอใจกันมากก็ขึ้นข่าว
//! ผลของกฎจริง (ไม่ใช่แค่ตัวเลข):
//!   ที่ดิน เสรี = ชาวบ้านเลือกที่สร้างบ้าน/ไถไร่เองเลย ไม่ต้องมาถามเรา (แต่เมืองจะเละ → จ้างสถาปนิกจัดผัง)
//!   ยุติธรรม สภา/ประมวลเข้มงวด = ข้อพิพาท+การขโมยถูกตัดสินกันเอง ไม่ต้องรอเรา (เข้มงวด = ขโมยน้อยลงมาก แต่คนใจดีไม่ชอบ)
//!   คลังกลาง = ใครหิวไปหยิบอาหารจากกล่องไกลขึ้นได้ (รัศมี 40 แทน 16)
//!   สวัสดิการ = เอาภาษีมาแจกอาหารคนหิว/ซื้อยารักษาคนป่วย · แรงงานส่วนรวม = ได้ภาษีเพิ่มแต่คนขี้เกียจ/ใจร้อนไม่ชอบ
//!   วันพัก = ทุก 7 วันไม่มีงานทั้งเมือง ทุกคนสุข+
use super::*;

pub const L_LAND: usize = 0;
pub const L_JUSTICE: usize = 1;
pub const L_COMMONS: usize = 2;
pub const L_WELFARE: usize = 3;
pub const L_CORVEE: usize = 4;
pub const L_REST: usize = 5;
pub const LAW_N: usize = 6;
pub const LAW_NAME: [&str; LAW_N] = ["ที่ดินและการสร้าง", "ความยุติธรรม", "ทรัพย์สินส่วนรวม", "สวัสดิการ", "แรงงานส่วนรวม", "วันพักของเมือง"];
pub const LAW_LV: [&[&str]; LAW_N] = [
    &["ขออนุญาตทุกครั้ง", "เสรี"],
    &["ท่านตัดสินเอง", "สภาหมู่บ้าน", "ประมวลเข้มงวด"],
    &["ของใครของมัน", "คลังกลาง"],
    &["ไม่มี", "แจกอาหาร", "อาหาร+ยา"],
    &["ไม่เกณฑ์", "เกณฑ์แรงงาน"],
    &["ไม่มี", "หยุดทุก 7 วัน"],
];
pub const LAW_DESC: [&[&str]; LAW_N] = [
    &["ชาวบ้านต้องมาขอที่ก่อนสร้างบ้าน/ไถไร่ (เมืองเป็นระเบียบกว่า)", "สร้างตรงไหนก็ได้ไม่ต้องขอ (เร็ว แต่เมืองจะเละ) · คนรักอิสระชอบ คนรักระเบียบไม่ชอบ"],
    &["ข้อพิพาทและการขโมยมาให้ท่านตัดสินเอง", "สภาหมู่บ้านไกล่เกลี่ยให้ ท่านไม่ต้องตัดสิน · คนช่างพูด/ใจดีชอบ", "ปรับและลงโทษหนัก ขโมยน้อยลงมาก ภาษีเพิ่มจากค่าปรับ · คนใจดีไม่ชอบ"],
    &["อาหารในกล่องของใคร ใครก็หยิบเฉพาะในรัศมีบ้านตัวเอง (16 ช่อง)", "ทุกคนหยิบอาหารจากกล่องทั้งเมืองได้ (40 ช่อง) · คนใจกว้างชอบ คนโลภไม่ชอบ"],
    &["ใครหิวใครป่วยก็ช่วยตัวเอง", "ใช้ภาษีแจกอาหารคนอดข้าว (2 เหรียญ/คน/วัน) · คนใจดีชอบ คนโลภไม่ชอบ", "ทั้งแจกอาหารและจ่ายยารักษาคนป่วย (4 เหรียญ/คน)"],
    &["ใครอยากทำอะไรก็ทำ", "คนทำงานจ่ายแรงให้ส่วนรวม ได้ภาษี +2/คน/วัน แต่ความสุข -1 · คนโลภ/ใจร้อนไม่ชอบ"],
    &["ทำงานทุกวัน", "ทุกวันที่ 7 ไม่มีงาน ทุกคนพักผ่อน สุข +2 · คนช่างพูดชอบ คนโลภไม่ชอบ"],
];

#[derive(Clone, Debug, Default)]
pub struct Charter {
    pub lv: [u8; LAW_N],
    /// ขึ้นข่าวไม่พอใจครั้งล่าสุดวันที่
    pub news_day: i32,
}

pub fn law_levels(l: usize) -> u8 { LAW_LV[l].len() as u8 }

/// ความเห็นของคนนิสัย tr ต่อกฎ l ระดับ v (-2..=+2) — กฎระดับ 0 = สภาพเดิม ไม่มีใครได้/เสียเพิ่ม
pub fn law_opinion(tr: &[u8; 5], l: usize, v: u8) -> i32 {
    let (kind, temper, greed, brave, social) = (tr[T_KIND] as i32, tr[T_TEMPER] as i32, tr[T_GREED] as i32, tr[T_BRAVE] as i32, tr[T_SOCIAL] as i32);
    let mut o = 0;
    match (l, v) {
        (L_LAND, 1) => {
            if greed >= 60 || temper >= 65 { o += 1; }
            if social >= 70 { o -= 1; }
            if kind >= 70 && greed < 40 { o -= 0; }
        }
        (L_JUSTICE, 1) => {
            if social >= 55 || kind >= 60 { o += 1; }
            if temper >= 75 { o -= 1; }
        }
        (L_JUSTICE, 2) => {
            if brave >= 65 && kind < 50 { o += 1; }
            if greed < 40 && kind < 60 { o += 1; }
            if kind >= 60 { o -= 2; }
            if greed >= 65 { o -= 1; }
        }
        (L_COMMONS, 1) => {
            if kind >= 55 && greed < 50 { o += 2; } else if kind >= 55 || social >= 65 { o += 1; }
            if greed >= 65 { o -= 2; } else if greed >= 50 { o -= 1; }
        }
        (L_WELFARE, 1 | 2) => {
            if kind >= 55 { o += 1; }
            if v == 2 && kind >= 70 { o += 1; }
            if greed >= 70 { o -= 1; }
            if greed >= 50 && kind < 40 { o -= 1; }
        }
        (L_CORVEE, 1) => {
            if temper >= 60 { o -= 1; }
            if greed >= 60 { o -= 1; }
            if social >= 70 && kind >= 50 { o += 1; }
            if brave >= 70 { o += 1; }
        }
        (L_REST, 1) => {
            if social >= 50 { o += 1; }
            if kind >= 70 { o += 1; }
            if greed >= 75 { o -= 1; }
            if brave >= 80 && greed >= 60 { o -= 1; }
        }
        _ => {}
    }
    o.clamp(-2, 2)
}
/// ความเห็นต่อกฎเดิม (ภาษี/เกณฑ์ทหาร/พรมแดน/เคอร์ฟิว/ห้ามอาวุธ) — ไว้แสดงอย่างเดียว ผลต่ออารมณ์มีอยู่แล้วในของเดิม
pub fn ml_opinion(tr: &[u8; 5], laws: u8) -> i32 {
    let (kind, temper, greed, brave, social) = (tr[T_KIND] as i32, tr[T_TEMPER] as i32, tr[T_GREED] as i32, tr[T_BRAVE] as i32, tr[T_SOCIAL] as i32);
    let mut o = 0;
    if laws & ML_LOWTAX != 0 && greed >= 50 { o += 1; }
    if laws & ML_HIGHTAX != 0 { o -= if greed >= 60 { 2 } else { 1 }; }
    if laws & ML_MILITIA != 0 { if brave >= 65 { o += 1; } else if brave < 35 { o -= 1; } }
    if laws & ML_OPEN != 0 && social >= 65 { o += 1; }
    if laws & ML_CURFEW != 0 { if social >= 60 || temper >= 60 { o -= 1; } else if brave < 35 { o += 1; } }
    if laws & ML_NOWEAPON != 0 { if brave >= 75 { o -= 1; } else if brave < 35 || kind >= 60 { o += 1; } }
    o
}

impl Game {
    pub fn ch_lv(&self, l: usize) -> u8 { self.rs.ch.lv[l] }
    /// ความเห็นรวมของชาวบ้านคนหนึ่งต่อกฎบัตรใหม่ทั้งหมด
    pub fn charter_opinion(&self, n: &npc::Npc) -> i32 {
        (0..LAW_N).map(|l| law_opinion(&n.soc.tr, l, self.rs.ch.lv[l])).sum()
    }
    /// (พอใจ, เฉย, ไม่พอใจ) ต่อกฎข้อ l (ระดับปัจจุบัน) · l == None = ทุกกฎ (รวมกฎเดิม)
    pub fn law_mood_split(&self, l: Option<usize>) -> (usize, usize, usize) {
        let (mut a, mut b, mut c) = (0, 0, 0);
        for n in self.npcs.iter().filter(|n| n.state == ST_HOME && !n.child) {
            let o = match l { Some(l) => law_opinion(&n.soc.tr, l, self.rs.ch.lv[l]), None => self.charter_opinion(n) + ml_opinion(&n.soc.tr, self.rs.laws) };
            if o >= 1 { a += 1; } else if o <= -1 { c += 1; } else { b += 1; }
        }
        (a, b, c)
    }
    /// ประกาศกฎ: เปลี่ยนระดับ + ชาวบ้านแสดงปฏิกิริยาทันที
    pub fn ch_set(&mut self, l: usize, v: u8) {
        if l >= LAW_N || v >= law_levels(l) || self.rs.ch.lv[l] == v { return; }
        let before = self.law_mood_split(Some(l));
        self.rs.ch.lv[l] = v;
        let (ok, mid, no) = self.law_mood_split(Some(l));
        // ปฏิกิริยาทันที: ผู้ได้ดีใจ ผู้เสียหงุดหงิด (เทียบกับก่อนประกาศ)
        let tr_of: Vec<(u32, [u8; 5])> = self.npcs.iter().filter(|n| n.state == ST_HOME && !n.child).map(|n| (n.id, n.soc.tr)).collect();
        for (id, tr) in tr_of {
            let d = law_opinion(&tr, l, v) - law_opinion(&tr, l, if v == 0 { 1 } else { 0 });
            let _ = id;
            if d == 0 { continue; }
            if let Some(i) = self.idx(id) { self.npcs[i].mood = (self.npcs[i].mood + d.signum() * 3).clamp(0, 100); }
        }
        let _ = before;
        self.news_add(format!("ประกาศกฎ \"{}\": {} — พอใจ {} · เฉย {} · ไม่พอใจ {}", LAW_NAME[l], LAW_LV[l][v as usize], ok, mid, no));
        if no > ok && no >= 2 { self.toast(format!("ชาวบ้านบ่นเรื่องกฎ \"{}\" ({} คนไม่พอใจ)", LAW_NAME[l], no)); }
        else { self.toast(format!("ประกาศแล้ว: {} — {}", LAW_NAME[l], LAW_LV[l][v as usize])); }
    }
    pub fn ch_cycle(&mut self, l: usize) {
        let v = (self.rs.ch.lv[l] + 1) % law_levels(l);
        self.ch_set(l, v);
    }

    // ---------------------------------------------------------------- ผลของกฎ
    /// รัศมีที่ชาวบ้านหยิบอาหารจากกล่องได้ (ช่อง)
    pub fn food_radius(&self) -> i32 { if self.rs.ch.lv[L_COMMONS] >= 1 { 40 } else { 16 } }
    /// วันพัก: วันที่ 7 ของทุกสัปดาห์
    pub fn rest_day(&self) -> bool { self.rs.ch.lv[L_REST] >= 1 && day_of(self).rem_euclid(7) == 6 }
    /// โอกาสที่คนโลภจะลองขโมยในวันหนึ่ง
    pub fn theft_chance(&self) -> f64 { match self.rs.ch.lv[L_JUSTICE] { 2 => 0.05, 1 => 0.12, _ => 0.18 } }
    fn nm36(&self, id: u32) -> &'static str { self.idx(id).map(|i| NPC_NAMES[self.npcs[i].name as usize]).unwrap_or("?") }

    /// สภาหมู่บ้าน/ประมวลเข้มงวด ตัดสินการขโมยเอง
    pub fn council_theft(&mut self, thief: u32, item: usize, qty: i32, witness: u32) {
        let strict = self.rs.ch.lv[L_JUSTICE] >= 2;
        let tn = self.nm36(thief);
        if let Some(i) = self.idx(thief) {
            let bed = self.npcs[i].bed;
            self.give_to_chest(bed, item, qty);
            let n = &mut self.npcs[i];
            n.soc.tr[T_GREED] = n.soc.tr[T_GREED].saturating_sub(if strict { 6 } else { 3 });
            n.mood = (n.mood - if strict { 8 } else { 3 }).max(0);
            n.soc.grudge = (n.soc.grudge + if strict { 12 } else { 5 }).min(100);
        }
        self.add_rel(witness, thief, if strict { -15 } else { -8 });
        if strict { self.tax_pool += 8; }
        self.news_add(if strict { format!("ศาลเข้มงวด: {} ถูกปรับ 8 เหรียญและต้องคืนของที่ขโมย", tn) } else { format!("สภาหมู่บ้านให้ {} คืนของที่ขโมยแล้ว", tn) });
    }
    /// สภาหมู่บ้าน/ประมวลเข้มงวด ตัดสินข้อพิพาทเอง
    pub fn council_dispute(&mut self, a: u32, b: u32) {
        let strict = self.rs.ch.lv[L_JUSTICE] >= 2;
        let (na, nb) = (self.nm36(a), self.nm36(b));
        if strict {
            for id in [a, b] { if let Some(i) = self.idx(id) { let n = &mut self.npcs[i]; n.mood = (n.mood - 4).max(0); n.soc.grudge = (n.soc.grudge + 5).min(100); } }
            let cur = self.rel_between(a, b);
            self.set_rel(a, b, (cur + 12).min(-10));
            self.tax_pool += 6;
            self.news_add(format!("ศาลเข้มงวดปรับ {} กับ {} ฝ่ายละ 3 เหรียญ ห้ามทะเลาะกันอีก", na, nb));
            return;
        }
        let soc = |g: &Game, id: u32| g.idx(id).map_or(0, |i| g.npcs[i].soc.tr[T_SOCIAL] as i32);
        let ok = (soc(self, a) + soc(self, b)) / 2 >= 40 || hash(a as i32 ^ day_of(self), b as i32, 36) < 0.6;
        if ok {
            let cur = self.rel_between(a, b);
            self.set_rel(a, b, (cur + 30).max(-20));
            for id in [a, b] { if let Some(i) = self.idx(id) { let n = &mut self.npcs[i]; n.mood = (n.mood + 2).min(100); n.soc.anger = (n.soc.anger - 15).max(0); } }
            self.news_add(format!("สภาหมู่บ้านไกล่เกลี่ย {} กับ {} จนคืนดีกันได้", na, nb));
        } else {
            for id in [a, b] { if let Some(i) = self.idx(id) { let n = &mut self.npcs[i]; n.mood = (n.mood - 2).max(0); } }
            self.news_add(format!("สภาหมู่บ้านไกล่เกลี่ย {} กับ {} ไม่สำเร็จ ทั้งคู่ยังไม่ลงรอยกัน", na, nb));
        }
    }
    /// ที่ดินเสรี: ชาวบ้านที่มาขอที่ ได้ตรงที่เลือกเลย (ถ้าตรงนั้นใช้ไม่ได้ก็ให้เลือกที่ใหม่เองโดยไม่ต้องรบกวนเรา)
    pub fn land_auto(&mut self) {
        if self.rs.ch.lv[L_LAND] == 0 || self.rs.pm.pick.is_some() { return; }
        let ids: Vec<u32> = self.rs.pm.asks.iter().map(|a| a.by).collect();
        for id in ids {
            let Some(a) = self.ask_of(id).cloned() else { continue };
            let mut at = a.clone();
            if a.kind != permit::AK_FARM { if let Some(d) = self.door_dir_for(a.kind, a.at, a.dir) { at.dir = d; } }
            let bad = if a.kind == permit::AK_FARM { self.farm_blocked(at.at, Some(id)) } else { self.prefab_blocked(a.kind, at.at, at.dir).or(self.land_taken(&self.area_cells(a.kind, at.at, at.dir), Some(id))) };
            if bad.is_none() { self.permit_yes(id); } else { self.permit_no(id); }
        }
    }

    /// ทุกเช้า: ความเห็น → อารมณ์ · สวัสดิการ · แรงงานส่วนรวม · วันพัก · ข่าวความไม่พอใจ
    pub fn charter_day(&mut self) {
        let day = day_of(self);
        let lv = self.rs.ch.lv;
        let ids: Vec<u32> = self.npcs.iter().filter(|n| n.state == ST_HOME && !n.child).map(|n| n.id).collect();
        // 1) ความเห็น → อารมณ์ (ตามกฎบัตรใหม่เท่านั้น)
        for &id in &ids {
            let Some(i) = self.idx(id) else { continue };
            let o = self.charter_opinion(&self.npcs[i]);
            let d = if o >= 3 { 2 } else if o >= 1 { 1 } else if o <= -3 { -2 } else if o <= -1 { -1 } else { 0 };
            let n = &mut self.npcs[i];
            n.mood = (n.mood + d).clamp(0, 100);
        }
        // 2) สวัสดิการ: อาหารคนอดข้าว · ยาคนป่วย
        if lv[L_WELFARE] >= 1 {
            let hungry: Vec<u32> = self.rs.sh.hungry.iter().filter(|(_, &d)| d >= 1).map(|(&id, _)| id).collect();
            let mut fed = 0;
            for id in hungry {
                if self.tax_pool < 2 { break; }
                if !ids.contains(&id) { continue; }
                self.tax_pool -= 2;
                self.rs.sh.hungry.remove(&id);
                if let Some(i) = self.idx(id) { self.npcs[i].mood = (self.npcs[i].mood + 3).min(100); }
                fed += 1;
            }
            if fed > 0 { self.news_add(format!("สวัสดิการเมืองแจกอาหารให้คนอดข้าว {} คน (ใช้ภาษี {} เหรียญ)", fed, fed * 2)); }
        }
        if lv[L_WELFARE] >= 2 {
            let sick: Vec<u32> = self.rs.lv.sick.keys().copied().collect();
            for sid in sick {
                if self.tax_pool < 4 { break; }
                self.tax_pool -= 4;
                self.rs.lv.sick.remove(&sid);
                let nm = self.nm36(sid);
                self.news_add(format!("สวัสดิการเมืองจ่ายค่ายารักษา {} จนหายป่วย", nm));
            }
        }
        // 3) แรงงานส่วนรวม: ได้ภาษี แต่เหนื่อย
        if lv[L_CORVEE] >= 1 {
            let mut got = 0;
            for &id in &ids {
                let Some(i) = self.idx(id) else { continue };
                if self.npcs[i].mood < 20 || !Game::willing(&self.npcs[i]) { continue; }
                let hard = self.npcs[i].soc.tr[T_TEMPER] >= 60 || self.npcs[i].soc.tr[T_GREED] >= 60;
                self.npcs[i].mood = (self.npcs[i].mood - if hard { 2 } else { 1 }).max(0);
                got += 2;
            }
            self.tax_pool += got;
        }
        // 4) วันพัก
        if self.rest_day() {
            for &id in &ids {
                let Some(i) = self.idx(id) else { continue };
                let n = &mut self.npcs[i];
                n.mood = (n.mood + if n.soc.tr[T_SOCIAL] >= 50 { 4 } else { 2 }).min(100);
            }
            self.news_add("วันนี้เป็นวันพักของเมือง ทุกคนหยุดงานพักผ่อน".to_string());
        }
        // 5) ข่าว: ชาวบ้านส่วนใหญ่ไม่พอใจ → บอกว่ากฎข้อไหนที่โดนบ่นมากสุด
        let pop = ids.len();
        if pop >= 4 && day - self.rs.ch.news_day >= 3 {
            let (_, _, no) = self.law_mood_split(None);
            if no * 2 > pop {
                let worst = (0..LAW_N).filter(|&l| lv[l] > 0).map(|l| (l, self.law_mood_split(Some(l)).2)).max_by_key(|x| x.1);
                if let Some((l, n)) = worst { if n >= 2 { self.news_add(format!("ชาวบ้านส่วนใหญ่ไม่พอใจ — โดยเฉพาะกฎ \"{}\" ({} คน) ลองทบทวนดู", LAW_NAME[l], n)); self.rs.ch.news_day = day; } }
            }
        }
    }

    pub fn ch_save(&self) -> String {
        let c = &self.rs.ch;
        if c.lv.iter().all(|&v| v == 0) && c.news_day == 0 { return String::new(); }
        format!("charter {} {}\n", c.lv.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(","), c.news_day)
    }
    pub fn ch_load(&mut self, r: &[&str]) {
        if let Some(a) = r.first() {
            for (i, v) in a.split(',').enumerate().take(LAW_N) { self.rs.ch.lv[i] = v.parse::<u8>().unwrap_or(0).min(law_levels(i) - 1); }
        }
        self.rs.ch.news_day = r.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn villager(g: &mut Game, id: u32, bed: (i32, i32), tr: [u8; 5]) {
        let mut n = Npc { id, name: (id % 8) as u8, look: 0, x: bed.0 as f32 * 16. + 8., y: bed.1 as f32 * 16. + 18., state: ST_HOME, hearts: 3, mood: 60, married: false, child: false, role: 0, bed, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(id, 0) };
        n.soc.tr = tr;
        g.npcs.push(n);
    }
    fn game() -> Game {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        g.npcs.clear();
        g
    }
    #[test]
    fn opinions_follow_personality() {
        let kind = [90, 20, 10, 50, 60];
        let greedy = [10, 50, 90, 50, 30];
        assert!(law_opinion(&kind, L_WELFARE, 1) > 0 && law_opinion(&greedy, L_WELFARE, 1) < 0, "สวัสดิการ: คนใจดีชอบ คนโลภไม่ชอบ");
        assert!(law_opinion(&kind, L_COMMONS, 1) > 0 && law_opinion(&greedy, L_COMMONS, 1) < 0, "คลังกลาง");
        assert!(law_opinion(&kind, L_JUSTICE, 2) < 0, "ประมวลเข้มงวด: คนใจดีไม่ชอบ");
        assert!(law_opinion(&greedy, L_CORVEE, 1) < 0, "เกณฑ์แรงงาน: คนโลภไม่ชอบ");
        assert_eq!(law_opinion(&kind, L_WELFARE, 0), 0, "ระดับ 0 = สภาพเดิม ไม่มีความเห็น");
    }
    #[test]
    fn enacting_a_law_splits_the_village_and_news_shows_it() {
        let mut g = game();
        villager(&mut g, 1, (0, 0), [90, 20, 10, 50, 60]);
        villager(&mut g, 2, (4, 0), [90, 20, 10, 50, 60]);
        villager(&mut g, 3, (8, 0), [10, 50, 90, 50, 30]);
        let m = |g: &Game, id| g.npcs[g.idx(id).unwrap()].mood;
        g.ch_set(L_WELFARE, 1);
        assert_eq!(g.rs.ch.lv[L_WELFARE], 1);
        assert_eq!(g.law_mood_split(Some(L_WELFARE)), (2, 0, 1));
        assert!(m(&g, 1) > 60 && m(&g, 3) < 60, "ผู้ได้ดีใจ ผู้เสียหงุดหงิด");
        assert!(g.news.iter().any(|n| n.1.contains("ประกาศกฎ") && n.1.contains("พอใจ 2")), "ข่าวบอกผล");
        // ปรับกลับ = ไม่มีใครได้เสีย
        g.ch_set(L_WELFARE, 0);
        assert_eq!(g.law_mood_split(Some(L_WELFARE)), (0, 3, 0));
    }
    #[test]
    fn daily_opinion_moves_mood_and_news_when_most_unhappy() {
        let mut g = game();
        for id in 1..=5 { villager(&mut g, id, (id as i32 * 4, 0), [10, 60, 90, 50, 30]); }
        g.rs.ch.lv[L_COMMONS] = 1; g.rs.ch.lv[L_CORVEE] = 1; g.rs.ch.news_day = -10;
        let m0 = g.npcs[0].mood;
        g.charter_day();
        assert!(g.npcs[0].mood < m0, "ไม่พอใจกฎ → อารมณ์ลดลงทุกเช้า");
        assert!(g.news.iter().any(|n| n.1.contains("ชาวบ้านส่วนใหญ่ไม่พอใจ")), "เกินครึ่งไม่พอใจ → ขึ้นข่าวบอกกฎที่โดนบ่น");
    }
    #[test]
    fn welfare_feeds_the_hungry_and_cures_the_sick_from_taxes() {
        let mut g = game();
        for id in 1..=3 { villager(&mut g, id, (id as i32 * 4, 0), [50, 50, 50, 50, 50]); }
        g.rs.sh.hungry.insert(1, 2); g.rs.sh.hungry.insert(2, 1);
        g.rs.lv.sick.insert(3, 2);
        g.tax_pool = 10;
        g.rs.ch.lv[L_WELFARE] = 1;
        g.charter_day();
        assert!(g.rs.sh.hungry.is_empty(), "แจกอาหารคนอดข้าว");
        assert_eq!(g.tax_pool, 6);
        assert!(g.is_sick(3), "ระดับ 1 ไม่จ่ายยา");
        g.rs.ch.lv[L_WELFARE] = 2;
        g.charter_day();
        assert!(!g.is_sick(3), "ระดับ 2 จ่ายยารักษา");
        assert_eq!(g.tax_pool, 2);
        // ภาษีไม่พอ = ไม่แจก
        g.rs.sh.hungry.insert(1, 3); g.tax_pool = 1;
        g.charter_day();
        assert!(g.rs.sh.hungry.contains_key(&1));
    }
    #[test]
    fn corvee_pays_taxes_but_tires_people_and_rest_day_recovers() {
        let mut g = game();
        for id in 1..=3 { villager(&mut g, id, (id as i32 * 4, 0), [50, 50, 50, 50, 50]); }
        g.rs.ch.lv[L_CORVEE] = 1;
        g.tax_pool = 0;
        g.charter_day();
        assert_eq!(g.tax_pool, 6, "คนละ 2 เหรียญ");
        assert!(g.npcs.iter().all(|n| n.mood == 59));
        g.rs.ch.lv[L_CORVEE] = 0; g.rs.ch.lv[L_REST] = 1;
        // วันพัก = วันที่ day%7==6
        let d = day_of(&g);
        g.time += ((6 - d.rem_euclid(7) + 7) % 7) as f32 * DAYLEN;
        assert!(g.rest_day(), "วันที่ 7 เป็นวันพัก");
        let before = g.npcs[0].mood;
        g.charter_day();
        assert!(g.npcs[0].mood > before);
        g.time += DAYLEN;
        assert!(!g.rest_day());
    }
    #[test]
    fn council_settles_theft_and_disputes_without_the_player() {
        let mut g = game();
        villager(&mut g, 1, (0, 0), [20, 50, 90, 30, 60]);
        villager(&mut g, 2, (6, 0), [60, 50, 20, 50, 80]);
        let mut cinv = [0; NI]; cinv[COIN] = 0;
        g.chests.insert((1, 1), cinv); g.w.placed.insert((1, 1), 14);
        g.rs.ch.lv[L_JUSTICE] = 1;
        g.council_theft(1, COIN, 10, 2);
        assert_eq!(g.chests[&(1, 1)][COIN], 10, "ของที่ขโมยถูกคืนเข้ากล่อง");
        assert!(g.nreqs.is_empty(), "ไม่ต้องรบกวนผู้เล่น");
        g.set_rel(1, 2, -70);
        g.council_dispute(1, 2);
        assert!(g.rel_between(1, 2) > -70, "สภาไกล่เกลี่ยแล้วความสัมพันธ์ดีขึ้น");
        // เข้มงวด: ปรับเข้าภาษี ขโมยโอกาสน้อยลง
        g.rs.ch.lv[L_JUSTICE] = 2;
        let p = g.tax_pool;
        g.council_theft(1, COIN, 5, 2);
        assert_eq!(g.tax_pool, p + 8);
        assert!(g.theft_chance() < 0.1);
    }
    #[test]
    fn commons_widens_food_radius() {
        let mut g = game();
        assert_eq!(g.food_radius(), 16);
        g.rs.ch.lv[L_COMMONS] = 1;
        assert_eq!(g.food_radius(), 40);
    }
    fn quest_town(n: u32) -> Game {
        let mut g = Game::new_world_seed(1, Sfx::default());
        g.npcs.clear();
        for k in 0..n {
            let b = (500 + k as i32 * 2, 500);
            g.w.placed.insert(b, 33);
            villager(&mut g, k + 1, b, [50, 50, 50, 50, 50]);
        }
        g.npc_next = n + 1;
        g
    }
    #[test]
    fn with_a_council_disputes_and_theft_never_reach_the_player() {
        // ข้อพิพาท: เกลียดกันมาก → ปกติขอให้เราตัดสิน · มีสภาแล้วสภาจัดการเอง
        let mut g = quest_town(2);
        g.rs.ch.lv[L_JUSTICE] = 1;
        g.set_rel(1, 2, -80);
        let mut settled = false;
        for d in 1..60 {
            g.time = DAYLEN * (d as f32 + 0.2);
            g.quest_dawn();
            assert!(g.nreqs.iter().all(|r| !matches!(r.k, QK::Dispute { .. })), "สภาตัดสินเอง ไม่ต้องรอเรา");
            if g.news.iter().any(|n| n.1.contains("สภาหมู่บ้านไกล่เกลี่ย")) { settled = true; break; }
        }
        assert!(settled, "สภาลงมือไกล่เกลี่ยแล้วขึ้นข่าว");
        assert!(g.rel_between(1, 2) > -80 || g.news.iter().any(|n| n.1.contains("ไม่สำเร็จ")));
        // ขโมย: ศาลเข้มงวดจัดการเอง ปรับเข้าภาษี
        let mut g = quest_town(3);
        g.rs.ch.lv[L_JUSTICE] = 2;
        g.npcs[0].soc.tr = [10, 50, 95, 50, 50];
        g.npcs[0].mood = 40;
        g.npcs[2].soc.tr[T_SOCIAL] = 95;
        g.w.placed.insert((503, 503), 14);
        let mut c = [0; NI]; c[COIN] = 30;
        g.chests.insert((503, 503), c);
        let mut caught = false;
        for d in 1..600 {
            g.time = DAYLEN * (d as f32 + 0.2);
            g.npcs[0].mood = 40;
            g.quest_dawn();
            assert!(g.nreqs.iter().all(|r| !matches!(r.k, QK::Theft { .. })), "ศาลตัดสินเอง");
            if g.news.iter().any(|n| n.1.contains("ศาลเข้มงวด")) { caught = true; break; }
        }
        assert!(caught, "มีการขโมยแล้วศาลจัดการ");
        assert!(g.tax_pool >= 8, "ค่าปรับเข้าคลังภาษี");
    }
    #[test]
    fn free_land_grants_villager_requests_without_asking_the_player() {
        let mut g = game();
        let c = (tfloor(g.p.x), tfloor(g.p.y));
        for y in c.1 - 12..=c.1 + 12 { for x in c.0 - 12..=c.0 + 12 { g.w.removed.insert((x, y)); let t = g.w.tile_at(x, y); if SOLIDT[t as usize] || t == T_SOIL { g.w.tmod.insert((x, y), 3); } } }
        let bed = (c.0 - 8, c.1 - 8);
        g.w.placed.insert(bed, 33);
        villager(&mut g, 1, bed, [50, 50, 50, 50, 50]);
        g.npcs[0].role = 1;
        let at = g.find_field(bed).expect("มีที่ทำไร่");
        g.rs.pm.asks.push(permit::LandAsk { by: 1, kind: permit::AK_FARM, at, dir: 0, came: false, snooze: 0. });
        g.permit_tick();
        assert_eq!(g.rs.pm.asks.len(), 1, "ปกติต้องรอเราตอบ");
        g.rs.ch.lv[L_LAND] = 1;
        g.permit_tick();
        assert!(g.rs.pm.asks.is_empty(), "ที่ดินเสรี: ไม่ต้องรอเรา");
        assert!(g.rs.pm.fields.iter().any(|f| f.by == 1), "ได้ผืนไร่แล้ว");
    }

    #[test]
    fn charter_save_roundtrip() {
        let mut g = game();
        g.rs.ch.lv = [1, 2, 1, 2, 1, 1]; g.rs.ch.news_day = 9;
        let s = g.ch_save();
        let mut h = game();
        let l = s.trim().strip_prefix("charter ").unwrap();
        let parts: Vec<&str> = l.split(' ').collect();
        h.ch_load(&parts);
        assert_eq!(h.rs.ch.lv, [1, 2, 1, 2, 1, 1]);
        assert_eq!(h.rs.ch.news_day, 9);
        assert!(game().ch_save().is_empty(), "ไม่ได้ตั้งกฎ = ไม่เขียนอะไรลงเซฟ");
    }
}
