// finale.rs — v12: บอสสุดท้าย "ราชันย์เงา" + ฉากจบ (เครดิต) แล้วเล่นต่อได้
//  อัญเชิญ: ไปที่แท่นบูชาในวงหินศักดิ์สิทธิ์ตอนกลางคืน (กด ใช้)
//  เงื่อนไข: ชนะดันเจี้ยน 3 แห่ง + แกนโกเลม 3 + เมือกราชาสไลม์ 3 + เศษดาว 3
use super::*;

pub const FINAL_KIND: u8 = 2;
pub const FINAL_HP: f32 = 900.;
pub const FINAL_DUNS: usize = 3;
pub const FINAL_COST: [(usize, i32); 3] = [(CORE, 3), (GEL, 3), (SHARD, 3)];
pub const ENDING_T: f32 = 26.;

impl Game {
    pub fn final_ready(&self) -> (bool, String) {
        let duns = self.dun_clear.len();
        let mut miss = vec![];
        if duns < FINAL_DUNS { miss.push(format!("ชนะดันเจี้ยน {}/{}", duns, FINAL_DUNS)); }
        for &(it, n) in FINAL_COST.iter() { if self.p.inv[it] < n { miss.push(format!("{} {}/{}", INAME[it], self.p.inv[it], n)); } }
        (miss.is_empty(), miss.join(" · "))
    }
    /// กลางคืนที่แท่นบูชา: อัญเชิญบอสสุดท้าย (คืน true ถ้าจัดการแล้ว)
    pub fn try_summon_final(&mut self, k: (i32, i32)) -> bool {
        if self.darkv <= 0.45 { return false; }
        if self.bosses.iter().any(|b| b.kind == FINAL_KIND) { self.toast("ราชันย์เงาอยู่ที่นี่แล้ว!"); return true; }
        let (ok, miss) = self.final_ready();
        if !ok {
            self.toast(format!("ศาลหินสั่นไหวในความมืด... ยังอัญเชิญราชันย์เงาไม่ได้: ต้องมี {}", miss));
            return false;
        }
        for &(it, n) in FINAL_COST.iter() { self.p.inv[it] -= n; }
        let (x, y) = (k.0 as f32 * 16. + 8., (k.1 - 3) as f32 * 16. + 12.);
        let hp = (FINAL_HP * enemy_mul()).max(300.);
        self.spawn_boss_hp(FINAL_KIND, x, y, hp);
        self.toast("ท้องฟ้ามืดมิด... ราชันย์เงาตื่นขึ้นมาแล้ว! นี่คือการต่อสู้ครั้งสุดท้าย");
        self.shake(4., 0.8);
        self.save();
        true
    }
    pub fn final_defeated(&mut self, x: f32, y: f32) {
        self.gain(COIN, 300, x, y - 8.);
        self.gain(GEM, 15, x, y - 18.);
        self.gain(SHARD, 5, x, y - 28.);
        self.p.maxhp += 10;
        self.p.hp = self.p.maxhp;
        self.final_won = true;
        self.ending = ENDING_T;
        self.save();
    }
    /// ระหว่างฉากจบ: เกมหยุด · แตะ/กดเพื่อปิดหลังผ่านไป 3 วินาที
    pub fn ending_tick(&mut self, dt: f32, inp: &Input) -> bool {
        if self.ending <= 0. { return false; }
        self.ending -= dt;
        self.p.invul = 1.;
        if self.ending < ENDING_T - 3. && (inp.atk || inp.act || inp.eat || inp.hb.is_some()) { self.ending = 0.; self.toast("ขอบคุณที่เล่น! โลกยังรอให้คุณสำรวจต่อ"); }
        true
    }
}

pub fn draw_ending(g: &Game, font: &Font) {
    if g.ending <= 0. { return; }
    let (sw, sh) = (screen_width(), screen_height());
    let u = ui_scale();
    let t = ENDING_T - g.ending;
    let a = (t / 1.5).min(1.) * 0.92;
    draw_rectangle(0., 0., sw, sh, Color::new(0.02, 0.02, 0.06, a));
    let days = (g.time / DAYLEN) as i32 + 1;
    let villagers = g.npcs.iter().filter(|n| n.state == ST_HOME).count();
    let disc = (0..CODEX.len()).map(|c| g.codex_progress(c).0).sum::<usize>();
    let lines: Vec<(String, f32, u32)> = vec![
        ("ชัยชนะ!".into(), 34., 0xffcc66),
        ("ราชันย์เงาถูกปราบแล้ว ความมืดหายไปจากโลก".into(), 15., 0xffffff),
        ("".into(), 10., 0),
        (format!("ใช้เวลา {} วัน · เลเวล {}", days, g.p.lvl), 14., 0xdddddd),
        (format!("ชาวบ้านในหมู่บ้าน {} คน · ค้นพบ {} อย่าง", villagers, disc), 14., 0xdddddd),
        (format!("เป้าหมายสำเร็จ {}/{} · ดันเจี้ยนที่ชนะ {}", g.adv_done.count_ones(), ADV.len(), g.dun_clear.len()), 14., 0xdddddd),
        ("".into(), 10., 0),
        ("รางวัล: เหรียญ 300 · อัญมณี 15 · เศษดาว 5 · เลือดสูงสุด +10".into(), 13., 0x9cff5a),
        ("".into(), 10., 0),
        ("PIXEL REALM".into(), 26., 0xffcc66),
        ("ทุกอย่างในเกมวาดและสร้างด้วยโค้ด".into(), 12., 0xaaaaaa),
        ("ออกแบบและพัฒนาร่วมกับ Claude".into(), 12., 0xaaaaaa),
        ("".into(), 10., 0),
        ("เล่นต่อได้ — โลกยังกว้างไม่มีที่สิ้นสุด".into(), 14., 0xffffff),
    ];
    let total: f32 = lines.iter().map(|l| l.1 * 1.7 * u).sum();
    let scroll = ((t - 2.).max(0.) * 18. * u).min((total - sh * 0.55).max(0.));
    let mut y = sh * 0.22 - scroll;
    for (s, fs, c) in &lines {
        if !s.is_empty() && y > -30. && y < sh + 30. { text_c(font, s, sw / 2., y, fs * u, Color::new(((c >> 16) & 255) as f32 / 255., ((c >> 8) & 255) as f32 / 255., (c & 255) as f32 / 255., a / 0.92)); }
        y += fs * 1.7 * u;
    }
    if t > 3. { text_c(font, "แตะหรือกดปุ่มเพื่อเล่นต่อ", sw / 2., sh - 20. * u, 12. * u, Color::new(1., 1., 1., 0.5 + (t * 3.).sin() * 0.3)); }
}
