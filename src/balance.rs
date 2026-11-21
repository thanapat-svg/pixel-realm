// balance.rs — v9: ระดับความยาก · ช่วงผ่อนแรงต้นเกม · เตือนหน้าหนาว · หนังสัตว์/เสื้อขนสัตว์ · คำแนะนำครั้งแรก · สมุดบันทึกการค้นพบ
use super::*;

/// 0 ง่าย · 1 ปกติ · 2 ยาก (ค่าส่วนกลาง เก็บใน .cfg)
pub fn diff() -> u8 { prefs::DIFF.load(std::sync::atomic::Ordering::Relaxed).min(3) }
pub fn diff_name() -> &'static str { ["ง่าย", "ปกติ", "ยาก", "สันติ"][diff() as usize] }
/// ดาเมจที่ผู้เล่นได้รับ
pub fn dmg_taken_mul() -> f32 { DMG[diff() as usize] }
/// [ง่าย, ปกติ, ยาก, สันติ]
pub const DMG: [f32; 4] = [0.6, 1.0, 1.4, 0.5];
pub const EMUL: [f32; 4] = [0.75, 1.0, 1.25, 0.6];
pub const NMUL: [f32; 4] = [0.7, 1.0, 1.25, 0.0];
pub const AMUL: [f32; 4] = [0.6, 1.0, 1.3, 0.0];
/// เลือด/พลังของมอนสเตอร์
pub fn enemy_mul() -> f32 { EMUL[diff() as usize] }
/// ความเร็วหิว/กระหาย/เหนื่อย
pub fn needs_mul() -> f32 { NMUL[diff() as usize] }
/// ระยะที่สัตว์ดุเริ่มไล่
pub fn aggro_mul() -> f32 { AMUL[diff() as usize] }

/// จำนวนวันผ่อนแรงต้นเกม
pub const GRACE_DAYS: i32 = 2;
pub const HIDE_COAT: i32 = 5;

pub const TIP_SEASON: u8 = 0;
pub const TIP_PREDATOR: u8 = 1;
pub const TIP_PREY: u8 = 2;
pub const TIP_REQUEST: u8 = 3;
pub const TIP_LANDMARK: u8 = 4;
pub const TIP_NIGHT: u8 = 5;
pub const TIP_WINTER: u8 = 6;
pub const TIP_HIDE: u8 = 7;
pub fn tip_text(k: u8) -> &'static str {
    match k {
        TIP_SEASON => "แนะนำ: ปีละ 4 ฤดู ฤดูละ 5 วัน — ร้อนกระหายเร็ว · ใบไม้ร่วงฝนบ่อย · หนาวหิวเร็ว พืชแทบไม่โต · ใบไม้ผลิพืชโตเร็ว",
        TIP_PREDATOR => "แนะนำ: สัตว์ดุ! หมูป่าพุ่งชนเมื่อเข้าใกล้ · หมาป่าล่าตอนกลางคืน · จระเข้ซุ่มในหนองน้ำ — ถอยห่างหรือสู้ด้วยธนู · ไม่เข้าใกล้คบเพลิง/กองไฟ",
        TIP_PREY => "แนะนำ: กวาง/กระต่ายวิ่งหนีเมื่อเห็นคน — ใช้ธนูยิงจากไกล · ล่าแล้วได้เนื้อ (ทำอาหารก่อนกิน) และหนังสัตว์",
        TIP_REQUEST => "แนะนำ: ชาวบ้านมีเรื่องขอให้ช่วย (กรอบคำพูดสีเหลือง) — เปิดเมนู → ชุมชน · ช่วยแล้วได้เหรียญและสนิทขึ้น · ไม่ช่วยใน 3 วันเขาจะผิดหวัง",
        TIP_LANDMARK => "แนะนำ: จุดสำคัญจะแสดงบนแผนที่ใหญ่ (M) — วงหินขอพรได้วันละครั้ง · ต้นไม้ยักษ์เก็บของได้วันละครั้ง",
        TIP_NIGHT => "แนะนำ: กลางคืนมอนสเตอร์ออกมาเยอะ — วางคบเพลิง/กองไฟรอบฐานเป็นเขตปลอดภัย หรือนอนบนเตียงข้ามคืน",
        TIP_WINTER => "แนะนำ: หน้าหนาวหิวเร็ว พืชแทบไม่โต ไม่มีแอปเปิล — ตุนอาหารในกล่องเก็บของ ทำเสื้อขนสัตว์ (เมนูคราฟต์) และอยู่ใกล้กองไฟ",
        _ => "แนะนำ: เก็บหนังสัตว์ 5 ผืนแล้วคราฟต์ \"เสื้อขนสัตว์\" — หน้าหนาว/ในหิมะจะไม่หิวเร็วขึ้น",
    }
}

/// หมวดสมุดบันทึก: (ชื่อ, รางวัล)
pub const CODEX: [(&str, &str); 4] = [
    ("ไบโอม", "เลือดสูงสุด +4"),
    ("จุดสำคัญ", "เศษดาว 4"),
    ("สัตว์ที่ล่าได้", "เหรียญ 40 + หนังสัตว์ 5"),
    ("หมู่บ้าน/แคมป์", "อัญมณี 6"),
];
pub const CODEX_BIOMES: [u8; 9] = [2, 3, 4, 5, 7, 8, T_SWAMP, T_SAVANNA, T_ASH];
pub const CODEX_ANIMALS: [u8; 6] = [A_DEER, A_RABBIT, A_BOAR, A_WOLF, A_CROC, A_ANTE];

impl Game {
    pub fn in_grace(&self) -> bool { day_of(self) < GRACE_DAYS }
    /// ตัวคูณความต้องการรวม (ความยาก × ช่วงผ่อนแรง)
    pub fn needs_drain_mul(&self) -> f32 { needs_mul() * if self.in_grace() { 0.75 } else { 1. } }

    pub fn tip(&mut self, k: u8) {
        if self.tips_seen & (1 << k) != 0 { return; }
        self.tips_seen |= 1 << k;
        self.tips.push((tip_text(k).to_string(), 9.));
        if self.tips.len() > 2 { self.tips.remove(0); }
    }
    /// ตรวจสถานการณ์ที่ควรแนะนำ (เรียกทุกวินาที)
    pub fn tips_check(&mut self) {
        if self.darkv > 0.45 && self.w.dun.is_none() { self.tip(TIP_NIGHT); }
        let (px, py) = (self.p.x, self.p.y);
        let near = |a: &Animal| (a.x - px).hypot(a.y - py) < 170.;
        if self.animals.iter().any(|a| predator(a.k) && near(a)) { self.tip(TIP_PREDATOR); }
        if self.animals.iter().any(|a| prey(a.k) && near(a)) { self.tip(TIP_PREY); }
        if !self.nreqs.is_empty() { self.tip(TIP_REQUEST); }
        if !self.marks_found.is_empty() { self.tip(TIP_LANDMARK); }
        if self.p.hides > 0 { self.tip(TIP_HIDE); }
        self.codex_check();
        self.adv_check();
    }
    /// เตือนก่อนเข้าหน้าหนาว (เรียกตอนรุ่งเช้า)
    pub fn winter_warn(&mut self) {
        let day = day_of(self);
        if season_of_day(day + 2) == 3 && season_of_day(day + 1) != 3 && season_of_day(day) != 3 {
            self.toast("อีก 2 วันจะเข้าฤดูหนาว — ตุนอาหารในกล่องเก็บของ ทำเสื้อขนสัตว์ เตรียมกองไฟ");
            self.tip(TIP_WINTER);
        } else if season_of_day(day + 1) == 3 && season_of_day(day) != 3 {
            self.toast("พรุ่งนี้เข้าฤดูหนาวแล้ว!");
            self.tip(TIP_WINTER);
        }
    }
    pub fn add_hide(&mut self, n: i32) {
        self.p.hides += n;
        self.toast(format!("ได้หนังสัตว์ (มี {} ผืน)", self.p.hides));
    }

    // ---------------- สมุดบันทึก ----------------
    /// (พบแล้ว, ทั้งหมด, รายชื่อที่พบ/ยังไม่พบ)
    pub fn codex_progress(&self, cat: usize) -> (usize, usize, Vec<(String, bool)>) {
        let v: Vec<(String, bool)> = match cat {
            0 => CODEX_BIOMES.iter().map(|&t| (TN[t as usize].to_string(), self.seen.contains(&t))).collect(),
            1 => (1..=4u8).map(|k| (mark_name(k).to_string(), self.marks_found.values().any(|&m| m == k))).collect(),
            2 => CODEX_ANIMALS.iter().map(|&a| (AN_NAME[a as usize].to_string(), self.an_hunted & (1 << a) != 0)).collect(),
            _ => (0..=3u8).map(|k| (camp_title(k).to_string(), self.camps_found.iter().any(|&c| self.w.camp_kind(c) == k))).collect(),
        };
        (v.iter().filter(|x| x.1).count(), v.len(), v)
    }
    pub fn codex_check(&mut self) {
        for cat in 0..CODEX.len() {
            if self.codex_done & (1 << cat) != 0 { continue; }
            let (a, b, _) = self.codex_progress(cat);
            if a < b { continue; }
            self.codex_done |= 1 << cat;
            match cat {
                0 => { self.p.maxhp += 4; self.p.hp = self.p.maxhp; }
                1 => self.p.inv[SHARD] += 4,
                2 => { self.p.inv[COIN] += 40; self.p.hides += 5; }
                _ => self.p.inv[GEM] += 6,
            }
            self.toast(format!("สมุดบันทึก: ค้นพบ{}ครบ! รางวัล {}", CODEX[cat].0, CODEX[cat].1));
            play(&self.sfx.lvl);
            self.save();
        }
    }
    pub fn balance_save_str(&self) -> String {
        format!("v9 {} {} {} {} {} {} {} {} {}\n{}", self.p.hides, self.p.coat as u8, self.tips_seen, self.an_hunted, self.codex_done, self.adv_done, self.boated as u8, self.p.boat as u8, self.final_won as u8, self.pets_save_str())
    }
    pub fn balance_load_line(&mut self, rest: &[&str]) {
        let f = |i: usize| rest.get(i).and_then(|v| v.parse::<u32>().ok()).unwrap_or(0);
        self.p.hides = f(0) as i32;
        self.p.coat = f(1) != 0;
        self.tips_seen = f(2);
        self.an_hunted = f(3);
        self.codex_done = f(4);
        self.adv_done = f(5);
        self.boated = f(6) != 0;
        self.p.boat = f(7) != 0;
        self.final_won = f(8) != 0;
    }
}
