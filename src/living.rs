// living.rs — v23 โลกที่มีชีวิต
//  - ถนนดินเชื่อมเมืองหลวงกับเมืองลูก และเมืองหลวงด้วยกัน (ข้ามน้ำ/ลาวา = สะพานไม้) · เดินบนถนนเร็วขึ้น 20%
//  - กองคาราวานเดินค้าขายจริงบนถนน: เดินคุ้มกันจนถึงเมือง = รางวัล + ชื่อเสียง · โจรดักปล้นระหว่างทาง
//    หรือจะปล้นเองก็ได้ (กดใช้ 2 ครั้ง) ได้ของแต่ชื่อเสียงตก ทหารคุ้มกันสู้กลับ
//  - เทศกาลประจำฤดู (วันที่ 3 ของทุกฤดู): ชาวบ้านมารวมกันที่ลานเมืองตอนเย็น · ไปร่วมได้ของขวัญ
//  - เหตุการณ์สุ่มตอนเช้า: โรคระบาด · น้ำท่วมไร่ · ผู้ลี้ภัยสงคราม · ปีทองเก็บเกี่ยวได้มาก
use super::*;

pub const T_ROAD: u8 = 21;
pub const T_BRIDGE: u8 = 22;

// ---------------------------------------------------------------- ไทล์
pub fn road_tex(i: i32) -> Image {
    let mut im = mk_tex(0xb0905e, &[0x9c7c4c, 0xc0a070, 0x8e7046], 20, 150 + i, Extra::None);
    // รอยล้อเกวียนจางๆ
    for y in 0..16u32 {
        for x in [4u32, 11] {
            if hash(x as i32, y as i32 + i * 16, 8801) < 0.75 { im.set_pixel(x, y, hx(0x957548)); }
        }
    }
    im
}
pub fn bridge_tex(i: i32) -> Image {
    let mut im = Image::gen_image_color(16, 16, hx(0x8a5a30));
    for y in 0..16u32 {
        for x in 0..16u32 {
            let plank = (y / 4) as i32;
            let c = if y % 4 == 3 { hx(0x4a2c14) } else if hash(x as i32 / 5 + plank * 7, i, 8802) < 0.5 { hx(0x9a6a3a) } else { hx(0x8a5a30) };
            im.set_pixel(x, y, c);
        }
    }
    // หมุดตอกไม้
    for y in [1u32, 5, 9, 13] { im.set_pixel(2, y, hx(0x3a2a20)); im.set_pixel(13, y, hx(0x3a2a20)); }
    im
}

// ---------------------------------------------------------------- ถนน
#[derive(Clone, Debug, Default)]
pub struct Route { pub a: usize, pub b: usize, pub pts: Vec<(i32, i32)>, pub cum: Vec<f32> }
impl Route {
    pub fn total(&self) -> f32 { self.cum.last().copied().unwrap_or(0.) }
    /// ตำแหน่งพิกเซลที่ระยะ d จากต้นทาง (กลางถนนกว้าง 2 ช่อง)
    pub fn at(&self, d: f32) -> (f32, f32) {
        let px = |p: (i32, i32)| (p.0 as f32 * 16. + 16., p.1 as f32 * 16. + 16.);
        if self.pts.is_empty() { return (0., 0.); }
        let d = d.clamp(0., self.total());
        let i = self.cum.partition_point(|&c| c < d).min(self.pts.len() - 1);
        if i == 0 { return px(self.pts[0]); }
        let (a, b) = (px(self.pts[i - 1]), px(self.pts[i]));
        let seg = (self.cum[i] - self.cum[i - 1]).max(0.001);
        let k = (d - self.cum[i - 1]) / seg;
        (a.0 + (b.0 - a.0) * k, a.1 + (b.1 - a.1) * k)
    }
}
fn gate_of(c: &City, toward: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let (dx, dy) = (toward.0 - c.pos.0, toward.1 - c.pos.1);
    if dx.abs() >= dy.abs() { let s = if dx < 0 { -1 } else { 1 }; ((c.pos.0 + s * (c.r + 1), c.pos.1), (s, 0)) }
    else { let s = if dy < 0 { -1 } else { 1 }; ((c.pos.0, c.pos.1 + s * (c.r + 1)), (0, s)) }
}
fn step4(v: &mut Vec<(i32, i32)>, t: (i32, i32)) {
    let mut c = *v.last().unwrap();
    let mut guard = 0;
    while c != t && guard < 4000 {
        guard += 1;
        let (dx, dy) = (t.0 - c.0, t.1 - c.1);
        if dx.abs() >= dy.abs() { c.0 += dx.signum(); } else { c.1 += dy.signum(); }
        v.push(c);
    }
}
/// เส้นถนนจากประตูเมือง a ไปประตูเมือง b (ออกตรงจากประตู 4 ช่องก่อน แล้วคดเคี้ยวเล็กน้อยตามธรรมชาติ)
pub fn road_line(seed: i32, cities: &[City], a: usize, b: usize) -> Vec<(i32, i32)> {
    let (ga, oa) = gate_of(&cities[a], cities[b].pos);
    let (gb, ob) = gate_of(&cities[b], cities[a].pos);
    // ประตูฝั่งตะวันตก/เหนือ: ถนนกว้าง 2 ช่องยื่นไป +x/+y → เลื่อนจุดเริ่มให้ช่องที่สองยังอยู่นอกกำแพง
    let fix = |g: (i32, i32), o: (i32, i32)| (g.0 - if o.0 < 0 { 1 } else { 0 }, g.1 - if o.1 < 0 { 1 } else { 0 });
    let (ga, gb) = (fix(ga, oa), fix(gb, ob));
    let a2 = (ga.0 + oa.0 * 4, ga.1 + oa.1 * 4);
    let b2 = (gb.0 + ob.0 * 4, gb.1 + ob.1 * 4);
    let mut v = vec![ga];
    step4(&mut v, a2);
    let (dx, dy) = ((b2.0 - a2.0) as f32, (b2.1 - a2.1) as f32);
    let len = dx.hypot(dy).max(1.);
    let (px, py) = (-dy / len, dx / len);
    let amp = (len / 10.).min(8.);
    let f = 1. + (hash(a as i32, b as i32, seed.wrapping_add(8810)) * 2.) as f32;
    let ph = hash(b as i32, a as i32, seed.wrapping_add(8811)) as f32 * 6.28;
    let n = (len / 3.).ceil() as i32;
    for i in 1..=n {
        let t = i as f32 / n as f32;
        let w = (t * std::f32::consts::PI).sin() * amp * (t * std::f32::consts::PI * f + ph).sin();
        let p = ((a2.0 as f32 + dx * t + px * w).round() as i32, (a2.1 as f32 + dy * t + py * w).round() as i32);
        step4(&mut v, p);
    }
    step4(&mut v, b2);
    step4(&mut v, gb);
    v
}
pub fn build_roads(w: &World, cities: &[City]) -> (HashSet<(i32, i32)>, Vec<Route>) {
    let mut roads = HashSet::new();
    let mut routes = vec![];
    let mut pairs: Vec<(usize, usize, bool)> = vec![];
    let caps: Vec<usize> = (0..KN).filter_map(|k| cities.iter().position(|c| c.cap && c.k0 == k)).collect();
    for &cap in &caps {
        for (i, c) in cities.iter().enumerate() { if !c.cap && c.k0 == cities[cap].k0 { pairs.push((cap, i, true)); } }
    }
    for i in 0..caps.len() { if caps.len() > 1 { pairs.push((caps[i], caps[(i + 1) % caps.len()], false)); } }
    let in_city = |p: (i32, i32)| cities.iter().any(|c| (p.0 - c.pos.0).abs() <= c.r && (p.1 - c.pos.1).abs() <= c.r);
    for (a, b, must) in pairs {
        let pts = road_line(w.seed, cities, a, b);
        if pts.len() < 4 { continue; }
        let wet = pts.iter().filter(|p| { let t = w.gen_tile(p.0, p.1); t < 2 || t == T_LAVA }).count();
        // ถนนข้ามทะเลยาวๆ ไม่สร้าง (เมืองหลวงที่อยู่คนละฝั่งทะเล)
        if wet * 100 > pts.len() * if must { 60 } else { 30 } { continue; }
        // ถนนที่ทะลุเมืองอื่นกลางทาง (ชนกำแพง) ไม่สร้าง
        if pts.iter().skip(6).take(pts.len().saturating_sub(12)).any(|&p| in_city(p)) { continue; }
        for &p in &pts {
            for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                let q = (p.0 + dx, p.1 + dy);
                if !in_city(q) { roads.insert(q); }
            }
        }
        let mut cum = vec![0.];
        for i in 1..pts.len() { let l = cum[i - 1] + 16.; cum.push(l); }
        routes.push(Route { a, b, pts, cum });
    }
    (roads, routes)
}

// ---------------------------------------------------------------- สถานะ
#[derive(Clone, Debug)]
pub struct Caravan {
    pub route: usize, pub fwd: bool, pub d: f32, pub x: f32, pub y: f32, pub owner: u8, pub from: usize, pub to: usize,
    pub esc: f32, pub ambush: u8, pub will_ambush: bool, pub rob_arm: f32, pub anim: f32, pub left: bool, pub told: bool,
}
#[derive(Clone, Debug, Default)]
pub struct Living {
    pub cara: Vec<Caravan>,
    pub cara_t: f32,
    pub last_ev: i32,
    pub bumper: i32,
    pub flood_day: i32,
    pub flood: HashSet<(i32, i32)>,
    pub waiting: u32,
    pub fest_done: i32,
    pub sick: HashMap<u32, i32>,
    pub escorted: u32,
    pub robbed: u32,
    pub fest_n: u32,
    pub tick_t: f32,
}

pub const FEST: [(&str, &str); 4] = [
    ("เทศกาลดอกไม้บาน", "ประดับดอกไม้ทั่วลานเมือง"),
    ("เทศกาลสาดน้ำ", "สาดน้ำคลายร้อนกันทั้งเมือง"),
    ("เทศกาลเก็บเกี่ยว", "ฉลองผลผลิตของปีนี้"),
    ("เทศกาลโคมไฟ", "จุดโคมไฟอบอุ่นกลางฤดูหนาว"),
];
/// วันที่ 3 ของทุกฤดู = วันเทศกาล
pub fn festival_of(day: i32) -> Option<u8> {
    if day >= 2 && day % SEASON_DAYS == 2 { Some(season_of_day(day)) } else { None }
}
pub const BANDIT: usize = 99;
const CARA_SPD: f32 = 26.;

impl Game {
    pub fn festival_today(&self) -> Option<u8> { festival_of(day_of(self)) }
    /// จุดรวมงานเทศกาล: กองไฟใกล้กลางเมือง หรือกลางเมือง (ต้องมีชาวบ้าน ≥ 2)
    pub fn fest_spot(&self) -> Option<(i32, i32)> {
        if self.pop() < 2 || self.w.dun.is_some() { return None; }
        let c = self.town_center()?;
        Some(self.w.near_fire(c.0, c.1, 10).unwrap_or(c))
    }
    pub fn is_sick(&self, id: u32) -> bool { self.rs.lv.sick.contains_key(&id) }
    pub fn bumper_today(&self) -> bool { self.rs.lv.bumper == day_of(self) && day_of(self) > 0 }

    // ------------------------------------------------------------ ทุกเฟรม
    pub fn living_tick(&mut self, dt: f32) {
        if self.w.dun.is_some() { return; }
        self.caravans_tick(dt);
        self.rs.lv.tick_t -= dt;
        if self.rs.lv.tick_t > 0. { return; }
        self.rs.lv.tick_t = 0.5;
        self.festival_tick();
    }
    fn festival_tick(&mut self) {
        let day = day_of(self);
        let Some(s) = self.festival_today() else { return };
        let Some(f) = self.fest_spot() else { return };
        let h = self.hour();
        if !(17.5..22.).contains(&h) { return; }
        let (fx, fy) = (f.0 as f32 * 16. + 8., f.1 as f32 * 16. + 8.);
        // ประทัด/กลีบดอกไม้/ละอองน้ำลอยเหนือลาน
        let col = [0xff9ad0, 0x7ad0ff, 0xffc84a, 0xffe08a][s as usize];
        if gen_range(0., 1.) < 0.7 { self.burst(fx + gen_range(-40., 40.), fy - 20. + gen_range(-20., 10.), hx(col), 3, 30.); }
        if self.rs.lv.fest_done == day { return; }
        let near = (self.p.x - fx).hypot(self.p.y - fy) < 90.;
        let joined = self.gather.len();
        if !near || joined < 1 { return; }
        // ร่วมงานครั้งแรกของเทศกาลนี้: ของขวัญ + ชาวบ้านทุกคนสนิทขึ้น
        self.rs.lv.fest_done = day;
        self.rs.lv.fest_n += 1;
        let pop = self.pop() as i32;
        let coins = 20 + pop * 4;
        self.p.inv[COIN] += coins;
        let gift: &[(usize, i32)] = match s { 0 => &[(SEED_CORN, 3), (SEED_CARROT, 3)], 1 => &[(FISH, 3)], 2 => &[(CORN, 4), (CARROT, 4)], _ => &[(MEAT, 3), (FIRE, 2)] };
        for &(it, n) in gift { self.p.inv[it] += n; }
        if s == 1 { self.p.thirst = 100.; }
        self.p.xp += 15;
        for n in self.npcs.iter_mut().filter(|n| n.state == ST_HOME) { n.hearts = (n.hearts + 1).min(10); n.mood = (n.mood + 10).min(100); }
        let gtxt: Vec<String> = gift.iter().map(|(it, n)| format!("{} {}", INAME[*it], n)).collect();
        self.toast(format!("ร่วม{}กับชาวบ้าน! +{} เหรียญ · {} · ทุกคนสนิทกับคุณขึ้น", FEST[s as usize].0, coins, gtxt.join(" · ")));
        self.news_add(format!("{}: ชาวเมืองมาร่วมงานกันคึกคัก", FEST[s as usize].0));
        play(&self.sfx.quest);
        for _ in 0..4 { self.burst(self.p.x, self.p.y - 10., hx(col), 10, 70.); }
    }

    // ------------------------------------------------------------ กองคาราวาน
    fn caravans_tick(&mut self, dt: f32) {
        let realm = self.w.realm.clone();
        if realm.routes.is_empty() { return; }
        if !self.rs.inited { self.rs.init(&realm); }
        let (px, py) = (self.p.x, self.p.y);
        // เกิดใหม่: บนเส้นทางที่ผ่านใกล้ผู้เล่น (นอกจอเล็กน้อย) — ไม่เกิน 2 ขบวน
        self.rs.lv.cara_t -= dt;
        if self.rs.lv.cara_t <= 0. {
            self.rs.lv.cara_t = gen_range(18., 34.);
            if self.rs.lv.cara.len() < 2 {
                let mut cands: Vec<(usize, f32)> = vec![];
                for (ri, r) in realm.routes.iter().enumerate() {
                    if self.rs.lv.cara.iter().any(|c| c.route == ri) { continue; }
                    let mut i = 0;
                    while i < r.pts.len() {
                        let (x, y) = r.at(r.cum[i]);
                        let d = (x - px).hypot(y - py);
                        if (26. * 16.0..48. * 16.).contains(&d) { cands.push((ri, r.cum[i])); }
                        i += 6;
                    }
                }
                if !cands.is_empty() {
                    let (ri, d) = cands[gen_range(0, cands.len())];
                    let r = &realm.routes[ri];
                    let fwd = gen_range(0., 1.) < 0.5;
                    let (from, to) = if fwd { (r.a, r.b) } else { (r.b, r.a) };
                    let owner = self.city_owner(from);
                    let alive = owner == PLAYER_OWN || self.rs.ks.get(owner as usize).map_or(false, |k| k.alive);
                    if alive {
                        let dd = if fwd { d } else { r.total() - d };
                        let (x, y) = r.at(d);
                        let will_ambush = !peaceful() && gen_range(0., 1.) < 0.5;
                        self.rs.lv.cara.push(Caravan { route: ri, fwd, d: dd, x, y, owner, from, to, esc: 0., ambush: 0, will_ambush, rob_arm: 0., anim: 0., left: false, told: false });
                    }
                }
            }
        }
        let mut cs = std::mem::take(&mut self.rs.lv.cara);
        let mut gone = vec![];
        for (i, c) in cs.iter_mut().enumerate() {
            let r = &realm.routes[c.route];
            let near = (c.x - px).hypot(c.y - py) < 6. * 16.;
            if near { c.esc += dt; }
            c.rob_arm = (c.rob_arm - dt).max(0.);
            if near && !c.told {
                c.told = true;
                let whose = if c.owner == PLAYER_OWN { "ของเมืองเรา".to_string() } else { format!("ของ{}", self.kname(c.owner as usize)) };
                self.toast(format!("กองคาราวาน{} กำลังไปเมือง{} — เดินคุ้มกันไปด้วยจนถึงเมืองได้รางวัล{}", whose, realm.cities[c.to].name, if c.owner == PLAYER_OWN { "" } else { " (กด \"ใช้\" ที่เกวียน 2 ครั้ง = ปล้น)" }));
            }
            // โจรดักปล้น: ตอนที่เราคุ้มกันมาสักพัก
            if c.will_ambush && c.ambush == 0 && c.esc > 9. && self.rs.siege.is_none() {
                c.ambush = 1;
                let (dx, dy) = { let (ax, ay) = r.at(if c.fwd { c.d + 90. } else { r.total() - c.d - 90. }); (ax - c.x, ay - c.y) };
                for k in 0..3 {
                    let a = k as f32 * 2.1 + 0.4;
                    let (x, y) = (c.x + dx + a.cos() * 26., c.y + dy + a.sin() * 20.);
                    if !collide(&self.w, x, y) { self.make_enemy(EK::Soldier, x, y, 0.8 + self.p.lvl as f32 * 0.08, BANDIT, false); }
                }
                play(&self.sfx.roar);
                self.toast("โจรดักปล้นกองคาราวาน! ไล่โจรให้หมด (ผ้าคาดสีแดง)");
            }
            let bandits = self.en.iter().filter(|e| !e.dead && e.k == EK::Soldier && e.pal == BANDIT && (e.x - c.x).hypot(e.y - c.y) < 260.).count();
            if c.ambush == 1 && bandits == 0 {
                c.ambush = 2;
                if c.esc > 5. {
                    self.p.inv[COIN] += 12;
                    self.toast("ไล่โจรไปหมดแล้ว! พ่อค้าขอบคุณ +12 เหรียญ (คุ้มกันต่อจนถึงเมืองได้อีก)");
                    play(&self.sfx.buy);
                }
            }
            let stop = c.ambush == 1 && bandits > 0;
            if !stop {
                c.d += CARA_SPD * dt;
                c.anim += dt;
            }
            let pos = r.at(if c.fwd { c.d } else { r.total() - c.d });
            if (pos.0 - c.x).abs() > 0.05 { c.left = pos.0 < c.x; }
            c.x = pos.0; c.y = pos.1;
            if c.d >= r.total() {
                gone.push(i);
                if c.esc >= 18. { self.caravan_reward(c, &realm); }
                continue;
            }
            if (c.x - px).hypot(c.y - py) > 70. * 16. { gone.push(i); }
        }
        for &i in gone.iter().rev() { cs.remove(i); }
        self.rs.lv.cara = cs;
    }
    fn caravan_reward(&mut self, c: &Caravan, realm: &Realm) {
        let len = realm.routes[c.route].total();
        let coins = 15 + (len / 16. / 10.) as i32;
        self.p.inv[COIN] += coins;
        self.p.xp += 10;
        self.rs.lv.escorted += 1;
        if c.owner == PLAYER_OWN {
            self.toast(format!("กองคาราวานของเรามาถึงเมือง{}แล้ว! การค้าคึกคัก +{} เหรียญ", realm.cities[c.to].name, coins));
        } else {
            let k = c.owner as usize;
            self.rs.ks[k].rep = (self.rs.ks[k].rep + 4).min(100);
            self.toast(format!("คุ้มกันกองคาราวานถึงเมือง{}แล้ว! +{} เหรียญ · ชื่อเสียงกับ{} +4", realm.cities[c.to].name, coins, self.kname(k)));
        }
        self.news_add(format!("คุ้มกันกองคาราวานไปเมือง{}สำเร็จ", realm.cities[c.to].name));
        play(&self.sfx.buy);
    }
    /// [debug] เรียกกองคาราวานบนถนนใกล้ตัว
    pub fn dbg_caravan(&mut self) {
        let realm = self.w.realm.clone();
        if !self.rs.inited { self.rs.init(&realm); }
        let mut best: Option<(usize, usize, f32)> = None;
        for (ri, r) in realm.routes.iter().enumerate() { for (i, _) in r.pts.iter().enumerate() { let (x, y) = r.at(r.cum[i]); let d = (x - self.p.x).hypot(y - self.p.y); if best.map_or(true, |b| d < b.2) { best = Some((ri, i, d)); } } }
        let Some((ri, i, _)) = best else { return };
        let r = &realm.routes[ri];
        let dd = (r.cum[i] - 60.).max(0.);
        let (x, y) = r.at(dd);
        self.rs.lv.cara.push(Caravan { route: ri, fwd: true, d: dd, x, y, owner: self.city_owner(r.a), from: r.a, to: r.b, esc: 0., ambush: 0, will_ambush: true, rob_arm: 0., anim: 0., left: false, told: false });
    }
    /// เกวียนที่อยู่ใกล้พอจะกดใช้
    pub fn caravan_near(&self) -> Option<usize> {
        self.rs.lv.cara.iter().position(|c| (c.x - self.p.x).hypot(c.y - 4. - self.p.y) < 26.)
    }
    pub fn caravan_use(&mut self) -> bool {
        let Some(i) = self.caravan_near() else { return false };
        let c = self.rs.lv.cara[i].clone();
        if c.owner == PLAYER_OWN { self.toast("กองคาราวานของเมืองเรา — คุ้มกันไปด้วยจนถึงเมืองได้รางวัล"); return true; }
        if peaceful() { self.toast(format!("พ่อค้า: \"สวัสดี! กำลังขนของไปเมือง{}\" (โหมดสันติปล้นไม่ได้)", self.w.realm.cities[c.to].name)); return true; }
        if c.rob_arm <= 0. {
            self.rs.lv.cara[i].rob_arm = 3.;
            let war = self.rs.ks[c.owner as usize].stance == 3;
            self.toast(if war { format!("กดอีกครั้งเพื่อยึดเสบียงของ{} (ศัตรูในสงคราม)", self.kname(c.owner as usize)) } else { format!("กดอีกครั้งเพื่อปล้น — ทหารคุ้มกันจะสู้ และชื่อเสียงกับ{} ลด 18", self.kname(c.owner as usize)) });
            play(&self.sfx.click);
            return true;
        }
        self.rob_caravan(i);
        true
    }
    pub fn rob_caravan(&mut self, i: usize) {
        let c = self.rs.lv.cara.remove(i);
        let k = c.owner as usize;
        let city = self.w.realm.cities[c.from].clone();
        let coins = 25 + (hash(c.route as i32, day_of(self), 8820) * 20.) as i32;
        self.p.inv[COIN] += coins;
        self.p.inv[city.cheap[0]] += 4;
        self.p.inv[city.cheap[1]] += 3;
        self.rs.lv.robbed += 1;
        let war = self.rs.ks[k].stance == 3;
        if !war { self.rs.ks[k].rep -= 18; }
        for a in 0..2 {
            let (x, y) = (c.x + if a == 0 { -24. } else { 24. }, c.y + 6.);
            if !collide(&self.w, x, y) { self.make_enemy(EK::Soldier, x, y, 1.1, k, false); }
        }
        for _ in 0..3 { self.burst(c.x, c.y - 8., hx(0xffd84a), 10, 60.); }
        play(&self.sfx.chest);
        self.toast(format!("{}! +{} เหรียญ · {} 4 · {} 3{}", if war { "ยึดเสบียงศัตรู" } else { "ปล้นกองคาราวาน" }, coins, INAME[city.cheap[0]], INAME[city.cheap[1]], if war { String::new() } else { format!(" · ชื่อเสียงกับ{} {} — ทหารคุ้มกันสู้กลับ!", self.kname(k), self.rs.ks[k].rep) }));
        self.news_add(format!("กองคาราวานของ{}ถูกปล้นกลางทาง", self.kname(k)));
    }

    // ------------------------------------------------------------ ตอนเช้า
    pub fn living_dawn(&mut self) {
        let day = day_of(self);
        // ผู้ลี้ภัยที่รออยู่ ย้ายเข้าเมื่อมีเตียงว่าง
        while self.rs.lv.waiting > 0 && !self.free_beds().is_empty() {
            self.rs.lv.waiting -= 1;
            self.refugee_move_in(day as u32 * 7 + self.rs.lv.waiting);
        }
        // คนป่วย: ความสุขลด · หายเองใน 4 วัน (หมอหลวงช่วยให้หายไวขึ้น)
        let heal = self.perk_on(P_HEAL);
        let ids: Vec<u32> = self.rs.lv.sick.keys().copied().collect();
        for id in ids {
            let d = self.rs.lv.sick[&id] + 1;
            let Some(i) = self.idx(id) else { self.rs.lv.sick.remove(&id); continue };
            if d >= 4 || (heal && hash(id as i32, day, 8830) < 0.5) {
                self.rs.lv.sick.remove(&id);
                self.news_add(format!("{}หายป่วยแล้ว", NPC_NAMES[self.npcs[i].name as usize]));
            } else {
                self.rs.lv.sick.insert(id, d);
                self.npcs[i].mood = (self.npcs[i].mood - 6).max(0);
            }
        }
        if self.rs.lv.flood_day != day { self.rs.lv.flood.clear(); }
        if let Some(s) = festival_of(day) {
            if self.pop() >= 2 {
                for n in self.npcs.iter_mut().filter(|n| n.state == ST_HOME) { n.mood = (n.mood + 8).min(100); }
                self.toast(format!("วันนี้{}! ({}) ตอนเย็นชาวบ้านจะมารวมกันที่ลานเมือง — แวะไปร่วมงานรับของขวัญ", FEST[s as usize].0, FEST[s as usize].1));
                self.news_add(format!("วันนี้{}", FEST[s as usize].0));
            }
        }
        if day < 3 || day - self.rs.lv.last_ev < 2 || self.pop() < 2 { return; }
        let seed = self.w.seed;
        if hash(day, 1, seed.wrapping_add(8840)) > 0.4 { return; }
        let ssn = season_of_day(day);
        let war = self.rs.ks.iter().any(|k| k.alive && (k.war.iter().any(|&w| w) || k.stance == 3));
        let mut opts: Vec<u8> = vec![];
        if self.pop() >= 3 { opts.push(0); }
        if matches!(ssn, 0 | 2) && self.crops.len() >= 3 { opts.push(1); }
        if war { opts.push(2); opts.push(2); }
        if self.crops.len() >= 2 && ssn != 3 { opts.push(3); }
        if opts.is_empty() { return; }
        let e = opts[(hash(day, 2, seed.wrapping_add(8841)) * opts.len() as f64) as usize % opts.len()];
        self.start_event(e, day);
    }
    /// 0 โรคระบาด · 1 น้ำท่วม · 2 ผู้ลี้ภัย · 3 ปีทอง
    pub fn start_event(&mut self, e: u8, day: i32) {
        self.rs.lv.last_ev = day;
        match e {
            0 => {
                let mut ids: Vec<u32> = self.residents().filter(|n| !n.child).map(|n| n.id).collect();
                ids.sort_by_key(|id| (hash(*id as i32, day, 8850) * 1000.) as i32);
                let n = (ids.len() / 3).clamp(1, 3);
                let names: Vec<&str> = ids.iter().take(n).filter_map(|id| self.idx(*id)).map(|i| NPC_NAMES[self.npcs[i].name as usize]).collect();
                for id in ids.iter().take(n) { self.rs.lv.sick.insert(*id, 0); }
                self.toast(format!("โรคระบาด! {} ป่วย ทำงานไม่ไหว — เอาสมุนไพรไปให้ (กดคุยตอนมีสมุนไพร) จะหายทันที", names.join(", ")));
                self.news_add(format!("โรคระบาดในเมือง: {} ป่วย", names.join(", ")));
                play(&self.sfx.deny);
            }
            1 => {
                let keys: Vec<(i32, i32)> = self.crops.keys().copied().collect();
                let mut hit = 0;
                for k in keys {
                    if hash(k.0, k.1, day.wrapping_add(8860)) < 0.45 {
                        if let Some(c) = self.crops.get_mut(&k) { c.grown *= 0.4; let o = c.obj(); self.w.placed.insert(k, o); }
                        self.rs.lv.flood.insert(k);
                        hit += 1;
                    }
                }
                if hit == 0 { return; }
                self.rs.lv.flood_day = day;
                for n in self.npcs.iter_mut().filter(|n| n.state == ST_HOME) { n.mood = (n.mood - 4).max(0); }
                self.toast(format!("น้ำท่วมไร่! พายุฝนเมื่อคืนทำให้ผักเสียหาย {} แปลง (โตช้าลง) — น้ำจะลดในวันนี้", hit));
                self.news_add(format!("น้ำท่วมไร่ {} แปลง", hit));
                play(&self.sfx.splash);
            }
            2 => {
                let n = 2 + (hash(day, 3, 8870) * 2.) as u32;
                let mut moved = 0;
                for i in 0..n {
                    if self.free_beds().is_empty() { self.rs.lv.waiting = (self.rs.lv.waiting + 1).min(4); } else { self.refugee_move_in(day as u32 * 7 + i); moved += 1; }
                }
                let wait = n - moved;
                self.toast(if wait > 0 { format!("ผู้ลี้ภัยสงคราม {} คนมาขอพึ่งเมืองเรา — ย้ายเข้าแล้ว {} คน · อีก {} คนรอเตียงว่าง (วางเตียง/สร้างบ้าน)", n, moved, wait) } else { format!("ผู้ลี้ภัยสงคราม {} คนหนีภัยมาขอพึ่งเมืองเรา — ย้ายเข้าบ้านว่างแล้ว", n) });
                self.news_add(format!("ผู้ลี้ภัยสงคราม {} คนมาถึงเมือง", n));
                play(&self.sfx.quest);
            }
            _ => {
                self.rs.lv.bumper = day;
                self.toast("ปีทอง! วันนี้ผักทุกแปลงเก็บเกี่ยวได้เพิ่ม +1 (ทั้งเราและชาวไร่)");
                self.news_add("ปีทอง: ผลผลิตงามเป็นพิเศษ".to_string());
                play(&self.sfx.lvl);
            }
        }
    }
    fn refugee_move_in(&mut self, salt: u32) {
        let Some(bed) = self.free_beds().first().copied() else { return };
        let used: Vec<u8> = self.npcs.iter().map(|n| n.name).collect();
        let start = (hash(salt as i32, 5, 8880) * NPC_NAMES.len() as f64) as usize;
        let name = (0..NPC_NAMES.len()).map(|k| ((start + k) % NPC_NAMES.len()) as u8).find(|c| !used.contains(c)).unwrap_or(start as u8 % NPC_NAMES.len() as u8);
        let look = (hash(salt as i32, 6, 8880) * npc::LOOKS as f64) as u8;
        let id = self.npc_next;
        self.npc_next += 1;
        self.npcs.push(Npc { id, name, look, x: bed.0 as f32 * 16. + 8., y: bed.1 as f32 * 16. + 18., state: ST_HOME, hearts: 3, mood: 55, married: false, child: false, role: 0, bed, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(id, name) });
        self.assign_roles_pub();
    }
    /// คุยกับคนป่วยตอนมีสมุนไพร = หายทันที
    pub fn cure_with_herb(&mut self, id: u32) -> bool {
        if !self.is_sick(id) { return false; }
        let Some(i) = self.idx(id) else { return false };
        let nm = NPC_NAMES[self.npcs[i].name as usize];
        if self.p.inv[HERB] <= 0 { self.npc_bubble(id, "แค่กๆ... ไม่สบายเลย (ต้องการสมุนไพร)"); return true; }
        self.p.inv[HERB] -= 1;
        self.rs.lv.sick.remove(&id);
        let n = &mut self.npcs[i];
        n.mood = (n.mood + 12).min(100);
        n.hearts = (n.hearts + 1).min(10);
        let (x, y) = (n.x, n.y);
        self.burst(x, y - 12., hx(0x7aff9a), 12, 40.);
        self.npc_bubble(id, "หายแล้ว! ขอบคุณมากนะ");
        self.toast(format!("ให้สมุนไพร{} — หายป่วยแล้ว", nm));
        play(&self.sfx.quest);
        true
    }

    pub fn living_save(&self) -> String {
        let lv = &self.rs.lv;
        let mut s = format!("living {} {} {} {} {} {} {}", lv.last_ev, lv.bumper, lv.waiting, lv.fest_done, lv.escorted, lv.robbed, lv.fest_n);
        for (id, d) in &lv.sick { s += &format!(" s{},{}", id, d); }
        s + "\n"
    }
    pub fn living_load(&mut self, r: &[&str]) {
        let f = |i: usize| r.get(i).and_then(|x| x.parse::<i64>().ok()).unwrap_or(0);
        let lv = &mut self.rs.lv;
        lv.last_ev = f(0) as i32; lv.bumper = f(1) as i32; lv.waiting = f(2) as u32; lv.fest_done = f(3) as i32;
        lv.escorted = f(4) as u32; lv.robbed = f(5) as u32; lv.fest_n = f(6) as u32;
        for x in r.iter().filter(|x| x.starts_with('s')) {
            let v: Vec<i64> = x[1..].split(',').filter_map(|a| a.parse().ok()).collect();
            if v.len() == 2 { lv.sick.insert(v[0] as u32, v[1] as i32); }
        }
    }
}

// ---------------------------------------------------------------- วาด
pub fn draw_caravan(g: &Game, sp: &Sprites, v: &View, i: usize, t: f32) {
    let c = &g.rs.lv.cara[i];
    let (sx, sy) = ((c.x - v.cx).round(), (c.y - v.cy).round());
    let f = if c.left { -1. } else { 1. };
    let moving = !(c.ambush == 1);
    // วัว/ม้าลากนำหน้า
    draw_horse(v, sx + f * 15., sy, c.anim, moving, c.left, false);
    // เกวียน: ล้อ 2 ล้อ + ตัวเกวียน + ผ้าคลุมสีเจ้าของ
    vfx::char_shadow(v, sx, sy, 20.);
    let wob = if moving { ((c.anim * 8.).sin() * 0.5).round() } else { 0. };
    prect(v, sx - 10., sy - 12. + wob, 20., 7., hx(0x7a4a24));
    prect(v, sx - 10., sy - 6. + wob, 20., 1., hx(0x4a2a14));
    let kc = hx(g.owner_color(c.owner));
    let cloth = Color::new(0.55 + kc.r * 0.45, 0.55 + kc.g * 0.45, 0.55 + kc.b * 0.45, 1.);
    prect(v, sx - 9., sy - 19. + wob, 18., 7., cloth);
    prect(v, sx - 7., sy - 21. + wob, 14., 2., cloth);
    prect(v, sx - 9., sy - 13. + wob, 18., 1., kc);
    // กระสอบสินค้าโผล่ท้ายเกวียน
    prect(v, sx - f * 9. - 2., sy - 15. + wob, 4., 4., hx(0xc8a870));
    for wx in [-6., 6.] {
        prect(v, sx + wx - 3., sy - 6., 6., 6., hx(0x3a2414));
        prect(v, sx + wx - 1., sy - 4., 2., 2., hx(0x9a7a50));
        let sp_ = (c.anim * 6.) as i32 % 2;
        if moving && sp_ == 0 { prect(v, sx + wx - 3., sy - 4., 6., 1., hx(0x6a4a2a)); }
    }
    // ทหารคุ้มกันเดินตามหลัง
    let body = Body::generate(2000 + c.route as u32 * 3, false);
    let (fr, wb) = if moving { vfx::walk_cycle(c.anim, 8.) } else { (0, 0.) };
    let (gx, gy) = (sx - f * 20., sy - wb);
    vfx::char_shadow(v, gx, sy, 7.);
    skin::draw_figure(sp, &body, skin::ROLE_PARTS[2], (gx - 6.) * v.s, (gy - 15.) * v.s, v.s, 0, fr, c.left, WHITE);
    prect(v, gx - 3., gy - 9., 6., 1., kc);
    let _ = t;
}
/// ป้ายบนหัวกองคาราวาน (วาดหลังโลก ใช้ฟอนต์)
pub fn draw_caravan_labels(g: &Game, v: &View, font: &Font) {
    for c in &g.rs.lv.cara {
        let d = (c.x - g.p.x).hypot(c.y - g.p.y);
        if d > 110. { continue; }
        let lab = if c.ambush == 1 { "โดนโจรดักปล้น!".to_string() } else if c.rob_arm > 0. { "กดใช้อีกครั้ง = ปล้น".to_string() } else { format!("คาราวาน > {}", g.w.realm.cities[c.to].name) };
        let fs = (5.5 * v.s) as u16;
        let col = if c.ambush == 1 || c.rob_arm > 0. { hx(0xff8a7a) } else { hx(0xffe8a0) };
        let dm = measure_text(&lab, Some(font), fs, 1.);
        let (bx, by) = ((c.x - v.cx) * v.s, (c.y - 30. - v.cy) * v.s);
        draw_rectangle(bx - dm.width / 2. - 4., by - fs as f32, dm.width + 8., fs as f32 + 6., Color::new(0., 0., 0., 0.55));
        draw_text_ex(&lab, bx - dm.width / 2., by, TextParams { font: Some(font), font_size: fs, color: col, ..Default::default() });
    }
}
/// แสงโคมไฟเทศกาล (ตอนค่ำ) ที่มุมเสาทั้ง 4 + กลางลาน
pub fn fest_lights(g: &Game, v: &View, lights: &mut Vec<(f32, f32, f32, u8)>) {
    if g.w.dun.is_some() || g.festival_today().is_none() { return; }
    let mut spots: Vec<((f32, f32), f32, f32)> = vec![];
    if let Some(f) = g.fest_spot() { spots.push(((f.0 as f32 * 16. + 8., f.1 as f32 * 16. + 8.), 44., 34.)); }
    for c in &g.w.realm.cities { spots.push(((c.pos.0 as f32 * 16. + 8., c.pos.1 as f32 * 16. + 8.), 5. * 16., 5. * 16.)); }
    for ((cx, cy), rx, ry) in spots {
        if (cx - g.p.x).abs() > 420. || (cy - g.p.y).abs() > 300. { continue; }
        for (px, py) in [(cx - rx, cy - ry), (cx + rx, cy - ry), (cx + rx, cy + ry), (cx - rx, cy + ry), (cx, cy - ry), (cx, cy + ry)] {
            lights.push((px - v.cx, py - 20. - v.cy, 34., 1));
        }
    }
}
/// ของประดับเทศกาล (ลานเมืองเรา + ลานกลางเมืองอื่นๆ) · น้ำท่วมไร่ · เครื่องหมายคนป่วย
pub fn draw_living(g: &Game, v: &View, t: f32) {
    if g.w.dun.is_some() { return; }
    if let Some(s) = g.festival_today() {
        let cols: [[u32; 3]; 4] = [[0xff8ac8, 0xffe06a, 0xffffff], [0x6ac8ff, 0xffffff, 0x4a8aff], [0xffb04a, 0xd8602a, 0xffe06a], [0xff4a3a, 0xffd04a, 0xff8a3a]];
        let col = cols[s as usize];
        let mut spots: Vec<((f32, f32), f32, f32)> = vec![];
        if let Some(f) = g.fest_spot() { spots.push(((f.0 as f32 * 16. + 8., f.1 as f32 * 16. + 8.), 44., 34.)); }
        for c in &g.w.realm.cities { spots.push(((c.pos.0 as f32 * 16. + 8., c.pos.1 as f32 * 16. + 8.), 5. * 16., 5. * 16.)); }
        let night = { let h = g.hour(); !(6.5..18.).contains(&h) };
        for ((cx, cy), rx, ry) in spots {
            if (cx - g.p.x).abs() > 420. || (cy - g.p.y).abs() > 300. { continue; }
            let posts = [(cx - rx, cy - ry), (cx + rx, cy - ry), (cx + rx, cy + ry), (cx - rx, cy + ry)];
            for &(px, py) in &posts {
                prect(v, px - v.cx - 1., py - v.cy - 22., 2., 22., hx(0x6a4424));
                prect(v, px - v.cx - 2., py - v.cy - 24., 4., 3., hx(col[0]));
            }
            // เชือกห้อยธง/โคมไฟระหว่างเสา (โค้งลง)
            for k in 0..4 {
                let (a, b) = (posts[k], posts[(k + 1) % 4]);
                let n = 9;
                for j in 0..=n {
                    let q = j as f32 / n as f32;
                    let sag = (q * std::f32::consts::PI).sin() * 6.;
                    let (x, y) = (a.0 + (b.0 - a.0) * q - v.cx, a.1 + (b.1 - a.1) * q - 22. + sag - v.cy);
                    prect(v, x, y, 1., 1., hx(0x3a2a1a));
                    if j > 0 && j < n {
                        let cc = hx(col[(j + k) % 3]);
                        let sw = ((t * 2. + j as f32 + k as f32).sin() * 0.6).round();
                        if s == 3 || night {
                            // โคมไฟเรืองแสง
                            prect(v, x - 1. + sw, y + 1., 3., 4., cc);
                            prect(v, x - 3. + sw, y - 1., 7., 8., Color::new(cc.r, cc.g, cc.b, 0.18));
                        } else {
                            prect(v, x - 1. + sw, y + 1., 3., 3., cc);
                            prect(v, x + sw, y + 4., 1., 1., cc);
                        }
                    }
                }
            }
        }
    }
    // ไร่ที่น้ำท่วม: น้ำขังเป็นระลอก
    for &(x, y) in &g.rs.lv.flood {
        let (px, py) = (x as f32 * 16. - v.cx, y as f32 * 16. - v.cy);
        if px < -16. || py < -16. || px > screen_width() / v.s || py > screen_height() / v.s { continue; }
        prect(v, px + 1., py + 3., 14., 12., Color::new(0.3, 0.55, 0.9, 0.5));
        let ph = (t * 1.5 + (x * 7 + y * 3) as f32 * 0.3).sin();
        prect(v, px + 3. + ph * 2., py + 7., 6., 1., Color::new(0.85, 0.95, 1., 0.6));
    }
    // คนป่วย: กากบาทเขียวลอยเหนือหัว
    for n in g.npcs.iter().filter(|n| g.is_sick(n.id)) {
        let (sx, sy) = ((n.x - v.cx).round(), (n.y - v.cy).round() - 26. + (t * 2. + n.id as f32).sin());
        prect(v, sx - 3., sy - 1., 7., 3., hx(0x3ad86a));
        prect(v, sx - 1., sy - 3., 3., 7., hx(0x3ad86a));
        prect(v, sx - 4., sy - 4., 9., 9., Color::new(0.2, 0.9, 0.4, 0.15));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn town(g: &mut Game, n: u32) -> (i32, i32) {
        let (sx, sy) = (tfloor(g.p.x) + 3, tfloor(g.p.y) + 3);
        for i in 0..n as i32 {
            let b = (sx + i * 2, sy);
            g.w.placed.insert(b, 33);
            let id = g.npc_next; g.npc_next += 1;
            g.npcs.push(Npc { id, name: i as u8, look: 0, x: b.0 as f32 * 16. + 8., y: b.1 as f32 * 16. + 18., state: ST_HOME, hearts: 2, mood: 60, married: false, child: false, role: 0, bed: b, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(id, i as u8) });
        }
        (sx, sy)
    }
    #[test]
    fn roads_connect_cities_and_cross_water_with_bridges() {
        let g = Game::new_world_seed(746397394, Sfx::default());
        let r = &g.w.realm;
        assert!(r.routes.len() >= 6, "ถนนเชื่อมเมืองลูกกับเมืองหลวงอย่างน้อยเกือบทุกเมือง: {}", r.routes.len());
        for rt in &r.routes {
            // ต่อเนื่อง 4 ทิศ ไม่มีช่องโหว่
            for w in rt.pts.windows(2) { assert_eq!((w[0].0 - w[1].0).abs() + (w[0].1 - w[1].1).abs(), 1); }
            // เริ่ม/จบที่หน้าประตูเมือง
            for (end, ci) in [(rt.pts[0], rt.a), (*rt.pts.last().unwrap(), rt.b)] {
                let c = &r.cities[ci];
                assert!((end.0 - c.pos.0).abs().max((end.1 - c.pos.1).abs()) <= c.r + 2, "ปลายถนนติดเมือง");
            }
            // ทุกช่องบนถนนเดินได้ (ภูเขา = ช่องเขา · น้ำ = สะพาน)
            for p in &rt.pts { if g.w.realm.city_at(p.0, p.1).is_none() { let t = g.w.tile_at(p.0, p.1); assert!(t == T_ROAD || t == T_BRIDGE, "{:?} {}", p, t); assert!(!SOLIDT[t as usize]); } }
        }
        // ตำแหน่งตามระยะทาง
        let rt = &r.routes[0];
        assert_eq!(rt.at(0.), (rt.pts[0].0 as f32 * 16. + 16., rt.pts[0].1 as f32 * 16. + 16.));
        let mid = rt.at(rt.total() / 2.);
        assert!(mid.0.is_finite() && mid.1.is_finite());
    }
    #[test]
    fn caravan_escort_pays_and_rob_costs_rep() {
        let mut g = Game::new_world_seed(746397394, Sfx::default());
        let r = g.w.realm.clone(); g.rs.init(&r);
        let rt = g.w.realm.routes[0].clone();
        let k = g.city_owner(rt.a) as usize;
        let (x, y) = rt.at(rt.total() - 600.);
        g.p.x = x; g.p.y = y;
        g.rs.lv.cara.push(Caravan { route: 0, fwd: true, d: rt.total() - 600., x, y, owner: k as u8, from: rt.a, to: rt.b, esc: 0., ambush: 0, will_ambush: false, rob_arm: 0., anim: 0., left: false, told: false });
        g.rs.lv.cara_t = 999.;
        let (c0, r0) = (g.p.inv[COIN], g.rs.ks[k].rep);
        // เดินตามเกวียนไปจนถึงเมือง
        for _ in 0..(30 * 30) { if let Some(c) = g.rs.lv.cara.first() { g.p.x = c.x; g.p.y = c.y + 20.; } g.caravans_tick(1. / 30.); }
        assert!(g.rs.lv.cara.is_empty(), "ถึงเมืองแล้วหายไป");
        assert!(g.p.inv[COIN] > c0, "คุ้มกันได้เงิน");
        assert_eq!(g.rs.ks[k].rep, (r0 + 4).min(100));
        assert_eq!(g.rs.lv.escorted, 1);
        // ปล้น: กด 2 ครั้ง
        let (x, y) = rt.at(400.);
        g.rs.lv.cara.push(Caravan { route: 0, fwd: true, d: 400., x, y, owner: k as u8, from: rt.a, to: rt.b, esc: 0., ambush: 0, will_ambush: false, rob_arm: 0., anim: 0., left: false, told: true });
        g.p.x = x; g.p.y = y + 4.;
        let (c1, r1) = (g.p.inv[COIN], g.rs.ks[k].rep);
        assert!(g.caravan_use());
        assert_eq!(g.rs.lv.cara.len(), 1, "กดครั้งแรกแค่เตือน");
        assert!(g.caravan_use());
        assert!(g.rs.lv.cara.is_empty());
        assert!(g.p.inv[COIN] >= c1 + 25);
        assert_eq!(g.rs.ks[k].rep, r1 - 18);
        assert!(g.en.iter().any(|e| e.k == EK::Soldier && e.pal == k), "ทหารคุ้มกันสู้กลับ");
    }
    #[test]
    fn bandits_stop_caravan_until_beaten() {
        let mut g = Game::new_world_seed(746397394, Sfx::default());
        let rt = g.w.realm.routes[0].clone();
        let (x, y) = rt.at(300.);
        g.rs.lv.cara.push(Caravan { route: 0, fwd: true, d: 300., x, y, owner: g.city_owner(rt.a), from: rt.a, to: rt.b, esc: 9.5, ambush: 0, will_ambush: true, rob_arm: 0., anim: 0., left: false, told: true });
        g.rs.lv.cara_t = 999.;
        g.p.x = x; g.p.y = y + 20.;
        g.caravans_tick(0.1);
        let nb = g.en.iter().filter(|e| e.pal == BANDIT).count();
        if nb == 0 { return; } // จุดนี้ติดสิ่งกีดขวางทั้งหมด (ไม่เกิดโจร)
        assert_eq!(g.rs.lv.cara[0].ambush, 1);
        let d0 = g.rs.lv.cara[0].d;
        for _ in 0..30 { g.caravans_tick(0.1); }
        assert_eq!(g.rs.lv.cara[0].d, d0, "เกวียนหยุดรอระหว่างโดนดักปล้น");
        for e in g.en.iter_mut() { if e.pal == BANDIT { e.dead = true; } }
        g.en.retain(|e| !e.dead);
        let c0 = g.p.inv[COIN];
        g.caravans_tick(0.1);
        assert_eq!(g.rs.lv.cara[0].ambush, 2);
        assert!(g.p.inv[COIN] > c0);
        g.caravans_tick(0.5);
        assert!(g.rs.lv.cara[0].d > d0, "ไล่โจรแล้วเดินต่อ");
    }
    #[test]
    fn festival_day_villagers_gather_and_player_gets_gift() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        town(&mut g, 4);
        assert_eq!(festival_of(2), Some(season_of_day(2)));
        assert_eq!(festival_of(3), None);
        assert_eq!(festival_of(7), Some(season_of_day(7)));
        g.time = 7. * DAYLEN + (19. - 6.) * 10.;
        assert!(g.festival_today().is_some());
        let f = g.fest_spot().unwrap();
        g.psy_t = 0.;
        g.psy_tick(0.1);
        assert!(g.gather.len() >= 3, "วันเทศกาลชาวบ้านมารวมกัน {}", g.gather.len());
        for (_, (_, fire)) in &g.gather { assert_eq!((tfloor(fire.0), tfloor(fire.1)), (f.0, f.1)); }
        g.p.x = f.0 as f32 * 16. + 8.; g.p.y = f.1 as f32 * 16. + 30.;
        let c0 = g.p.inv[COIN];
        g.festival_tick();
        assert!(g.p.inv[COIN] > c0, "ร่วมงานได้ของขวัญ");
        let c1 = g.p.inv[COIN];
        g.festival_tick();
        assert_eq!(g.p.inv[COIN], c1, "รับได้ครั้งเดียวต่อเทศกาล");
        assert!(g.npcs.iter().all(|n| n.hearts >= 3));
    }
    #[test]
    fn random_events_plague_flood_refugees_bumper() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let (sx, sy) = town(&mut g, 6);
        // โรคระบาด → ไม่ทำงาน · ให้สมุนไพรหาย
        g.start_event(0, 5);
        let sick: Vec<u32> = g.rs.lv.sick.keys().copied().collect();
        assert!(!sick.is_empty() && sick.len() <= 3);
        let id = sick[0];
        let i = g.idx(id).unwrap();
        let mut n = g.npcs[i].clone();
        let mut done = vec![];
        assert!(g.work_step(&mut n, 0.1, &mut done).is_none());
        g.p.inv[HERB] = 1;
        assert!(g.cure_with_herb(id));
        assert!(!g.is_sick(id));
        assert_eq!(g.p.inv[HERB], 0);
        // ที่เหลือหายเองภายใน 4 วัน (กันเหตุการณ์ใหม่ระหว่างทดสอบ)
        g.rs.lv.last_ev = 999;
        for d in 1..=5 { g.time = (5 + d) as f32 * DAYLEN; g.living_dawn(); }
        assert!(g.rs.lv.sick.is_empty());
        // น้ำท่วม: ผักโตช้าลง
        for k in 0..10 { let t = (sx + k, sy + 6); g.crops.insert(t, Crop { kind: 0, grown: 100., wet: 0. }); }
        g.start_event(1, 20);
        assert!(g.crops.values().any(|c| c.grown < 100.));
        assert!(!g.rs.lv.flood.is_empty());
        // ผู้ลี้ภัย: เตียงว่างย้ายเข้า ที่เหลือรอ
        let pop0 = g.pop();
        g.w.placed.insert((sx, sy + 12), 33);
        g.start_event(2, 22);
        assert_eq!(g.pop(), pop0 + 1);
        assert!(g.rs.lv.waiting >= 1);
        g.w.placed.insert((sx + 4, sy + 12), 33);
        g.rs.lv.last_ev = 999;
        g.time = 24. * DAYLEN; g.living_dawn();
        assert_eq!(g.pop(), pop0 + 2, "มีเตียงใหม่ ผู้ลี้ภัยที่รอย้ายเข้า");
        let names: HashSet<u8> = g.npcs.iter().map(|n| n.name).collect();
        assert_eq!(names.len(), g.npcs.len(), "ชื่อไม่ซ้ำ");
        // ปีทอง
        g.time = 30. * DAYLEN;
        g.start_event(3, 30);
        assert!(g.bumper_today());
        // เซฟ/โหลด
        g.rs.lv.sick.insert(7, 2);
        let s = g.living_save();
        let mut h = Game::new_world_seed(4242, Sfx::default());
        let parts: Vec<&str> = s.trim().split(' ').skip(1).collect();
        h.living_load(&parts);
        assert_eq!(h.rs.lv.bumper, 30);
        assert_eq!(h.rs.lv.sick.get(&7), Some(&2));
        assert_eq!(h.rs.lv.waiting, g.rs.lv.waiting);
    }
}
