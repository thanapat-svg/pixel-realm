// needs.rs — ความหิว / กระหายน้ำ / ความเหนื่อย ของผู้เล่น (v7b) ตามตรรกะชีวิตจริง
//  - น้ำหมดเร็วกว่าอาหาร (1.5 วัน vs 3 วัน) · นอนหลับฟื้นแรง · ทะเลทรายร้อนกระหายเร็ว · หิมะหนาวหิวเร็ว (ผิงไฟช่วยได้)
//  - ของดิบ (ปลา/เนื้อ/เห็ด) เสี่ยงท้องเสีย · น้ำทะเลเค็ม ดื่มแล้วยิ่งกระหาย · น้ำจากบ่อ/ฝนสะอาด
//  - หมดแรง/หิว/กระหาย → เดินช้าลง ไม่ฟื้นเลือด · หมดเกลี้ยง → เลือดลดทีละน้อย (โหมดปกติเหลือ 1 · โหมดยากหมดสติได้)
use super::*;

pub const HUNGER_RATE: f32 = 100. / (3. * DAYLEN);
pub const THIRST_RATE: f32 = 100. / (1.5 * DAYLEN);
pub const REST_RATE: f32 = 100. / (4. * DAYLEN);
pub const NEED_LABEL: [&str; 3] = ["อิ่ม", "น้ำ", "แรง"];

/// (ความอิ่มที่ได้, โอกาสท้องเสีย) ของแต่ละอย่าง
pub fn nutrition(item: usize) -> Option<(f32, f32)> {
    Some(match item {
        BERRY => (12., 0.),
        CARROT => (18., 0.),
        CORN => (22., 0.),
        FISH => (25., 0.20),
        MEAT => (30., 0.30),
        SHROOM => (10., 0.25),
        HERB => (5., 0.),
        DISH_STEW => (60., 0.),
        DISH_FISH => (45., 0.),
        DISH_SPEED | DISH_CRIT => (35., 0.),
        _ => return None,
    })
}

impl Game {
    pub fn survival_mode(&self) -> u8 { prefs::survival() }

    /// (ตัวคูณความกระหาย, ตัวคูณความหิว) ตามภูมิอากาศใต้เท้า
    pub fn climate(&self) -> (f32, f32) {
        if self.w.dun.is_some() { return (1., 1.); }
        let t = self.w.tile_at(tfloor(self.p.x), tfloor(self.p.y));
        let fire = self.w.near_fire(tfloor(self.p.x), tfloor(self.p.y), 3).is_some();
        match t { 8 => (2.2, 1.0), 7 => (1.0, if fire { 1.0 } else { 1.5 }), T_SAVANNA => (1.6, 1.0), T_ASH => (2.6, 1.0), T_SWAMP => (1.0, 1.1), _ => (1., 1.) }
    }
    /// ตัวคูณความเร็วเดินจากสภาพร่างกาย
    pub fn need_speed_mul(&self) -> f32 {
        self.need_speed_mul_mode(self.survival_mode())
    }
    pub fn need_speed_mul_mode(&self, mode: u8) -> f32 {
        if mode == 0 { return 1.; }
        let m = self.p.hunger.min(self.p.thirst).min(self.p.rest);
        let b = if m <= 0. { 0.7 } else if m < 15. { 0.85 } else if m < 30. { 0.95 } else { 1. };
        b * if self.p.sick > 0. { 0.9 } else { 1. }
    }
    /// ฟื้นเลือดตามธรรมชาติได้ไหม (หิว/กระหาย/ป่วย = ไม่ฟื้น)
    pub fn can_regen(&self) -> bool {
        self.survival_mode() == 0 || (self.p.hunger >= 20. && self.p.thirst >= 20. && self.p.sick <= 0.)
    }
    pub fn feed(&mut self, amt: f32) { self.p.hunger = (self.p.hunger + amt).clamp(0., 100.); }
    pub fn hydrate(&mut self, amt: f32) { self.p.thirst = (self.p.thirst + amt).clamp(0., 100.); }

    pub fn needs_update(&mut self, dt: f32) {
        let mode = self.survival_mode();
        self.needs_tick(dt, mode);
    }
    pub fn needs_tick(&mut self, dt: f32, mode: u8) {
        if self.p.sick > 0. { self.p.sick = (self.p.sick - dt).max(0.); }
        if mode == 0 { self.p.hunger = 100.; self.p.thirst = 100.; self.p.rest = 100.; self.p.starve = 0.; return; }
        let (tm, hm) = self.climate();
        let act = if self.p.moving { 1.25 } else { 1. } * if self.p.atk_t > 0. { 1.2 } else { 1. };
        let (h0, t0, r0) = (self.p.hunger, self.p.thirst, self.p.rest);
        let sickm = if self.p.sick > 0. { 1.5 } else { 1. };
        self.p.hunger = (h0 - HUNGER_RATE * hm * self.p.body.hunger_mul() * act * sickm * dt).max(0.);
        self.p.thirst = (t0 - THIRST_RATE * tm * act * dt).max(0.);
        self.p.rest = (r0 - REST_RATE * dt).max(0.);
        // ฝนตก (กลางแจ้ง) ช่วยให้ได้น้ำดื่ม
        if self.rain_on && self.rain > 0.4 && self.w.dun.is_none() { self.hydrate(1.2 * dt); }
        // แจ้งเตือนตอนข้ามเกณฑ์ (ครั้งเดียวต่อรอบ)
        let (h1, t1, r1) = (self.p.hunger, self.p.thirst, self.p.rest);
        if t0 >= 30. && t1 < 30. { self.toast("เริ่มกระหายน้ำ — หาน้ำดื่ม (เข้าใกล้ริมน้ำแล้วกดใช้)"); }
        else if t0 >= 10. && t1 < 10. { self.toast("กระหายน้ำมาก! เดินช้าลงและไม่ฟื้นเลือด"); }
        if h0 >= 30. && h1 < 30. { self.toast("เริ่มหิว — หาอะไรกิน (ผลไม้ พืชผล ปลา เนื้อ)"); }
        else if h0 >= 10. && h1 < 10. { self.toast("หิวมาก! เดินช้าลงและไม่ฟื้นเลือด"); }
        if r0 >= 25. && r1 < 25. { self.toast("เริ่มเพลีย — กลับไปนอนที่เตียงตอนกลางคืน"); }
        // หมดเกลี้ยง → เลือดลด
        let starving = h1 <= 0. || t1 <= 0.;
        if starving {
            self.p.starve += dt * if h1 <= 0. && t1 <= 0. { 2. } else { 1. };
            if self.p.starve >= 8. {
                self.p.starve = 0.;
                let why = if t1 <= 0. { "กระหายน้ำ" } else { "หิวโหย" };
                if self.p.hp > 1 {
                    self.p.hp -= 1;
                    let (x, y) = (self.p.x, self.p.y - 20.);
                    self.add_float(x, y, format!("-1 {}", why), hx(0xcc8844), None);
                    play(&self.sfx.hurt);
                } else if mode >= 2 {
                    self.faint(&format!("หมดสติเพราะ{}...", why));
                }
            }
        } else {
            self.p.starve = 0.;
        }
    }

    /// กินของ (ดิบ/ผลไม้/สมุนไพร/อาหาร) — คืน false ถ้ากินไม่ได้
    pub fn eat_generic(&mut self, item: usize) -> bool {
        let roll = gen_range(0., 1.);
        self.eat_generic_roll(item, roll)
    }
    pub fn eat_generic_roll(&mut self, item: usize, roll: f32) -> bool {
        let Some((food, risk)) = nutrition(item) else { return false };
        if self.p.inv[item] <= 0 { return false; }
        if item == HERB {
            // สมุนไพร: แก้ท้องเสีย + ฟื้นเลือดนิดหน่อย
            if self.p.sick <= 0. && self.p.hp >= self.p.maxhp && self.p.hunger >= 95. { self.toast("ตอนนี้ยังไม่จำเป็นต้องใช้สมุนไพร"); return false; }
            self.p.inv[HERB] -= 1;
            let cured = self.p.sick > 0.;
            self.p.sick = 0.;
            self.p.hp = (self.p.hp + 1).min(self.p.maxhp);
            self.feed(food);
            self.toast(if cured { "กินสมุนไพร — อาการท้องเสียหายแล้ว" } else { "กินสมุนไพร" });
            play(&self.sfx.eat);
            return true;
        }
        if self.p.hunger >= 95. && self.p.hp >= self.p.maxhp { self.toast("อิ่มแล้ว ไม่หิว"); return false; }
        self.p.inv[item] -= 1;
        self.feed(food);
        let hp_gain = if item == BERRY { 3 } else { 0 };
        if hp_gain > 0 { self.p.hp = (self.p.hp + hp_gain).min(self.p.maxhp); }
        let (x, y) = (self.p.x, self.p.y - 18.);
        self.add_float(x, y, format!("อิ่ม +{}", food as i32), hx(0xffbb66), None);
        play(&self.sfx.eat);
        if risk > 0. && roll < risk {
            self.p.sick = 60.;
            self.p.hunger = (self.p.hunger - food * 0.5).max(0.); // อาเจียนบางส่วน
            if self.p.hp > 1 { self.p.hp -= 1; }
            self.toast(if item == SHROOM { "เห็ดมีพิษอ่อน ๆ — ท้องเสีย! (กินสมุนไพรหรือนอนพักจะหาย)" } else { "ของดิบ ท้องเสีย! (ปรุงให้สุกก่อนจะปลอดภัย · กินสมุนไพรหรือนอนพักจะหาย)" });
        }
        true
    }

    /// ดื่มน้ำจากไทล์ตรงหน้า (น้ำจืด = ดื่มได้ · ทะเลลึก = เค็ม) — คืน true ถ้าจัดการแล้ว
    pub fn drink_ahead(&mut self) -> bool {
        let roll = gen_range(0., 1.);
        self.drink_ahead_roll(roll)
    }
    pub fn drink_ahead_roll(&mut self, roll: f32) -> bool {
        if self.survival_mode() == 0 { return false; }
        let Some((tx, ty)) = self.water_ahead() else { return false };
        if self.p.thirst >= 90. { return false; } // ไม่กระหายแล้ว → ปล่อยให้ตกปลาแทน
        let (x, y) = (self.p.x, self.p.y - 18.);
        if self.is_salt(tx, ty) {
            self.p.thirst = (self.p.thirst - 6.).max(0.);
            self.toast("น้ำทะเลเค็ม... ดื่มแล้วยิ่งกระหายน้ำ (หาน้ำจืดจากแหล่งน้ำ/บ่อ/ฝนแทน)");
            return true;
        }
        self.hydrate(30.);
        self.add_float(x, y, "ดื่มน้ำ +30", hx(0x66ccff), None);
        play(&self.sfx.pickup);
        if roll < 0.10 && self.p.sick <= 0. {
            self.p.sick = 45.;
            self.toast("น้ำจากธรรมชาติไม่สะอาด — ท้องเสีย! (บ่อน้ำที่สร้างเองสะอาดกว่า)");
        }
        true
    }
    /// น้ำทะเลเค็ม: ทะเลลึก หรือน้ำตื้นที่ติดทะเลลึกภายใน 3 ช่อง · น้ำตื้นโดดๆ (บึง/ทะเลสาบ) เป็นน้ำจืด
    pub fn is_salt(&self, tx: i32, ty: i32) -> bool {
        match self.w.tile_at(tx, ty) {
            0 => true,
            1 => { for dy in -3..=3 { for dx in -3..=3 { if self.w.tile_at(tx + dx, ty + dy) == 0 { return true; } } } false }
            _ => false,
        }
    }
    pub fn drink_well(&mut self) {
        if self.survival_mode() == 0 { self.toast("น้ำในบ่อใสสะอาด"); return; }
        if self.p.thirst >= 98. { self.toast("ไม่กระหายแล้ว"); return; }
        self.hydrate(45.);
        let (x, y) = (self.p.x, self.p.y - 18.);
        self.add_float(x, y, "ดื่มน้ำ +45", hx(0x66ccff), None);
        play(&self.sfx.pickup);
    }
    /// หลับข้ามคืน: ใช้ทรัพยากรครึ่งอัตรา ฟื้นแรงเต็ม หายท้องเสีย (ไม่ตายตอนหลับ)
    pub fn sleep_needs(&mut self, elapsed: f32) {
        if self.survival_mode() == 0 { return; }
        self.p.hunger = (self.p.hunger - HUNGER_RATE * 0.5 * elapsed).max(3.);
        self.p.thirst = (self.p.thirst - THIRST_RATE * 0.6 * elapsed).max(3.);
        self.p.rest = 100.;
        self.p.sick = 0.;
        self.p.starve = 0.;
    }
    pub fn need_status(&self) -> Option<&'static str> {
        if self.survival_mode() == 0 { return None; }
        if self.p.thirst < 10. { Some("กระหายน้ำมาก") } else if self.p.hunger < 10. { Some("หิวมาก") }
        else if self.p.sick > 0. { Some("ท้องเสีย") } else if self.p.thirst < 30. { Some("กระหายน้ำ") }
        else if self.p.hunger < 30. { Some("หิว") } else if self.p.rest < 25. { Some("เพลีย") } else { None }
    }

    /// มีแหล่งน้ำจืดหรือบ่อน้ำภายใน r ช่องจากจุดนี้ไหม (ชาวบ้านใช้ตอนรุ่งเช้า)
    pub fn water_within(&self, tx: i32, ty: i32, r: i32) -> bool {
        if self.w.placed.iter().any(|(k, &o)| o == 39 && (k.0 - tx).abs() <= r && (k.1 - ty).abs() <= r) { return true; }
        let mut y = ty - r;
        while y <= ty + r {
            let mut x = tx - r;
            while x <= tx + r { if self.w.tile_at(x, y) == 1 && !self.is_salt(x, y) { return true; } x += 1; }
            y += 1;
        }
        false
    }
}

pub fn draw_need_bars(g: &Game, font: &Font, x: f32, y: f32, bw: f32, u: f32) -> f32 {
    if g.survival_mode() == 0 { return 0.; }
    let gap = 4. * u;
    let w = (bw - 2. * gap) / 3.;
    let h = 15. * u;
    let vals = [g.p.hunger, g.p.thirst, g.p.rest];
    let cols = [0xe0912f, 0x3a9be8, 0x9a7bd8];
    let t = get_time() as f32;
    for k in 0..3 {
        let bx = x + k as f32 * (w + gap);
        let v = vals[k];
        draw_rectangle(bx, y, w, h, hx(0x1a1a22));
        let low = v < 15.;
        let col = if low && (t * 4.).sin() > 0. { hx(0xff4444) } else { hx(cols[k]) };
        draw_rectangle(bx, y, w * (v / 100.).clamp(0., 1.), h, col);
        draw_rectangle_lines(bx, y, w, h, 1.5, BLACK);
        text_c(font, NEED_LABEL[k], bx + w / 2., y + h - 3. * u, 10. * u, WHITE);
    }
    if let Some(s) = g.need_status() {
        text(font, s, x, y + h + 11. * u, 10. * u, hx(0xffbb77));
        return h + 14. * u;
    }
    h + 4. * u
}
