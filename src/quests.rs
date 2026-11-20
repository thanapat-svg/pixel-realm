// quests.rs — v8e: คำขอ/ปัญหาจากชาวบ้าน (เกิดจากนิสัย ความต้องการ และความสัมพันธ์จริง)
//  - ขอของ: คนหิว/ไม่มีความสุขขอของกินที่ชอบ · คนโลภอยากได้อัญมณี
//  - ล่าสัตว์: คนขี้ขลาดขอให้กำจัดหมาป่า/หมูป่า/จระเข้ที่ทำให้กลัว
//  - ฝากของ: คนช่างพูดฝากของขวัญไปให้เพื่อนสนิท
//  - ไกล่เกลี่ย: คนใจดีขอให้ช่วยให้สองคนที่ทะเลาะกันคืนดี (คุยกับทั้งคู่ในวันเดียวกัน)
//  - ข้อพิพาท: สองคนที่เกลียดกันมากขอให้คุณตัดสิน — เข้าข้างคนหนึ่ง อีกคนแค้น · ประนีประนอมได้ถ้าสนิทกับทั้งคู่
//  - ขโมย: คนโลภ+ใจดำแอบเอาของจากกล่องเก็บของ คนช่างพูดเห็นแล้วมาบอก → ทวงคืน / ยกโทษ / ไล่ออก
//  ไม่ช่วยภายใน 3 วัน = ผิดหวัง · ปฏิเสธตรงๆ = ผิดหวังน้อยกว่า (คนใจร้อนโกรธ)
use super::*;

pub const REQ_DAYS: i32 = 3;
pub const REQ_MAX: usize = 4;

#[derive(Clone, Debug, PartialEq)]
pub enum QK {
    Fetch { item: usize, qty: i32 },
    Hunt { animal: u8, n: i32, done: i32 },
    Deliver { item: usize, to: u32 },
    Mediate { a: u32, b: u32, ta: bool, tb: bool },
    Dispute { a: u32, b: u32 },
    Theft { thief: u32, item: usize, qty: i32 },
}

#[derive(Clone, Debug, PartialEq)]
pub struct NReq {
    pub from: u32,
    pub k: QK,
    pub day: i32,
    pub reward: i32,
}

/// ปุ่มที่ใช้ได้ในหน้าคุยกับชาวบ้าน
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum QA { Hand, Report, DeliverTo, Side(u32), Compromise, Reclaim, Forgive, Expel, Decline }

impl Game {
    fn nmq(&self, id: u32) -> &'static str { self.idx(id).map(|i| NPC_NAMES[self.npcs[i].name as usize]).unwrap_or("?") }
    fn qroll(&self, a: u32, salt: i32) -> f64 { hash(day_of(self).wrapping_mul(977).wrapping_add(a as i32), salt, 6100) }

    pub fn req_text(&self, r: &NReq) -> String {
        let f = self.nmq(r.from);
        match &r.k {
            QK::Fetch { item, qty } => format!("{}: \"ช่วยหา{} {} ชิ้นให้หน่อยได้ไหม\" (รางวัล {} เหรียญ)", f, INAME[*item], qty, r.reward),
            QK::Hunt { animal, n, done } => format!("{}: \"{}แถวนี้ทำให้ฉันกลัวจนไม่กล้าออกไปไหน ช่วยกำจัด {} ตัวที\" ({}/{} · รางวัล {} เหรียญ)", f, AN_NAME[*animal as usize], n, done, n, r.reward),
            QK::Deliver { item, to } => format!("{}: \"ช่วยเอา{}ไปให้{}หน่อย เขาชอบมาก\" (ใช้ของของคุณ · คุยกับ{}เพื่อส่ง)", f, INAME[*item], self.nmq(*to), self.nmq(*to)),
            QK::Mediate { a, b, ta, tb } => format!("{}: \"{} กับ {} ทะเลาะกันอีกแล้ว ช่วยไปคุยกับทั้งสองคนในวันเดียวกันทีนะ\" ({}{})", f, self.nmq(*a), self.nmq(*b), if *ta { "✓" } else { "·" }, if *tb { "✓" } else { "·" }),
            QK::Dispute { a, b } => format!("ข้อพิพาท: {} กับ {} เกลียดกันมาก ทั้งคู่อยากให้คุณตัดสินว่าใครถูก", self.nmq(*a), self.nmq(*b)),
            QK::Theft { thief, item, qty } => format!("{}: \"ฉันเห็นนะ {} แอบเอา{} {} ชิ้นไปจากกล่องเก็บของ\" — จะจัดการอย่างไร?", f, self.nmq(*thief), INAME[*item], qty),
        }
    }
    /// คำขอที่เกี่ยวกับชาวบ้านคนนี้ (เป็นคนขอ หรือเป็นคู่กรณี/ผู้รับ)
    pub fn reqs_for(&self, id: u32) -> Vec<usize> {
        (0..self.nreqs.len()).filter(|&i| {
            let r = &self.nreqs[i];
            r.from == id || match r.k { QK::Deliver { to, .. } => to == id, QK::Dispute { a, b } => a == id || b == id, _ => false }
        }).collect()
    }
    pub fn has_req(&self, id: u32) -> bool { self.nreqs.iter().any(|r| r.from == id) }

    /// รางวัลตามนิสัย: คนโลภจ่ายน้อย คนใจดีจ่ายมาก
    fn pay(&self, id: u32, base: i32) -> i32 {
        let Some(i) = self.idx(id) else { return base };
        let t = &self.npcs[i].soc.tr;
        ((base as f32) * (1.25 - t[T_GREED] as f32 / 150. + t[T_KIND] as f32 / 300.)).round().max(1.) as i32
    }

    // ---------------- เกิดคำขอใหม่ตอนรุ่งเช้า ----------------
    pub fn quest_dawn(&mut self) {
        let day = day_of(self);
        // ชาวบ้านที่เกี่ยวข้องย้ายออกไปแล้ว → คำขอหายไป
        let alive: HashSet<u32> = self.npcs.iter().filter(|n| n.state == ST_HOME).map(|n| n.id).collect();
        self.nreqs.retain(|r| alive.contains(&r.from) && match r.k { QK::Deliver { to, .. } => alive.contains(&to), QK::Mediate { a, b, .. } | QK::Dispute { a, b } => alive.contains(&a) && alive.contains(&b), QK::Theft { thief, .. } => alive.contains(&thief), _ => true });
        // หมดเวลา → ผิดหวัง
        let mut keep = vec![];
        for r in std::mem::take(&mut self.nreqs) {
            if day - r.day <= REQ_DAYS { keep.push(r); continue; }
            let nm = self.nmq(r.from);
            match r.k {
                QK::Dispute { a, b } => {
                    for id in [a, b] { if let Some(i) = self.idx(id) { let n = &mut self.npcs[i]; n.mood = (n.mood - 6).max(0); n.soc.grudge = (n.soc.grudge + 5).min(100); } }
                    self.news_add(format!("ข้อพิพาทระหว่าง {} กับ {} ไม่ได้รับการตัดสิน ทั้งคู่ผิดหวังในตัวคุณ", self.nmq(a), self.nmq(b)));
                }
                _ => {
                    if let Some(i) = self.idx(r.from) {
                        let n = &mut self.npcs[i];
                        n.mood = (n.mood - 5).max(0);
                        n.soc.anger = (n.soc.anger + n.soc.tr[T_TEMPER] as i32 / 4).min(100);
                    }
                    self.news_add(format!("{} ผิดหวังที่คุณไม่ช่วยตามที่ขอ", nm));
                }
            }
        }
        self.nreqs = keep;
        let ids: Vec<u32> = self.npcs.iter().filter(|n| n.state == ST_HOME && !n.child && !refuses(n)).map(|n| n.id).collect();
        // ขโมย (ก่อนคำขออื่น เพราะต้องมีพยาน)
        for &id in &ids {
            if self.nreqs.len() >= REQ_MAX { break; }
            let Some(i) = self.idx(id) else { continue };
            let (tr, mood, bed) = (self.npcs[i].soc.tr, self.npcs[i].mood, self.npcs[i].bed);
            if tr[T_GREED] < 70 || tr[T_KIND] > 35 || mood >= 70 { continue; }
            if self.qroll(id, 1) > 0.18 { continue; }
            let chest = self.chests.iter().filter(|(k, c)| (k.0 - bed.0).abs() <= 20 && (k.1 - bed.1).abs() <= 20 && (c[COIN] > 0 || c[GEM] > 0)).map(|(k, _)| *k).next();
            let Some(ck) = chest else { continue };
            let c = self.chests.get_mut(&ck).unwrap();
            let (item, qty) = if c[GEM] > 0 { (GEM, 1) } else { (COIN, c[COIN].min(10)) };
            c[item] -= qty;
            self.news_add(format!("{} หายไปจากกล่องเก็บของ {} ชิ้น!", INAME[item], qty));
            let witness = ids.iter().copied().filter(|&w| w != id && !self.has_req(w)).max_by_key(|&w| self.idx(w).map_or(0, |j| self.npcs[j].soc.tr[T_SOCIAL] as i32 + self.npcs[j].soc.tr[T_KIND] as i32 / 2));
            if let Some(w) = witness {
                self.nreqs.push(NReq { from: w, k: QK::Theft { thief: id, item, qty }, day, reward: 0 });
            }
        }
        // ข้อพิพาท: คู่ที่เกลียดกันมาก
        'd: for x in 0..ids.len() {
            for y in x + 1..ids.len() {
                if self.nreqs.len() >= REQ_MAX { break 'd; }
                let (a, b) = (ids[x], ids[y]);
                if self.rel_between(a, b) > -50 || self.has_req(a) || self.has_req(b) { continue; }
                if self.nreqs.iter().any(|r| matches!(r.k, QK::Dispute { a: p, b: q } | QK::Mediate { a: p, b: q, .. } if (p, q) == (a, b) || (p, q) == (b, a))) { continue; }
                if self.qroll(a ^ (b << 8), 2) < 0.5 {
                    self.nreqs.push(NReq { from: a, k: QK::Dispute { a, b }, day, reward: 0 });
                    self.news_add(format!("{} กับ {} ขอให้คุณช่วยตัดสินเรื่องที่ทะเลาะกัน", self.nmq(a), self.nmq(b)));
                }
            }
        }
        // คำขอส่วนตัว
        for &id in &ids {
            if self.nreqs.len() >= REQ_MAX { break; }
            if self.has_req(id) { continue; }
            let Some(i) = self.idx(id) else { continue };
            let n = self.npcs[i].clone();
            let tr = n.soc.tr;
            let roll = self.qroll(id, 3);
            // ไกล่เกลี่ย: คนใจดีทนเห็นเพื่อนทะเลาะกันไม่ได้
            if tr[T_KIND] >= 55 {
                let pair = ids.iter().flat_map(|&p| ids.iter().map(move |&q| (p, q))).find(|&(p, q)| p < q && p != id && q != id && self.rel_between(p, q) <= -30 && self.rel_between(p, q) > -50);
                if let Some((a, b)) = pair {
                    if roll < 0.35 && !self.nreqs.iter().any(|r| matches!(r.k, QK::Mediate { a: p, b: q, .. } if (p, q) == (a, b))) {
                        let reward = self.pay(id, 10);
                        self.nreqs.push(NReq { from: id, k: QK::Mediate { a, b, ta: false, tb: false }, day, reward });
                        continue;
                    }
                }
            }
            // หิว/ไม่มีความสุข: ขอของกินที่ชอบ
            if n.mood < 55 && roll < 0.4 {
                let item = if nutrition(n.soc.love as usize).is_some() { n.soc.love as usize } else { [BERRY, MEAT, FISH][(self.qroll(id, 4) * 3.) as usize % 3] };
                let qty = 2 + (self.qroll(id, 5) * 3.) as i32;
                let reward = self.pay(id, 4 * qty);
                self.nreqs.push(NReq { from: id, k: QK::Fetch { item, qty }, day, reward });
                continue;
            }
            // ขี้ขลาด: กลัวสัตว์ดุ
            if tr[T_BRAVE] < 35 && roll < 0.22 {
                let animal = [A_WOLF, A_BOAR, A_WOLF, A_CROC][(self.qroll(id, 6) * 4.) as usize % 4];
                let nn = if animal == A_CROC { 1 } else { 2 };
                let reward = self.pay(id, 12 * nn);
                self.nreqs.push(NReq { from: id, k: QK::Hunt { animal, n: nn, done: 0 }, day, reward });
                continue;
            }
            // โลภ: อยากได้อัญมณี (จ่ายเหรียญให้)
            if tr[T_GREED] >= 60 && roll < 0.15 {
                let qty = 1 + (self.qroll(id, 7) * 2.) as i32;
                self.nreqs.push(NReq { from: id, k: QK::Fetch { item: GEM, qty }, day, reward: 18 * qty });
                continue;
            }
            // ช่างพูด: ฝากของขวัญถึงเพื่อนสนิท
            if tr[T_SOCIAL] >= 60 && roll < 0.2 {
                let friend = ids.iter().copied().filter(|&o| o != id && self.rel_between(id, o) >= 25).max_by_key(|&o| self.rel_between(id, o));
                if let Some(to) = friend {
                    let item = self.idx(to).map(|j| self.npcs[j].soc.love as usize).unwrap_or(BERRY);
                    self.nreqs.push(NReq { from: id, k: QK::Deliver { item, to }, day, reward: 0 });
                }
            }
        }
    }

    // ---------------- ความคืบหน้า ----------------
    pub fn quest_hunt(&mut self, animal: u8) {
        for r in self.nreqs.iter_mut() {
            if let QK::Hunt { animal: a, n, done } = &mut r.k {
                if *a == animal && *done < *n { *done += 1; }
            }
        }
    }
    /// คุยกับชาวบ้าน → นับความคืบหน้าการไกล่เกลี่ย
    pub fn quest_talked(&mut self, id: u32) {
        let mut done: Option<usize> = None;
        for (k, r) in self.nreqs.iter_mut().enumerate() {
            if let QK::Mediate { a, b, ta, tb } = &mut r.k {
                if *a == id { *ta = true; }
                if *b == id { *tb = true; }
                if *ta && *tb { done = Some(k); }
            }
        }
        if let Some(k) = done {
            let r = self.nreqs.remove(k);
            if let QK::Mediate { a, b, .. } = r.k {
                let cur = self.rel_between(a, b);
                self.set_rel(a, b, (cur + 35).max(15));
                for x in [a, b, r.from] { if let Some(i) = self.idx(x) { let n = &mut self.npcs[i]; n.hearts = (n.hearts + 1).min(10); n.mood = (n.mood + 6).min(100); } }
                self.p.inv[COIN] += r.reward;
                self.news_add(format!("คุณช่วยให้ {} กับ {} คืนดีกัน", self.nmq(a), self.nmq(b)));
                self.toast(format!("ไกล่เกลี่ยสำเร็จ! {} กับ {} คืนดีกัน · {} ขอบคุณ (+{} เหรียญ)", self.nmq(a), self.nmq(b), self.nmq(r.from), r.reward));
                play(&self.sfx.lvl);
            }
        }
    }
    /// ปลายเที่ยวของรอบรายวัน: ไกล่เกลี่ยต้องคุยวันเดียวกัน
    pub fn quest_reset_daily(&mut self) {
        for r in self.nreqs.iter_mut() { if let QK::Mediate { ta, tb, .. } = &mut r.k { *ta = false; *tb = false; } }
    }

    /// ปุ่มที่ใช้ได้กับคำขอ ri เมื่อคุยกับ id
    pub fn req_actions(&self, ri: usize, id: u32) -> Vec<(QA, String, bool)> {
        let r = &self.nreqs[ri];
        let mut v = vec![];
        match &r.k {
            QK::Fetch { item, qty } if r.from == id => v.push((QA::Hand, format!("มอบ{} x{}", INAME[*item], qty), self.p.inv[*item] >= *qty)),
            QK::Hunt { n, done, .. } if r.from == id => v.push((QA::Report, if done >= n { "รายงานผล".to_string() } else { format!("ล่าแล้ว {}/{}", done, n) }, done >= n)),
            QK::Deliver { item, to } if *to == id => v.push((QA::DeliverTo, format!("ส่ง{}จาก{}", INAME[*item], self.nmq(r.from)), self.p.inv[*item] >= 1)),
            QK::Dispute { a, b } => {
                v.push((QA::Side(*a), format!("เข้าข้าง{}", self.nmq(*a)), true));
                v.push((QA::Side(*b), format!("เข้าข้าง{}", self.nmq(*b)), true));
                v.push((QA::Compromise, "ให้ประนีประนอม".to_string(), true));
            }
            QK::Theft { .. } if r.from == id => {
                v.push((QA::Reclaim, "ทวงของคืน".to_string(), true));
                v.push((QA::Forgive, "ยกโทษให้".to_string(), true));
                v.push((QA::Expel, "ไล่ออก".to_string(), true));
            }
            _ => {}
        }
        if r.from == id && !matches!(r.k, QK::Theft { .. } | QK::Dispute { .. }) { v.push((QA::Decline, "ปฏิเสธ".to_string(), true)); }
        v
    }

    fn thank(&mut self, id: u32, coins: i32, hearts: u8) {
        if let Some(i) = self.idx(id) {
            let n = &mut self.npcs[i];
            n.hearts = (n.hearts + hearts).min(10);
            n.mood = (n.mood + 10).min(100);
            n.soc.anger = 0;
            n.soc.grudge = (n.soc.grudge - 10).max(0);
        }
        if coins > 0 { self.p.inv[COIN] += coins; }
        self.p.xp += 5;
        self.check_lv();
        play(&self.sfx.lvl);
    }

    /// ทำตามปุ่ม — คืนข้อความผลลัพธ์
    pub fn req_do(&mut self, ri: usize, act: QA) -> Option<String> {
        let r = self.nreqs.get(ri)?.clone();
        let fname = self.nmq(r.from);
        let msg = match (&r.k, act) {
            (QK::Fetch { item, qty }, QA::Hand) => {
                if self.p.inv[*item] < *qty { return None; }
                self.p.inv[*item] -= qty;
                self.thank(r.from, r.reward, 1);
                format!("{} ดีใจมาก! (+{} เหรียญ · สนิทขึ้น)", fname, r.reward)
            }
            (QK::Hunt { n, done, animal }, QA::Report) => {
                if done < n { return None; }
                self.thank(r.from, r.reward, 1);
                self.news_add(format!("คุณกำจัด{}ที่ทำให้ {} กลัว", AN_NAME[*animal as usize], fname));
                format!("{} รู้สึกปลอดภัยขึ้น (+{} เหรียญ)", fname, r.reward)
            }
            (QK::Deliver { item, to }, QA::DeliverTo) => {
                if self.p.inv[*item] < 1 { return None; }
                self.p.inv[*item] -= 1;
                self.add_rel(r.from, *to, 18);
                if let Some(i) = self.idx(*to) { self.npcs[i].mood = (self.npcs[i].mood + 8).min(100); }
                self.thank(r.from, 0, 1);
                self.news_add(format!("{} ได้รับของขวัญจาก {} สองคนสนิทกันขึ้น", self.nmq(*to), fname));
                format!("ส่งของให้{}แล้ว — {} ขอบคุณคุณ", self.nmq(*to), fname)
            }
            (QK::Dispute { a, b }, QA::Side(w)) => {
                let l = if w == *a { *b } else { *a };
                if let Some(i) = self.idx(w) { let n = &mut self.npcs[i]; n.hearts = (n.hearts + 1).min(10); n.mood = (n.mood + 8).min(100); }
                if let Some(i) = self.idx(l) { let n = &mut self.npcs[i]; n.soc.grudge = (n.soc.grudge + 25).min(100); n.soc.anger = (n.soc.anger + 30).min(100); n.mood = (n.mood - 8).max(0); }
                self.add_rel(*a, *b, -10);
                // คนอื่นๆ มองตามความสัมพันธ์กับสองฝ่าย
                let others: Vec<u32> = self.npcs.iter().filter(|n| n.state == ST_HOME && n.id != *a && n.id != *b).map(|n| n.id).collect();
                for o in others {
                    let d = (self.rel_between(o, w) - self.rel_between(o, l)).signum() * 3;
                    if let Some(i) = self.idx(o) { self.npcs[i].mood = (self.npcs[i].mood + d).clamp(0, 100); }
                }
                self.news_add(format!("คุณตัดสินให้ {} ถูก — {} ไม่พอใจอย่างมาก", self.nmq(w), self.nmq(l)));
                format!("เข้าข้าง{} · {} โกรธคุณ", self.nmq(w), self.nmq(l))
            }
            (QK::Dispute { a, b }, QA::Compromise) => {
                let ha = self.idx(*a).map_or(0, |i| self.npcs[i].hearts);
                let hb = self.idx(*b).map_or(0, |i| self.npcs[i].hearts);
                if ha >= 5 && hb >= 5 {
                    let cur = self.rel_between(*a, *b);
                    self.set_rel(*a, *b, (cur + 45).max(10));
                    for id in [*a, *b] { if let Some(i) = self.idx(id) { self.npcs[i].mood = (self.npcs[i].mood + 6).min(100); } }
                    self.news_add(format!("{} กับ {} ยอมประนีประนอมเพราะเชื่อใจคุณ", self.nmq(*a), self.nmq(*b)));
                    "ทั้งคู่ยอมฟังคุณและหันหน้าเข้าหากัน".to_string()
                } else {
                    for id in [*a, *b] { if let Some(i) = self.idx(id) { let n = &mut self.npcs[i]; n.soc.anger = (n.soc.anger + 12).min(100); } }
                    self.news_add(format!("{} กับ {} ไม่ยอมฟังคำไกล่เกลี่ยของคุณ", self.nmq(*a), self.nmq(*b)));
                    "ไม่มีใครฟัง — ต้องสนิทกับทั้งคู่ (5 ขึ้นไป) ก่อน".to_string()
                }
            }
            (QK::Theft { thief, item, qty }, QA::Reclaim) => {
                self.p.inv[*item] += qty;
                if let Some(i) = self.idx(*thief) { let n = &mut self.npcs[i]; n.soc.anger = (n.soc.anger + 30).min(100); n.soc.grudge = (n.soc.grudge + 15).min(100); n.soc.tr[T_GREED] = n.soc.tr[T_GREED].saturating_sub(5); }
                self.add_rel(r.from, *thief, -20);
                self.thank(r.from, 0, 1);
                self.news_add(format!("{} ถูกทวงของที่ขโมยคืน", self.nmq(*thief)));
                format!("ได้{}คืน {} ชิ้น · {} ไม่พอใจที่ถูกจับได้", INAME[*item], qty, self.nmq(*thief))
            }
            (QK::Theft { thief, .. }, QA::Forgive) => {
                if let Some(i) = self.idx(*thief) { let n = &mut self.npcs[i]; n.hearts = (n.hearts + 1).min(10); n.soc.tr[T_GREED] = n.soc.tr[T_GREED].saturating_sub(15); n.soc.tr[T_KIND] = (n.soc.tr[T_KIND] + 8).min(100); }
                if let Some(i) = self.idx(r.from) { self.npcs[i].mood = (self.npcs[i].mood - 4).max(0); }
                self.news_add(format!("คุณยกโทษให้ {} — เขาสำนึกผิดและสัญญาว่าจะไม่ทำอีก", self.nmq(*thief)));
                format!("{} สำนึกผิด (ความโลภลดลง) · {} ไม่ค่อยพอใจ", self.nmq(*thief), fname)
            }
            (QK::Theft { thief, item, qty }, QA::Expel) => {
                let tn = self.nmq(*thief);
                let friends: Vec<u32> = self.npcs.iter().filter(|n| n.state == ST_HOME && n.id != *thief).map(|n| n.id).collect();
                for f in friends {
                    let d = if self.rel_between(f, *thief) >= 20 { -6 } else { 2 };
                    if let Some(i) = self.idx(f) { self.npcs[i].mood = (self.npcs[i].mood + d).clamp(0, 100); }
                }
                self.p.inv[*item] += qty;
                if let Some(i) = self.idx(*thief) { self.npcs.remove(i); }
                self.nreqs.retain(|q| q.from != *thief);
                self.thank(r.from, 0, 1);
                self.news_add(format!("{} ถูกไล่ออกจากหมู่บ้านเพราะขโมยของ", tn));
                format!("ไล่{}ออก ได้ของคืน · เพื่อนของเขาเสียใจ", tn)
            }
            (_, QA::Decline) => {
                if let Some(i) = self.idx(r.from) { let n = &mut self.npcs[i]; n.mood = (n.mood - 3).max(0); n.soc.anger = (n.soc.anger + n.soc.tr[T_TEMPER] as i32 / 8).min(100); }
                format!("ปฏิเสธ{}แล้ว", fname)
            }
            _ => return None,
        };
        if let Some(k) = self.nreqs.iter().position(|q| *q == r) { self.nreqs.remove(k); }
        self.save();
        Some(msg)
    }

    // ---------------- เซฟ ----------------
    pub fn quests_save_str(&self) -> String {
        let mut s = String::new();
        for r in &self.nreqs {
            let (c, a, b, d, e) = match r.k {
                QK::Fetch { item, qty } => (0, item as i32, qty, 0, 0),
                QK::Hunt { animal, n, done } => (1, animal as i32, n, done, 0),
                QK::Deliver { item, to } => (2, item as i32, to as i32, 0, 0),
                QK::Mediate { a, b, ta, tb } => (3, a as i32, b as i32, ta as i32, tb as i32),
                QK::Dispute { a, b } => (4, a as i32, b as i32, 0, 0),
                QK::Theft { thief, item, qty } => (5, thief as i32, item as i32, qty, 0),
            };
            s += &format!("nreq {} {} {} {} {} {} {} {}\n", r.from, r.day, r.reward, c, a, b, d, e);
        }
        s
    }
    pub fn quests_load_line(&mut self, rest: &[&str]) {
        let f = |i: usize| rest.get(i).and_then(|v| v.parse::<i32>().ok()).unwrap_or(0);
        if rest.len() < 8 { return; }
        let (a, b, d, e) = (f(4), f(5), f(6), f(7));
        let k = match f(3) {
            0 => QK::Fetch { item: (a as usize).min(NI - 1), qty: b },
            1 => QK::Hunt { animal: a as u8, n: b, done: d },
            2 => QK::Deliver { item: (a as usize).min(NI - 1), to: b as u32 },
            3 => QK::Mediate { a: a as u32, b: b as u32, ta: d != 0, tb: e != 0 },
            4 => QK::Dispute { a: a as u32, b: b as u32 },
            5 => QK::Theft { thief: a as u32, item: (b as usize).min(NI - 1), qty: d },
            _ => return,
        };
        self.nreqs.push(NReq { from: f(0) as u32, day: f(1), reward: f(2), k });
    }
}
