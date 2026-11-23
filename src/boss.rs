//! บอสประจำไบโอม: ราชาสไลม์ (ป่าทึบ) · โกเลมหินยักษ์ (เขตภูเขา/หินผา)
//! แต่ละตัวมีสกิลเฉพาะตัวพร้อมเส้นเตือน (telegraph) และเฟส 2 เมื่อเลือดต่ำกว่าครึ่ง

use super::*;

pub const BOSS_R: f32 = 15.;
pub const BNAME: [&str; 4] = ["ราชาสไลม์", "โกเลมหินยักษ์", "ราชันย์เงา", "แม่ทัพผู้พิทักษ์"];
pub const BHP: [f32; 4] = [70., 110., 900., 200.];
/// v24 แม่ทัพประจำเมืองหลวง (เฝ้าปราสาทตอนเราบุก)
pub const CHAMP: u8 = 3;
const DIR8: [(i32, i32); 8] = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1), (1, -1), (-1, 1)];

#[derive(Clone)]
pub struct Boss {
    pub kind: u8,
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    pub maxhp: f32,
    /// สกิลที่กำลังเตรียม: 0 = ไม่มี
    pub atk: u8,
    pub st: f32,
    pub st0: f32,
    pub cd: f32,
    pub tx: f32,
    pub ty: f32,
    pub dash: f32,
    pub dvx: f32,
    pub dvy: f32,
    pub flash: f32,
    pub anim: f32,
    pub ph2: bool,
    pub dead: bool,
    pub burn: f32,
    pub slow: f32,
    pub btick: f32,
    /// อาณาจักรของแม่ทัพ (CHAMP)
    pub k: usize,
}

impl Game {
    pub fn spawn_boss(&mut self, kind: u8, x: f32, y: f32) {
        let mul = 1. + (x - self.spawn.0).hypot(y - self.spawn.1) / 16. / 400.;
        let hp = (BHP[kind as usize] * mul).ceil();
        self.spawn_boss_hp(kind, x, y, hp);
    }
    pub fn spawn_boss_hp(&mut self, kind: u8, x: f32, y: f32, hp: f32) {
        self.bosses.push(Boss {
            kind, x, y, hp, maxhp: hp, atk: 0, st: 0., st0: 1., cd: 1.5, tx: x, ty: y, dash: 0., dvx: 0., dvy: 0., flash: 0., anim: 0., ph2: false, dead: false, burn: 0., slow: 0., btick: 0., k: 0,
        });
        if kind == CHAMP { return; }
        self.toast(format!("{} ปรากฏตัว!", BNAME[kind as usize]));
        play(&self.sfx.roar);
        self.shake(2.0, 0.35);
    }

    fn deep_ok(&self, tx: i32, ty: i32, kind: u8) -> bool {
        let want = |t: u8| if kind == 0 { t == 4 } else { t == 5 || t == 6 };
        let rad = if kind == 0 { 5 } else { 3 };
        want(self.w.tile_at(tx, ty)) && DIR8.iter().all(|&(dx, dy)| want(self.w.tile_at(tx + dx * rad, ty + dy * rad)))
    }

    /// สุ่มเกิดบอสเมื่อผู้เล่นอยู่ลึกในป่าทึบ / เขตหินผา
    pub fn try_spawn_boss(&mut self) {
        if !self.bosses.is_empty() || self.boss_cd > 0. {
            return;
        }
        let (ptx, pty) = (tfloor(self.p.x), tfloor(self.p.y));
        let kind = match self.w.tile_at(ptx, pty) {
            4 => 0u8,
            5 | 6 => 1u8,
            _ => return,
        };
        if self.w.near_city(ptx, pty, 40) || !self.deep_ok(ptx, pty, kind) || gen_range(0., 1.) > 0.08 {
            return;
        }
        for _ in 0..14 {
            let a = gen_range(0., std::f32::consts::TAU);
            let r = gen_range(8., 14.) * 16.;
            let (x, y) = (self.p.x + a.cos() * r, self.p.y + a.sin() * r);
            let (tx, ty) = (tfloor(x), tfloor(y));
            if self.deep_ok(tx, ty, kind) && !self.w.solid_at(tx, ty) && self.w.camp_near(tx, ty, 10).is_none() {
                self.spawn_boss(kind, x, y);
                return;
            }
        }
    }

    fn minion(&mut self, x: f32, y: f32) {
        self.en.push(Enemy {
            path: Path::default(),
            k: EK::Slime, x, y, hp: 3., spd: 26., dmg: 1, xp: 1, coin: 0, vx: 0., vy: 0., kb: 0., flash: 0., wt: 0., wdx: 0., wdy: 0.,
            anim: gen_range(0., 5.), pal: 0, dead: false, night: false, burn: 0., slow: 0., stun: 0., btick: 0., sk: 0.,
        });
    }
    pub fn aoe(&mut self, x: f32, y: f32, r: f32, dmg: i32) {
        let (dx, dy) = (self.p.x - x, self.p.y - y);
        let d = dx.hypot(dy).max(0.001);
        if d < r && self.p.invul <= 0. {
            self.hurt(dmg, dx / d, dy / d);
        }
    }
    pub fn walkable(&self, x: f32, y: f32) -> bool {
        !SOLIDT[self.w.tile_at(tfloor(x), tfloor(y)) as usize]
    }

    pub fn upd_bosses(&mut self, dt: f32) {
        let mut bs = std::mem::take(&mut self.bosses);
        for b in bs.iter_mut() {
            if !b.dead {
                self.upd_boss(b, dt);
            }
        }
        bs.retain(|b| !b.dead);
        if self.died {
            bs.clear();
            self.proj.clear();
            self.boss_cd = self.boss_cd.max(90.);
        }
        let added = std::mem::take(&mut self.bosses);
        bs.extend(added);
        self.bosses = bs;
    }

    fn upd_boss(&mut self, b: &mut Boss, dt: f32) {
        b.anim += dt;
        if b.flash > 0. {
            b.flash -= dt;
        }
        let (dx, dy) = (self.p.x - b.x, self.p.y - b.y);
        let d = dx.hypot(dy).max(0.001);
        if d > if b.kind == FINAL_KIND { 3000. } else { 1100. } {
            b.dead = true;
            self.boss_cd = 90.;
            return;
        }
        if !b.ph2 && b.hp < b.maxhp * 0.5 {
            b.ph2 = true;
            self.toast(format!("{} โกรธจัด!", boss_name(self, b)));
        }
        if b.kind == CHAMP { self.champ_ai(b, dt, dx, dy, d); return; }
        let king = b.kind == 0;
        if b.slow > 0. { b.slow -= dt; }
        if b.burn > 0. {
            b.burn -= dt;
            b.hp = (b.hp - dt * 3.).max(1.);
            if gen_range(0., 1.) < dt * 14. { self.parts.push(Particle { x: b.x + gen_range(-8., 8.), y: b.y - gen_range(4., 20.), vx: 0., vy: -24., l: 0.4, c: hx(0xff8a20) }); }
        }
        let fast = (if b.ph2 { 1.35 } else { 1.0 }) * if b.slow > 0. { 0.55 } else { 1. };
        // โกเลมพุ่งชน
        if b.dash > 0. {
            b.dash -= dt;
            let (nx, ny) = (b.x + b.dvx * dt, b.y + b.dvy * dt);
            if self.walkable(nx, ny) {
                b.x = nx;
                b.y = ny;
            } else {
                b.dash = 0.;
            }
            if self.p.invul <= 0. && d < BOSS_R + 6. {
                self.hurt(4, dx / d, dy / d);
            }
            if b.dash <= 0. {
                b.cd = 1.4;
                self.burst(b.x, b.y - 4., hx(0x9a9aa6), 10, 70.);
            }
            return;
        }
        if b.atk == 0 {
            b.cd -= dt;
            let spd = if king { if (b.anim * 1.3).fract() < 0.55 { 38. } else { 0. } } else { 22. } * fast;
            if d > BOSS_R + 6. && spd > 0. {
                let (nx, ny) = (b.x + dx / d * spd * dt, b.y + dy / d * spd * dt);
                if self.walkable(nx, ny) {
                    b.x = nx;
                    b.y = ny;
                }
            }
            if b.cd <= 0. && d < 300. {
                let r = gen_range(0., 1.);
                if king {
                    if r < 0.5 {
                        b.atk = 1; // กระโดดกระแทก
                        b.st0 = 0.95;
                        b.tx = self.p.x;
                        b.ty = self.p.y;
                    } else {
                        b.atk = 2; // พ่นเมือก
                        b.st0 = 0.6;
                    }
                } else if r < 0.35 {
                    b.atk = 1; // ขว้างก้อนหิน
                    b.st0 = 0.7;
                } else if r < 0.7 || d < 60. {
                    b.atk = 2; // กระทืบพื้น
                    b.st0 = 1.0;
                    b.tx = b.x;
                    b.ty = b.y;
                } else {
                    b.atk = 3; // พุ่งชน
                    b.st0 = 0.85;
                    b.dvx = dx / d;
                    b.dvy = dy / d;
                }
                b.st = b.st0;
            }
        } else {
            b.st -= dt;
            if b.st <= 0. {
                self.boss_exec(b, dx, dy, d);
            }
        }
        // ชนตัว (ราชาสไลม์ลอยอยู่กลางอากาศตอนกระโดด)
        let airborne = king && b.atk == 1 && b.st < 0.35;
        if !airborne && d < BOSS_R + 4. && self.p.invul <= 0. {
            self.hurt(if king { 2 } else { 3 }, dx / d, dy / d);
        }
    }

    fn boss_exec(&mut self, b: &mut Boss, dx: f32, dy: f32, d: f32) {
        let king = b.kind == 0;
        let base = dy.atan2(dx);
        match (king, b.atk) {
            (true, 1) => {
                if self.walkable(b.tx, b.ty) {
                    b.x = b.tx;
                    b.y = b.ty;
                }
                self.aoe(b.x, b.y, 30., if b.ph2 { 4 } else { 3 });
                self.burst(b.x, b.y - 4., hx(0x6cd06c), 26, 120.);
                play(&self.sfx.boom);
                for _ in 0..(if b.ph2 { 3 } else { 2 }) {
                    let (mx, my) = (b.x + gen_range(-20., 20.), b.y + gen_range(-14., 14.));
                    if self.walkable(mx, my) {
                        self.minion(mx, my);
                    }
                }
            }
            (true, _) => {
                let n = if b.ph2 { 5 } else { 3 };
                for i in 0..n {
                    let a = base + (i as f32 - (n as f32 - 1.) / 2.) * 0.3;
                    self.proj.push(Proj { x: b.x, y: b.y - 10., vx: a.cos() * 95., vy: a.sin() * 95., kind: 2, dmg: 2, life: 2.4, mine: false, dead: false });
                }
                play(&self.sfx.shoot);
            }
            (false, 1) => {
                let n = if b.ph2 { 2 } else { 1 };
                for i in 0..n {
                    let a = base + (i as f32 - (n as f32 - 1.) / 2.) * 0.4;
                    self.proj.push(Proj { x: b.x, y: b.y - 18., vx: a.cos() * 105., vy: a.sin() * 105., kind: 3, dmg: 3, life: 2.6, mine: false, dead: false });
                }
                play(&self.sfx.shoot);
            }
            (false, 2) => {
                self.aoe(b.x, b.y, 46., 4);
                let n = if b.ph2 { 12 } else { 8 };
                for i in 0..n {
                    let a = i as f32 / n as f32 * std::f32::consts::TAU;
                    self.proj.push(Proj { x: b.x, y: b.y - 6., vx: a.cos() * 85., vy: a.sin() * 85., kind: 5, dmg: 2, life: 1.3, mine: false, dead: false });
                }
                self.burst(b.x, b.y - 2., hx(0x9a9aa6), 24, 110.);
                play(&self.sfx.boom);
                // ราชันย์เงา: เรียกโครงกระดูก + วงกระสุนเงาชั้นที่สอง
                if b.kind == FINAL_KIND {
                    for k in 0..(if b.ph2 { 3 } else { 2 }) {
                        let a = k as f32 * 2.1 + b.anim;
                        let (mx, my) = (b.x + a.cos() * 30., b.y + a.sin() * 20.);
                        if self.walkable(mx, my) { self.make_enemy(EK::Skel, mx, my, 1.2, 0, false); }
                    }
                    for i in 0..16 {
                        let a = (i as f32 + 0.5) / 16. * std::f32::consts::TAU;
                        self.proj.push(Proj { x: b.x, y: b.y - 6., vx: a.cos() * 60., vy: a.sin() * 60., kind: 2, dmg: 3, life: 2.2, mine: false, dead: false });
                    }
                }
            }
            _ => {
                b.dash = 0.55;
                b.dvx *= 190.;
                b.dvy *= 190.;
            }
        }
        let _ = d;
        play(&self.sfx.battk);
        self.shake(1.6, 0.18);
        b.atk = 0;
        b.cd = if b.ph2 { 1.0 } else { 1.7 } + gen_range(0., 0.8);
    }

    pub fn hit_boss(&mut self, i: usize, dmg: i32) {
        let (d, crit) = self.roll_dmg(dmg);
        self.damage_boss(i, d, crit);
    }

    pub fn boss_die(&mut self, b: &Boss) {
        if b.kind == CHAMP { self.champion_down(b); return; }
        let (x, y) = (b.x, b.y - 12.);
        let col = match b.kind { 0 => hx(0x6cd06c), FINAL_KIND => hx(0x8a4ae8), _ => hx(0x9a9aa6) };
        if b.kind == FINAL_KIND {
            self.burst(x, y, col, 80, 200.);
            self.p.xp += 200;
            self.p.kills += 1;
            self.check_lv();
            self.boss_cd = 240.;
            self.shake(5., 1.);
            self.final_defeated(x, y);
            play(&self.sfx.lvl);
            return;
        }
        self.burst(x, y, col, 44, 150.);
        self.burst(x, y, hx(0xffd84a), 20, 110.);
        if b.kind == 0 {
            self.gain(GEL, rnd(3, 5), x, y - 8.);
            self.gain(COIN, rnd(40, 70), x, y - 18.);
            self.gain(GEM, rnd(1, 3), x, y - 28.);
        } else {
            self.gain(CORE, rnd(2, 4), x, y - 8.);
            self.gain(COIN, rnd(50, 90), x, y - 18.);
            self.gain(GEM, rnd(3, 5), x, y - 28.);
            self.gain(STONE, 8, x, y - 38.);
        }
        self.p.xp += 40;
        self.p.kills += 1;
        self.check_lv();
        self.boss_cd = 240.;
        self.shake(3.5, 0.5);
        if self.w.dun.is_some() {
            self.dungeon_boss_down();
        }
        self.toast(format!("ปราบ{}ได้แล้ว! ได้ของหายาก", BNAME[b.kind as usize]));
        play(&self.sfx.lvl);
        self.save();
    }
}

// ---------------- วาด ----------------
/// วงเตือนบนพื้น (วาดก่อนวัตถุ)
pub fn draw_telegraphs(g: &Game, v: &View) {
    for b in &g.bosses {
        if b.atk == 0 {
            continue;
        }
        let k = 1. - (b.st / b.st0).clamp(0., 1.);
        let red = Color::new(1., 0.15, 0.1, 0.16 + 0.3 * k);
        let edge = Color::new(1., 0.35, 0.2, 0.8);
        let ring = |x: f32, y: f32, r: f32| {
            let (sx, sy) = ((x - v.cx) * v.s, (y - v.cy) * v.s);
            draw_circle(sx, sy, r * v.s, red);
            draw_circle(sx, sy, r * k * v.s, Color::new(1., 0.2, 0.1, 0.35));
            draw_circle_lines(sx, sy, r * v.s, 2., edge);
        };
        match (b.kind, b.atk) {
            (0, 1) => ring(b.tx, b.ty, 30.),
            (CHAMP, 1) => ring(b.x, b.y, CHAMP_R),
            (CHAMP, 2) => {
                let (x0, y0) = ((b.x - v.cx) * v.s, (b.y - v.cy) * v.s);
                draw_line(x0, y0, x0 + b.dvx * 120. * v.s, y0 + b.dvy * 120. * v.s, 14. * v.s, red);
            }
            (1, 2) => ring(b.x, b.y, 46.),
            (1, 3) => {
                let (x0, y0) = ((b.x - v.cx) * v.s, (b.y - v.cy) * v.s);
                draw_line(x0, y0, x0 + b.dvx * 105. * v.s, y0 + b.dvy * 105. * v.s, 14. * v.s, red);
            }
            _ => {}
        }
    }
}

pub fn draw_boss(g: &Game, sp: &Sprites, v: &View, i: usize) {
    let b = &g.bosses[i];
    let (sx, sy) = ((b.x - v.cx).round(), (b.y - v.cy).round());
    let c = if b.flash > 0. { Color::new(1., 0.55, 0.55, 1.) } else { WHITE };
    if b.kind == CHAMP { war::draw_champion(g, sp, v, b); return; }
    if b.kind == 0 {
        let lift = if b.atk == 1 && b.st < 0.35 { (std::f32::consts::PI * (1. - b.st / 0.35)).sin() * 26. } else { 0. };
        let sq = if b.atk == 1 && b.st >= 0.35 { 0.2 } else if b.atk == 2 { 0.12 } else { (b.anim * 1.3 * std::f32::consts::TAU).sin() * 0.1 };
        let (ww, hh) = ((32. * (1. + sq)).round(), (26. * (1. - sq)).round());
        let sh = 1. - lift / 40.;
        prect(v, (sx - 12. * sh).round(), sy - 2., (24. * sh).round(), 4., Color::new(0., 0., 0., 0.3));
        blit_sz(sp, v, sp.ex.king, (sx - ww / 2.).floor(), sy - hh - lift, ww, hh, c);
    } else {
        let raised = b.atk == 1 || b.atk == 2;
        let jx = if b.atk == 2 { ((b.anim * 40.).sin() * 1.).round() } else { 0. };
        let bob = if b.atk == 0 { ((b.anim * 4.) as i32 % 2) as f32 } else { 0. };
        if b.kind == FINAL_KIND {
            // โกเลมใหญ่ขึ้น 1.6 เท่า ย้อมสีม่วงเงา + ออร่ามืดรอบตัว
            let r = sp.ex.golem[raised as usize];
            let (w, h) = (r.w * 1.6, r.h * 1.6);
            let pulse = 0.25 + (b.anim * 3.).sin().abs() * 0.15;
            prect(v, sx - 20., sy - 3., 40., 6., Color::new(0., 0., 0., 0.4));
            for k in 0..3 { let rr = 22. + k as f32 * 6.; draw_circle(sx * v.s - v.cx * 0., (sy - h / 2.) * v.s, rr * v.s, Color::new(0.35, 0.1, 0.6, pulse / (k as f32 + 1.5))); }
            let tint = if b.flash > 0. { Color::new(1., 0.6, 0.9, 1.) } else if b.ph2 { Color::new(0.75, 0.35, 0.95, 1.) } else { Color::new(0.55, 0.35, 0.85, 1.) };
            blit_sz(sp, v, r, (sx - w / 2. + jx).floor(), sy - h - bob, w, h, tint);
            return;
        }
        prect(v, sx - 13., sy - 2., 26., 4., Color::new(0., 0., 0., 0.3));
        blit(sp, v, sp.ex.golem[raised as usize], sx - 16. + jx, sy - 34. - bob, false, c);
    }
}

pub fn draw_boss_bar(g: &Game, font: &Font) {
    let Some(b) = g.bosses.iter().filter(|b| (b.x - g.p.x).hypot(b.y - g.p.y) < 420.).min_by(|a, c| {
        let da = (a.x - g.p.x).hypot(a.y - g.p.y);
        let dc = (c.x - g.p.x).hypot(c.y - g.p.y);
        da.partial_cmp(&dc).unwrap()
    }) else {
        return;
    };
    let u = ui_scale();
    let sw = screen_width();
    let (w, h) = ((sw * 0.4).min(380. * u), 18. * u);
    let (x, y) = (sw / 2. - w / 2., 16. * u);
    text_c(font, &boss_name(g, b), sw / 2., y + 4. * u, 17. * u, hx(0xffcc66));
    let y = y + 10. * u;
    draw_rectangle(x, y, w, h, hx(0x2a0e0e));
    draw_rectangle(x, y, w * (b.hp / b.maxhp).clamp(0., 1.), h, if b.ph2 { hx(0xff5a1a) } else { hx(0xcc2222) });
    draw_rectangle_lines(x, y, w, h, 2., BLACK);
    text_c(font, &format!("{}/{}", b.hp.max(0.) as i32, b.maxhp as i32), sw / 2., y + h - 2. * u, 11. * u, WHITE);
}
