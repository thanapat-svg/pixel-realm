// v22 — เส้นทางสู่การรวมแผ่นดิน: 5 บท (ค่าย → หมู่บ้าน → เมือง → อาณาจักร → รวมแผ่นดิน)
// แต่ละภารกิจตรวจจากสถานะเกมจริง ทำข้ามบทได้ (นับเลย) · ครบบท = รางวัล + ฉายา + ความสามารถพิเศษ
// ครบบทสุดท้าย (ทั้ง 4 อาณาจักร: เป็นพันธมิตร / ยึดเมืองหลวง / ล่มสลาย) = ฉากจบ แล้วเล่นต่อได้
use super::*;

pub const CHAPTERS: [(&str, &str, &str, &str); 5] = [
    ("บทที่ 1 · ค่ายเล็กๆ", "เริ่มต้นชีวิตใหม่ในดินแดนที่ไม่มีใครรู้จัก", "ผู้บุกเบิก", "รางวัล: 40 เหรียญ ไม้ 20 หิน 10"),
    ("บทที่ 2 · หมู่บ้าน", "ผู้คนเริ่มเชื่อใจ — ดูแลให้ทุกคนอยู่ดีกินดี", "ผู้ใหญ่บ้าน", "รางวัล: 80 เหรียญ · ชาวไร่เก็บเกี่ยวได้มากขึ้น (+1 ต่อแปลง)"),
    ("บทที่ 3 · เมือง", "ชื่อเสียงขจรไกล — ถึงเวลาเปิดประตูสู่โลกกว้าง", "เจ้าเมือง", "รางวัล: 150 เหรียญ อัญมณี 2 · ภาษี +20%"),
    ("บทที่ 4 · อาณาจักร", "แผ่นดินนี้ไม่ได้มีราชาแค่คนเดียว", "กษัตริย์", "รางวัล: 250 เหรียญ อัญมณี 4 · กำลังป้องกันเมือง +25%"),
    ("บทที่ 5 · รวมแผ่นดิน", "ด้วยไมตรีหรือคมดาบ — ทำให้ทุกอาณาจักรเป็นหนึ่งเดียว", "จักรพรรดิแห่งแผ่นดิน", "รางวัล: ฉากจบ + มงกุฎทอง 500 เหรียญ"),
];
pub const MN: usize = 23;
/// (บท, ชื่อ, คำใบ้, เหรียญ) — 19..=22 ใช้ชื่ออาณาจักรจริงตอนแสดง
pub const MISSIONS: [(u8, &str, &str, i32); MN] = [
    (0, "วางเตียงของตัวเอง", "สร้างเตียง (เมนู → สร้าง) แล้วนอน = จุดเกิดใหม่", 5),
    (0, "มีกล่องเก็บของ", "วางกล่องที่บ้าน — ชาวบ้านจะเอาผลงานมาเก็บที่นี่", 5),
    (0, "ชวนชาวบ้านคนแรกมาอยู่", "ช่วยผู้หลงทาง (! สีเหลืองบนแผนที่) แล้วมีเตียงว่างให้", 15),
    (0, "ผ่านคืนแรก", "อยู่รอดจนถึงเช้าวันที่ 2", 5),
    (1, "ประชากร 4 คน", "ชวนผู้หลงทาง หรือรอผู้อพยพ (ต้องมีเตียงว่าง)", 15),
    (1, "มีไร่ 6 แปลง", "ให้ชาวบ้านเป็นชาวไร่ เขาจะมาขอที่ทำไร่ กด \"ให้เลย\" (หรือไถเอง)", 10),
    (1, "ทุกคนมีน้ำดื่ม", "บ่อน้ำ/โอ่งใกล้เตียง หรืออยู่ใกล้แหล่งน้ำจืด", 10),
    (1, "ทุกคนมีบ้านมีหลังคา", "ล้อมเตียงด้วยกำแพง+ประตู หรือบ้านสำเร็จรูป", 15),
    (1, "มีอาชีพ 3 แบบ", "เปลี่ยนอาชีพได้ที่หน้าชาวบ้าน (กดที่คน → เปลี่ยนอาชีพ)", 10),
    (2, "ประชากร 10 คน", "บ้านดีๆ น้ำ อาหาร ชื่อเสียง = คนอยากย้ายมา", 25),
    (2, "สร้างปราสาท", "ศูนย์กลางของเมือง (เมนู → สร้าง)", 20),
    (2, "กำแพงเมือง 12 ชิ้น", "กำแพงหินใหญ่ — ลากนิ้ววางเป็นแนว", 15),
    (2, "ค้าขายที่ตลาดเมืองอื่น", "เดินไปเมืองของอาณาจักรอื่น แตะแผงตลาด แล้วซื้อหรือขาย", 15),
    (2, "ทำสัญญาการค้า", "เข้าเฝ้าที่ปราสาทเมืองหลวงอาณาจักรใดก็ได้", 20),
    (3, "สถาปนาอาณาจักร", "ดูเงื่อนไขที่แท็บอาณาจักร", 40),
    (3, "มีพันธมิตร 1 อาณาจักร", "ชื่อเสียงสูงพอ แล้วขอเป็นพันธมิตรที่ปราสาท", 30),
    (3, "ชนะศึกป้องกันเมือง", "เมื่อทัพศัตรูบุก อยู่ใกล้เมืองแล้วสู้จนชนะ", 30),
    (3, "ประชากร 16 คน", "ขยายเมือง — ขุนนาง/ทหารผ่านศึกยื่นขอย้ายมาได้", 30),
    (3, "กองทหาร 4 นาย", "ให้ชาวบ้านเป็นทหาร/แม่ทัพ/ยาม", 25),
    (4, "", "ผูกพันธมิตร (ไมตรี) หรือยึดเมืองหลวง (ชนะสงคราม)", 50),
    (4, "", "ผูกพันธมิตร (ไมตรี) หรือยึดเมืองหลวง (ชนะสงคราม)", 50),
    (4, "", "ผูกพันธมิตร (ไมตรี) หรือยึดเมืองหลวง (ชนะสงคราม)", 50),
    (4, "", "ผูกพันธมิตร (ไมตรี) หรือยึดเมืองหลวง (ชนะสงคราม)", 50),
];

/// สถานะเส้นทาง (เก็บในเซฟ)
#[derive(Clone, Debug, Default)]
pub struct Saga { pub done: u32, pub chap: u8, pub trades: u32, pub ended: bool, pub end_day: i32 }

impl Game {
    pub fn mission_name(&self, i: usize) -> String {
        if i >= 19 { let k = i - 19; return format!("รวม{}", if k < self.w.realm.ks.len() { self.kname(k) } else { "อาณาจักร".into() }); }
        MISSIONS[i].1.to_string()
    }
    /// อาณาจักร k รวมกับเราแล้วหรือยัง: พันธมิตร · เมืองหลวงเป็นของเรา · ล่มสลาย
    pub fn unified(&self, k: usize) -> bool {
        let Some(st) = self.rs.ks.get(k) else { return false };
        let capi = self.w.realm.ks[k].cap;
        !st.alive || st.stance == 2 || self.rs.owner.get(capi).copied() == Some(PLAYER_OWN)
    }
    pub fn mission_ok(&self, i: usize) -> bool {
        let pop = self.pop();
        match i {
            0 => self.bed.is_some(),
            1 => !self.chests.is_empty() || self.w.placed.values().any(|&o| o == 14),
            2 => pop >= 1,
            3 => day_of(self) >= 1,
            4 => pop >= 4,
            5 => self.town_stats().plots >= 6,
            6 => pop >= 4 && self.town_stats().no_water.is_empty(),
            7 => pop >= 4 && { let s = self.town_stats(); s.outdoor.is_empty() && s.no_bed.is_empty() },
            8 => { let r: HashSet<u8> = self.npcs.iter().filter(|n| n.state == ST_HOME && !n.child && n.role != 0).map(|n| n.role).collect(); r.len() >= 3 }
            9 => pop >= 10,
            10 => self.has_castle(),
            11 => self.walls() >= 12,
            12 => self.saga.trades >= 1,
            13 => self.rs.ks.iter().any(|k| k.alive && (k.stance == 1 || k.stance == 2)),
            14 => self.rs.founded,
            15 => self.rs.ks.iter().any(|k| k.alive && k.stance == 2),
            16 => self.rs.siege_wins >= 1,
            17 => pop >= 16,
            18 => self.npcs.iter().filter(|n| n.state == ST_HOME && matches!(n.role, 2 | R_SOLDIER | R_GENERAL)).count() >= 4,
            19..=22 => self.rs.inited && self.unified(i - 19),
            _ => false,
        }
    }
    pub fn chapter_done(&self, c: u8) -> bool { (0..MN).filter(|&i| MISSIONS[i].0 == c).all(|i| self.saga.done & (1 << i) != 0) }
    /// บทปัจจุบัน = บทแรกที่ยังไม่ครบ
    pub fn cur_chapter(&self) -> u8 { (0..5u8).find(|&c| !self.chapter_done(c)).unwrap_or(5) }
    pub fn title(&self) -> Option<&'static str> { (0..5).rev().find(|&c| self.saga.chap & (1 << c) != 0).map(|c| CHAPTERS[c].2) }
    /// ความสามารถพิเศษจากบทที่ผ่าน: 1 ไร่+1 · 2 ภาษี+20% · 3 ป้องกัน+25%
    pub fn saga_perk(&self, c: u8) -> bool { self.saga.chap & (1 << c) != 0 }

    /// ตรวจทุก ~2 วิ (เรียกจาก update)
    pub fn saga_tick(&mut self) {
        if !self.rs.inited { return; }
        for i in 0..MN {
            if self.saga.done & (1 << i) != 0 || !self.mission_ok(i) { continue; }
            self.saga.done |= 1 << i;
            let c = MISSIONS[i].3;
            self.p.inv[COIN] += c;
            self.p.xp += c;
            self.check_lv();
            self.toast(format!("ภารกิจสำเร็จ: {} (+{} เหรียญ)", self.mission_name(i), c));
            play(&self.sfx.lvl);
        }
        for ch in 0..5u8 {
            if self.saga.chap & (1 << ch) != 0 || !self.chapter_done(ch) { continue; }
            self.saga.chap |= 1 << ch;
            let (coins, items): (i32, &[(usize, i32)]) = match ch { 0 => (40, &[(WOOD, 20), (STONE, 10)]), 1 => (80, &[]), 2 => (150, &[(GEM, 2)]), 3 => (250, &[(GEM, 4)]), _ => (500, &[]) };
            self.p.inv[COIN] += coins;
            for &(it, n) in items { self.p.inv[it] += n; }
            let (name, _, title, _) = CHAPTERS[ch as usize];
            self.news_add(format!("{} สำเร็จ! ได้รับฉายา \"{}\"", name, title));
            self.toast(format!("{} สำเร็จ! ฉายาใหม่: {}", name, title));
            play(&self.sfx.lvl);
            if ch == 4 && !self.saga.ended { self.saga.ended = true; self.saga.end_day = day_of(self); self.rs.ending = Some(0.); }
        }
    }
    pub fn saga_save(&self) -> String { format!("saga {} {} {} {} {}\n", self.saga.done, self.saga.chap, self.saga.trades, self.saga.ended as u8, self.saga.end_day) }
    pub fn saga_load(&mut self, f: &[&str]) {
        let n = |i: usize| f.get(i).and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
        self.saga = Saga { done: n(0) as u32, chap: n(1) as u8, trades: n(2) as u32, ended: n(3) > 0, end_day: n(4) as i32 };
    }
    /// ภารกิจที่แสดงบนแถบด้านบน: ภารกิจแรกที่ยังไม่สำเร็จของบทปัจจุบัน
    pub fn saga_next(&self) -> Option<usize> {
        let c = self.cur_chapter();
        if c >= 5 { return None; }
        (0..MN).find(|&i| MISSIONS[i].0 == c && self.saga.done & (1 << i) == 0)
    }
}

// ---------------------------------------------------------------- แท็บ "เส้นทาง"
pub fn tab_saga(g: &mut Game, font: &Font, c: Rect, mut y: f32, u: f32) -> f32 {
    let cur = g.cur_chapter();
    let title = g.title().unwrap_or("ผู้มาเยือน");
    header_s(font, c, y, &format!("เส้นทางสู่การรวมแผ่นดิน · ฉายา: {}", title), u);
    y += 30. * u;
    for ch in 0..5u8 {
        let (name, desc, ttl, reward) = CHAPTERS[ch as usize];
        let done = g.saga_perk(ch);
        let locked = ch > cur;
        let n_done = (0..MN).filter(|&i| MISSIONS[i].0 == ch && g.saga.done & (1 << i) != 0).count();
        let n_all = (0..MN).filter(|&i| MISSIONS[i].0 == ch).count();
        let col = if done { hx(0x77dd77) } else if ch == cur { hx(0xffcc66) } else { hx(0x8888aa) };
        let hh = 46. * u;
        if y + hh > c.y && y < c.y + c.h {
            draw_rectangle(c.x + 4. * u, y + 2. * u, c.w - 8. * u, hh - 4. * u, Color::new(0.12, 0.13, 0.2, 0.95));
            draw_rectangle(c.x + 4. * u, y + 2. * u, 5. * u, hh - 4. * u, col);
            text(font, &format!("{}{}", name, if done { " — สำเร็จ" } else { "" }), c.x + 16. * u, y + 19. * u, 14. * u, col);
            text(font, &format!("{} · {}/{} · ฉายา \"{}\"", desc, n_done, n_all, ttl), c.x + 16. * u, y + 36. * u, 10. * u, hx(0xaaaabb));
        }
        y += hh;
        // แสดงภารกิจบทปัจจุบัน+บทถัดไป (ทำล่วงหน้าได้) · บทไกลกว่านั้นซ่อนไว้
        if locked && ch > cur + 1 { continue; }
        for i in (0..MN).filter(|&i| MISSIONS[i].0 == ch) {
            let ok = g.saga.done & (1 << i) != 0;
            let rh = 36. * u;
            if y + rh > c.y && y < c.y + c.h {
                let box_c = if ok { hx(0x66dd66) } else { hx(0x555a70) };
                draw_rectangle_lines(c.x + 20. * u, y + 8. * u, 14. * u, 14. * u, 2., box_c);
                if ok { draw_rectangle(c.x + 23. * u, y + 11. * u, 8. * u, 8. * u, box_c); }
                text(font, &format!("{}  (+{})", g.mission_name(i), MISSIONS[i].3), c.x + 42. * u, y + 18. * u, 12. * u, if ok { hx(0x99cc99) } else { WHITE });
                if !ok { text(font, MISSIONS[i].2, c.x + 42. * u, y + 32. * u, 10. * u, hx(0x9999aa)); }
            }
            y += rh;
        }
        if y + 22. * u > c.y && y < c.y + c.h { text(font, reward, c.x + 20. * u, y + 14. * u, 10. * u, if done { hx(0x77aa77) } else { hx(0xddbb66) }); }
        y += 24. * u;
    }
    if g.saga.ended {
        if y > c.y - 40. * u && y < c.y + c.h { text(font, &format!("คุณรวมแผ่นดินสำเร็จในวันที่ {} — แผ่นดินนี้สงบสุขภายใต้การนำของคุณ", g.saga.end_day + 1), c.x + 12. * u, y + 16. * u, 12. * u, hx(0xffd84a)); }
        y += 30. * u;
    }
    y
}
fn header_s(font: &Font, c: Rect, y: f32, s: &str, u: f32) {
    if y + 26. * u < c.y || y > c.y + c.h { return; }
    draw_rectangle(c.x, y, c.w, 26. * u, hx(0x2a2d40));
    text(font, s, c.x + 10. * u, y + 18. * u, 14. * u, hx(0xffcc66));
}

// ---------------------------------------------------------------- ฉากจบ
/// คืน true เมื่อกด "เล่นต่อ"
pub fn draw_saga_end(g: &mut Game, font: &Font, t: f32) -> bool {
    let (sw, sh) = (screen_width(), screen_height());
    let u = ui_scale();
    let a = (t / 2.).min(1.);
    draw_rectangle(0., 0., sw, sh, Color::new(0.02, 0.02, 0.06, 0.92 * a));
    // ดาวระยิบ
    for i in 0..60 {
        let (x, y) = (hash(i, 1, 77) as f32 * sw, hash(i, 2, 77) as f32 * sh * 0.6);
        let b = ((t * 1.5 + i as f32).sin() * 0.5 + 0.5) * a;
        draw_rectangle(x, y, 2. * u, 2. * u, Color::new(1., 1., 0.85, b * 0.8));
    }
    // มงกุฎทอง (วาดด้วยสี่เหลี่ยม)
    let (cx, cy, s) = (sw / 2., sh * 0.18, 6. * u);
    let gold = Color::new(1., 0.82, 0.25, a);
    let dk = Color::new(0.75, 0.55, 0.1, a);
    draw_rectangle(cx - 6. * s, cy, 12. * s, 3. * s, gold);
    for k in 0..5 { let x = cx - 6. * s + k as f32 * 2.75 * s; let h = if k % 2 == 0 { 4. } else { 2.5 }; draw_rectangle(x, cy - h * s, 1.5 * s, h * s, gold); draw_rectangle(x + 0.25 * s, cy - h * s - 0.8 * s, s, s, hx(0xff5a7a)); }
    draw_rectangle(cx - 6. * s, cy + 3. * s, 12. * s, 0.6 * s, dk);
    let lines: Vec<(String, f32, Color)> = vec![
        ("แผ่นดินเป็นหนึ่งเดียว".into(), 30., hx(0xffd84a)),
        (format!("{} \"{}\"", g.my_name(), CHAPTERS[4].2), 18., WHITE),
        (String::new(), 10., WHITE),
        (format!("ใช้เวลา {} วัน · ประชากร {} คน · ชื่อเสียง {}", g.saga.end_day + 1, g.pop(), g.fame()), 14., hx(0xcccccc)),
        (format!("พันธมิตร {} · ยึดเมืองหลวง {} · ชนะศึกป้องกันเมือง {} ครั้ง", g.rs.ks.iter().filter(|k| k.stance == 2).count(), (0..g.rs.ks.len()).filter(|&k| g.rs.owner.get(g.w.realm.ks[k].cap).copied() == Some(PLAYER_OWN)).count(), g.rs.siege_wins), 14., hx(0xcccccc)),
        (format!("ดันเจี้ยนที่พิชิต {} · สัตว์ที่ล่า {} ชนิด · เลเวล {}", g.dun_clear.len(), g.an_hunted.count_ones(), g.p.lvl), 14., hx(0xcccccc)),
        (String::new(), 10., WHITE),
        ("จากค่ายเล็กๆ กลางป่า สู่แผ่นดินที่สงบสุข".into(), 13., hx(0xaabbdd)),
        ("ผู้คนจะเล่าขานเรื่องราวของคุณไปอีกหลายชั่วอายุคน".into(), 13., hx(0xaabbdd)),
    ];
    let mut y = sh * 0.32;
    for (k, (s, fs, col)) in lines.iter().enumerate() {
        let la = ((t - 0.6 - k as f32 * 0.35) / 0.6).clamp(0., 1.);
        if !s.is_empty() { text_c(font, s, sw / 2., y, fs * u, Color::new(col.r, col.g, col.b, la)); }
        y += fs * u * 1.7;
    }
    let br = Rect::new(sw / 2. - 90. * u, sh - 90. * u, 180. * u, 44. * u);
    if t > 3. {
        draw_rectangle(br.x, br.y, br.w, br.h, hx(0xffcc66));
        text_c(font, "เล่นต่อ (โลกอิสระ)", br.x + br.w / 2., br.y + br.h * 0.65, 15. * u, hx(0x222222));
        let m: Vec2 = mouse_position().into();
        let tap = touches().iter().any(|t| t.phase == TouchPhase::Started && br.contains(t.position)) || (is_mouse_button_pressed(MouseButton::Left) && br.contains(m)) || is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Escape);
        if tap { return true; }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    fn mk(id: u32, bed: (i32, i32), role: u8) -> npc::Npc {
        npc::Npc { id, name: (id % 12) as u8, look: 0, x: bed.0 as f32 * 16. + 8., y: bed.1 as f32 * 16. + 18., state: ST_HOME, hearts: 3, mood: 70, married: false, child: false, role, bed, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(id, (id % 12) as u8) }
    }
    #[test]
    fn missions_complete_from_game_state_and_chapter_rewards_once() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        g.realm_tick(0.01);
        assert_eq!(g.cur_chapter(), 0);
        let (sx, sy) = (tfloor(g.spawn.0), tfloor(g.spawn.1));
        g.w.placed.insert((sx + 2, sy + 2), 33);
        g.bed = Some((sx + 2, sy + 2));
        g.w.placed.insert((sx - 2, sy + 2), 14);
        g.chests.insert((sx - 2, sy + 2), [0; NI]);
        g.w.placed.insert((sx + 4, sy + 2), 33);
        g.npcs.push(mk(1, (sx + 4, sy + 2), 0));
        g.time = DAYLEN * 1.2;
        let c0 = g.p.inv[COIN];
        g.saga_tick();
        assert!(g.chapter_done(0) && g.saga_perk(0), "บทที่ 1 สำเร็จ");
        assert_eq!(g.title(), Some("ผู้บุกเบิก"));
        assert_eq!(g.cur_chapter(), 1);
        let gained = g.p.inv[COIN] - c0;
        assert_eq!(gained, 5 + 5 + 15 + 5 + 40, "เหรียญภารกิจ+รางวัลบท");
        g.saga_tick();
        assert_eq!(g.p.inv[COIN] - c0, gained, "ไม่ได้ซ้ำ");
        // เซฟ/โหลด
        g.saga.trades = 3;
        let h = Game::load_str(&g.save_string(), Sfx::default()).ok().unwrap();
        assert_eq!((h.saga.done, h.saga.chap, h.saga.trades), (g.saga.done, g.saga.chap, 3));
    }
    #[test]
    fn unify_land_by_alliance_or_conquest_triggers_ending() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        g.realm_tick(0.01);
        // ทำบทก่อนหน้าให้ครบ (ทดสอบเฉพาะบทสุดท้าย)
        g.saga.done = (1 << 19) - 1;
        g.saga.chap = 0b1111;
        g.rs.ks[0].stance = 2;
        g.rs.ks[1].alive = false;
        let cap2 = g.w.realm.ks[2].cap;
        g.rs.owner[cap2] = PLAYER_OWN;
        g.saga_tick();
        assert!(!g.saga.ended, "ยังเหลือ 1 อาณาจักร");
        assert_eq!(g.saga_next(), Some(22));
        assert!(g.mission_name(22).contains(&g.kname(3)));
        g.rs.ks[3].stance = 2;
        g.saga_tick();
        assert!(g.saga.ended && g.rs.ending.is_some(), "รวมแผ่นดินแล้ว → ฉากจบ");
        assert_eq!(g.title(), Some("จักรพรรดิแห่งแผ่นดิน"));
        assert_eq!(g.saga_next(), None);
    }
}
