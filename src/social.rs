// social.rs — นิสัย ความชอบ อารมณ์ ความสัมพันธ์ของชาวบ้าน (v7a)
// แนวคิด: ทุกอย่างมีเหตุผลแบบชีวิตจริง
//  - ทุกคนมีนิสัย 5 แบบ (ใจดี/ใจร้อน/โลภ/กล้า/ช่างพูด) และของที่ชอบ/เกลียด (รู้จักกันนานขึ้นถึงจะรู้)
//  - โกรธ = อารมณ์ระยะสั้น (หายใน 1-3 วัน) · แค้น = ความฝังใจระยะยาว (จางช้า คนใจดีให้อภัยเร็วกว่า)
//  - ความรู้สึกต่อผู้เล่น = สนิท*10 − แค้น − โกรธ/2 → กำหนดว่าจะคุย/ขาย/ทำงานให้หรือไม่
//  - ชาวบ้านมีความสัมพันธ์กันเอง (ทะเลาะ เป็นเพื่อน ง้อ ย้ายออก) เหตุการณ์เกิดตอนรุ่งเช้าแบบกำหนดแน่นอนจากวัน+id
use super::*;

pub const TR_N: [&str; 5] = ["ใจดี", "ใจร้อน", "โลภ", "กล้าหาญ", "ช่างพูด"];
pub const TR_LOW: [&str; 5] = ["เย็นชา", "ใจเย็น", "ไม่โลภ", "ขี้ขลาด", "ขี้อาย"];
pub const T_KIND: usize = 0;
pub const T_TEMPER: usize = 1;
pub const T_GREED: usize = 2;
pub const T_BRAVE: usize = 3;
pub const T_SOCIAL: usize = 4;
/// ของที่ให้เป็นของขวัญได้
pub const GIFTS: [usize; 10] = [GEM, BERRY, CORN, CARROT, FISH, MEAT, HERB, SHROOM, DISH_STEW, DISH_FISH];

pub fn gift_base(item: usize) -> i32 {
    match item { GEM | DISH_STEW | DISH_FISH => 2, _ => 1 }
}

#[derive(Clone, Debug)]
pub struct Soc {
    pub tr: [u8; 5],
    pub love: u8,
    pub hate: u8,
    pub anger: i32,
    pub grudge: i32,
    pub last_talk: i32,
    pub last_gift: u8,
    pub bore: u8,
    pub apol_day: i32,
    pub hit_cd: f32,
}
impl Soc {
    pub fn generate(id: u32, name: u8) -> Soc {
        let h = |k: i32| hash(id as i32, name as i32 * 7 + 3, 4100 + k);
        let tr = [0, 1, 2, 3, 4].map(|k| (10. + h(k) * 85.) as u8);
        let n = GIFTS.len();
        let li = (h(10) * n as f64) as usize % n;
        let mut hi = (h(11) * n as f64) as usize % n;
        if hi == li { hi = (li + 1 + (h(12) * 8.) as usize) % n; }
        if hi == li { hi = (li + 1) % n; }
        Soc { tr, love: GIFTS[li] as u8, hate: GIFTS[hi] as u8, anger: 0, grudge: 0, last_talk: 0, last_gift: 255, bore: 0, apol_day: -1, hit_cd: 0. }
    }
    /// ลำดับนิสัยเด่น → (ดัชนี, สูง?) เรียงจากเด่นสุด
    pub fn traits_ranked(&self) -> Vec<(usize, bool)> {
        let mut v: Vec<(usize, bool)> = (0..5).map(|k| (k, self.tr[k] >= 50)).collect();
        v.sort_by_key(|&(k, _)| -((self.tr[k] as i32 - 50).abs()));
        v
    }
    pub fn trait_name(k: usize, high: bool) -> &'static str { if high { TR_N[k] } else { TR_LOW[k] } }
    pub fn top_trait(&self) -> &'static str {
        let (k, hi) = self.traits_ranked()[0];
        Soc::trait_name(k, hi)
    }
}

pub fn opinion(n: &Npc) -> i32 { n.hearts as i32 * 10 - n.soc.grudge - n.soc.anger / 2 }
pub fn opinion_label(op: i32) -> &'static str {
    match op {
        i32::MIN..=-61 => "เกลียดคุณ",
        -60..=-31 => "ไม่ชอบหน้า",
        -30..=-6 => "ไม่ไว้ใจ",
        -5..=14 => "คนแปลกหน้า",
        15..=39 => "คุ้นเคย",
        40..=69 => "เพื่อน",
        70..=89 => "เพื่อนสนิท",
        _ => "คนสำคัญ",
    }
}
/// ตอนนี้ไม่ยอมคุย/รับของ/ขายของ
pub fn refuses(n: &Npc) -> bool { n.soc.anger >= 40 || opinion(n) <= -40 }

pub fn base_rel(a: &Npc, b: &Npc) -> i32 {
    let (ta, tb) = (&a.soc.tr, &b.soc.tr);
    let d = |k: usize| (ta[k] as i32 - tb[k] as i32).abs();
    let mut r = 18 - d(T_SOCIAL) / 4 - d(T_KIND) / 6;
    r += (ta[T_KIND] as i32 + tb[T_KIND] as i32 - 100) / 10;
    r -= (ta[T_TEMPER] as i32 + tb[T_TEMPER] as i32 - 100) / 8;
    r -= (ta[T_GREED] as i32 * tb[T_GREED] as i32 / 100 - 25) / 5;
    r += ((hash(a.id.min(b.id) as i32, a.id.max(b.id) as i32, 4900) - 0.5) * 24.) as i32;
    r.clamp(-60, 60)
}

impl Game {
    pub fn news_add(&mut self, s: String) {
        let day = day_of(self);
        self.news.push((day, s));
        if self.news.len() > 40 { self.news.remove(0); }
    }
    fn roll(&self, a: u32, b: u32, salt: i32) -> f64 {
        hash((day_of(self) as i32).wrapping_mul(131).wrapping_add(a as i32), b as i32, 5000 + salt)
    }
    pub fn rel_between(&self, a: u32, b: u32) -> i32 {
        let (Some(ia), Some(ib)) = (self.idx(a), self.idx(b)) else { return 0 };
        let base = base_rel(&self.npcs[ia], &self.npcs[ib]);
        (base + self.rels.get(&(a.min(b), a.max(b))).copied().unwrap_or(0)).clamp(-100, 100)
    }
    /// ตั้งความสัมพันธ์ให้ได้ค่าที่ต้องการ (−100..100)
    pub fn set_rel(&mut self, a: u32, b: u32, target: i32) {
        let (Some(ia), Some(ib)) = (self.idx(a), self.idx(b)) else { return };
        let base = base_rel(&self.npcs[ia], &self.npcs[ib]);
        self.rels.insert((a.min(b), a.max(b)), (target.clamp(-100, 100) - base).clamp(-150, 150));
    }
    pub fn add_rel(&mut self, a: u32, b: u32, d: i32) {
        let e = self.rels.entry((a.min(b), a.max(b))).or_insert(0);
        *e = (*e + d).clamp(-150, 150);
    }
    fn nm(&self, id: u32) -> &'static str {
        self.idx(id).map(|i| NPC_NAMES[self.npcs[i].name as usize]).unwrap_or("?")
    }

    // ---------------- ผู้เล่นทำอะไรกับชาวบ้าน ----------------
    pub fn npc_talk(&mut self, id: u32) {
        let Some(i) = self.idx(id) else { return };
        let day = day_of(self);
        let nm = NPC_NAMES[self.npcs[i].name as usize];
        if self.npcs[i].talked { self.toast("วันนี้คุยกันไปแล้ว พรุ่งนี้ค่อยมาใหม่"); return; }
        if refuses(&self.npcs[i]) {
            self.toast(format!("{} ยังโกรธ/ไม่พอใจอยู่ ไม่อยากคุยด้วย (ลองขอโทษหรือให้ของขวัญที่เขาชอบ)", nm));
            return;
        }
        let open = self.npcs[i].soc.tr[T_SOCIAL] as f64;
        let ok = self.roll(id, 77, 10) < 0.45 + open / 200.;
        let n = &mut self.npcs[i];
        n.talked = true;
        n.soc.last_talk = day;
        n.mood = (n.mood + 5 + (n.soc.tr[T_SOCIAL] as i32) / 20).min(100);
        if ok { n.hearts = (n.hearts + 1).min(10); }
        self.toast(if ok { format!("คุยกับ{} — สนิทกันขึ้น", nm) } else { format!("คุยกับ{} — เขาไม่ค่อยเปิดใจ ค่อยๆ ทำความรู้จักกันไป", nm) });
        play(&self.sfx.pickup);
        self.quest_talked(id);
        self.save();
    }
    pub fn npc_gift_item(&mut self, id: u32, item: usize) {
        let Some(i) = self.idx(id) else { return };
        let nm = NPC_NAMES[self.npcs[i].name as usize];
        if self.npcs[i].gifted { self.toast("วันนี้ให้ของขวัญไปแล้ว"); return; }
        if self.p.inv[item] < 1 { return; }
        if refuses(&self.npcs[i]) { self.toast(format!("{} ไม่ยอมรับของขวัญตอนนี้", nm)); return; }
        let day = day_of(self);
        self.p.inv[item] -= 1;
        let n = &mut self.npcs[i];
        n.gifted = true;
        n.soc.last_talk = day;
        let mut v = if item == n.soc.love as usize { 3 } else if item == n.soc.hate as usize { -1 } else { gift_base(item) };
        let mut line = if v == 3 { "ชอบมาก! เหมือนรู้ใจกันเลย" } else if v < 0 { "ของแบบนี้... ฉันไม่ชอบเลย" } else { "ขอบคุณนะ" };
        if n.soc.last_gift as usize == item { n.soc.bore = n.soc.bore.saturating_add(1); } else { n.soc.bore = 0; n.soc.last_gift = item as u8; }
        if n.soc.bore >= 2 && v > 0 { v /= 2; line = "ของชิ้นนี้เบื่อแล้วนะ ขอเปลี่ยนอย่างอื่นบ้าง"; }
        if v > 0 {
            n.hearts = (n.hearts as i32 + v).clamp(0, 10) as u8;
            n.mood = (n.mood + 6 + v * 5).min(100);
            n.soc.anger = (n.soc.anger - 12 * v).max(0);
            n.soc.grudge = (n.soc.grudge - 3 * v).max(0);
        } else if v < 0 {
            n.hearts = n.hearts.saturating_sub(1);
            n.soc.anger = (n.soc.anger + 15).min(100);
            n.soc.grudge = (n.soc.grudge + 3).min(100);
        }
        self.toast(format!("มอบ{}ให้{} — \"{}\"", INAME[item], nm, line));
        play(&self.sfx.gem);
        self.save();
    }
    pub fn npc_apologize(&mut self, id: u32) {
        let Some(i) = self.idx(id) else { return };
        let day = day_of(self);
        let nm = NPC_NAMES[self.npcs[i].name as usize];
        let n = &self.npcs[i];
        if n.soc.anger <= 0 && n.soc.grudge <= 0 { self.toast(format!("ไม่มีเรื่องที่ต้องขอโทษ{}", nm)); return; }
        if n.soc.apol_day == day { self.toast("วันนี้ขอโทษไปแล้ว ให้เวลาเขาหน่อย"); return; }
        let p = 0.35 + n.soc.tr[T_KIND] as f64 / 200. + n.hearts as f64 * 0.04 - n.soc.anger as f64 / 300.;
        let ok = self.roll(id, 78, 11) < p;
        let n = &mut self.npcs[i];
        n.soc.apol_day = day;
        if ok {
            n.soc.anger = (n.soc.anger - 35).max(0);
            n.soc.grudge = (n.soc.grudge - 8 - n.soc.tr[T_KIND] as i32 / 10).max(0);
            n.mood = (n.mood + 5).min(100);
            self.toast(format!("{} ยอมรับคำขอโทษ", nm));
        } else {
            n.soc.anger = (n.soc.anger - 5).max(0);
            self.toast(format!("{} ยังไม่หายโกรธ คำขอโทษเปล่าๆ ยังไม่พอ", nm));
        }
        self.save();
    }
    /// ผู้เล่นฟันโดนชาวบ้าน (เฉพาะตอนไม่ได้โดนมอนสเตอร์)
    pub fn npc_hit(&mut self, i: usize) {
        if self.npcs[i].soc.hit_cd > 0. { return; }
        let nm = NPC_NAMES[self.npcs[i].name as usize];
        let n = &mut self.npcs[i];
        n.soc.hit_cd = 1.0;
        n.soc.anger = (n.soc.anger + 45).min(100);
        n.soc.grudge = (n.soc.grudge + 12).min(100);
        n.hearts = n.hearts.saturating_sub(2);
        n.mood = (n.mood - 10).max(0);
        let (x, y) = (n.x, n.y - 14.);
        self.add_float(x, y, "โอ๊ย!", hx(0xff6666), None);
        self.toast(format!("ระวัง! คุณฟันโดน{} — เขาโกรธมาก", nm));
    }
    /// ชาวบ้านคนอื่นรู้สึกอย่างไรเมื่อคุณไล่ใครสักคน
    pub fn social_on_dismiss(&mut self, id: u32) {
        let others: Vec<u32> = self.npcs.iter().filter(|n| n.id != id && n.state == ST_HOME && !n.child).map(|n| n.id).collect();
        let gone = self.nm(id);
        let mut upset = 0;
        for o in others {
            if self.rel_between(o, id) >= 15 {
                if let Some(i) = self.idx(o) {
                    let n = &mut self.npcs[i];
                    n.mood = (n.mood - 8).max(0);
                    n.soc.grudge = (n.soc.grudge + 6).min(100);
                    upset += 1;
                }
            }
        }
        self.news_add(if upset > 0 { format!("{} ถูกไล่ออก เพื่อนๆ {} คนไม่พอใจ", gone, upset) } else { format!("{} ถูกไล่ออกจากหมู่บ้าน", gone) });
    }
    /// ข่าวแต่งงาน: คนสนิทมากอาจอกหัก คนใจดียินดีด้วย คนที่สนิทปานกลางมาร่วมแสดงความยินดี
    pub fn social_on_marry(&mut self, spouse: u32) {
        let sp = self.nm(spouse);
        let others: Vec<u32> = self.npcs.iter().filter(|n| n.id != spouse && n.state == ST_HOME && !n.child).map(|n| n.id).collect();
        for o in others {
            let Some(i) = self.idx(o) else { continue };
            let name = NPC_NAMES[self.npcs[i].name as usize];
            let n = &mut self.npcs[i];
            if n.hearts >= 8 {
                if n.soc.tr[T_KIND] >= 60 {
                    n.mood = (n.mood - 5).max(0);
                    self.news_add(format!("{} ยิ้มให้คู่บ่าวสาวทั้งที่ใจหายวาบ", name));
                } else {
                    n.hearts -= 3;
                    n.soc.grudge = (n.soc.grudge + 15).min(100);
                    n.soc.anger = (n.soc.anger + 30).min(100);
                    n.mood = (n.mood - 10).max(0);
                    self.news_add(format!("{} อิจฉา{}และไม่พอใจที่คุณเลือกแต่งงาน", name, sp));
                }
            } else if n.hearts >= 4 {
                n.mood = (n.mood + 8).min(100);
                n.hearts = (n.hearts + 1).min(10);
            }
        }
        self.news_add(format!("งานแต่งงานของคุณกับ{} จัดขึ้นอย่างอบอุ่น", sp));
    }

    // ---------------- ราคา/ความร่วมมือ ----------------
    /// ราคาที่พ่อค้าเสนอให้ผู้เล่น (None = ไม่ขายให้)
    pub fn npc_price(&self, id: u32, base: i32) -> Option<i32> {
        let n = &self.npcs[self.idx(id)?];
        let op = opinion(n);
        if op <= -20 || n.soc.anger >= 40 { return None; }
        let f = if op < 0 { 1.3 } else if op < 15 { 1.1 } else if op < 40 { 1.0 } else if op < 70 { 0.92 } else { 0.85 };
        let g = 1. + (n.soc.tr[T_GREED] as f32 - 50.) / 400.;
        Some(((base as f32 * f * g).round() as i32).max(1))
    }
    /// ยาม/ชาวไร่ยอมทำงานให้ไหม (ถ้าไม่ชอบหน้าคุณมากจะหยุดงาน)
    pub fn willing(n: &Npc) -> bool { opinion(n) > -30 && n.soc.anger < 60 }

    // ---------------- คำพูด ----------------
    pub fn npc_say(&self, n: &Npc) -> String {
        let t = &n.soc.tr;
        let pick = |v: &[&str]| v[(hash(day_of(self) as i32, n.id as i32, 5200) * v.len() as f64) as usize % v.len()].to_string();
        if n.state == ST_FOLLOW {
            return if t[T_BRAVE] < 35 { pick(&["ฉันกลัวมาก... อย่าทิ้งฉันไว้ข้างนอกนะ", "ขอบคุณที่มาช่วย ฉันไม่กล้าเดินคนเดียวอีกแล้ว"]) }
                else { pick(&["ขอบคุณที่ช่วยไว้! ถ้ามีเตียงว่างที่ฐาน ฉันอยากไปอยู่ด้วย", "ฉันหลงทางมาหลายวัน... พาฉันไปที่ปลอดภัยทีนะ"]) };
        }
        if n.soc.anger >= 40 {
            return if t[T_TEMPER] >= 55 { pick(&["อย่ามายุ่งกับฉัน!", "ไปให้พ้น! ตอนนี้ฉันกำลังหัวเสีย"]) } else { pick(&["...", "ฉันไม่อยากพูดกับคุณตอนนี้"]) };
        }
        if n.soc.grudge >= 30 { return pick(&["ฉันยังจำเรื่องนั้นได้นะ", "คุณคิดว่าฉันจะลืมง่ายๆ เหรอ"]); }
        if opinion(n) < -5 { return pick(&["คุณไม่ค่อยใส่ใจพวกเราเลยนะ", "ฉันยังไม่ไว้ใจคุณเท่าไหร่"]); }
        if n.mood < 25 { return "ฉันไม่ไหวแล้ว... หิวและเหนื่อยเหลือเกิน".to_string(); }
        if n.mood < 55 { return "ก็พออยู่ได้ แต่ถ้ามีอาหารมากกว่านี้คงดี".to_string(); }
        if n.married { return pick(&["วันนี้อากาศดีนะที่รัก", "มีคุณอยู่ข้างๆ ฉันก็สบายใจ"]); }
        if n.hearts >= 6 {
            return if t[T_KIND] >= 55 { "มีอะไรให้ช่วยบอกได้เลยนะ เราเป็นเพื่อนกัน".to_string() }
                else if t[T_SOCIAL] >= 55 { "มานั่งคุยกันหน่อยไหม วันนี้มีเรื่องสนุกเยอะเลย".to_string() }
                else { "...มีคุณเป็นเพื่อนก็ไม่เลวนะ".to_string() };
        }
        match n.soc.top_trait() {
            "ใจดี" => pick(&["ยินดีต้อนรับนะ มีอะไรให้ช่วยไหม", "วันนี้เป็นไงบ้าง เหนื่อยไหม"]),
            "ใจร้อน" => pick(&["มีอะไรก็ว่ามาเลย ฉันไม่ชอบรอ", "ถ้าจะคุยก็คุยให้เร็ว"]),
            "โลภ" => pick(&["ได้อะไรดีๆ มาบ้างไหม", "เหรียญเยอะๆ ไม่เสียหายหรอกนะ"]),
            "กล้าหาญ" => pick(&["มอนสเตอร์มาเมื่อไหร่ ฉันไม่กลัวหรอก", "ที่นี่ปลอดภัยแน่ถ้ามีฉัน"]),
            "ช่างพูด" => pick(&["อ้าว มาพอดี ฉันมีเรื่องจะเล่าเยอะเลย", "ได้ข่าวหรือยัง เรื่องเมื่อวานน่ะ"]),
            "ขี้อาย" => pick(&["เอ่อ... สวัสดี", "ฉันไม่ค่อยถนัดพูดนะ"]),
            "ขี้ขลาด" => pick(&["กลางคืนน่ากลัวจัง ขอให้ปลอดภัยเถอะ", "ฉันไม่กล้าออกไปไกลหรอก"]),
            "เย็นชา" => pick(&["อืม", "มีอะไร"]),
            _ => pick(&["ที่นี่อยู่สบายดี ขอบคุณนะ", "วันนี้ก็เรื่อยๆ"]),
        }
    }

    // ---------------- รุ่งเช้า ----------------
    /// คืนจำนวนข่าวใหม่
    pub fn social_dawn(&mut self) -> usize {
        let day = day_of(self);
        let news0 = self.news.len();
        let mut news_n = 0usize;
        // 1) อารมณ์จาง · สายสัมพันธ์จางถ้าไม่เจอกัน
        for n in self.npcs.iter_mut() {
            n.soc.anger = (n.soc.anger - 20).max(0);
            n.soc.grudge = (n.soc.grudge - (1 + n.soc.tr[T_KIND] as i32 / 40)).max(0);
            n.soc.hit_cd = 0.;
            if n.soc.last_talk == 0 { n.soc.last_talk = day; }
            if n.state == ST_HOME && !n.child && day - n.soc.last_talk >= 6 {
                if n.married { n.mood = (n.mood - 4).max(0); n.soc.last_talk = day - 3; }
                else if n.hearts > 0 { n.hearts -= 1; n.soc.last_talk = day - 3; }
            }
        }
        // 2) ความสัมพันธ์ระหว่างชาวบ้าน
        let ids: Vec<u32> = self.npcs.iter().filter(|n| n.state == ST_HOME && !n.child).map(|n| n.id).collect();
        let mut leavers: Vec<(u32, String)> = vec![];
        for x in 0..ids.len() {
            for y in x + 1..ids.len() {
                let (a, b) = (ids[x], ids[y]);
                if leavers.iter().any(|l| l.0 == a || l.0 == b) { continue; }
                let r = self.rel_between(a, b);
                let (Some(ia), Some(ib)) = (self.idx(a), self.idx(b)) else { continue };
                let (na, nb) = (self.npcs[ia].clone(), self.npcs[ib].clone());
                let (an, bn) = (NPC_NAMES[na.name as usize], NPC_NAMES[nb.name as usize]);
                let temper = (na.soc.tr[T_TEMPER] as f64 + nb.soc.tr[T_TEMPER] as f64) / 200.;
                let hostile = ((10 - r).max(0) as f64 / 70.).min(1.);
                let p_quarrel = 0.004 + hostile * (0.05 + 0.25 * temper);
                if self.roll(a, b, 1) < p_quarrel {
                    self.add_rel(a, b, -(6 + (temper * 10.) as i32));
                    for id in [a, b] { if let Some(i) = self.idx(id) { self.npcs[i].mood = (self.npcs[i].mood - 6).max(0); } }
                    self.news_add(format!("{} กับ {} ทะเลาะกัน", an, bn));
                    if self.rel_between(a, b) <= -60 && !na.married && !nb.married {
                        let sa = na.soc.tr[T_BRAVE] as i32 + na.soc.tr[T_SOCIAL] as i32;
                        let sb = nb.soc.tr[T_BRAVE] as i32 + nb.soc.tr[T_SOCIAL] as i32;
                        let (lid, ln, on) = if sa <= sb { (a, an, bn) } else { (b, bn, an) };
                        leavers.push((lid, format!("{} ทนอยู่ร่วมกับ {} ไม่ไหว จึงย้ายออกไป", ln, on)));
                    }
                } else if r >= 20 && self.roll(a, b, 2) < 0.20 + na.soc.tr[T_SOCIAL] as f64 / 500. {
                    self.add_rel(a, b, 4);
                    for id in [a, b] { if let Some(i) = self.idx(id) { self.npcs[i].mood = (self.npcs[i].mood + 3).min(100); } }
                    if self.roll(a, b, 3) < 0.3 { self.news_add(format!("{} กับ {} นั่งคุยกันถึงดึก สนิทกันขึ้น", an, bn)); }
                } else if r < 0 && na.soc.tr[T_KIND] >= 55 && self.roll(a, b, 4) < 0.12 {
                    self.add_rel(a, b, 10);
                    self.news_add(format!("{} ไปง้อ {} คืนดีกันแล้ว", an, bn));
                }
            }
        }
        // 3) ทนผู้นำ (ผู้เล่น) ไม่ไหว
        for n in self.npcs.iter().filter(|n| n.state == ST_HOME && !n.child && !n.married) {
            if n.soc.anger >= 70 && n.soc.grudge >= 50 && n.hearts <= 1 && !leavers.iter().any(|l| l.0 == n.id) {
                leavers.push((n.id, format!("{} โกรธคุณมากจนไม่อยากอยู่ที่นี่อีกต่อไป", NPC_NAMES[n.name as usize])));
            }
        }
        for (lid, msg) in leavers {
            if let Some(i) = self.idx(lid) {
                self.npcs.remove(i);
                let friends: Vec<u32> = self.npcs.iter().filter(|n| n.state == ST_HOME).map(|n| n.id).collect();
                for f in friends {
                    if self.rel_between(f, lid) >= 20 { if let Some(j) = self.idx(f) { self.npcs[j].mood = (self.npcs[j].mood - 5).max(0); } }
                }
                self.news_add(msg);
            }
        }
        news_n += self.news.len().saturating_sub(news0);
        news_n
    }

    // ---------------- เซฟ ----------------
    pub fn soc_save_str(&self) -> String {
        let mut s = String::new();
        for n in &self.npcs {
            let t = &n.soc.tr;
            s += &format!("nsoc {} {} {} {} {} {} {} {} {} {} {} {} {} {}\n", n.id, t[0], t[1], t[2], t[3], t[4], n.soc.love, n.soc.hate, n.soc.anger, n.soc.grudge, n.soc.last_talk, n.soc.last_gift, n.soc.bore, n.soc.apol_day);
        }
        for ((a, b), d) in &self.rels { s += &format!("nrel {} {} {}\n", a, b, d); }
        for (d, t) in &self.news { s += &format!("nnews {} {}\n", d, t.replace('\n', " ")); }
        s
    }
    pub fn soc_load_line(&mut self, tag: &str, rest: &[&str]) {
        let f = |i: usize| rest.get(i).and_then(|v| v.parse::<i32>().ok()).unwrap_or(0);
        match tag {
            "nsoc" => {
                let id = f(0) as u32;
                if let Some(i) = self.idx(id) {
                    let s = &mut self.npcs[i].soc;
                    for k in 0..5 { s.tr[k] = f(1 + k).clamp(0, 100) as u8; }
                    s.love = (f(6) as usize).min(NI - 1) as u8;
                    s.hate = (f(7) as usize).min(NI - 1) as u8;
                    s.anger = f(8).clamp(0, 100);
                    s.grudge = f(9).clamp(0, 100);
                    s.last_talk = f(10);
                    s.last_gift = f(11).clamp(0, 255) as u8;
                    s.bore = f(12).clamp(0, 255) as u8;
                    s.apol_day = f(13);
                }
            }
            "nrel" => { self.rels.insert((f(0) as u32, f(1) as u32), f(2).clamp(-150, 150)); }
            "nnews" => { let t = rest.get(1..).unwrap_or(&[]).join(" "); if !t.is_empty() { self.news.push((f(0), t)); } }
            _ => {}
        }
    }
}
