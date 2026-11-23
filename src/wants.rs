//! v31 "เขาอยากได้อะไร" — รวมทุกอย่างที่ชาวบ้านต้องการไว้ที่เดียว กดมอบได้ทันที หรือกดไม่ให้
//! - คำขอ "ช่วยหาของ" (เควสต์) · ของที่ฝากเอามาส่งให้เขา
//! - วัสดุสร้างบ้าน/บ่อน้ำที่ยังขาด → มอบตรงๆ จากกระเป๋า (ไม่ต้องเดินไปใส่กล่อง)
//! - ของโปรดที่เขาบอกว่า "ช่วงนี้อยากได้" (เป็นของขวัญวันละครั้ง)
//! โชว์เป็นแถบเหนือหัวเขาตอนเรายืนใกล้ๆ และเป็นหัวข้อแรกในหน้าของเขา · ของอื่นๆ ยังให้ได้เหมือนเดิม
use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum WantK {
    /// คำขอในระบบเควสต์ (ดัชนี nreqs)
    Req(usize),
    /// วัสดุโครงการ (ของที่ขาด)
    Build(Vec<(usize, i32)>),
    /// ของโปรด
    Love(usize),
}
#[derive(Clone, Debug)]
pub struct Want { pub k: WantK, pub text: String, pub give: String, pub can: bool }

impl Game {
    fn project_missing(&self, id: u32) -> Vec<(usize, i32)> {
        let Some(p) = self.rs.projects.iter().find(|p| p.by == id && p.ok && !p.started) else { return vec![] };
        let Some(i) = self.idx(id) else { return vec![] };
        let bed = self.npcs[i].bed;
        p.need.iter().map(|&(it, n)| {
            let h = p.got.iter().filter(|x| x.0 == it).map(|x| x.1).sum::<i32>() + self.chests.iter().filter(|(k, _)| (k.0 - bed.0).abs() <= 20 && (k.1 - bed.1).abs() <= 20).map(|(_, c)| c[it]).sum::<i32>();
            (it, n - h)
        }).filter(|x| x.1 > 0).collect()
    }
    /// ทุกอย่างที่คนนี้อยากได้จากเราตอนนี้ (เรียงตามความสำคัญ)
    pub fn wants(&self, id: u32) -> Vec<Want> {
        let mut v = vec![];
        let Some(i) = self.idx(id) else { return v };
        let n = &self.npcs[i];
        for (ri, r) in self.nreqs.iter().enumerate() {
            match &r.k {
                QK::Fetch { item, qty } if r.from == id => v.push(Want { k: WantK::Req(ri), text: format!("{} x{} (รางวัล {} เหรียญ)", INAME[*item], qty, r.reward), give: "มอบ".into(), can: self.p.inv[*item] >= *qty }),
                QK::Deliver { item, to } if *to == id => v.push(Want { k: WantK::Req(ri), text: format!("{} ที่{}ฝากมาให้", INAME[*item], self.idx(r.from).map(|k| NPC_NAMES[self.npcs[k].name as usize]).unwrap_or("")), give: "ส่งให้".into(), can: self.p.inv[*item] >= 1 }),
                _ => {}
            }
        }
        let miss = self.project_missing(id);
        // v35 เขาหาวัสดุเองอยู่ — โชว์ปุ่มขอเฉพาะตอนหาเองไม่ได้/รอนานแล้ว (และวันที่เรากด "ไม่ให้" ก็ไม่ถามซ้ำ)
        if !miss.is_empty() && self.sh_stuck_build(id, &miss) && self.rs.want_no.get(&id) != Some(&day_of(self)) {
            let s: Vec<String> = miss.iter().map(|(it, q)| format!("{} {}", INAME[*it], q)).collect();
            let kind = self.rs.projects.iter().find(|p| p.by == id && p.ok && !p.started).map(|p| p.kind).unwrap_or(0);
            v.push(Want { k: WantK::Build(miss.clone()), text: format!("วัสดุ{}ที่หาเองไม่ได้: {}", if is_prefab(kind) { "สร้างบ้าน" } else { "ส่วนรวม" }, s.join(" · ")), give: "มอบ".into(), can: miss.iter().any(|(it, _)| self.p.inv[*it] > 0) });
        }
        let love = n.soc.love as usize;
        let day = day_of(self);
        if n.state == ST_HOME && !n.child && n.hearts >= 2 && !n.gifted && !refuses(n) && n.mood >= 30 && self.rs.want_no.get(&id) != Some(&day) {
            v.push(Want { k: WantK::Love(love), text: format!("{} (ของโปรด · ให้แล้วสนิทขึ้นมาก)", INAME[love]), give: "มอบ 1".into(), can: self.p.inv[love] >= 1 });
        }
        v
    }
    /// ของที่ยังขาด (ชิ้น, ขาดอีกเท่าไร) สำหรับคำขอนี้ — ใช้โชว์คู่มือ "หาได้จากไหน"
    pub fn want_lack(&self, k: &WantK) -> Vec<(usize, i32)> {
        let one = |it: usize, n: i32| if self.p.inv[it] < n { vec![(it, n - self.p.inv[it])] } else { vec![] };
        match k {
            WantK::Req(ri) => match self.nreqs.get(*ri).map(|r| &r.k) {
                Some(QK::Fetch { item, qty }) => one(*item, *qty),
                Some(QK::Deliver { item, .. }) => one(*item, 1),
                _ => vec![],
            },
            WantK::Build(miss) => miss.iter().filter(|&&(it, q)| self.p.inv[it] < q).map(|&(it, q)| (it, q - self.p.inv[it])).collect(),
            WantK::Love(it) => one(*it, 1),
        }
    }
    /// กด "มอบ"
    pub fn want_give(&mut self, id: u32, w: &WantK) {
        match w {
            WantK::Req(ri) => {
                let ri = *ri;
                let Some(r) = self.nreqs.get(ri) else { return };
                let act = if matches!(r.k, QK::Deliver { .. }) { QA::DeliverTo } else { QA::Hand };
                match self.req_do(ri, act) { Some(m) => { self.toast(m); self.npc_bubble(id, "ขอบคุณมากเลย!"); } None => self.toast("ของในกระเป๋ายังไม่พอ") }
            }
            WantK::Build(miss) => {
                let mut gave = vec![];
                let Some(p) = self.rs.projects.iter().position(|p| p.by == id && p.ok && !p.started) else { return };
                for &(it, q) in miss {
                    let t = self.p.inv[it].min(q);
                    if t <= 0 { continue; }
                    self.p.inv[it] -= t;
                    self.rs.projects[p].got.push((it, t));
                    gave.push(format!("{} {}", INAME[it], t));
                }
                if gave.is_empty() { self.toast("ยังไม่มีวัสดุที่เขาขาดในกระเป๋า"); return; }
                self.rs.help.remove(&id);
                if let Some(i) = self.idx(id) { self.npcs[i].mood = (self.npcs[i].mood + 3).min(100); }
                self.toast(format!("มอบ {} แล้ว", gave.join(" · ")));
                self.npc_bubble(id, "ขอบคุณ! ช่วยได้เยอะเลย");
                play(&self.sfx.pickup);
                self.projects_try_start();
                self.save();
            }
            WantK::Love(it) => {
                let it = *it;
                if self.p.inv[it] < 1 { self.toast(format!("ไม่มี{}ในกระเป๋า", INAME[it])); return; }
                self.npc_gift_item(id, it);
                self.npc_bubble(id, "ชอบมาก! เหมือนรู้ใจกันเลย");
            }
        }
    }
    /// กด "ไม่ให้"
    pub fn want_no(&mut self, id: u32, w: &WantK) {
        match w {
            WantK::Req(ri) => { if let Some(m) = self.req_do(*ri, QA::Decline) { self.toast(m); } }
            WantK::Build(_) => {
                // v35 ไม่ให้ = เขาก็ยังพยายามหาเองต่อ (ไม่เลิกโครงการ ไม่โกรธ) แค่วันนี้ไม่ถามซ้ำ
                let day = day_of(self);
                self.rs.want_no.insert(id, day);
                self.npc_bubble(id, "ไม่เป็นไร เดี๋ยวหาเอง");
                self.toast("ไม่ให้วัสดุ — เขาจะพยายามหาเองต่อ");
            }
            WantK::Love(_) => {
                let day = day_of(self);
                self.rs.want_no.insert(id, day);
                self.npc_bubble(id, "ไม่เป็นไรจ้า");
            }
        }
    }
}

// ---------------------------------------------------------------- แถบ "อยากได้" เหนือปุ่มลัด
/// (id, ข้อความ, กรอบแถบ, ปุ่มมอบ, ปุ่มไม่ให้, ข้อมูล)
pub fn want_strip(g: &Game, v: &View) -> Option<(u32, String, Rect, Rect, Rect, WantK, bool)> {
    let (id, q) = v19::quick_btns(g, v)?;
    if g.rs.gift_to == Some(id) { return None; }
    // การ์ดขออนุญาตที่ดินของอีกคนขึ้นอยู่ด้านบน — ไม่ซ้อนทับกัน ตอบการ์ดก่อน
    if g.permit_card().is_some() || g.rs.crew.yld.is_some() { return None; }
    let ws = g.wants(id);
    let w = ws.first()?;
    let more = ws.len() - 1;
    let u = ui_scale();
    let text = format!("อยากได้: {}{}", w.text, if more > 0 { format!(" (+อีก {} เรื่อง ดูที่ \"อื่นๆ\")", more) } else { String::new() });
    let (bw, bh) = (64. * u, 26. * u);
    let x0 = q[0].x;
    let x1 = q[2].x + q[2].w;
    let wd = (x1 - x0).max(300. * u);
    let cx = (x0 + x1) / 2.;
    let r = Rect::new(cx - wd / 2., q[0].y - bh - 30. * u - 6. * u, wd, bh + 30. * u);
    let give = Rect::new(r.x + r.w - bw * 2. - 10. * u, r.y + r.h - bh - 4. * u, bw, bh);
    let no = Rect::new(r.x + r.w - bw - 4. * u, r.y + r.h - bh - 4. * u, bw, bh);
    Some((id, text, r, give, no, w.k.clone(), w.can))
}
pub fn want_tap(g: &mut Game, v: &View, tp: Vec2) -> bool {
    let Some((id, _, r, give, no, k, can)) = want_strip(g, v) else { return false };
    if give.contains(tp) { if can { g.want_give(id, &k); } else { g.toast("ของในกระเป๋ายังไม่พอ"); } }
    else if no.contains(tp) { g.want_no(id, &k); }
    else if !r.contains(tp) { return false; }
    true
}
pub fn draw_want_strip(g: &Game, sp: &Sprites, v: &View, font: &Font) {
    let Some((_, text_s, r, give, no, wk, can)) = want_strip(g, v) else { return };
    let u = ui_scale();
    draw_rectangle(r.x, r.y, r.w, r.h, Color::new(0.08, 0.07, 0.03, 0.85));
    draw_rectangle_lines(r.x, r.y, r.w, r.h, 2., hx(0xffd84a));
    let tw = r.w - 12. * u;
    let ws = wrap_th(font, &text_s, 12. * u, tw);
    let line = if ws.len() > 1 { format!("{}...", ws[0]) } else { ws.first().cloned().unwrap_or_default() };
    text(font, &line, r.x + 6. * u, r.y + 16. * u, 12. * u, hx(0xffe9a0));
    draw_rectangle(give.x, give.y, give.w, give.h, if can { hx(0x2f7a3a) } else { Color::new(0.25, 0.25, 0.28, 0.9) });
    draw_rectangle_lines(give.x, give.y, give.w, give.h, 2., Color::new(1., 1., 1., 0.5));
    text_c(font, "มอบ", give.x + give.w / 2., give.y + give.h * 0.7, 13. * u, if can { WHITE } else { hx(0x9a9a9a) });
    draw_rectangle(no.x, no.y, no.w, no.h, hx(0x7a2f2f));
    draw_rectangle_lines(no.x, no.y, no.w, no.h, 2., Color::new(1., 1., 1., 0.5));
    text_c(font, "ไม่ให้", no.x + no.w / 2., no.y + no.h * 0.7, 13. * u, WHITE);
    // v32 ขาดของ: บอกว่าหาได้จากไหน (รูป + ทิศ/ระยะ) — รายละเอียดเต็มอยู่ในหน้าของเขา
    let lack = g.want_lack(&wk);
    if let Some(&(it, _)) = lack.first() {
        let bx = r.x + 6. * u;
        let maxw = give.x - bx - 6. * u;
        match source::item_howto(g, it).first() {
            Some(sr) => {
                let sz = 20. * u;
                let by = give.y + (give.h - sz) / 2.;
                draw_rectangle(bx - 2., by - 2., maxw + 2., sz + 4., Color::new(0.14, 0.15, 0.22, 0.9));
                source::draw_pic(sp, sr.pic, bx, by, sz);
                let rel = source::rel(g, sr.find);
                let mut tx = bx + sz + 4. * u;
                if let Some((dx, dy)) = rel { source::draw_arrow(tx + 6. * u, give.y + give.h / 2., 6. * u, dx, dy, hx(0x9ad8ff)); tx += 16. * u; }
                let lbl = match rel { Some((dx, dy)) => format!("{} {}", sr.short, source::dir_text(dx, dy)), None => format!("ขาด{} · {}", INAME[it], sr.short) };
                text(font, &source::fit_text(font, &lbl, 11. * u, bx + maxw - tx), tx, give.y + give.h * 0.7, 11. * u, hx(0xdde0ee));
            }
            None => text(font, "(ของไม่พอ)", r.x + 6. * u, give.y + give.h * 0.7, 11. * u, hx(0xaaaaaa)),
        }
    } else if !can { text(font, "(ของไม่พอ)", r.x + 6. * u, give.y + give.h * 0.7, 11. * u, hx(0xaaaaaa)); }
}

#[cfg(test)]
mod tests_v31 {
    use super::*;
    fn mk(id: u32, bed: (i32, i32)) -> npc::Npc {
        let mut n = npc::Npc { id, name: (id % 12) as u8, look: 0, x: bed.0 as f32 * 16. + 8., y: bed.1 as f32 * 16. + 18., state: ST_HOME, hearts: 3, mood: 70, married: false, child: false, role: 0, bed, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(id, (id % 12) as u8) };
        n.soc.anger = 0; n.soc.grudge = 0;
        n
    }
    fn g1() -> (Game, (i32, i32)) {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let (sx, sy) = (tfloor(g.spawn.0) + 80, tfloor(g.spawn.1) + 80);
        for y in sy - 10..=sy + 10 { for x in sx - 10..=sx + 10 { g.w.tmod.insert((x, y), 3); g.w.removed.insert((x, y)); } }
        g.w.placed.insert((sx, sy), 33);
        g.npcs.push(mk(1, (sx, sy)));
        (g, (sx, sy))
    }
    #[test]
    fn fetch_request_give_and_refuse_in_one_tap() {
        let (mut g, _) = g1();
        let day = day_of(&g);
        g.nreqs.push(NReq { from: 1, k: QK::Fetch { item: CARROT, qty: 2 }, day, reward: 15 });
        let w = g.wants(1);
        assert!(matches!(w[0].k, WantK::Req(_)) && !w[0].can, "ของไม่พอ = ปุ่มจาง");
        g.p.inv[CARROT] = 3;
        let coins = g.p.inv[COIN];
        let k = g.wants(1)[0].k.clone();
        g.want_give(1, &k);
        assert_eq!(g.p.inv[CARROT], 1);
        assert_eq!(g.p.inv[COIN], coins + 15);
        assert!(g.nreqs.is_empty());
        // ไม่ให้ = ปิดคำขอ
        g.nreqs.push(NReq { from: 1, k: QK::Fetch { item: GEM, qty: 1 }, day, reward: 18 });
        let k = g.wants(1)[0].k.clone();
        g.want_no(1, &k);
        assert!(g.nreqs.is_empty());
    }
    #[test]
    fn give_building_materials_directly_without_chest() {
        let (mut g, (sx, sy)) = g1();
        g.rs.projects.push(Project { by: 1, kind: 39, at: (sx + 4, sy + 3), need: project_cost(39), started: false, dir: 0, ok: true, got: vec![] });
        // v35 เขาหาวัสดุเองก่อน — ยังไม่ขอเรา จนกว่าจะรอเกิน 3 วัน
        assert!(g.wants(1).iter().all(|w| !matches!(w.k, WantK::Build(_))), "ยังหาเองอยู่ ไม่ขอ");
        g.rs.sh.wait.insert(1, v35::STUCK_BUILD_DAYS);
        let w = g.wants(1);
        let k = w.iter().find(|w| matches!(w.k, WantK::Build(_))).expect("ขอวัสดุ").k.clone();
        for &(it, n) in &project_cost(39) { g.p.inv[it] += n + 2; }
        g.want_give(1, &k);
        assert!(g.rs.projects.iter().any(|p| p.started), "มอบครบ = เริ่มสร้างเลย (ไม่ต้องมีกล่อง)");
        for &(it, n) in &project_cost(39) { assert!(g.p.inv[it] >= 2 && g.p.inv[it] < n + 2 || n == 0); }
        // ไม่ให้วัสดุ = เขายังหาเองต่อ (ไม่เลิกโครงการ) วันนั้นไม่ถามซ้ำ
        let (mut h, (sx, sy)) = g1();
        h.rs.projects.push(Project { by: 1, kind: 39, at: (sx + 4, sy + 3), need: project_cost(39), started: false, dir: 0, ok: true, got: vec![] });
        h.rs.sh.wait.insert(1, v35::STUCK_BUILD_DAYS);
        let k = h.wants(1).iter().find(|w| matches!(w.k, WantK::Build(_))).unwrap().k.clone();
        h.want_no(1, &k);
        assert!(!h.rs.projects.is_empty(), "ไม่เลิกโครงการ");
        assert!(h.wants(1).iter().all(|w| !matches!(w.k, WantK::Build(_))), "วันนี้ไม่ถามซ้ำ");
    }
    #[test]
    fn partial_materials_are_kept_and_counted() {
        let (mut g, (sx, sy)) = g1();
        let need = project_cost(39);
        g.rs.projects.push(Project { by: 1, kind: 39, at: (sx + 4, sy + 3), need: need.clone(), started: false, dir: 0, ok: true, got: vec![] });
        g.rs.sh.wait.insert(1, v35::STUCK_BUILD_DAYS);
        let (it0, n0) = need[0];
        g.p.inv[it0] = 1;
        let k = g.wants(1).iter().find(|w| matches!(w.k, WantK::Build(_))).unwrap().k.clone();
        g.want_give(1, &k);
        assert_eq!(g.p.inv[it0], 0);
        let miss = match &g.wants(1).iter().find(|w| matches!(w.k, WantK::Build(_))).unwrap().k { WantK::Build(m) => m.clone(), _ => unreachable!() };
        assert_eq!(miss.iter().find(|x| x.0 == it0).map(|x| x.1).unwrap_or(0), n0 - 1, "นับของที่มอบไปแล้ว");
    }
    #[test]
    fn villager_stands_still_while_we_talk() {
        let (mut g, (sx, sy)) = g1();
        g.p.x = sx as f32 * 16. + 30.; g.p.y = sy as f32 * 16. + 18.;
        g.npcs[0].wt = 0.; g.npcs[0].wd = (60., 0.);
        g.rs.quick_id = Some(1);
        let (x0, y0) = (g.npcs[0].x, g.npcs[0].y);
        for _ in 0..30 { g.rs.quick_id = Some(1); g.npc_tick(1. / 30.); }
        assert!((g.npcs[0].x - x0).abs() < 0.5 && (g.npcs[0].y - y0).abs() < 0.5, "หยุดรอให้กดปุ่ม");
        g.rs.quick_id = None;
        for _ in 0..60 { g.npc_tick(1. / 30.); }
        assert!((g.npcs[0].x - x0).abs() + (g.npcs[0].y - y0).abs() > 1., "เดินต่อเมื่อเราไปแล้ว");
    }
    #[test]
    fn favourite_item_gift_and_no_thanks_for_today() {
        let (mut g, _) = g1();
        let love = g.npcs[0].soc.love as usize;
        assert!(g.wants(1).iter().any(|w| w.k == WantK::Love(love)));
        g.p.inv[love] = 2;
        let h0 = g.npcs[0].hearts;
        g.want_give(1, &WantK::Love(love));
        assert!(g.npcs[0].hearts > h0 && g.p.inv[love] == 1);
        assert!(!g.wants(1).iter().any(|w| matches!(w.k, WantK::Love(_))), "ให้แล้ววันนี้ไม่ขอซ้ำ");
        let (mut h, _) = g1();
        h.want_no(1, &WantK::Love(love));
        assert!(!h.wants(1).iter().any(|w| matches!(w.k, WantK::Love(_))), "ไม่ให้ = วันนี้ไม่ถามอีก");
    }
}
