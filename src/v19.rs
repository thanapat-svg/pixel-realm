// v19 — ผู้ติดตามนั่งเรือตาม · ชาวบ้านนอนเตียงจริง · คุย/ให้ของเร็ว (ปุ่มเหนือหัว) · วาง/ลากสร้างตรงจุดที่แตะ
use super::*;

/// ระยะสร้าง (ช่อง) — ไกลกว่าระยะทำฟาร์ม จะได้วางทั้งแนวกำแพงโดยไม่ต้องเดินตาม
pub const BUILD_REACH: f32 = 8.5;
/// ระยะที่ปุ่มลัดเหนือหัวชาวบ้านโผล่ (พิกเซล ~2.5 ช่อง)
pub const QUICK_R: f32 = 40.;
/// ชั่วโมงนอน
pub fn sleep_hr(h: f32) -> bool { h >= 21. || h < 5.5 }

impl Game {
    pub fn build_reach(&self, c: (i32, i32)) -> bool {
        (c.0 as f32 * 16. + 8. - self.p.x).hypot(c.1 as f32 * 16. + 8. - (self.p.y - 3.)) <= BUILD_REACH * 16.
    }

    pub fn v19_tick(&mut self, dt: f32) {
        self.boat_party_tick();
        self.rs.bed_t -= dt;
        if self.rs.bed_t <= 0. { self.rs.bed_t = 3.; self.bed_reassign(); }
        if let Some(id) = self.rs.gift_to { if self.npc_near_r(QUICK_R) != Some(id) { self.rs.gift_to = None; } }
    }

    // ---------------------------------------------------------------- เรือ
    /// คนที่ตามเรา (เรียงตามลำดับนั่งเรือ): ชาวบ้านที่ตามมา · ผู้ติดตามที่จ้าง · สัตว์เลี้ยง
    fn boat_party_tick(&mut self) {
        let on = self.on_boat();
        let p = (self.p.x, self.p.y);
        if on {
            if self.rs.trail.last().map_or(true, |l| (l.0 - p.0).hypot(l.1 - p.1) > 1.5) {
                self.rs.trail.push(p);
                if self.rs.trail.len() > 500 { self.rs.trail.remove(0); }
            }
            let mut k = 1;
            let trail = std::mem::take(&mut self.rs.trail);
            let at = |k: usize| trail_back(&trail, 17. * k as f32);
            for n in self.npcs.iter_mut().filter(|n| n.state == ST_FOLLOW) {
                if let Some(q) = at(k) { n.left = q.0 > n.x + 0.3 || (n.left && (q.0 - n.x).abs() < 0.3); n.x = q.0; n.y = q.1; n.moving = false; }
                k += 1;
            }
            if let Some(c) = self.comp.as_mut() { if let Some(q) = at(k) { c.left = q.0 < c.x; c.x = q.0; c.y = q.1; } k += 1; }
            for a in self.animals.iter_mut().filter(|a| a.tame) {
                if let Some(q) = at(k) { a.flip = q.0 < a.x; a.x = q.0; a.y = q.1; a.vx = 0.; a.vy = 0.; }
                k += 1;
            }
            self.rs.trail = trail;
        } else if !self.rs.trail.is_empty() {
            // ขึ้นฝั่ง: ให้ทุกคนขึ้นบกข้างๆ เรา
            self.rs.trail.clear();
            let spots: Vec<(f32, f32)> = (0..24).map(|i| { let a = i as f32 * 0.9; let r = 14. + (i / 6) as f32 * 8.; (p.0 + a.cos() * r, p.1 + a.sin() * r) }).filter(|&(x, y)| !self.w.solid_at(tfloor(x), tfloor(y - 3.)) && self.w.tile_at(tfloor(x), tfloor(y - 3.)) >= 2).collect();
            let mut si = 0;
            let mut next = || { let s = spots.get(si).copied().unwrap_or((p.0 - 10., p.1)); si += 1; s };
            for n in self.npcs.iter_mut().filter(|n| n.state == ST_FOLLOW) { if self.w.tile_at(tfloor(n.x), tfloor(n.y - 2.)) < 2 { let s = next(); n.x = s.0; n.y = s.1; } }
            if let Some(c) = self.comp.as_mut() { if self.w.tile_at(tfloor(c.x), tfloor(c.y - 2.)) < 2 { let s = next(); c.x = s.0; c.y = s.1; } }
            for a in self.animals.iter_mut().filter(|a| a.tame) { if self.w.tile_at(tfloor(a.x), tfloor(a.y - 2.)) < 2 { let s = next(); a.x = s.0; a.y = s.1; } }
        }
    }
    /// ตำแหน่งนี้กำลังนั่งเรือ (ตามเราอยู่บนน้ำ)
    pub fn rides_boat(&self, x: f32, y: f32) -> bool {
        !self.rs.trail.is_empty() && self.w.dun.is_none() && self.w.tile_at(tfloor(x), tfloor(y - 2.)) < 2
    }

    // ---------------------------------------------------------------- เตียง/การนอน
    /// เตียงหาย → หาเตียงว่างใหม่ · นอนกลางแจ้งแต่มีเตียงว่างในบ้าน → ย้ายเข้าบ้าน
    pub fn bed_reassign(&mut self) {
        if self.w.dun.is_some() { return; }
        let me = self.bed;
        let ids: Vec<u32> = self.npcs.iter().filter(|n| n.state == ST_HOME).map(|n| n.id).collect();
        for id in ids {
            let Some(i) = self.idx(id) else { continue };
            let bed = self.npcs[i].bed;
            let ok = self.w.placed.get(&bed) == Some(&33);
            let indoor = ok && self.w.room_from(bed).is_some();
            if ok && indoor { continue; }
            let free: Vec<(i32, i32)> = self.free_beds().into_iter().filter(|b| Some(*b) != me).collect();
            let (nx, ny) = (self.npcs[i].x, self.npcs[i].y);
            let pick = free.into_iter().filter(|b| !ok || self.w.room_from(*b).is_some())
                .min_by_key(|b| { let room = self.w.room_from(*b).is_some(); (!room as i32, ((b.0 as f32 * 16. - nx).hypot(b.1 as f32 * 16. - ny)) as i32) });
            if let Some(b) = pick {
                let nm = NPC_NAMES[self.npcs[i].name as usize];
                self.npcs[i].bed = b;
                self.rs.npaths.remove(&id);
                self.toast(format!("{}ย้ายไปนอนเตียงใหม่{}", nm, if self.w.room_from(b).is_some() { "ในบ้าน" } else { "" }));
            }
        }
    }
    /// จุดยืนข้างเตียง (ตอนตื่น) และจุดนอนบนเตียง
    pub fn bed_spot(bed: (i32, i32)) -> (f32, f32) { (bed.0 as f32 * 16. + 8., bed.1 as f32 * 16. + 18.) }
    pub fn bed_lie(bed: (i32, i32)) -> (f32, f32) { (bed.0 as f32 * 16. + 8., bed.1 as f32 * 16. + 15.) }
    pub fn npc_sleeping(&self, n: &npc::Npc) -> bool { self.rs.sleeping.contains(&n.id) }

    // ---------------------------------------------------------------- คุย/ให้ของเร็ว
    /// คุยทันที (ไม่ต้องเปิดเมนู): ได้ผลวันละครั้ง ครั้งต่อไปเป็นคุยเล่น
    pub fn quick_talk(&mut self, id: u32) {
        let Some(i) = self.idx(id) else { return };
        if self.rs.sleeping.contains(&id) { self.npc_bubble(id, "Zzz... (หลับอยู่)"); return; }
        let first = !self.npcs[i].talked && !refuses(&self.npcs[i]);
        if first { self.npc_talk(id); }
        let n = self.npcs[i].clone();
        let pick = ((self.time * 0.7) as usize + id as usize) % 3;
        // คำพูดตามนิสัย/อารมณ์ (ระบบเดิม) · บางครั้งบอกของที่อยากได้
        let line: String = if pick == 0 && n.hearts >= 2 && !refuses(&n) && n.mood >= 30 { format!("ช่วงนี้อยากได้{}จัง", INAME[n.soc.love as usize]) } else { self.npc_say(&n) };
        self.npc_bubble(id, &line);
        if !first { play(&self.sfx.click); }
    }
    pub fn npc_bubble(&mut self, id: u32, s: &str) {
        if let Some(p) = self.psy.get_mut(&id) { p.bubble = Some((s.to_string(), 3.2)); }
    }
    /// แตะ "ให้ของ" → แตะของในแถบล่างเพื่อให้
    pub fn quick_gift_toggle(&mut self, id: u32) {
        if self.rs.gift_to == Some(id) { self.rs.gift_to = None; return; }
        if hotbar_items(self).is_empty() { self.toast("ไม่มีของในกระเป๋าที่จะให้"); return; }
        self.rs.gift_to = Some(id);
        play(&self.sfx.click);
    }
    /// ใช้แทน hotbar_use ตอนอยู่ในโหมดให้ของ (คืน true ถ้าให้แล้ว)
    pub fn quick_gift_item(&mut self, k: usize) -> bool {
        let Some(id) = self.rs.gift_to else { return false };
        let items = hotbar_items(self);
        let Some(&it) = items.get(k) else { return true };
        self.rs.gift_to = None;
        let before = self.p.inv[it];
        self.npc_gift_item(id, it);
        if self.p.inv[it] < before {
            let Some(i) = self.idx(id) else { return true };
            let n = &self.npcs[i];
            let line = if it == n.soc.love as usize { "ชอบมาก! เหมือนรู้ใจกันเลย" } else if it == n.soc.hate as usize { "ของแบบนี้... ไม่ชอบเลย" } else { "ขอบคุณนะ!" };
            self.npc_bubble(id, line);
        }
        true
    }

    // ---------------------------------------------------------------- สร้างตรงจุดที่แตะ/ลาก
    /// แตะ = วางตรงนั้น · ลาก = วางต่อเนื่อง (กำแพง หน้าต่าง กำแพงเมือง พื้น ไถดิน) · บ้านสำเร็จรูป = แตะซ้ำที่เดิมเพื่อยืนยัน
    pub fn build_drag(&mut self, t: (i32, i32), started: bool) {
        let Some(sel) = self.build_sel else { return };
        self.build_cur = Some(t);
        self.cur_pos = (self.p.x, self.p.y);
        let o = PIECES[sel].obj;
        if paintable(o) {
            if started {
                self.rs.drag_last = Some(t);
                self.place_piece();
                return;
            }
            let Some(prev) = self.rs.drag_last else { self.rs.drag_last = Some(t); return };
            if prev == t { return; }
            // เดินทีละช่องแบบ 4 ทิศ (ลากเฉียงก็ไม่มีรูรั่ว กำแพงต่อกันสนิท)
            let mut c = prev;
            let mut guard = 0;
            while c != t && guard < 40 {
                guard += 1;
                let (dx, dy) = (t.0 - c.0, t.1 - c.1);
                if dx.abs() >= dy.abs() { c.0 += dx.signum(); } else { c.1 += dy.signum(); }
                self.place_quiet(o, c, sel);
            }
            self.rs.drag_last = Some(t);
        } else if started {
            if is_prefab(o) {
                if self.rs.build_arm == Some(t) { self.rs.build_arm = None; self.place_piece(); }
                else { self.rs.build_arm = Some(t); self.toast("ดูตำแหน่งให้ดี แล้วแตะซ้ำที่เดิมเพื่อสร้าง"); }
            } else {
                self.place_piece();
            }
        }
    }
    fn place_quiet(&mut self, o: u8, c: (i32, i32), sel: usize) {
        if !self.build_reach(c) || self.w.dun.is_some() { return; }
        if o == OBJ_FLOOR_W || o == OBJ_FLOOR_S {
            let want = if o == OBJ_FLOOR_W { T_FLOORW } else { T_FLOORS };
            if self.w.tile_at(c.0, c.1) != want && self.place_floor_quiet(c, o) { self.room_t = 0.; play(&self.sfx.door); }
            return;
        }
        if o == OBJ_TILL { if self.till_at(c) { self.save(); } return; }
        let pc = &PIECES[sel];
        if self.can_place_at(c.0, c.1) && can_pay(&self.p, pc.cost) {
            for &(it, n) in pc.cost { self.p.inv[it] -= n; }
            self.w.placed.insert(c, o);
            self.obj_dmg.remove(&c);
            self.room_t = 0.;
            play(&self.sfx.door);
            self.burst(c.0 as f32 * 16. + 8., c.1 as f32 * 16. + 10., hx(0xc8a070), 4, 30.);
        }
    }
}
pub fn paintable(o: u8) -> bool { matches!(o, 10 | 11 | 38 | O_WINDOW | OBJ_FLOOR_W | OBJ_FLOOR_S | OBJ_TILL) }

/// จุดบนเส้นทางเรือย้อนหลังไป dist พิกเซล
pub fn trail_back(tr: &[(f32, f32)], dist: f32) -> Option<(f32, f32)> {
    if tr.is_empty() { return None; }
    let mut acc = 0.;
    for i in (1..tr.len()).rev() {
        let (a, b) = (tr[i], tr[i - 1]);
        let d = (a.0 - b.0).hypot(a.1 - b.1);
        if acc + d >= dist { let k = (dist - acc) / d.max(0.001); return Some((a.0 + (b.0 - a.0) * k, a.1 + (b.1 - a.1) * k)); }
        acc += d;
    }
    Some(tr[0])
}

// ---------------------------------------------------------------- ปุ่มลัดเหนือหัวชาวบ้าน
/// (id, [คุย, ให้ของ, อื่นๆ]) ในพิกัดจอ
pub fn quick_btns(g: &Game, v: &View) -> Option<(u32, [Rect; 3])> {
    if g.build_sel.is_some() || g.big_map || g.w.dun.is_some() { return None; }
    let id = g.npc_near_r(QUICK_R)?;
    let n = g.npcs.iter().find(|n| n.id == id)?;
    let u = ui_scale();
    let (bw, bh, gap) = (52. * u, 26. * u, 6. * u);
    let cx = (n.x - v.cx) * v.s;
    let top = (n.y - v.cy - 34.) * v.s - bh - 4. * u;
    let x0 = cx - (bw * 3. + gap * 2.) / 2.;
    let r = |i: usize| Rect::new(x0 + i as f32 * (bw + gap), top, bw, bh);
    Some((id, [r(0), r(1), r(2)]))
}
pub fn draw_quick(g: &Game, v: &View, font: &Font) {
    let Some((id, rs)) = quick_btns(g, v) else { return };
    let u = ui_scale();
    let gifting = g.rs.gift_to == Some(id);
    let labels = ["คุย", if gifting { "ยกเลิก" } else { "ให้ของ" }, "อื่นๆ"];
    for (i, r) in rs.iter().enumerate() {
        let on = i == 1 && gifting;
        draw_rectangle(r.x, r.y, r.w, r.h, if on { Color::new(0.95, 0.75, 0.25, 0.92) } else { Color::new(0.08, 0.08, 0.14, 0.78) });
        draw_rectangle_lines(r.x, r.y, r.w, r.h, 2., Color::new(1., 0.86, 0.45, 0.9));
        text_c(font, labels[i], r.x + r.w / 2., r.y + r.h * 0.72, 13. * u, if on { hx(0x2a1a00) } else { WHITE });
    }
    if gifting {
        let (hx0, hy, s) = hotbar_geom(u);
        let n = hotbar_items(g).len().max(1) as f32;
        let w = n * (s + 2. * u);
        draw_rectangle_lines(hx0 - 3. * u, hy - 3. * u, w + 4. * u, s + 6. * u, 3., hx(0xffd84a));
        let nm = g.npcs.iter().find(|n| n.id == id).map(|n| NPC_NAMES[n.name as usize]).unwrap_or("");
        let msg = format!("แตะของในแถบล่างที่จะให้{}", nm);
        let (cx, ty) = (rs[1].x + rs[1].w / 2., rs[0].y - 8. * u);
        let d = measure_text(&msg, Some(font), (13. * u) as u16, 1.);
        draw_rectangle(cx - d.width / 2. - 6. * u, ty - 15. * u, d.width + 12. * u, 20. * u, Color::new(0., 0., 0., 0.7));
        text_c(font, &msg, cx, ty, 13. * u, hx(0xffd84a));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn mk(id: u32, state: u8, bed: (i32, i32), x: f32, y: f32) -> npc::Npc {
        npc::Npc { id, name: (id % 12) as u8, look: 0, x, y, state, hearts: 3, mood: 70, married: false, child: false, role: 0, bed, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(id, (id % 12) as u8) }
    }
    fn flat(g: &mut Game, cx: i32, cy: i32, r: i32) { for y in cy - r..=cy + r { for x in cx - r..=cx + r { g.w.tmod.insert((x, y), 3); g.w.removed.insert((x, y)); } } }
    #[test]
    fn trail_back_walks_along_path() {
        let tr: Vec<(f32, f32)> = (0..20).map(|i| (i as f32 * 2., 0.)).collect();
        let p = trail_back(&tr, 10.).unwrap();
        assert!((p.0 - 28.).abs() < 0.01);
        assert_eq!(trail_back(&tr, 999.), Some((0., 0.)));
    }
    #[test]
    fn followers_ride_boat_and_come_ashore() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let (cx, cy) = (6000, 6000);
        flat(&mut g, cx, cy, 20);
        for y in cy - 20..=cy + 20 { for x in cx..=cx + 20 { g.w.tmod.insert((x, y), 1); } }
        g.p.boat = true;
        g.p.x = (cx - 2) as f32 * 16. + 8.; g.p.y = cy as f32 * 16. + 12.;
        g.npcs.push(mk(1, ST_FOLLOW, (0, 0), g.p.x - 10., g.p.y));
        // พายเรือออกไปทางขวา 8 ช่อง
        for i in 0..160 { g.p.x += 1.; if i % 2 == 0 { g.v19_tick(0.016); } }
        assert!(g.on_boat());
        let n = &g.npcs[0];
        assert!(g.rides_boat(n.x, n.y), "ผู้ติดตามอยู่บนน้ำ (นั่งเรือ)");
        assert!((g.p.x - n.x) > 10. && (g.p.x - n.x) < 30., "นั่งตามหลังเรา {}", g.p.x - n.x);
        // กลับขึ้นฝั่ง
        g.p.x = (cx - 3) as f32 * 16. + 8.;
        g.v19_tick(0.016);
        let n = &g.npcs[0];
        assert!(g.w.tile_at(tfloor(n.x), tfloor(n.y - 2.)) >= 2, "ขึ้นบกด้วย");
        assert!(g.rs.trail.is_empty());
    }
    #[test]
    fn villager_moves_to_new_bed_and_sleeps_on_it() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let (sx, sy) = (tfloor(g.spawn.0) + 40, tfloor(g.spawn.1) + 40);
        flat(&mut g, sx, sy, 14);
        // เตียงเดิมกลางแจ้ง · สร้างบ้านมีเตียงใหม่
        let old = (sx - 8, sy);
        g.w.placed.insert(old, 33);
        for x in sx..=sx + 4 { g.w.placed.insert((x, sy), 11); g.w.placed.insert((x, sy + 4), if x == sx + 2 { 12 } else { 11 }); }
        for y in sy..=sy + 4 { g.w.placed.insert((sx, y), 11); g.w.placed.insert((sx + 4, y), 11); }
        let nb = (sx + 2, sy + 1);
        g.w.placed.insert(nb, 33);
        g.npcs.push(mk(1, ST_HOME, old, old.0 as f32 * 16., old.1 as f32 * 16. + 18.));
        g.p.x = (sx - 12) as f32 * 16.; g.p.y = sy as f32 * 16.;
        g.bed_reassign();
        assert_eq!(g.npcs[0].bed, nb, "ย้ายเข้าเตียงในบ้าน");
        // เตียงถูกรื้อ → หาเตียงว่าง
        g.w.placed.remove(&nb);
        g.bed_reassign();
        assert_eq!(g.npcs[0].bed, old);
        g.w.placed.insert(nb, 33);
        g.bed_reassign();
        assert_eq!(g.npcs[0].bed, nb);
        // ดึกแล้ว: เดินไปนอนบนเตียง
        while !sleep_hr(g.hour()) { g.time += 5.; }
        g.last_day = day_of(&g);
        g.npcs[0].x = (sx + 2) as f32 * 16. + 8.; g.npcs[0].y = (sy + 7) as f32 * 16. + 11.;
        for _ in 0..(30 * 40) { g.npc_tick(1. / 30.); if g.rs.sleeping.contains(&1) { break; } }
        assert!(g.rs.sleeping.contains(&1), "นอนแล้ว");
        let lie = Game::bed_lie(nb);
        assert!((g.npcs[0].x - lie.0).abs() < 0.5 && (g.npcs[0].y - lie.1).abs() < 0.5, "นอนบนเตียง");
        // เช้า: ลุก
        while sleep_hr(g.hour()) { g.time += 5.; }
        g.last_day = day_of(&g);
        g.npc_tick(0.1);
        assert!(!g.rs.sleeping.contains(&1));
    }
    #[test]
    fn quick_talk_and_gift_without_menu() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        g.npcs.push(mk(1, ST_FOLLOW, (0, 0), g.p.x + 6., g.p.y));
        g.psy_tick(0.1);
        assert_eq!(g.npc_near_r(QUICK_R), Some(1));
        let h0 = g.npcs[0].hearts;
        g.quick_talk(1);
        assert!(g.npcs[0].talked, "คุยแล้วได้ผลทันที");
        assert!(g.psy.get(&1).map_or(false, |p| p.bubble.is_some()), "มีคำพูดลอยเหนือหัว");
        g.quick_talk(1); // คุยเล่นซ้ำได้ ไม่เพิ่มหัวใจอีก
        assert!(g.npcs[0].hearts <= h0 + 1);
        g.p.inv = [0; NI];
        g.p.inv[BERRY] = 3;
        g.quick_gift_toggle(1);
        assert_eq!(g.rs.gift_to, Some(1));
        assert!(g.quick_gift_item(0));
        assert_eq!(g.p.inv[BERRY], 2);
        assert!(g.npcs[0].gifted);
        assert_eq!(g.rs.gift_to, None);
    }
    #[test]
    fn drag_builds_continuous_wall_line_within_reach() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let (sx, sy) = (tfloor(g.spawn.0) + 60, tfloor(g.spawn.1) + 60);
        flat(&mut g, sx, sy, 14);
        g.p.x = sx as f32 * 16. + 8.; g.p.y = sy as f32 * 16. + 12.;
        g.p.inv[STONE] = 200; g.p.inv[WOOD] = 200;
        g.build_sel = PIECES.iter().position(|p| p.obj == 11);
        // ลากเฉียงจาก (-4,-3) ไป (4,2)
        g.build_drag((sx - 4, sy - 3), true);
        for i in 1..=8 { g.build_drag((sx - 4 + i, sy - 3 + (i * 5) / 8), false); }
        let walls: Vec<(i32, i32)> = g.w.placed.iter().filter(|(_, &o)| o == 11).map(|(k, _)| *k).collect();
        assert!(walls.len() >= 12, "วางต่อเนื่อง {}", walls.len());
        // ต่อกันแบบ 4 ทิศ (ไม่มีรูเฉียง)
        for w in &walls { if *w != (sx + 4, sy + 2) && *w != (sx - 4, sy - 3) { let nb = walls.iter().filter(|o| (o.0 - w.0).abs() + (o.1 - w.1).abs() == 1).count(); assert!(nb >= 1); } }
        assert!(g.w.placed.get(&(sx + 4, sy + 2)) == Some(&11));
        // ไกลเกินระยะ: ไม่วาง
        g.build_drag((sx + 12, sy), true);
        assert!(g.w.placed.get(&(sx + 12, sy)).is_none());
        // บ้านสำเร็จรูป: แตะครั้งแรกแค่เล็ง
        g.build_sel = PIECES.iter().position(|p| p.obj == OBJ_TENT);
        g.p.inv = [99; NI];
        g.build_drag((sx, sy + 5), true);
        assert_eq!(g.rs.build_arm, Some((sx, sy + 5)));
    }
    #[test]
    fn hand_cycle_includes_bare_hands() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        g.p.sword = 3; g.p.bow = true; g.p.weapon = 0; g.p.bare = false;
        g.swap_weapon(); assert_eq!((g.p.weapon, g.p.bare), (1, false), "ดาบ → ธนู");
        g.swap_weapon(); assert_eq!((g.p.weapon, g.p.bare), (0, true), "ธนู → มือเปล่า");
        assert_eq!(g.p.weapon_name(), "มือเปล่า");
        g.swap_weapon(); assert_eq!((g.p.weapon, g.p.bare), (0, false), "มือเปล่า → ดาบ");
        // บันทึก/โหลดจำว่าถือมือเปล่า
        g.p.bare = true;
        let h = Game::load_str(&g.save_string(), Sfx::default()).ok().unwrap();
        assert!(h.p.bare && h.p.sword == 3);
    }
    #[test]
    fn sword_passes_through_villagers() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        g.p.dir = 2; g.p.flip = false;
        g.npcs.push(mk(1, ST_HOME, (0, 0), g.p.x + 12., g.p.y));
        let (h, a) = (g.npcs[0].hearts, g.npcs[0].soc.anger);
        for _ in 0..3 { g.p.atk_cd = 0.; g.do_attack(); }
        assert_eq!((g.npcs[0].hearts, g.npcs[0].soc.anger), (h, a), "ไม่บาดเจ็บ ไม่โกรธ");
    }
}
