//! ระบบ RPG + ความสะใจในการต่อสู้: สเตตัส/เลเวล · ตีบวก (Forge) · Dash · Auto-aim · ไอเทมดรอป · ผู้ติดตาม · เควสต์ · ฝน

use super::*;

// ======================= สเตตัส =======================
pub const STN: [&str; 4] = ["พลังชีวิต", "พลังโจมตี", "ความเร็วเดิน", "คริติคอล"];
pub const STD: [&str; 4] = ["เลือดสูงสุด +3 ต่อแต้ม", "ดาเมจทุกอาวุธ +12% ต่อแต้ม", "เดินเร็ว +3% ต่อแต้ม", "โอกาสคริติคอล +3% ต่อแต้ม (x1.8)"];
pub const ST_MAX: u8 = 10;
pub const SP_PER_LV: i32 = 2;

impl Player {
    pub fn crit_rate(&self) -> f32 {
        0.05 + 0.03 * self.st[3] as f32 + if self.buff_crit > 0. { 0.2 } else { 0. }
    }
    pub fn atk_mul(&self, weapon: u8) -> f32 {
        (1. + 0.12 * self.st[1] as f32) * (1. + 0.2 * self.wplus[weapon as usize] as f32) * if weapon == 0 { self.body.str_mul() } else { 1. }
    }
}

// ======================= โต๊ะตีเหล็ก =======================
pub const AURA: [u32; 6] = [0xffffff, 0x66ff77, 0x55aaff, 0xc468ff, 0xffa030, 0xff4060];
pub const PLUS_MAX: u8 = 5;

/// ต้นทุนตีบวกไปเป็นระดับ lv (1..=5)
pub fn forge_cost(lv: u8) -> &'static [(usize, i32)] {
    match lv {
        1 => &[(STONE, 6), (GEM, 1)],
        2 => &[(STONE, 10), (GEM, 2)],
        3 => &[(STONE, 14), (GEM, 3), (SHARD, 2)],
        4 => &[(GEM, 4), (SHARD, 4), (GEL, 3)],
        _ => &[(GEM, 6), (SHARD, 6), (GEL, 3), (CORE, 3)],
    }
}

impl Player {
    /// เป้าหมายตีบวก: 0 ดาบ · 1 ธนู · 2 คทา · 3 หมวก · 4 เกราะอก · 5 รองเท้า
    pub fn forge_owned(&self, t: usize) -> bool {
        match t {
            0 => true,
            1 => self.bow,
            2 => self.staff >= 1,
            _ => self.armor[t - 3] > 0,
        }
    }
    pub fn forge_plus(&self, t: usize) -> u8 {
        if t < 3 { self.wplus[t] } else { self.aplus[t - 3] }
    }
    pub fn forge_name(&self, t: usize) -> String {
        match t {
            0 => SWN[self.sword].to_string(),
            1 => "ธนู".to_string(),
            2 => STAFF_N[self.staff as usize].replace('-', "คทา"),
            _ => format!("{}{}", SLOT_N[t - 3], if self.armor[t - 3] > 0 { format!(" ({})", ATN[self.armor[t - 3] as usize]) } else { String::new() }),
        }
    }
}

impl Game {
    pub fn forge(&mut self, t: usize) {
        if !self.p.forge_owned(t) {
            return;
        }
        let cur = self.p.forge_plus(t);
        if cur >= PLUS_MAX {
            return;
        }
        let cost = forge_cost(cur + 1);
        if !can_pay(&self.p, cost) {
            play(&self.sfx.deny);
            self.toast("วัตถุดิบไม่พอ");
            return;
        }
        for &(it, n) in cost {
            self.p.inv[it] -= n;
        }
        if t < 3 { self.p.wplus[t] += 1 } else { self.p.aplus[t - 3] += 1 }
        let n = self.p.forge_name(t);
        self.toast(format!("ตีบวก {} +{} สำเร็จ!", n, cur + 1));
        play(&self.sfx.upgrade);
        let (x, y) = (self.p.x, self.p.y - 8.);
        self.burst(x, y, hx(AURA[(cur + 1) as usize]), 26, 100.);
        self.shake(1.2, 0.15);
        self.save();
    }

    // ---------- สเตตัส ----------
    pub fn spend_stat(&mut self, i: usize) {
        if self.p.sp <= 0 || self.p.st[i] >= ST_MAX {
            return;
        }
        self.p.sp -= 1;
        self.p.st[i] += 1;
        if i == 0 {
            self.p.maxhp += 3;
            self.p.hp += 3;
        }
        play(&self.sfx.craft);
        self.save();
    }
}

// ======================= คำนวณดาเมจ + เอฟเฟกต์ปะทะ =======================
impl Game {
    /// ดาเมจที่ผู้เล่นทำได้จริง (สเตตัส + ตีบวก + คริติคอล)
    pub fn roll_dmg(&self, base: i32) -> (i32, bool) {
        let f = base as f32 * self.p.atk_mul(self.p.weapon);
        let mut d = f.floor() as i32 + if gen_range(0., 1.) < f.fract() { 1 } else { 0 };
        d = d.max(1);
        let crit = gen_range(0., 1.) < self.p.crit_rate();
        if crit {
            d = (d as f32 * 1.8).ceil() as i32;
        }
        (d, crit)
    }
    pub fn shake(&mut self, mag: f32, t: f32) {
        if mag >= self.cshake.0 || self.cshake.1 <= 0. {
            self.cshake = (mag, t);
        }
    }
    pub fn shake_off(&self) -> (f32, f32) {
        if self.cshake.1 > 0. && crate::prefs::shake() {
            (gen_range(-1., 1.) * self.cshake.0, gen_range(-1., 1.) * self.cshake.0)
        } else {
            (0., 0.)
        }
    }
    pub fn add_float_sz(&mut self, x: f32, y: f32, t: impl Into<String>, c: Color, sz: f32) {
        self.floats.push(Float { x, y, t: t.into(), c, l: 1.1, icon: None, sz });
    }
    /// ทำดาเมจใส่มอนสเตอร์ (ไม่ผ่านการคำนวณสเตตัส: ใช้กับผู้ติดตามด้วย)
    pub fn damage_enemy(&mut self, i: usize, dmg: i32, crit: bool, dx: f32, dy: f32) {
        let (ex, ey);
        {
            let e = &mut self.en[i];
            e.hp -= dmg as f32;
            e.flash = 0.18;
            e.kb = if crit { 0.24 } else { 0.15 };
            let ks = if crit { 220. } else { 140. };
            e.vx = dx * ks;
            e.vy = dy * ks;
            ex = e.x;
            ey = e.y - if e.k == EK::Bat { 14. } else { 5. };
        }
        self.hitstop = self.hitstop.max(if crit { 0.09 } else { 0.045 });
        if crit {
            self.add_float_sz(ex, ey - 10., format!("{}!", dmg), hx(0xff9a2a), 1.6);
            self.shake(1.5, 0.12);
            self.burst(ex, ey, hx(0xffd84a), 8, 70.);
            play(&self.sfx.crit);
        } else {
            self.add_float(ex, ey - 8., format!("-{}", dmg), hx(0xffff55), None);
        }
        self.burst(ex, ey, WHITE, 4, 50.);
        if self.en[i].hp <= 0. && !self.en[i].dead {
            self.en[i].dead = true;
            let e = self.en[i].clone();
            self.kill(&e);
        }
    }
    pub fn damage_boss(&mut self, i: usize, dmg: i32, crit: bool) {
        let (x, y);
        {
            let b = &mut self.bosses[i];
            b.hp -= dmg as f32;
            b.flash = 0.12;
            x = b.x;
            y = b.y - 28.;
        }
        self.hitstop = self.hitstop.max(if crit { 0.1 } else { 0.05 });
        if crit {
            self.add_float_sz(x, y, format!("{}!", dmg), hx(0xff9a2a), 1.7);
            self.shake(1.8, 0.14);
            play(&self.sfx.crit);
        } else {
            self.add_float(x, y, format!("-{}", dmg), hx(0xffff55), None);
        }
        self.burst(x, y + 12., WHITE, 4, 60.);
        play(&self.sfx.hit);
        if self.bosses[i].hp <= 0. && !self.bosses[i].dead {
            self.bosses[i].dead = true;
            let b = self.bosses[i].clone();
            self.boss_die(&b);
        }
    }
    /// หามอนสเตอร์/บอสที่ใกล้ที่สุดในระยะ (คืนทิศทางจากผู้เล่น)
    pub fn auto_target(&self, range: f32) -> Option<Vec2> {
        let (px, py) = (self.p.x, self.p.y - 6.);
        let mut best = range;
        let mut res = None;
        for e in &self.en {
            if e.dead {
                continue;
            }
            let ey = e.y - if e.k == EK::Bat { 14. } else { 5. };
            let d = (e.x - px).hypot(ey - py);
            if d < best {
                best = d;
                res = Some(vec2(e.x - px, ey - py));
            }
        }
        for a in self.animals.iter().filter(|a| matches!(a.st, AS::Chase | AS::Charge) && predator(a.k)) {
            let d = (a.x - px).hypot(a.y - 5. - py);
            if d < best {
                best = d;
                res = Some(vec2(a.x - px, a.y - 5. - py));
            }
        }
        for b in &self.bosses {
            let d = (b.x - px).hypot(b.y - 12. - py) - BOSS_R;
            if !b.dead && d < best {
                best = d;
                res = Some(vec2(b.x - px, b.y - 12. - py));
            }
        }
        res
    }
    /// สร้างมอนสเตอร์ตามค่าพื้นฐาน × ตัวคูณความยาก (กลางคืน = เวอร์ชันเก่งกว่า ดรอปเศษดาว)
    pub fn make_enemy(&mut self, k: EK, x: f32, y: f32, mul: f32, pal: usize, night: bool) {
        if peaceful() && self.w.dun.is_none() { return; }
        let mul = mul * enemy_mul();
        let (hp, spd, dmg, xp, coin) = match k {
            EK::Slime => (3., 24., 1., 3., 1.),
            EK::Skel => (6., 34., 2., 7., 3.),
            EK::Bat => (2., 46., 1., 4., 1.),
        };
        let (hm, dm, sm, xm) = if night { (1.6, 1.3, 1.12, 1.5) } else { (1., 1., 1., 1.) };
        self.en.push(Enemy {
            k, x, y, hp: (hp * mul * hm).ceil(), spd: spd * (0.9 + mul * 0.1).min(1.5) * sm, dmg: (dmg * mul.min(3.) * dm).ceil() as i32,
            xp: (xp * mul * xm).ceil() as i32, coin: (coin * mul).ceil() as i32, vx: 0., vy: 0., kb: 0., flash: 0., wt: 0., wdx: 0., wdy: 0.,
            anim: gen_range(0., 5.), pal, dead: false, night, burn: 0., slow: 0., stun: 0., btick: 0.,
        });
    }
}

// ======================= Dash =======================
impl Game {
    pub fn try_dash(&mut self, mv: Vec2) {
        if self.p.dash_cd > 0. || self.p.dash_t > 0. {
            return;
        }
        let dir = if mv.length() > 0.2 {
            mv.normalize()
        } else {
            let f = self.p.facing();
            vec2(f.0, f.1)
        };
        self.p.dash_dir = dir;
        self.p.dash_t = 0.2;
        self.p.dash_cd = 1.0;
        self.p.ghost_t = 0.;
        self.p.invul = self.p.invul.max(0.32);
        self.face(dir);
        let (x, y) = (self.p.x, self.p.y);
        self.burst(x, y, hx(0xcfe8ff), 8, 40.);
        play(&self.sfx.dash);
    }
    pub fn upd_dash(&mut self, dt: f32) {
        if self.p.dash_cd > 0. {
            self.p.dash_cd -= dt;
        }
        if self.p.dash_t > 0. {
            self.p.dash_t -= dt;
            self.p.ghost_t -= dt;
            if self.p.ghost_t <= 0. {
                self.p.ghost_t = 0.03;
                self.ghosts.push((self.p.x, self.p.y, self.p.dir, self.p.flip, 0.3));
            }
        }
        for g in self.ghosts.iter_mut() {
            g.4 -= dt;
        }
        self.ghosts.retain(|g| g.4 > 0.);
    }
}

// ======================= ไอเทมดรอปบนพื้น =======================
pub struct Loot {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub item: usize,
    pub n: i32,
    pub life: f32,
}

impl Game {
    pub fn spawn_loot(&mut self, x: f32, y: f32, item: usize, n: i32) {
        if self.drops.len() > 90 {
            self.drops.remove(0);
        }
        let a = gen_range(0., std::f32::consts::TAU);
        let s = gen_range(20., 60.);
        self.drops.push(Loot { x, y, vx: a.cos() * s, vy: a.sin() * s, item, n, life: 60. });
    }
    /// เก็บของดรอปเข้ากระเป๋า
    pub fn collect_loot(&mut self, i: usize) {
        let l = self.drops.swap_remove(i);
        self.p.inv[l.item] += l.n;
        self.add_float(l.x, l.y - 6., format!("+{}", l.n), WHITE, Some(l.item));
        self.quest_item(l.item, l.n);
        if self.pick_cd <= 0. {
            self.pick_cd = 0.05;
            play(if l.item == COIN { &self.sfx.gem } else { &self.sfx.pickup });
        }
    }
    pub fn upd_drops(&mut self, dt: f32) {
        if self.pick_cd > 0. {
            self.pick_cd -= dt;
        }
        let (px, py) = (self.p.x, self.p.y - 4.);
        let mut take: Vec<usize> = vec![];
        for (i, l) in self.drops.iter_mut().enumerate() {
            l.life -= dt;
            let (dx, dy) = (px - l.x, py - l.y);
            let d = dx.hypot(dy).max(0.001);
            if d < 40. && l.life < 59.6 {
                let s = 150. + (40. - d) * 4.;
                l.vx = dx / d * s;
                l.vy = dy / d * s;
            } else {
                l.vx *= 1. - (5. * dt).min(1.);
                l.vy *= 1. - (5. * dt).min(1.);
            }
            l.x += l.vx * dt;
            l.y += l.vy * dt;
            if d < 7. {
                take.push(i);
            }
        }
        take.sort_unstable_by(|a, b| b.cmp(a));
        for i in take {
            self.collect_loot(i);
        }
        self.drops.retain(|l| l.life > 0.);
    }
}

pub fn draw_loot(g: &Game, sp: &Sprites, v: &View, i: usize, t_now: f32) {
    let l = &g.drops[i];
    let (sx, sy) = (l.x - v.cx, l.y - v.cy);
    let bob = ((t_now * 5. + l.x).sin() * 1.5).round();
    prect(v, (sx - 3.).round(), (sy + 1.).round(), 6., 1., Color::new(0., 0., 0., 0.25));
    let a = if l.life < 5. && ((l.life * 8.) as i32) % 2 == 0 { 0.4 } else { 1. };
    blit_sz(sp, v, sp.icons[l.item], (sx - 4.).round(), (sy - 8. + bob).round(), 8., 8., Color::new(1., 1., 1., a));
}

// ======================= ผู้ติดตาม =======================
pub struct Comp {
    pub kind: u8,
    pub x: f32,
    pub y: f32,
    pub atk_cd: f32,
    pub anim: f32,
    pub left: bool,
}
pub const COMP_N: [&str; 2] = ["หมาป่า", "อัศวินจิ๋ว"];
pub const COMP_D: [&str; 2] = ["ไวมาก โจมตีถี่ เก็บของดรอปให้", "แข็งแรง โจมตีหนัก เก็บของดรอปให้"];
pub const COMP_PRICE: [i32; 2] = [80, 160];

impl Game {
    pub fn hire(&mut self, kind: u8) {
        let price = COMP_PRICE[kind as usize];
        if self.comp.as_ref().map_or(false, |c| c.kind == kind) {
            self.toast("จ้างคนนี้อยู่แล้ว");
            return;
        }
        if self.p.inv[COIN] < price {
            play(&self.sfx.deny);
            self.toast("เหรียญไม่พอ");
            return;
        }
        self.p.inv[COIN] -= price;
        self.comp = Some(Comp { kind, x: self.p.x - 10., y: self.p.y, atk_cd: 0., anim: 0., left: false });
        play(&self.sfx.quest);
        self.toast(format!("จ้าง{}แล้ว! จะคอยช่วยสู้และเก็บของให้", COMP_N[kind as usize]));
        self.save();
    }
    pub fn dismiss_comp(&mut self) {
        if self.comp.take().is_some() {
            self.toast("ปล่อยผู้ติดตามแล้ว");
            self.save();
        }
    }
    pub fn tp_comp(&mut self) {
        let (x, y) = (self.p.x - 10., self.p.y);
        if let Some(c) = self.comp.as_mut() {
            c.x = x;
            c.y = y;
        }
    }
    pub fn upd_comp(&mut self, dt: f32) {
        let Some(mut c) = self.comp.take() else { return };
        c.anim += dt;
        if c.atk_cd > 0. {
            c.atk_cd -= dt;
        }
        let spd = if c.kind == 0 { 84. } else { 70. };
        let (px, py) = (self.p.x, self.p.y);
        let dp = (px - c.x).hypot(py - c.y);
        if dp > 230. {
            c.x = px - 10.;
            c.y = py;
        }
        // เป้าหมาย: มอนสเตอร์ใกล้ผู้เล่น
        let mut tgt: Option<(f32, f32, bool, usize)> = None;
        let mut best = 115.;
        for (i, e) in self.en.iter().enumerate() {
            if e.dead {
                continue;
            }
            let d = (e.x - px).hypot(e.y - py);
            if d < best {
                best = d;
                tgt = Some((e.x, e.y, false, i));
            }
        }
        for (i, b) in self.bosses.iter().enumerate() {
            let d = (b.x - px).hypot(b.y - py);
            if !b.dead && d < best + 30. {
                best = d;
                tgt = Some((b.x, b.y, true, i));
            }
        }
        let (mut mx, mut my) = (0f32, 0f32);
        if let Some((tx, ty, boss, i)) = tgt {
            let (dx, dy) = (tx - c.x, ty - c.y);
            let d = dx.hypot(dy).max(0.001);
            let reach = if boss { BOSS_R + 6. } else { 11. };
            if d > reach {
                mx = dx / d * spd * 1.4;
                my = dy / d * spd * 1.4;
            } else if c.atk_cd <= 0. {
                let dmg = if c.kind == 0 { 2 + self.p.lvl / 4 } else { 4 + self.p.lvl / 3 };
                c.atk_cd = if c.kind == 0 { 0.7 } else { 0.95 };
                if boss {
                    self.damage_boss(i, (dmg / 2).max(1), false);
                } else {
                    self.damage_enemy(i, dmg, false, dx / d, dy / d);
                }
            }
            if dx.abs() > 1. {
                c.left = dx < 0.;
            }
        } else {
            // เก็บของดรอปใกล้ๆ
            let mut near: Option<(usize, f32)> = None;
            for (i, l) in self.drops.iter().enumerate() {
                let d = (l.x - c.x).hypot(l.y - c.y);
                if d < 130. && (l.x - px).hypot(l.y - py) < 170. && near.map_or(true, |n| d < n.1) {
                    near = Some((i, d));
                }
            }
            if let Some((i, d)) = near {
                let (dx, dy) = (self.drops[i].x - c.x, self.drops[i].y - c.y);
                if d < 6. {
                    self.collect_loot(i);
                } else {
                    mx = dx / d * spd * 1.3;
                    my = dy / d * spd * 1.3;
                    c.left = dx < 0.;
                }
            } else if dp > 26. {
                let s = spd * (1. + (dp - 26.) / 70.).min(2.2);
                mx = (px - c.x) / dp * s;
                my = (py - c.y) / dp * s;
                c.left = mx < 0.;
            }
        }
        let (nx, ny) = (c.x + mx * dt, c.y + my * dt);
        if !collide(&self.w, nx, c.y) {
            c.x = nx;
        }
        if !collide(&self.w, c.x, ny) {
            c.y = ny;
        }
        self.comp = Some(c);
    }
}

pub fn draw_comp(g: &Game, sp: &Sprites, v: &View) {
    let Some(c) = &g.comp else { return };
    let (sx, sy) = ((c.x - v.cx).round(), (c.y - v.cy).round());
    let f = ((c.anim * 8.) as usize) % 2;
    prect(v, sx - 4., sy - 1., 8., 2., Color::new(0., 0., 0., 0.25));
    let r = sp.ex.comp[c.kind as usize * 2 + f];
    if c.kind == 0 {
        blit(sp, v, r, sx - 6., sy - r.h + 1., c.left, WHITE);
    } else {
        blit(sp, v, r, sx - 4., sy - r.h + 1., c.left, WHITE);
    }
}

// ======================= เควสต์ย่อย =======================
#[derive(Clone)]
pub struct Quest {
    pub camp: (i32, i32),
    pub kind: u8,
    pub goal: i32,
    pub prog: i32,
    pub coin: i32,
    pub shard: i32,
    /// พิมพ์เขียวพิเศษ: 0 ไม่มี · 1 เกราะ · 2 อาวุธ
    pub bp: u8,
}
pub fn quest_title(q: &Quest) -> String {
    let g = q.goal;
    match q.kind {
        0 => format!("ปราบมอนสเตอร์ {} ตัว", g),
        1 => format!("ปราบโครงกระดูก {} ตัว", g),
        2 => format!("ปราบสไลม์ {} ตัว", g),
        3 => format!("ปราบค้างคาว {} ตัว", g),
        4 => format!("ปราบมอนสเตอร์ราตรี {} ตัว (ออกล่าตอนกลางคืน)", g),
        5 => format!("หาอัญมณี {} ชิ้น", g),
        6 => format!("เก็บหิน {} ก้อน", g),
        _ => format!("เก็บไม้ {} ท่อน", g),
    }
}
pub fn quest_reward_text(q: &Quest) -> String {
    let mut s = format!("{} เหรียญ", q.coin);
    if q.shard > 0 {
        s += &format!(" + เศษดาว {}", q.shard);
    }
    match q.bp {
        1 => s += " + พิมพ์เขียวเกราะหายาก",
        2 => s += " + พิมพ์เขียวอาวุธหายาก",
        _ => {}
    }
    s
}

impl Game {
    /// เควสต์ที่แคมป์นี้เสนอ (กำหนดแน่นอนจากแคมป์ + จำนวนที่ทำแล้ว)
    pub fn offered_quest(&self, camp: (i32, i32)) -> Quest {
        let n = self.qn.get(&camp).copied().unwrap_or(0);
        let h = hash(camp.0, camp.1, self.w.seed.wrapping_add(31 * n + 17));
        let kind = ((h * 8.) as u8).min(7);
        let r = hash(camp.0, camp.1, self.w.seed.wrapping_add(53 * n + 91));
        let (lo, hi, per) = match kind {
            0 => (8, 14, 4),
            1 => (4, 8, 8),
            2 => (8, 14, 4),
            3 => (5, 9, 6),
            4 => (3, 5, 14),
            5 => (3, 6, 12),
            6 => (14, 26, 2),
            _ => (18, 30, 2),
        };
        let goal = lo + (r * (hi - lo + 1) as f64) as i32;
        let bp = if n == 0 && self.p.inv[BP_ARM] == 0 { 1 } else if n == 1 && self.p.inv[BP_WPN] == 0 { 2 } else { 0 };
        Quest { camp, kind, goal, prog: 0, coin: goal * per + 10 + n * 6, shard: if kind == 4 { 3 } else { 1 + (n % 2) }, bp }
    }
    pub fn accept_quest(&mut self, camp: (i32, i32)) {
        if self.quest.is_some() {
            self.toast("คุณมีเควสต์ค้างอยู่แล้ว");
            return;
        }
        let q = self.offered_quest(camp);
        self.toast(format!("รับเควสต์: {}", quest_title(&q)));
        self.quest = Some(q);
        play(&self.sfx.quest);
        self.save();
    }
    pub fn abandon_quest(&mut self) {
        if self.quest.take().is_some() {
            self.toast("ยกเลิกเควสต์แล้ว");
            self.save();
        }
    }
    pub fn claim_quest(&mut self) {
        let Some(q) = self.quest.clone() else { return };
        if q.prog < q.goal {
            return;
        }
        self.p.inv[COIN] += q.coin;
        self.p.inv[SHARD] += q.shard;
        match q.bp {
            1 => self.p.inv[BP_ARM] += 1,
            2 => self.p.inv[BP_WPN] += 1,
            _ => {}
        }
        *self.qn.entry(q.camp).or_insert(0) += 1;
        self.quest = None;
        self.p.xp += 15;
        self.check_lv();
        play(&self.sfx.quest);
        self.toast(format!("เควสต์สำเร็จ! ได้ {}", quest_reward_text(&q)));
        self.save();
    }
    fn quest_step(&mut self, n: i32) {
        let Some(q) = self.quest.as_mut() else { return };
        if q.prog >= q.goal {
            return;
        }
        q.prog = (q.prog + n).min(q.goal);
        if q.prog >= q.goal {
            self.toast("เควสต์ครบแล้ว! กลับไปส่งที่แคมป์พ่อค้า");
            play(&self.sfx.quest);
        }
    }
    pub fn quest_kill(&mut self, k: EK, night: bool) {
        let kind = match &self.quest {
            Some(q) => q.kind,
            None => return,
        };
        let ok = match kind {
            0 => true,
            1 => k == EK::Skel,
            2 => k == EK::Slime,
            3 => k == EK::Bat,
            4 => night,
            _ => false,
        };
        if ok {
            self.quest_step(1);
        }
    }
    pub fn quest_item(&mut self, item: usize, n: i32) {
        let kind = match &self.quest {
            Some(q) => q.kind,
            None => return,
        };
        if (kind == 5 && item == GEM) || (kind == 6 && item == STONE) || (kind == 7 && item == WOOD) {
            self.quest_step(n);
        }
    }
}

// ======================= สภาพอากาศ (ฝน) =======================
impl Game {
    pub fn upd_weather(&mut self, dt: f32) {
        // ชนิดอากาศตามไบโอมที่ยืนอยู่: หิมะ = ดินแดนหิมะ · ทะเลทราย = ไม่มีฝน · ที่เหลือ = ฝน
        let bt = self.w.biome_at(tfloor(self.p.x), tfloor(self.p.y));
        self.weather_kind = if self.w.dun.is_some() { 0 } else { self.season_weather(match bt { 7 => 2, 8 => 0, _ => 1 }, bt) };
        self.rain_t -= dt;
        if self.rain_t <= 0. {
            self.rain_on = !self.rain_on;
            let (mon, moff) = self.rain_len_mul();
            self.rain_t = if self.rain_on { gen_range(45., 100.) * mon } else { gen_range(110., 260.) * moff };
            if self.rain_on && self.w.dun.is_none() {
                match self.weather_kind {
                    1 => self.toast("ฝนเริ่มตกแล้ว"),
                    2 => self.toast("หิมะตก — เดินช้าลง"),
                    _ => {}
                }
            }
        }
        let target = if self.rain_on && self.weather_kind != 0 { 1. } else { 0. };
        self.rain += (target - self.rain).clamp(-0.4 * dt, 0.3 * dt);
        if self.thunder > 0. {
            self.thunder -= dt;
        } else if self.rain > 0.8 && gen_range(0., 1.) < dt * 0.05 {
            self.thunder = 0.25;
            self.shake(0.8, 0.2);
        }
    }
}

impl Game {
    /// ตัวคูณความเร็วเดินจากสภาพอากาศ: หิมะช้าลงได้ถึง 18% · ฝน 5%
    pub fn weather_mul(&self) -> f32 {
        if self.indoors() { return 1.; }
        match self.weather_kind {
            2 => 1. - 0.18 * self.rain.min(1.),
            1 => 1. - 0.05 * self.rain.min(1.),
            _ => 1.,
        }
    }
}

pub fn draw_weather(g: &Game, v: &View, t_now: f32) {
    if g.weather_kind == 2 { draw_snow(g, v, t_now); } else { draw_rain(g, v, t_now); }
}
/// หิมะ: เกล็ดลอยลงช้าๆ แกว่งซ้ายขวา
pub fn draw_snow(g: &Game, v: &View, t_now: f32) {
    if g.rain < 0.02 {
        return;
    }
    let n = (150. * g.rain) as i32;
    let col = Color::new(1., 1., 1., 0.8 * g.rain.min(1.));
    for i in 0..n {
        let f = i as f32;
        let bx = ((f * 12.9898).sin() * 43758.5).rem_euclid(1.) * (v.vw + 20.);
        let x = bx + (t_now * 0.9 + f).sin() * 5. - 6.;
        let y = (f * 91.3 + t_now * (28. + (i % 4) as f32 * 9.)).rem_euclid(v.vh + 12.) - 6.;
        let sz = if i % 3 == 0 { 2. } else { 1. };
        draw_rectangle(x * v.s, y * v.s, sz * v.s, sz * v.s, col);
    }
}

/// วาดเม็ดฝนทับหน้าจอ
pub fn draw_rain(g: &Game, v: &View, t_now: f32) {
    if g.rain < 0.02 {
        return;
    }
    let n = (170. * g.rain) as i32;
    let col = Color::new(0.72, 0.82, 1., 0.45 * g.rain.min(1.));
    for i in 0..n {
        let f = i as f32;
        let x = ((f * 12.9898).sin() * 43758.5).rem_euclid(1.) * (v.vw + 20.) - 4.;
        let y = (f * 91.3 + t_now * (210. + (i % 5) as f32 * 14.)).rem_euclid(v.vh + 12.) - 6.;
        draw_line((x - 6.) * v.s, y * v.s, (x - 8.) * v.s, (y + 5.) * v.s, v.s.max(1.), col);
    }
    if g.thunder > 0. {
        draw_rectangle(0., 0., screen_width(), screen_height(), Color::new(1., 1., 1., (g.thunder * 2.).min(0.5)));
    }
}
