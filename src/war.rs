// war.rs — v24 การต่อสู้ที่ลึกขึ้น
//  - มอนมีท่าตาย (สไลม์ละลาย · โครงกระดูกล้มเป็นกองกระดูก · ค้างคาวร่วง · ทหารล้มลง · สัตว์ประหลาดหงายท้อง · ปีศาจสลายเป็นควัน)
//  - ศัตรูประจำภูมิประเทศ: แมงป่อง (ทะเลทราย) · หมาป่า (หิมะ มาเป็นฝูง) · ปูยักษ์ (ชายหาด) · ปีศาจเถ้า (ภูเขาไฟ ยิงลูกไฟ)
//  - แม่ทัพผู้พิทักษ์ประจำเมืองหลวงทุกอาณาจักร (บอส: ฟันกวาด · พุ่งแทง · เรียกองครักษ์)
//  - ทัพของเรา: ทหารในเมืองตามเราไปตีเมืองศัตรู · ศึกยึดเมือง · บาดเจ็บ (ไม่ตาย) พักฟื้น 1 วัน
use super::*;

// ---------------------------------------------------------------- ท่าตาย
#[derive(Clone)]
pub struct Dying { pub k: EK, pub x: f32, pub y: f32, pub pal: usize, pub t: f32, pub left: bool, pub seed: u32 }
pub const DIE_T: f32 = 0.9;

pub fn draw_dying(g: &Game, sp: &Sprites, v: &View, i: usize) {
    let d = &g.dying[i];
    let q = (d.t / DIE_T).clamp(0., 1.);
    let a = 1. - ((d.t - 0.5) / (DIE_T - 0.5)).clamp(0., 1.);
    let (sx, sy) = ((d.x - v.cx).round(), (d.y - v.cy).round());
    let col = Color::new(1., 1., 1., a);
    match d.k {
        EK::Slime => {
            // แบนลงเป็นแอ่งเมือกแล้วจางหาย
            let k = (q * 2.).min(1.);
            let (ww, hh) = ((12. * (1. + k * 0.9)).round(), (10. * (1. - k * 0.75)).round().max(2.));
            blit_sz(sp, v, sp.slime[d.pal.min(3)], (sx - ww / 2.).floor(), sy - hh, ww, hh, col);
        }
        EK::Skel => {
            // หัวกะโหลกกระเด็น ตัวทรุดลงเป็นกองกระดูก
            let r = sp.skel;
            let k = (q * 2.5).min(1.);
            let sink = (k * 9.).round();
            draw_texture_ex(&sp.tex, (sx - 5.) * v.s, (sy - 15. + 5. + sink) * v.s, col, DrawTextureParams { source: Some(Rect::new(r.x, r.y + 5., r.w, (r.h - 5. - sink).max(1.))), dest_size: Some(vec2(r.w * v.s, (r.h - 5. - sink).max(1.) * v.s)), ..Default::default() });
            let dir = if d.left { 1. } else { -1. };
            let hx_ = sx - 5. + dir * k * 9.;
            let hy = sy - 15. + (k * std::f32::consts::PI).sin() * -6. + k * 11.;
            draw_texture_ex(&sp.tex, hx_ * v.s, hy * v.s, col, DrawTextureParams { source: Some(Rect::new(r.x, r.y, r.w, 5.)), dest_size: Some(vec2(r.w * v.s, 5. * v.s)), ..Default::default() });
            for b in 0..4 { let bx = sx - 5. + b as f32 * 3. + (hash(d.seed as i32, b, 9401) * 2.) as f32; prect(v, bx, sy - 2., 3., 1., Color::new(0.9, 0.9, 0.85, a)); }
        }
        EK::Bat => {
            // ร่วงลงพื้นหมุนคว้าง
            let bi = 0;
            let r = sp.bat[bi];
            let fall = (q * 1.6).min(1.);
            let y = sy - 18. + fall * 14.;
            draw_texture_ex(&sp.tex, (sx - 6.) * v.s, y * v.s, col, DrawTextureParams { source: Some(r), dest_size: Some(vec2(r.w * v.s, r.h * v.s)), rotation: fall * 6.0, flip_y: fall > 0.6, ..Default::default() });
        }
        EK::Soldier => {
            // ล้มหงายลงช้าๆ
            let body = Body::generate(1000 + d.pal as u32 * 7 + (d.x as u32 % 5), false);
            let k = (q * 2.).min(1.);
            let rot = if d.left { 1. } else { -1. } * k * std::f32::consts::FRAC_PI_2;
            skin::draw_figure_rot(sp, &body, skin::ROLE_PARTS[2], (sx - 6.) * v.s, (sy - 15. + k * 8.) * v.s, v.s, rot, col);
        }
        EK::Imp => {
            // หดตัวสลายเป็นควันดำ
            let k = (q * 1.8).min(1.);
            if k < 1. { draw_beast_s(v, d.k, sx, sy, 0., d.left, Color::new(1., 1., 1., a), false, false, a, 0., 1. - k * 0.8); }
            for j in 0..5 {
                let ang = j as f32 * 1.26 + d.seed as f32 * 0.01;
                let rr = 3. + q * 12.;
                prect(v, sx + ang.cos() * rr - 1.5, sy - 12. + ang.sin() * rr * 0.6 - q * 8., 3., 3., Color::new(0.25, 0.22, 0.25, a * 0.8));
            }
        }
        _ => {
            // แมงป่อง/หมาป่า/ปู: เด้งขึ้นแล้วหงายท้อง ขากระตุก
            let hop = (q * std::f32::consts::PI * 1.6).sin().max(0.) * 5.;
            let kick = (d.t * 30.).sin() * (1. - q);
            draw_beast(v, d.k, sx, sy - hop, kick, d.left, Color::new(1., 1., 1., a), false, q > 0.12, a, 0.);
        }
    }
}

// ---------------------------------------------------------------- สไปรต์สัตว์ประหลาดใหม่ (วาดจากโค้ด)
const SCORP: [&str; 10] = [
    "...ss...........",
    "..o..o..........",
    "..o...o.........",
    "...o..o.........",
    "......o.........",
    ".....oDDDDDo.cc.",
    "....oDhhhhDDoc.c",
    "....oDDDDDDDDoc.",
    "....l.l.l.l.l...",
    "...l.l.l.l.l....",
];
const WOLF: [&str; 11] = [
    "............oo..",
    "...........oDDo.",
    "..........oDDeDo",
    "o........oDDDDDn",
    "oo.ooooooDDDDDo.",
    ".oDDDDDDDDDDDo..",
    "..oDDDDDDDDDDo..",
    "..oDhhhhhhhDDo..",
    "...oo.o...o.oo..",
    "...o..o...o..o..",
    "...o..o...o..o..",
];
const CRAB: [&str; 9] = [
    ".cc........cc.",
    "c..c......c..c",
    ".cc..e..e..cc.",
    "..o..o..o..o..",
    "..oDDDDDDDDo..",
    ".oDDDDDDDDDDo.",
    ".oDDhhhhhhDDo.",
    "..oooooooooo..",
    ".l.l.l..l.l.l.",
];
const IMP: [&str; 11] = [
    "..h......h..",
    "..ho....oh..",
    "...oDDDDo...",
    "w..DeDDeD..w",
    "ww.DDDDDD.ww",
    "www.DmmD.www",
    "wwwoDDDDowww",
    ".w.DDDDDD.w.",
    "...DDDDDD...",
    "....D..D....",
    "....t.......",
];
fn pal_of(k: EK, ch: char) -> Option<u32> {
    match (k, ch) {
        (EK::Scorp, 'o') => Some(0x6a3a14), (EK::Scorp, 'D') => Some(0xc8823a), (EK::Scorp, 'h') => Some(0xe8a85a), (EK::Scorp, 'c') => Some(0xa86a2a), (EK::Scorp, 's') => Some(0x3a1a0a), (EK::Scorp, 'l') => Some(0x6a3a14),
        (EK::Wolf, 'o') => Some(0x6a7080), (EK::Wolf, 'D') => Some(0xc8ccd8), (EK::Wolf, 'h') => Some(0xeef0f6), (EK::Wolf, 'e') => Some(0x2a2a3a), (EK::Wolf, 'n') => Some(0x2a2a3a),
        (EK::Crab, 'o') => Some(0x8a2a1a), (EK::Crab, 'D') => Some(0xd8483a), (EK::Crab, 'h') => Some(0xf07a5a), (EK::Crab, 'c') => Some(0xc83a2a), (EK::Crab, 'e') => Some(0xffffff), (EK::Crab, 'l') => Some(0x8a2a1a),
        (EK::Imp, 'o') => Some(0x4a1414), (EK::Imp, 'D') => Some(0x9a2e2a), (EK::Imp, 'h') => Some(0x2a1a1a), (EK::Imp, 'e') => Some(0xffc030), (EK::Imp, 'm') => Some(0x2a0a0a), (EK::Imp, 'w') => Some(0x4a1a2a), (EK::Imp, 't') => Some(0x9a2e2a),
        _ => None,
    }
}
fn art(k: EK) -> &'static [&'static str] {
    match k { EK::Scorp => &SCORP, EK::Wolf => &WOLF, EK::Crab => &CRAB, _ => &IMP }
}
/// วาดสัตว์ประหลาดใหม่ (sx, sy = จุดเท้ากลางตัว) · flip_y = หงายท้อง (ท่าตาย) · sk = เวลาสกิล (เตรียมโจมตี = กระพริบ)
pub fn draw_beast(v: &View, k: EK, sx: f32, sy: f32, anim: f32, left: bool, tint: Color, white: bool, flip_y: bool, a: f32, sk: f32) {
    draw_beast_s(v, k, sx, sy, anim, left, tint, white, flip_y, a, sk, 1.);
}
pub fn draw_beast_s(v: &View, k: EK, sx: f32, sy: f32, anim: f32, left: bool, tint: Color, white: bool, flip_y: bool, a: f32, sk: f32, scale: f32) {
    let rows = art(k);
    let h = rows.len() as f32;
    let w = rows[0].len() as f32;
    let fly = k == EK::Imp;
    let hover = if fly { 10. + (anim * 4.).sin() * 2. } else { 0. };
    let frame = ((anim * if k == EK::Wolf { 10. } else { 7. }) as i32) & 1;
    // เงา
    if !flip_y || a > 0.3 { prect(v, sx - w * 0.35, sy - 1., w * 0.7, 2., Color::new(0., 0., 0., 0.25 * a)); }
    // เตรียมโจมตี: เรืองแดง
    let warn = sk < 0.35 && sk > 0. && !flip_y && matches!(k, EK::Scorp | EK::Wolf | EK::Imp);
    for (ry, row) in rows.iter().enumerate() {
        for (rx, ch) in row.chars().enumerate() {
            let Some(c) = pal_of(k, ch) else { continue };
            // ขาสลับ 2 เฟรม · ปีกปีศาจกระพือ
            let mut ox = 0.;
            let mut oy = 0.;
            if ch == 'l' && frame == 1 { ox = 1.; }
            if k == EK::Wolf && ry >= 8 && frame == 1 { ox = if rx % 4 < 2 { 1. } else { -1. }; }
            if ch == 'w' && frame == 1 { oy = -1.; }
            let fx = if left { w - 1. - rx as f32 } else { rx as f32 } + ox;
            let fy = if flip_y { ry as f32 - oy } else { ry as f32 + oy };
            let mut col = hx(c);
            if white { col = Color::new(1., 1., 1., 1.); } else {
                col = Color::new(col.r * tint.r, col.g * tint.g, col.b * tint.b, a);
                if warn && ch == 'D' { col = Color::new((col.r + 0.5).min(1.), col.g * 0.7, col.b * 0.7, a); }
            }
            col.a = a;
            let px = sx - w * scale / 2. + fx * scale;
            let py = if flip_y { sy - h * scale + (h - 1. - fy) * scale } else { sy - h * scale + fy * scale } - hover;
            prect(v, px, py, scale, scale, col);
        }
    }
}

// ---------------------------------------------------------------- พฤติกรรม
impl Game {
    /// คืนความเร็ว (mx, my) ของสัตว์ประหลาดใหม่ตอนเห็นเรา (None = ใช้พฤติกรรมเดิม)
    pub fn beast_ai(&mut self, e: &mut Enemy, dx: f32, dy: f32, d: f32, dt: f32) -> Option<(f32, f32)> {
        if !matches!(e.k, EK::Scorp | EK::Wolf | EK::Crab | EK::Imp) || d > 150. { return None; }
        e.sk -= dt;
        let (nx, ny) = (dx / d, dy / d);
        match e.k {
            EK::Scorp => {
                // เดินช้า เข้าใกล้แล้วพุ่งต่อยด้วยหาง
                if d < 38. && e.sk <= 0. {
                    e.sk = 2.2;
                    e.kb = 0.2; e.vx = nx * 170.; e.vy = ny * 170.;
                    return Some((0., 0.));
                }
                Some((nx * e.spd, ny * e.spd))
            }
            EK::Wolf => {
                // วนล้อมรอบตัวเรา แล้วกระโจนกัด
                if d < 64. && e.sk <= 0. {
                    e.sk = 1.6 + gen_range(0., 0.6);
                    e.kb = 0.22; e.vx = nx * 210.; e.vy = ny * 210.;
                    return Some((0., 0.));
                }
                if d < 56. {
                    let s = if e.pal % 2 == 0 { 1. } else { -1. };
                    Some(((-ny * s * 0.9 - nx * 0.2) * e.spd, (nx * s * 0.9 - ny * 0.2) * e.spd))
                } else { Some((nx * e.spd, ny * e.spd)) }
            }
            EK::Crab => {
                // เดินเฉียงข้าง (ขยับซ้าย-ขวาเป็นหลัก)
                let wob = (e.anim * 3.).sin() * 0.6;
                Some((nx * e.spd * 1.3, (ny * 0.35 + wob * 0.3) * e.spd))
            }
            _ => {
                // ปีศาจเถ้า: ลอยวน ยิงลูกไฟ · บางทีวาร์ปหลบ
                if e.sk <= 0. && d < 140. {
                    e.sk = 2.6;
                    if gen_range(0., 1.) < 0.3 {
                        let a = gen_range(0., std::f32::consts::TAU);
                        let (tx, ty) = (self.p.x + a.cos() * 60., self.p.y + a.sin() * 40.);
                        self.burst(e.x, e.y - 12., hx(0x3a3036), 10, 40.);
                        e.x = tx; e.y = ty;
                        self.burst(e.x, e.y - 12., hx(0x3a3036), 10, 40.);
                    } else {
                        let (ddx, ddy) = (self.p.x - e.x, self.p.y - 6. - (e.y - 12.));
                        let l = ddx.hypot(ddy).max(1.);
                        self.proj.push(Proj { x: e.x, y: e.y - 12., vx: ddx / l * 90., vy: ddy / l * 90., kind: 7, dmg: e.dmg, life: 2.2, mine: false, dead: false });
                        play_vol(&self.sfx.shoot, 0.5);
                    }
                }
                let orbit = if d < 70. { -0.6 } else { 0.8 };
                Some(((nx * orbit + (e.anim * 2.).cos() * 0.5) * e.spd, (ny * orbit + (e.anim * 2.7).sin() * 0.5) * e.spd))
            }
        }
    }
    pub fn beast_loot(&mut self, e: &Enemy) {
        let (lx, ly) = (e.x, e.y - 6.);
        match e.k {
            EK::Scorp => { if gen_range(0., 1.) < 0.3 { self.spawn_loot(lx, ly, SHARD, 1); } if gen_range(0., 1.) < 0.15 { self.spawn_loot(lx, ly, GEM, 1); } }
            EK::Wolf => { if gen_range(0., 1.) < 0.6 { self.spawn_loot(lx, ly, MEAT, 1); } if gen_range(0., 1.) < 0.3 { self.add_hide(1); } }
            EK::Crab => { if gen_range(0., 1.) < 0.5 { self.spawn_loot(lx, ly, FISH, 1); } }
            EK::Imp => { if gen_range(0., 1.) < 0.5 { self.spawn_loot(lx, ly, SHARD, rnd(1, 2)); } if gen_range(0., 1.) < 0.1 { self.spawn_loot(lx, ly, GEM, 1); } }
            _ => {}
        }
    }
}

// ---------------------------------------------------------------- ทัพของเรา + ศึกยึดเมือง
#[derive(Clone, Debug)]
pub struct Assault { pub ci: usize, pub k: usize, pub total: u32, pub spawned: u32, pub t: f32, pub champ: u8 }
#[derive(Clone, Debug, Default)]
pub struct WarState {
    pub march: bool,
    pub target: Option<usize>,
    pub army: Vec<u32>,
    pub hp: HashMap<u32, f32>,
    pub hit_cd: HashMap<u32, f32>,
    /// id → วันที่บาดเจ็บ (หายเช้าวันถัดไป)
    pub wounded: HashMap<u32, i32>,
    pub assault: Option<Assault>,
    pub captured: u32,
    pub champs: u32,
}
pub const CHAMP_R: f32 = 36.;
pub fn soldier_hp(tier: u8) -> f32 { 12. + tier as f32 * 3. }

impl Game {
    pub fn army_candidates(&self) -> Vec<u32> {
        self.residents().filter(|n| matches!(n.role, 2 | R_SOLDIER | R_GENERAL) && n.mood >= 20 && Game::willing(n) && !n.child && !self.is_sick(n.id) && !self.rs.war.wounded.contains_key(&n.id)).map(|n| n.id).collect()
    }
    pub fn army_pos(&self) -> Vec<(f32, f32)> {
        if !self.rs.war.march { return vec![]; }
        self.rs.war.army.iter().filter_map(|id| self.idx(*id)).map(|i| (self.npcs[i].x, self.npcs[i].y)).collect()
    }
    /// ยกทัพ: ทหารทุกคนที่พร้อมรบตามเราไป (เป้าหมาย = เมืองศัตรู)
    pub fn army_call(&mut self, target: Option<usize>) {
        let ids = self.army_candidates();
        let hp = soldier_hp(self.tier());
        let w = &mut self.rs.war;
        w.march = true;
        w.target = target;
        for id in &ids { w.hp.entry(*id).or_insert(hp); }
        w.army = ids.clone();
        // ทหารมาหาเราทันที (เดินมาจากเมือง = วาร์ปมาอยู่ข้างหลัง)
        for id in &ids { if let Some(i) = self.idx(*id) { let n = &mut self.npcs[i]; n.x = self.p.x + gen_range(-20., 20.); n.y = self.p.y + 16. + gen_range(0., 10.); self.rs.sleeping.remove(id); } }
        let to = target.map(|ci| format!(" → เมือง{} ({} ช่อง ทาง{})", self.w.realm.cities[ci].name, self.dist_to(self.w.realm.cities[ci].pos), self.dir_word(self.w.realm.cities[ci].pos))).unwrap_or_default();
        if ids.is_empty() { self.toast(format!("ยกทัพ{} — ยังไม่มีทหารพร้อมรบ ไปคนเดียวก่อน (ตั้งชาวบ้านใจกล้าเป็นทหารในแท็บชุมชน)", to)); }
        else { self.toast(format!("ยกทัพ! ทหาร {} นายตามคุณไป{} — เข้าเมืองศัตรูเพื่อเริ่มศึกยึดเมือง", ids.len(), to)); }
        play(&self.sfx.quest);
    }
    pub fn army_home(&mut self, why: &str) {
        if !self.rs.war.march { return; }
        self.rs.war.march = false;
        self.rs.war.target = None;
        let ids = std::mem::take(&mut self.rs.war.army);
        for id in ids { if let Some(i) = self.idx(id) { let b = self.npcs[i].bed; self.npcs[i].x = b.0 as f32 * 16. + 8.; self.npcs[i].y = b.1 as f32 * 16. + 18.; } }
        if !why.is_empty() { self.toast(why.to_string()); }
    }
    fn wound(&mut self, id: u32) {
        let day = day_of(self);
        self.rs.war.wounded.insert(id, day);
        self.rs.war.army.retain(|&a| a != id);
        self.rs.war.hp.remove(&id);
        if let Some(i) = self.idx(id) {
            let (x, y) = (self.npcs[i].x, self.npcs[i].y);
            self.burst(x, y - 10., hx(0xffffff), 8, 40.);
            let nm = NPC_NAMES[self.npcs[i].name as usize];
            let b = self.npcs[i].bed;
            self.npcs[i].x = b.0 as f32 * 16. + 8.; self.npcs[i].y = b.1 as f32 * 16. + 18.;
            self.npcs[i].mood = (self.npcs[i].mood - 5).max(0);
            self.toast(format!("{}บาดเจ็บ! ถอยกลับไปพักฟื้นที่บ้าน (หายพรุ่งนี้เช้า)", nm));
        }
    }

    pub fn war_tick(&mut self, dt: f32) {
        if self.rs.war.march {
            // ทหารที่ไม่อยู่แล้ว/ป่วย ออกจากทัพ
            let ok: Vec<u32> = self.rs.war.army.iter().copied().filter(|id| self.idx(*id).map_or(false, |i| self.npcs[i].state == ST_HOME) && !self.is_sick(*id)).collect();
            self.rs.war.army = ok;
            // ศัตรูที่ประชิดทหารของเรา = ทหารเสียเลือด
            let ids = self.rs.war.army.clone();
            let mut hurt: Vec<u32> = vec![];
            for id in ids {
                let Some(i) = self.idx(id) else { continue };
                let (nx, ny) = (self.npcs[i].x, self.npcs[i].y);
                let cd = self.rs.war.hit_cd.entry(id).or_insert(0.);
                *cd -= dt;
                if *cd > 0. { continue; }
                let hit = self.en.iter().filter(|e| !e.dead && e.kb <= 0. && e.stun <= 0.).find(|e| (e.x - nx).hypot(e.y - ny) < 12.).map(|e| e.dmg)
                    .or_else(|| self.bosses.iter().find(|b| !b.dead && (b.x - nx).hypot(b.y - ny) < BOSS_R + 8.).map(|_| 3));
                if let Some(dmg) = hit {
                    *self.rs.war.hit_cd.get_mut(&id).unwrap() = 0.9;
                    let hp = self.rs.war.hp.entry(id).or_insert(10.);
                    *hp -= dmg as f32;
                    let dead = *hp <= 0.;
                    self.add_float(nx, ny - 22., format!("-{}", dmg), hx(0xff8a8a), None);
                    if dead { hurt.push(id); }
                }
            }
            for id in hurt { self.wound(id); }
        }
        self.assault_tick(dt);
    }
    /// เข้าเมืองศัตรูที่กำลังรบกับเรา = เริ่มศึกยึดเมือง
    pub fn assault_begin(&mut self, ci: usize) -> bool {
        if peaceful() || self.rs.war.assault.is_some() || self.rs.siege.is_some() || self.w.dun.is_some() { return false; }
        let o = self.city_owner(ci);
        if o == PLAYER_OWN { return false; }
        let k = o as usize;
        let c = self.w.realm.cities[ci].clone();
        let st = self.rs.ks[k].strength;
        let total = ((5. + st / 14.) as u32).min(14) + if c.cap { 3 } else { 0 };
        let cap = c.cap && self.w.realm.ks[k].cap == ci;
        self.rs.war.assault = Some(Assault { ci, k, total, spawned: 0, t: 0., champ: if cap { 1 } else { 0 } });
        let army = if self.rs.war.march { self.rs.war.army.len() } else { 0 };
        self.toast(format!("ศึกยึดเมือง{}! ทหาร{} {} นายออกมาตั้งรับ{}{}", c.name, self.kname(k), total, if cap { " · แม่ทัพผู้พิทักษ์เฝ้าปราสาท" } else { "" }, if army > 0 { format!(" · ทัพเรา {} นาย", army) } else { " · ไปคนเดียว (ยกทัพได้ที่แท็บอาณาจักร)".to_string() }));
        self.news_add(format!("{}บุกเมือง{}ของ{}", self.my_name(), c.name, self.kname(k)));
        play(&self.sfx.roar);
        self.shake(1.5, 0.3);
        true
    }
    fn assault_tick(&mut self, dt: f32) {
        let Some(mut a) = self.rs.war.assault.take() else { return };
        a.t += dt;
        let c = self.w.realm.cities[a.ci].clone();
        let (cx, cy) = (c.pos.0 as f32 * 16. + 8., c.pos.1 as f32 * 16. + 8.);
        // ออกจากเมืองไกล (หนี/ตาย/กลับบ้าน) = การบุกล้มเหลว
        let far = (self.p.x - cx).abs().max((self.p.y - cy).abs()) > (c.r + 14) as f32 * 16.;
        if far || self.w.dun.is_some() || self.city_owner(a.ci) != a.k as u8 {
            self.en.retain(|e| !(e.k == EK::Soldier && e.pal == a.k));
            self.bosses.retain(|b| b.kind != CHAMP);
            if self.city_owner(a.ci) == a.k as u8 { self.toast(format!("ถอยออกจากเมือง{} — การบุกยังไม่สำเร็จ", c.name)); }
            return;
        }
        let alive = self.en.iter().filter(|e| !e.dead && e.k == EK::Soldier && e.pal == a.k).count() as u32;
        if a.spawned < a.total && alive < 6 && a.t >= a.spawned as f32 * 1.3 {
            // ทหารตั้งรับออกมาจากรอบปราสาท
            let ang = a.spawned as f32 * 2.4;
            let (x, y) = (cx + ang.cos() * 4. * 16., cy + ang.sin() * 3. * 16.);
            let spot = (0..40).map(|i| { let q = i as f32 * 2.4; let r = (i as f32).sqrt() * 12.; (x + q.cos() * r, y + q.sin() * r) }).find(|&(x, y)| !collide(&self.w, x, y) && !self.w.solid_at(tfloor(x), tfloor(y - 3.)));
            if let Some((x, y)) = spot { self.make_enemy(EK::Soldier, x, y, 1. + self.rs.ks[a.k].strength / 120., a.k, false); }
            a.spawned += 1;
        }
        if a.spawned >= a.total && alive == 0 {
            match a.champ {
                1 => {
                    // ทหารหมดแล้ว แม่ทัพออกมาเอง
                    a.champ = 2;
                    let spot = (0..60).map(|i| { let q = i as f32 * 2.4; let r = (i as f32).sqrt() * 14.; (cx + q.cos() * r, cy + 20. + q.sin() * r) }).find(|&(x, y)| self.walkable(x, y)).unwrap_or((cx, cy + 24.));
                    self.spawn_champion(a.k, spot.0, spot.1);
                }
                2 => { if !self.bosses.iter().any(|b| b.kind == CHAMP && !b.dead) { a.champ = 3; } }
                _ => { self.capture_city(a.ci, a.k); return; }
            }
        }
        self.rs.war.assault = Some(a);
    }
    pub fn capture_city(&mut self, ci: usize, k: usize) {
        if self.rs.owner.len() <= ci { return; }
        self.rs.owner[ci] = PLAYER_OWN;
        self.rs.war.captured += 1;
        let c = self.w.realm.cities[ci].clone();
        self.rs.ks[k].strength *= 0.7;
        self.rs.cit.retain(|x| x.city != ci);
        let coins = if c.cap { 200 } else { 80 };
        self.p.inv[COIN] += coins;
        self.p.xp += if c.cap { 80 } else { 40 };
        self.check_lv();
        for n in self.npcs.iter_mut().filter(|n| n.state == ST_HOME) { n.mood = (n.mood + 8).min(100); }
        let left = (0..self.w.realm.cities.len()).filter(|&j| self.city_owner(j) == k as u8).count();
        let mut msg = format!("ยึดเมือง{}ได้แล้ว! เป็นเมืองของเรา (ส่งภาษีทุกวัน) +{} เหรียญ", c.name, coins);
        if left == 0 {
            self.rs.ks[k].alive = false;
            for x in 0..self.rs.ks.len() { self.rs.ks[x].war[k] = false; self.rs.ks[k].war[x] = false; self.rs.ks[x].ally[k] = false; }
            msg += &format!(" · {}ล่มสลาย!", self.kname(k));
            self.news_add(format!("{}ล่มสลาย! {}ยึดเมืองสุดท้ายได้", self.kname(k), self.my_name()));
        }
        self.news_add(format!("{}ยึดเมือง{}จาก{}", self.my_name(), c.name, self.kname(k)));
        self.toast(msg);
        play(&self.sfx.lvl);
        self.shake(2., 0.4);
        for _ in 0..4 { self.burst(self.p.x, self.p.y - 12., hx(MY_COLOR), 14, 90.); }
        self.army_home("");
        self.save();
    }
    pub fn spawn_champion(&mut self, k: usize, x: f32, y: f32) {
        let hp = (BHP[CHAMP as usize] + self.rs.ks[k].strength * 1.2 + self.p.lvl as f32 * 8.).ceil();
        self.spawn_boss_hp(CHAMP, x, y, hp);
        if let Some(b) = self.bosses.last_mut() { b.k = k; b.cd = 1.2; }
        self.toast(format!("แม่ทัพแห่ง{}ออกมาเผชิญหน้า! ล้มแม่ทัพเพื่อยึดเมืองหลวง", self.kname(k)));
        play(&self.sfx.roar);
        self.shake(2.2, 0.4);
    }
    pub fn champion_down(&mut self, b: &Boss) {
        let (x, y) = (b.x, b.y - 12.);
        self.burst(x, y, hx(self.w.realm.ks[b.k].color), 40, 150.);
        self.burst(x, y, hx(0xffd84a), 20, 110.);
        self.gain(COIN, rnd(80, 120), x, y - 8.);
        self.gain(GEM, rnd(3, 5), x, y - 18.);
        self.p.xp += 60;
        self.p.kills += 1;
        self.rs.war.champs += 1;
        self.check_lv();
        self.shake(3.5, 0.5);
        self.toast(format!("ล้มแม่ทัพแห่ง{}ได้แล้ว!", self.kname(b.k)));
        play(&self.sfx.lvl);
    }
    /// แม่ทัพ: เดินเข้าหา · ฟันกวาดรอบตัว · พุ่งแทง · เรียกองครักษ์ (เฟส 2 ถี่ขึ้น)
    pub fn champ_ai(&mut self, b: &mut Boss, dt: f32, dx: f32, dy: f32, d: f32) {
        if b.slow > 0. { b.slow -= dt; }
        if b.burn > 0. { b.burn -= dt; b.hp = (b.hp - dt * 3.).max(1.); }
        let fast = (if b.ph2 { 1.3 } else { 1. }) * if b.slow > 0. { 0.55 } else { 1. };
        if b.dash > 0. {
            b.dash -= dt;
            let (nx, ny) = (b.x + b.dvx * dt, b.y + b.dvy * dt);
            if self.walkable(nx, ny) { b.x = nx; b.y = ny; } else { b.dash = 0.; }
            if self.p.invul <= 0. && d < BOSS_R + 6. { self.hurt(4, dx / d, dy / d); }
            if b.dash <= 0. { b.cd = 1.2; self.burst(b.x, b.y - 4., hx(0xc8c8d0), 10, 70.); }
            return;
        }
        if b.atk == 0 {
            b.cd -= dt;
            if d > BOSS_R + 4. {
                let sp = 30. * fast;
                let (nx, ny) = (b.x + dx / d * sp * dt, b.y + dy / d * sp * dt);
                if self.walkable(nx, ny) { b.x = nx; b.y = ny; }
            }
            if b.cd <= 0. && d < 260. {
                let r = gen_range(0., 1.);
                let guards = self.en.iter().filter(|e| !e.dead && e.k == EK::Soldier && e.pal == b.k).count();
                if d < 50. || r < 0.4 { b.atk = 1; b.st0 = if b.ph2 { 0.6 } else { 0.8 }; }
                else if r < 0.78 || guards >= 3 { b.atk = 2; b.st0 = 0.7; b.dvx = dx / d; b.dvy = dy / d; }
                else { b.atk = 3; b.st0 = 0.9; }
                b.st = b.st0;
            }
        } else {
            b.st -= dt;
            if b.st <= 0. {
                match b.atk {
                    1 => {
                        self.aoe(b.x, b.y, CHAMP_R, if b.ph2 { 4 } else { 3 });
                        for j in 0..14 { let q = j as f32 / 14. * std::f32::consts::TAU; self.parts.push(Particle { x: b.x + q.cos() * CHAMP_R, y: b.y + q.sin() * CHAMP_R * 0.7, vx: q.cos() * 30., vy: q.sin() * 20., l: 0.35, c: hx(0xffffff) }); }
                        play(&self.sfx.battk);
                        self.shake(1.4, 0.15);
                    }
                    2 => { b.dash = 0.5; b.dvx *= 230.; b.dvy *= 230.; play(&self.sfx.dash); }
                    _ => {
                        for j in 0..(if b.ph2 { 3 } else { 2 }) {
                            let q = j as f32 * 2.1 + b.anim;
                            let (mx, my) = (b.x + q.cos() * 30., b.y + q.sin() * 22.);
                            if self.walkable(mx, my) { self.make_enemy(EK::Soldier, mx, my, 1.1, b.k, false); }
                        }
                        self.toast("แม่ทัพเรียกองครักษ์!");
                        play(&self.sfx.roar);
                    }
                }
                b.atk = 0;
                b.cd = if b.ph2 { 0.9 } else { 1.5 } + gen_range(0., 0.7);
            }
        }
        if b.dash <= 0. && d < BOSS_R + 3. && self.p.invul <= 0. { self.hurt(2, dx / d, dy / d); }
    }

    pub fn war_dawn(&mut self) {
        let day = day_of(self);
        let healed: Vec<u32> = self.rs.war.wounded.iter().filter(|(_, &d)| day > d).map(|(id, _)| *id).collect();
        for id in healed {
            self.rs.war.wounded.remove(&id);
            if let Some(i) = self.idx(id) { self.news_add(format!("{}หายบาดเจ็บ กลับมาประจำการ", NPC_NAMES[self.npcs[i].name as usize])); }
        }
        let hp = soldier_hp(self.tier());
        for v in self.rs.war.hp.values_mut() { *v = hp; }
    }
    pub fn war_save(&self) -> String {
        let w = &self.rs.war;
        let mut s = format!("war {} {}", w.captured, w.champs);
        for (id, d) in &w.wounded { s += &format!(" w{},{}", id, d); }
        s + "\n"
    }
    pub fn war_load(&mut self, r: &[&str]) {
        let f = |i: usize| r.get(i).and_then(|x| x.parse::<u32>().ok()).unwrap_or(0);
        self.rs.war.captured = f(0);
        self.rs.war.champs = f(1);
        for x in r.iter().filter(|x| x.starts_with('w')) {
            let v: Vec<i64> = x[1..].split(',').filter_map(|a| a.parse().ok()).collect();
            if v.len() == 2 { self.rs.war.wounded.insert(v[0] as u32, v[1] as i32); }
        }
    }
    /// เมืองที่ใกล้ที่สุดของอาณาจักร k (เป้ายกทัพ)
    pub fn nearest_city_of(&self, k: usize) -> Option<usize> {
        (0..self.w.realm.cities.len()).filter(|&ci| self.city_owner(ci) == k as u8).min_by_key(|&ci| self.dist_to(self.w.realm.cities[ci].pos))
    }
}

pub fn boss_name(g: &Game, b: &Boss) -> String {
    if b.kind == CHAMP && b.k < g.w.realm.ks.len() { format!("แม่ทัพแห่ง{}", g.w.realm.ks[b.k].name) } else { BNAME[b.kind as usize].to_string() }
}

/// แม่ทัพ: อัศวินตัวใหญ่ ผ้าคลุมสีอาณาจักร หมวกขนนก ดาบยาว
pub fn draw_champion(g: &Game, sp: &Sprites, v: &View, b: &Boss) {
    let (sx, sy) = ((b.x - v.cx).round(), (b.y - v.cy).round());
    let kc = hx(g.w.realm.ks.get(b.k).map_or(0xd04a4a, |k| k.color));
    let left = g.p.x < b.x;
    let sc = 1.7;
    let moving = b.atk == 0 && b.dash <= 0.;
    let (fr, wb) = if moving { vfx::walk_cycle(b.anim, 7.) } else { (0, 0.) };
    vfx::char_shadow(v, sx, sy, 20.);
    // ออร่าเฟส 2
    if b.ph2 { draw_circle(sx * v.s, (sy - 14.) * v.s, 18. * v.s, Color::new(kc.r, kc.g, kc.b, 0.18 + (b.anim * 5.).sin().abs() * 0.1)); }
    // ผ้าคลุมด้านหลัง
    let fl = (b.anim * 3.).sin() * 1.5;
    prect(v, sx - 7. + if left { 2. } else { -2. } + fl * 0.3, sy - 22. - wb, 14., 18., Color::new(kc.r * 0.75, kc.g * 0.75, kc.b * 0.75, 1.));
    let body = Body::generate(7000 + b.k as u32 * 13, false);
    let tint = if b.flash > 0. { Color::new(1.8, 1.4, 1.4, 1.) } else if b.atk != 0 && (b.st * 12.) as i32 % 2 == 0 { Color::new(1.3, 1.1, 1.1, 1.) } else { WHITE };
    skin::draw_figure(sp, &body, skin::ROLE_PARTS[2], (sx - 6. * sc) * v.s, (sy - 15. * sc - wb) * v.s, v.s * sc, 0, fr, left, tint);
    // แถบสีอาณาจักร + ขนนกบนหมวก
    prect(v, sx - 5., sy - 15. - wb, 10., 2., kc);
    prect(v, sx - 1., sy - 30. - wb, 3., 4., kc);
    prect(v, sx, sy - 33. - wb, 2., 3., Color::new((kc.r + 0.3).min(1.), (kc.g + 0.3).min(1.), (kc.b + 0.3).min(1.), 1.));
    // ดาบยาว: ยกขึ้นตอนเตรียมฟัน · ชี้ไปข้างหน้าตอนพุ่ง
    let f = if left { -1. } else { 1. };
    let (hx0, hy0) = (sx + f * 9., sy - 14. - wb);
    let ang = match b.atk { 1 => -1.9 + (1. - b.st / b.st0.max(0.01)) * -1.2, 2 => -0.1, _ => if b.dash > 0. { 0. } else { -0.9 } };
    let (ca, sa) = (ang.cos(), ang.sin());
    for j in 0..14 {
        let (px, py) = (hx0 + f * ca * j as f32, hy0 + sa * j as f32);
        prect(v, px, py, 2., 2., if j < 3 { hx(0x6a4a2a) } else { hx(0xe0e4ee) });
    }
    prect(v, hx0 - 2. + f * ca * 3., hy0 + sa * 3. - 1., 4., 2., hx(0xd8b040));
}

/// แถบสถานะทัพ / ศึกยึดเมือง
pub fn draw_war_hud(g: &Game, font: &Font, u: f32) {
    let w = &g.rs.war;
    let mut lines: Vec<(String, Color)> = vec![];
    if let Some(a) = &w.assault {
        let c = &g.w.realm.cities[a.ci];
        let alive = g.en.iter().filter(|e| !e.dead && e.k == EK::Soldier && e.pal == a.k).count() as u32;
        let left = a.total - a.spawned + alive;
        let s = match a.champ { 2 => format!("ศึกยึดเมือง{} — ล้มแม่ทัพ!", c.name), _ if left > 0 => format!("ศึกยึดเมือง{} — ทหารตั้งรับเหลือ {} นาย", c.name, left), _ => format!("ศึกยึดเมือง{} — กำลังยึด...", c.name) };
        lines.push((s, hx(0xff9a8a)));
    }
    if w.march {
        let mut s = format!("ทัพของเรา {} นาย", w.army.len());
        if w.assault.is_none() { if let Some(ci) = w.target { let c = &g.w.realm.cities[ci]; s += &format!(" · เป้าหมาย: เมือง{} {} ช่อง ทาง{}", c.name, g.dist_to(c.pos), g.dir_word(c.pos)); } }
        lines.push((s, hx(0x9adcff)));
    }
    if lines.is_empty() { return; }
    let x = 14. * u;
    let mut y = screen_height() - 110. * u - 30. * u * (lines.len() as f32 - 1.);
    for (s, c) in lines {
        let fs = 13. * u;
        let dm = measure_text(&s, Some(font), fs as u16, 1.);
        draw_rectangle(x - 4. * u, y - fs, dm.width + 10. * u, fs + 8. * u, Color::new(0., 0., 0., 0.55));
        draw_text_ex(&s, x, y, TextParams { font: Some(font), font_size: fs as u16, color: c, ..Default::default() });
        y += fs + 12. * u;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn soldier_town(g: &mut Game, n: u32) -> Vec<u32> {
        let (sx, sy) = (tfloor(g.p.x) + 3, tfloor(g.p.y) + 3);
        let mut ids = vec![];
        for i in 0..n as i32 {
            let b = (sx + i * 2, sy);
            g.w.placed.insert(b, 33);
            let id = g.npc_next; g.npc_next += 1;
            let mut soc = Soc::generate(id, i as u8);
            soc.tr[T_BRAVE] = 80;
            g.npcs.push(Npc { id, name: i as u8, look: 0, x: b.0 as f32 * 16. + 8., y: b.1 as f32 * 16. + 18., state: ST_HOME, hearts: 3, mood: 70, married: false, child: false, role: R_SOLDIER, bed: b, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc });
            g.rs.manual.insert(id, R_SOLDIER);
            ids.push(id);
        }
        ids
    }
    #[test]
    fn enemies_leave_a_death_animation_then_vanish() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        g.make_enemy(EK::Wolf, g.p.x + 30., g.p.y, 1., 0, false);
        let n = g.en.len();
        assert!(n >= 1);
        g.damage_enemy(n - 1, 999, false, 1., 0.);
        assert_eq!(g.dying.len(), 1, "ท่าตายเริ่ม");
        assert_eq!(g.dying[0].k, EK::Wolf);
        for _ in 0..40 { g.upd_enemies(1. / 30.); }
        assert!(g.dying.is_empty(), "ท่าตายจบใน {} วินาที", DIE_T);
    }
    #[test]
    fn biome_beasts_have_their_own_moves() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        for k in [EK::Scorp, EK::Wolf, EK::Crab, EK::Imp] {
            g.en.clear(); g.proj.clear();
            g.make_enemy(k, g.p.x + 30., g.p.y, 1., 0, false);
            let mut e = g.en[0].clone();
            e.sk = 0.;
            let (dx, dy) = (g.p.x - e.x, g.p.y - e.y);
            let d = dx.hypot(dy);
            let mv = g.beast_ai(&mut e, dx, dy, d, 0.016);
            assert!(mv.is_some(), "{}", k as u8);
            match k {
                EK::Scorp | EK::Wolf => assert!(e.kb > 0. && e.vx < -100., "พุ่งเข้าหาเรา (อยู่ทางซ้าย)"),
                EK::Imp => assert!(!g.proj.is_empty() || (e.x - (g.p.x + 30.)).abs() > 1., "ยิงลูกไฟหรือวาร์ป"),
                _ => { let (mx, my) = mv.unwrap(); assert!(mx.abs() > my.abs(), "ปูเดินข้าง"); }
            }
            // ไกลเกินไป = ใช้พฤติกรรมเดินเล่นเดิม
            assert!(g.beast_ai(&mut e, 400., 0., 400., 0.016).is_none());
        }
        // ศัตรูเดิมไม่เปลี่ยน
        g.make_enemy(EK::Slime, g.p.x + 30., g.p.y, 1., 0, false);
        let mut s = g.en.last().unwrap().clone();
        assert!(g.beast_ai(&mut s, 30., 0., 30., 0.016).is_none());
    }
    #[test]
    fn army_follows_and_assault_captures_side_city() {
        let mut g = Game::new_world_seed(746397394, Sfx::default());
        let r = g.w.realm.clone(); g.rs.init(&r);
        let ids = soldier_town(&mut g, 3);
        let ci = (0..r.cities.len()).find(|&c| !r.cities[c].cap).unwrap();
        let k = g.city_owner(ci) as usize;
        g.rs.ks[k].stance = 3;
        g.army_call(Some(ci));
        assert_eq!(g.rs.war.army.len(), 3);
        // ทหารเดินตามเรา
        g.p.x += 200.;
        for _ in 0..90 { g.npc_tick(1. / 30.); }
        for id in &ids { let i = g.idx(*id).unwrap(); assert!((g.npcs[i].x - g.p.x).hypot(g.npcs[i].y - g.p.y) < 120., "ตามมา"); }
        // ไปถึงเมือง → ศึก
        let c = r.cities[ci].clone();
        g.p.x = c.pos.0 as f32 * 16. + 8.; g.p.y = (c.pos.1 + 2) as f32 * 16. + 8.;
        assert!(g.assault_begin(ci));
        let total = g.rs.war.assault.as_ref().unwrap().total;
        assert!(total >= 5);
        let mut spawned = 0;
        for _ in 0..(60 * 30) {
            g.war_tick(1. / 30.);
            spawned = spawned.max(g.en.iter().filter(|e| e.k == EK::Soldier && e.pal == k).count());
            for e in g.en.iter_mut() { if e.k == EK::Soldier && e.pal == k { e.dead = true; } }
            g.en.retain(|e| !e.dead);
            if g.rs.war.assault.is_none() { break; }
        }
        assert!(spawned >= 1, "มีทหารตั้งรับ");
        assert_eq!(g.city_owner(ci), PLAYER_OWN, "ยึดเมืองได้");
        assert!(!g.rs.war.march, "ทัพกลับบ้าน");
        assert_eq!(g.rs.war.captured, 1);
    }
    #[test]
    fn capital_needs_champion_and_last_city_ends_kingdom() {
        let mut g = Game::new_world_seed(746397394, Sfx::default());
        let r = g.w.realm.clone(); g.rs.init(&r);
        let k = 0;
        let cap = r.ks[k].cap;
        // เมืองลูกเป็นของเราแล้ว เหลือเมืองหลวง
        for ci in 0..r.cities.len() { if g.city_owner(ci) == k as u8 && ci != cap { g.rs.owner[ci] = PLAYER_OWN; } }
        g.rs.ks[k].stance = 3;
        let c = r.cities[cap].clone();
        g.p.x = c.pos.0 as f32 * 16. + 8.; g.p.y = (c.pos.1 + 3) as f32 * 16. + 8.;
        assert!(g.assault_begin(cap));
        let mut champ_seen = false;
        for _ in 0..(90 * 30) {
            g.war_tick(1. / 30.);
            for e in g.en.iter_mut() { if e.k == EK::Soldier && e.pal == k { e.dead = true; } }
            g.en.retain(|e| !e.dead);
            if let Some(i) = g.bosses.iter().position(|b| b.kind == CHAMP) {
                champ_seen = true;
                assert_eq!(g.bosses[i].k, k);
                assert!(g.bosses[i].maxhp >= BHP[CHAMP as usize]);
                assert_eq!(g.city_owner(cap), k as u8, "แม่ทัพยังอยู่ = ยังยึดไม่ได้");
                let b = g.bosses.remove(i);
                g.boss_die(&b);
            }
            if g.rs.war.assault.is_none() { break; }
        }
        assert!(champ_seen, "เมืองหลวงมีแม่ทัพ");
        assert_eq!(g.city_owner(cap), PLAYER_OWN);
        assert!(!g.rs.ks[k].alive, "เมืองสุดท้าย = อาณาจักรล่มสลาย");
        assert!(g.unified(k), "นับเป็นรวมแผ่นดิน (ยึด)");
        assert_eq!(g.rs.war.champs, 1);
    }
    #[test]
    fn soldiers_get_wounded_not_killed_and_recover_next_day() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let r = g.w.realm.clone(); g.rs.init(&r);
        let ids = soldier_town(&mut g, 2);
        g.army_call(None);
        let id = ids[0];
        let i = g.idx(id).unwrap();
        g.npcs[i].x = g.p.x + 60.; g.npcs[i].y = g.p.y;
        let (x, y) = (g.npcs[i].x + 4., g.npcs[i].y);
        g.make_enemy(EK::Soldier, x, y, 3., 99, false);
        for _ in 0..(30 * 30) { if let Some(e) = g.en.first_mut() { e.x = x; e.y = y; } g.war_tick(1. / 30.); if g.rs.war.wounded.contains_key(&id) { break; } }
        assert!(g.rs.war.wounded.contains_key(&id), "บาดเจ็บ");
        assert!(g.idx(id).is_some(), "ไม่ตาย");
        assert!(!g.rs.war.army.contains(&id));
        assert!(!g.army_candidates().contains(&id), "บาดเจ็บออกรบไม่ได้");
        let mut n = g.npcs[g.idx(id).unwrap()].clone();
        let mut done = vec![];
        assert!(g.work_step(&mut n, 0.1, &mut done).is_none(), "พักไม่ทำงาน");
        // เซฟ/โหลด
        let s = g.war_save();
        let mut h = Game::new_world_seed(4242, Sfx::default());
        let parts: Vec<&str> = s.trim().split(' ').skip(1).collect();
        h.war_load(&parts);
        assert!(h.rs.war.wounded.contains_key(&id));
        g.time += DAYLEN;
        g.war_dawn();
        assert!(!g.rs.war.wounded.contains_key(&id), "หายเช้าวันถัดไป");
    }
}
