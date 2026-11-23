// v21 — ที่ปรึกษา: ดูสถานการณ์เมืองแล้วบอกว่าควรทำอะไร (เด้งแถบคำแนะนำ + รายการในหน้าภาพรวมเมือง)
// แต่ละข้อมีปุ่มพาไปทำได้ทันที: เข้าโหมดสร้างพร้อมเลือกชิ้นให้ · เปิดแท็บที่เกี่ยวข้อง · เปิดหน้าชาวบ้านคนนั้น
use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AdvAct { Build(u8), Tab(u8), Npc(u32), Role(u8), Tax, None }

#[derive(Clone, Debug)]
pub struct Advice {
    /// คีย์กันเด้งซ้ำ
    pub id: String,
    /// 0 ด่วนมาก · 1 สำคัญ · 2 แนะนำ · 3 เล็กน้อย
    pub prio: u8,
    pub text: String,
    pub act: AdvAct,
    pub btn: &'static str,
}

pub const PRIO_C: [u32; 4] = [0xff6a5a, 0xffb84a, 0xffe08a, 0xa8d8ff];
pub const PRIO_N: [&str; 4] = ["ด่วน", "สำคัญ", "แนะนำ", "เล็กน้อย"];

/// สถิติหลักของเมือง (ใช้ทั้งหน้าภาพรวมและที่ปรึกษา)
#[derive(Clone, Debug, Default)]
pub struct TownStats {
    pub pop: usize,
    pub beds: usize,
    pub free_beds: usize,
    pub food: i32,
    pub food_days: f32,
    pub water_ok: usize,
    pub no_water: Vec<u32>,
    pub outdoor: Vec<u32>,
    pub no_bed: Vec<u32>,
    pub hungry: Vec<u32>,
    pub unhappy: Vec<u32>,
    pub mood_avg: i32,
    pub chests: usize,
    pub plots: usize,
    pub ripe: usize,
    pub roles: Vec<(u8, usize)>,
}

impl Game {
    pub fn food_near(&self, bed: (i32, i32)) -> i32 {
        self.chests.iter().filter(|(k, _)| (k.0 - bed.0).abs() <= 16 && (k.1 - bed.1).abs() <= 16).map(|(_, c)| FOODS.iter().map(|&i| c[i]).sum::<i32>()).sum()
    }
    pub fn town_stats(&self) -> TownStats {
        let mut s = TownStats::default();
        let res: Vec<&npc::Npc> = self.npcs.iter().filter(|n| n.state == ST_HOME).collect();
        s.pop = res.len();
        s.beds = self.beds().len();
        let me = self.bed;
        s.free_beds = self.free_beds().into_iter().filter(|b| Some(*b) != me).count();
        s.food = self.food_stock();
        s.food_days = if s.pop > 0 { s.food as f32 / s.pop as f32 } else { 99. };
        let served = self.water_served();
        let mut mood = 0;
        let mut roles: HashMap<u8, usize> = HashMap::new();
        for n in &res {
            mood += n.mood;
            if !n.child { *roles.entry(n.role).or_default() += 1; }
            let bed_ok = self.w.placed.get(&n.bed) == Some(&33);
            if !bed_ok { s.no_bed.push(n.id); } else if self.w.room_from(n.bed).is_none() { s.outdoor.push(n.id); }
            if served.contains(&n.bed) { s.water_ok += 1; } else { s.no_water.push(n.id); }
            if self.food_near(n.bed) <= 0 { s.hungry.push(n.id); }
            if n.mood < 35 { s.unhappy.push(n.id); }
        }
        s.mood_avg = if s.pop > 0 { mood / s.pop as i32 } else { 0 };
        if let Some(c) = self.town_center() {
            s.chests = self.chests.keys().filter(|k| (k.0 - c.0).abs() <= 24 && (k.1 - c.1).abs() <= 24).count();
            for (k, cr) in &self.crops { if (k.0 - c.0).abs() <= 24 && (k.1 - c.1).abs() <= 24 { s.plots += 1; if cr.stage() == 2 { s.ripe += 1; } } }
        }
        let mut r: Vec<(u8, usize)> = roles.into_iter().collect();
        r.sort_by_key(|x| (std::cmp::Reverse(x.1), x.0));
        s.roles = r;
        s
    }
    /// ปัญหาของชาวบ้านคนนี้ (เหตุผลที่ไม่มีความสุข)
    pub fn npc_issues(&self, n: &npc::Npc, served: &HashSet<(i32, i32)>) -> Vec<&'static str> {
        let mut v = vec![];
        if n.state != ST_HOME { return v; }
        let bed_ok = self.w.placed.get(&n.bed) == Some(&33);
        if !bed_ok { v.push("ไม่มีเตียง"); } else if self.w.room_from(n.bed).is_none() { v.push("นอนกลางแจ้ง"); }
        if !served.contains(&n.bed) { v.push("ไม่มีน้ำดื่ม"); }
        if self.food_near(n.bed) <= 0 { v.push("ไม่มีอาหารในกล่องใกล้บ้าน"); }
        if refuses(n) { v.push("โกรธคุณอยู่"); }
        if v.is_empty() && n.mood < 35 { v.push("เหงา/ไม่พอใจ — ลองคุยหรือให้ของที่ชอบ"); }
        v
    }
    /// สิ่งที่ตอนนี้ควรทำ เรียงตามความสำคัญ
    pub fn advice(&self) -> Vec<Advice> {
        let mut v: Vec<Advice> = vec![];
        let mut add = |id: String, prio: u8, text: String, act: AdvAct, btn: &'static str| v.push(Advice { id, prio, text, act, btn });
        if self.w.dun.is_some() { return vec![]; }
        let s = self.town_stats();
        let nm = |id: u32| self.npcs.iter().find(|n| n.id == id).map(|n| NPC_NAMES[n.name as usize]).unwrap_or("");
        // ---- ภัยคุกคาม (ด่วน)
        if !peaceful() {
            for (k, st) in self.rs.ks.iter().enumerate() {
                if let Some(a) = &st.army {
                    if a.vs.is_none() {
                        if let Some(tc) = self.town_center() {
                            let d = (a.x - tc.0 as f32).hypot(a.y - tc.1 as f32);
                            let hrs = d / 80. * 24.;
                            add(format!("army{}", k), 0, format!("ทัพของ{} ({} นาย) กำลังมา อีกราว {:.0} ชั่วโมงในเกม — เตรียมทหาร/กำแพง หรือเจรจา", self.kname(k), a.n, hrs.max(1.)), AdvAct::Tab(11), "ดูอาณาจักร");
                        }
                    }
                }
                if let Some((dl, amt)) = st.ult { add(format!("ult{}", k), 0, format!("{}ยื่นคำขาด: จ่าย {} เหรียญ ภายในวันที่ {} ไม่งั้นจะบุก", self.kname(k), amt, dl + 1), AdvAct::Tab(11), "ตอบคำขาด"); }
                if st.vas_ask { add(format!("vas{}", k), 1, format!("{}ขอให้เราเป็นเมืองขึ้น — ตอบได้ที่แท็บอาณาจักร", self.kname(k)), AdvAct::Tab(11), "ดูอาณาจักร"); }
                if st.help.is_some() { add(format!("help{}", k), 1, format!("พันธมิตร{}ขอความช่วยเหลือ — ไม่ช่วยอาจถูกตัดสัมพันธ์", self.kname(k)), AdvAct::Tab(11), "ดูอาณาจักร"); }
            }
        }
        if s.pop == 0 {
            if self.tut >= TUT_N { add("lonely".into(), 3, "ยังไม่มีชาวบ้าน — ออกสำรวจหาผู้หลงทาง (เครื่องหมาย ! สีเหลือง) แล้วชวนมาอยู่ · ต้องมีเตียงว่าง".into(), AdvAct::None, ""); }
            v.sort_by_key(|a| a.prio);
            return v;
        }
        // ---- ความเป็นอยู่
        if !s.no_bed.is_empty() { add("nobed".into(), 0, format!("{} คนไม่มีเตียง ({}) — จะย้ายออกถ้าไม่มีที่นอน", s.no_bed.len(), nm(s.no_bed[0])), AdvAct::Build(33), "วางเตียง"); }
        if !s.no_water.is_empty() {
            let (obj, what) = if s.no_water.len() == 1 { (O_WJAR, "โอ่งน้ำ") } else if s.no_water.len() <= 6 { (39, "บ่อน้ำครัวเรือน") } else { (O_BIGWELL, "บ่อน้ำกลางเมือง") };
            add("water".into(), 0, format!("{} คนไม่มีน้ำดื่มใกล้บ้าน (ความสุขลดทุกวัน) — สร้าง{}ใกล้เตียงพวกเขา", s.no_water.len(), what), AdvAct::Build(obj), "สร้างเลย");
        }
        if s.chests == 0 { add("chest".into(), 1, "ยังไม่มีกล่องเก็บของในเมือง — ชาวบ้านเอาผลงานไปเก็บไม่ได้ และหาอาหารไม่เจอ".into(), AdvAct::Build(14), "วางกล่อง"); }
        if !s.hungry.is_empty() { add("food".into(), 0, format!("{} คนไม่มีอาหารในกล่องใกล้บ้าน — ใส่อาหารในกล่อง หรือให้คนเป็นชาวไร่/นักล่า", s.hungry.len()), AdvAct::Role(1), "หาชาวไร่"); }
        else if s.food_days < 2. { add("foodlow".into(), 1, format!("อาหารในกล่องพออีกราว {:.0} วัน — เพิ่มชาวไร่ หรือใส่อาหารเพิ่ม", s.food_days.max(0.)), AdvAct::Role(1), "หาชาวไร่"); }
        if !s.outdoor.is_empty() { add("roof".into(), 2, format!("{} คนนอนกลางแจ้ง — ล้อมเตียงด้วยกำแพง+ประตู หรือวางบ้านสำเร็จรูป จะมีความสุขขึ้น", s.outdoor.len()), AdvAct::Build(OBJ_HOUSE_S), "วางบ้าน"); }
        let served = self.water_served();
        for &id in s.unhappy.iter().take(2) {
            if let Some(n) = self.npcs.iter().find(|n| n.id == id) {
                let why = self.npc_issues(n, &served);
                add(format!("sad{}", id), 1, format!("{}ไม่มีความสุข ({}%) — {}", nm(id), n.mood, why.first().copied().unwrap_or("ลองคุยดู")), AdvAct::Npc(id), "ดูคนนี้");
            }
        }
        for (id, msg) in &self.rs.help { add(format!("proj{}", id), 2, msg.clone(), AdvAct::Npc(*id), "ดูคนนี้"); }
        // v30 ชาวไร่หาที่ทำไร่ไม่ได้ (เช่น เมืองกลางทะเลทราย)
        for (&id, _) in &self.rs.pm.nospot { if let Some(i) = self.idx(id) { if self.npcs[i].role == 1 { add(format!("nospot{}", id), 1, format!("{}หาที่ทำไร่ใกล้บ้านไม่ได้ — ดินแถวนั้นไถไม่ได้/มีของขวาง (ทรายไถได้เฉพาะริมน้ำ) ลองเคลียร์ที่ หรือให้เขาย้ายไปนอนใกล้ทุ่งหญ้า", NPC_NAMES[self.npcs[i].name as usize]), AdvAct::Npc(id), "ดูคนนี้"); } } }
        // v30 คำขอที่ดินที่ยังไม่ได้ตอบ (คนที่ยืนถามอยู่ตรงหน้ามีการ์ดของตัวเอง)
        for a in &self.rs.pm.asks { if a.came || self.rs.work.get(&a.by).map_or(false, |j| j.kind == J_ASK) { continue; } if let Some(i) = self.idx(a.by) { add(format!("land{}", a.by), 1, format!("{}{} — รออนุญาตอยู่", NPC_NAMES[self.npcs[i].name as usize], self.ask_what(a)), AdvAct::Npc(a.by), "ตอบเขา"); } }
        // v23 เหตุการณ์
        let sick: Vec<u32> = self.rs.lv.sick.keys().copied().collect();
        if !sick.is_empty() { add("sick".into(), 1, format!("{} คนป่วย (กากบาทเขียวบนหัว) ทำงานไม่ได้ — {}", sick.len(), if self.p.inv[HERB] > 0 { "เดินไปกดคุยเพื่อให้สมุนไพร (ชาวบ้านก็ช่วยกันหายาเองด้วย)" } else { "หาสมุนไพร (ต้นไม้ยักษ์/ปราบโครงกระดูก) แล้วเอาไปให้ — ชาวบ้านก็ออกหาให้กันเองด้วย" }), AdvAct::Npc(sick[0]), "ดูคนป่วย"); }
        if self.rs.lv.waiting > 0 { add("refugee".into(), 1, format!("ผู้ลี้ภัย {} คนรอเตียงว่าง — วางเตียงหรือสร้างบ้านให้ จะย้ายเข้าตอนเช้า", self.rs.lv.waiting), AdvAct::Build(33), "วางเตียง"); }
        // ---- งาน/การเติบโต
        let has = |r: u8| s.roles.iter().any(|x| x.0 == r && x.1 > 0);
        if s.pop >= 3 && !has(1) { add("farmer".into(), 2, "ยังไม่มีชาวไร่ — เปลี่ยนอาชีพใครสักคนเป็น \"ชาวไร่\" (กดที่คน → เปลี่ยนอาชีพ) เขาจะทำไร่ให้เอง".into(), AdvAct::Role(1), "เลือกคน"); }
        if !self.sites.is_empty() && !has(R_BUILDER) { add("builder".into(), 3, "มีไซต์ก่อสร้างค้างอยู่แต่ไม่มีช่างไม้ — ให้ใครสักคนเป็นช่างไม้ จะสร้างเร็วขึ้นแม้คุณไม่อยู่".into(), AdvAct::Role(R_BUILDER), "เลือกคน"); }
        if s.free_beds == 0 && s.pop >= 2 && !self.rs.petitions.is_empty() { add("beds".into(), 2, format!("มี {} คนยื่นขอย้ายเข้าเมือง แต่ไม่มีเตียงว่าง", self.rs.petitions.len()), AdvAct::Build(33), "วางเตียง"); }
        else if !self.rs.petitions.is_empty() { add("petition".into(), 2, format!("มี {} คน (ขุนนาง/ทหารผ่านศึก) ขอย้ายเข้าเมือง — ดูได้ที่แท็บอาณาจักร", self.rs.petitions.len()), AdvAct::Tab(11), "ดูคำขอ"); }
        if s.ripe >= 6 && !has(1) { add("ripe".into(), 3, format!("ผักสุกรอเก็บ {} แปลง", s.ripe), AdvAct::None, ""); }
        if self.tax_pool >= 40 && (self.tier() >= 3 || self.rs.founded) { add("tax".into(), 3, format!("ภาษีสะสม {} เหรียญ — เก็บได้เลย", self.tax_pool), AdvAct::Tax, "เก็บภาษี"); }
        if !self.rs.founded && self.found_reqs().iter().all(|r| r.1) { add("found".into(), 1, "เมืองของคุณพร้อมสถาปนาเป็นอาณาจักรแล้ว!".into(), AdvAct::Tab(11), "สถาปนา"); }
        // ป้องกันอ่อนกว่าอาณาจักรที่เป็นศัตรู/ตึงเครียด
        if !peaceful() && s.pop >= 4 {
            let def = self.defense();
            if let Some((k, st)) = self.rs.ks.iter().enumerate().filter(|(_, st)| st.alive && (st.stance == 3 || st.rep < -20)).max_by(|a, b| a.1.strength.partial_cmp(&b.1.strength).unwrap()) {
                if st.strength > def * 1.5 { add("defense".into(), 1, format!("กำลังป้องกันเมือง ({:.0}) อ่อนกว่า{} ({:.0}) มาก — ให้ชาวบ้านเป็นทหาร/แม่ทัพ สร้างกำแพงเมือง หรือผูกมิตร", def, self.kname(k), st.strength), AdvAct::Tab(11), "ดูอาณาจักร"); }
            }
        }
        v.sort_by_key(|a| a.prio);
        v
    }

    /// ทุก ~4 วิ: เลือกคำแนะนำที่ควรเด้ง (ไม่ซ้ำภายใน 1 วัน · กดปิด = ไม่เด้งเรื่องนั้นอีก 2 วัน)
    pub fn advisor_tick(&mut self, dt: f32) {
        if let Some((_, t)) = self.rs.adv_show.as_mut() { *t -= dt; if *t <= 0. { self.rs.adv_show = None; } }
        self.rs.adv_t -= dt;
        if self.rs.adv_t > 0. { return; }
        self.rs.adv_t = 4.;
        if !prefs::advisor() || self.rs.adv_show.is_some() || self.tut < TUT_N || war::in_combat(self) { return; }
        let now = self.time;
        let list = self.advice();
        if let Some(a) = list.into_iter().filter(|a| a.prio <= 2).find(|a| self.rs.adv_cd.get(&a.id).map_or(true, |&t| now >= t)) {
            self.rs.adv_cd.insert(a.id.clone(), now + DAYLEN);
            if a.prio == 0 { play(&self.sfx.click); }
            self.rs.adv_show = Some((a, 9.));
        }
    }
    pub fn adv_dismiss(&mut self) {
        if let Some((a, _)) = self.rs.adv_show.take() { self.rs.adv_cd.insert(a.id, self.time + DAYLEN * 2.); }
    }
    /// ทำตามคำแนะนำ
    pub fn adv_do(&mut self, act: AdvAct) {
        match act {
            AdvAct::Build(o) => {
                if let Some(i) = PIECES.iter().position(|p| p.obj == o) {
                    self.build_sel = Some(i);
                    self.build_cur = None;
                    let ok = can_pay(&self.p, PIECES[i].cost);
                    self.toast(if ok { format!("โหมดสร้าง: {} — แตะตรงที่จะวาง", PIECES[i].name) } else { format!("{}: ของยังไม่พอ ({})", PIECES[i].name, cost_str(PIECES[i].cost)) });
                }
            }
            AdvAct::Tab(t) => self.req = Some(Req::Tab(t)),
            AdvAct::Npc(id) => self.req = Some(Req::Npc(id)),
            AdvAct::Role(r) => self.req = Some(Req::RolePick(r)),
            AdvAct::Tax => self.collect_tax(),
            AdvAct::None => {}
        }
    }
}
pub fn cost_str(c: &[(usize, i32)]) -> String { c.iter().map(|(i, n)| format!("{} {}", INAME[*i], n)).collect::<Vec<_>>().join(" ") }

// ---------------------------------------------------------------- แถบคำแนะนำ (เด้งบนจอ)
/// (แถบ, ปุ่มทำ, ปุ่มปิด)
/// v27 ป๊อปอัปคำแนะนำอยู่ในโซนกลาง ใต้แถบเป้าหมาย · โซนกว้าง = ปุ่มอยู่ขวา 2 บรรทัด · โซนแคบ = ปุ่มอยู่ล่าง 3 บรรทัด
pub fn adv_wide(u: f32) -> bool { hud::layout().col_w >= 460. * u }
pub fn adv_rects(g: &Game) -> Option<(Rect, Option<Rect>, Rect)> {
    if war::in_combat(g) || g.permit_card().is_some() { return None; }
    let (a, _) = g.rs.adv_show.as_ref()?;
    let u = ui_scale();
    let l = hud::layout();
    let w = l.col_w.min(600. * u);
    let x = l.col_x + l.col_w / 2. - w / 2.;
    let th = hud::top_banner_h(g, u);
    let y = 8. * u + if th > 0. { th + 6. * u } else { 0. };
    let has_go = a.act != AdvAct::None;
    if adv_wide(u) {
        let h = 52. * u;
        let close = Rect::new(x + w - 46. * u, y + 10. * u, 38. * u, 32. * u);
        let go = if has_go { Some(Rect::new(close.x - 92. * u, y + 10. * u, 86. * u, 32. * u)) } else { None };
        Some((Rect::new(x, y, w, h), go, close))
    } else {
        let h = 92. * u;
        let close = Rect::new(x + w - 50. * u, y + h - 36. * u, 42. * u, 30. * u);
        let go = if has_go { Some(Rect::new(close.x - 96. * u, y + h - 36. * u, 90. * u, 30. * u)) } else { None };
        Some((Rect::new(x, y, w, h), go, close))
    }
}
pub fn draw_advice(g: &Game, font: &Font) {
    let Some((r, go, close)) = adv_rects(g) else { return };
    let (a, t) = g.rs.adv_show.as_ref().unwrap();
    let u = ui_scale();
    let alpha = (t / 0.4).min(1.);
    let pc = hx(PRIO_C[a.prio as usize]);
    draw_rectangle(r.x, r.y, r.w, r.h, Color::new(0.05, 0.06, 0.1, 0.88 * alpha));
    draw_rectangle(r.x, r.y, 5. * u, r.h, pc);
    draw_rectangle_lines(r.x, r.y, r.w, r.h, 2., Color::new(pc.r, pc.g, pc.b, 0.8 * alpha));
    text(font, &format!("คำแนะนำ · {}", PRIO_N[a.prio as usize]), r.x + 12. * u, r.y + 15. * u, 11. * u, pc);
    let wide = adv_wide(u);
    let tw = if wide { go.map_or(close.x, |g| g.x) - r.x - 20. * u } else { r.w - 22. * u };
    let ws = wrap_th(font, &a.text, 12. * u, tw);
    let maxl = if wide { 2 } else { 3 };
    for (k, l) in ws.iter().take(maxl).enumerate() {
        let l = if k + 1 == maxl && ws.len() > maxl { format!("{}...", l) } else { l.clone() };
        text(font, &l, r.x + 12. * u, r.y + 31. * u + k as f32 * 14. * u, 12. * u, WHITE);
    }
    if let Some(b) = go {
        draw_rectangle(b.x, b.y, b.w, b.h, Color::new(pc.r * 0.35, pc.g * 0.35, pc.b * 0.25, 0.95));
        draw_rectangle_lines(b.x, b.y, b.w, b.h, 2., pc);
        text_c(font, a.btn, b.x + b.w / 2., b.y + b.h * 0.68, 12. * u, WHITE);
    }
    draw_rectangle(close.x, close.y, close.w, close.h, Color::new(0.2, 0.2, 0.25, 0.9));
    text_c(font, "ปิด", close.x + close.w / 2., close.y + close.h * 0.68, 12. * u, hx(0xcccccc));
    // แถบเวลาที่เหลือ
    draw_rectangle(r.x, r.y + r.h - 2. * u, r.w * (t / 9.).clamp(0., 1.), 2. * u, pc);
}

#[cfg(test)]
mod tests {
    use super::*;
    fn mk(id: u32, bed: (i32, i32)) -> npc::Npc {
        let mut n = npc::Npc { id, name: (id % 12) as u8, look: 0, x: bed.0 as f32 * 16. + 8., y: bed.1 as f32 * 16. + 18., state: ST_HOME, hearts: 3, mood: 70, married: false, child: false, role: 0, bed, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(id, (id % 12) as u8) };
        n.soc.anger = 0; n.soc.grudge = 0;
        n
    }
    fn town(g: &mut Game) -> (i32, i32) {
        let (sx, sy) = (tfloor(g.spawn.0) + 120, tfloor(g.spawn.1) + 120);
        for y in sy - 45..=sy + 45 { for x in sx - 45..=sx + 45 { g.w.tmod.insert((x, y), 3); g.w.removed.insert((x, y)); } }
        g.tut = TUT_DONE;
        (sx, sy)
    }
    #[test]
    fn advisor_spots_problems_and_offers_fixes() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let (sx, sy) = town(&mut g);
        for i in 0..4 { let b = (sx + i * 2, sy); g.w.placed.insert(b, 33); g.npcs.push(mk(i as u32 + 1, b)); }
        let a = g.advice();
        let ids: Vec<&str> = a.iter().map(|x| x.id.as_str()).collect();
        assert!(ids.contains(&"water") && ids.contains(&"chest") && ids.contains(&"food") && ids.contains(&"roof") && ids.contains(&"farmer"), "{:?}", ids);
        assert!(a.windows(2).all(|w| w[0].prio <= w[1].prio), "เรียงตามความสำคัญ");
        let w = a.iter().find(|x| x.id == "water").unwrap();
        assert_eq!(w.act, AdvAct::Build(39), "4 คน → แนะนำบ่อครัวเรือน");
        // แก้ปัญหาแล้วคำแนะนำหายไป
        g.w.placed.insert((sx + 3, sy + 3), 39);
        g.w.placed.insert((sx - 2, sy), 14);
        let mut ch = [0; NI]; ch[BERRY] = 40; g.chests.insert((sx - 2, sy), ch);
        let ids: Vec<String> = g.advice().into_iter().map(|x| x.id).collect();
        assert!(!ids.iter().any(|i| i == "water" || i == "chest" || i == "food"), "{:?}", ids);
        // ปุ่ม "สร้างเลย" เข้าโหมดสร้างพร้อมเลือกชิ้นให้
        g.adv_do(AdvAct::Build(O_BIGWELL));
        assert_eq!(g.build_sel.map(|i| PIECES[i].obj), Some(O_BIGWELL));
    }
    #[test]
    fn popup_shows_once_per_day_and_dismiss_waits_longer() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let (sx, sy) = town(&mut g);
        g.w.placed.insert((sx, sy), 33);
        g.npcs.push(mk(1, (sx, sy)));
        g.advisor_tick(5.);
        let first = g.rs.adv_show.as_ref().map(|a| a.0.id.clone()).expect("เด้งคำแนะนำ");
        g.adv_dismiss();
        assert!(g.rs.adv_show.is_none());
        g.advisor_tick(5.);
        let second = g.rs.adv_show.as_ref().map(|a| a.0.id.clone());
        assert_ne!(second.as_deref(), Some(first.as_str()), "เรื่องที่ปิดไปไม่เด้งซ้ำทันที");
        // ปิดระบบที่ปรึกษาในตั้งค่า → ไม่เด้ง
        g.rs.adv_show = None;
        prefs::ADVISOR.store(false, std::sync::atomic::Ordering::Relaxed);
        g.rs.adv_cd.clear();
        g.advisor_tick(5.);
        prefs::ADVISOR.store(true, std::sync::atomic::Ordering::Relaxed);
        assert!(g.rs.adv_show.is_none());
    }
    #[test]
    fn town_stats_and_npc_issues() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let (sx, sy) = town(&mut g);
        g.w.placed.insert((sx, sy), 33);
        g.npcs.push(mk(1, (sx, sy)));
        g.npcs.push(mk(2, (sx + 9, sy + 9)));
        g.npcs[0].role = 1;
        let s = g.town_stats();
        assert_eq!((s.pop, s.no_bed.len(), s.outdoor.len(), s.no_water.len()), (2, 1, 1, 2));
        assert_eq!(s.roles.iter().find(|r| r.0 == 1).map(|r| r.1), Some(1));
        let served = g.water_served();
        let iss = g.npc_issues(&g.npcs[1].clone(), &served);
        assert!(iss.contains(&"ไม่มีเตียง") && iss.contains(&"ไม่มีน้ำดื่ม"));
    }
}
