// ambience.rs — v15: เสียงบรรยากาศสมจริง + เสียงเดินตามพื้นผิว
//
// เสียงบรรยากาศ 11 ลูป (assets/audio/amb_<ชื่อ>.ogg สร้างโดย tools/sound_hq.py) ผสมพร้อมกันได้หลายเสียง
// ความดังแต่ละลูปคำนวณจากสถานการณ์ทุก 0.25 วินาที แล้วค่อยๆ เฟดเข้าหาเป้า (ไม่กระตุก)
//   ฝน/หิมะ ตามความแรง · อยู่ในบ้าน = ได้ยินฝนตกบนหลังคาแทน และเสียงข้างนอกเบาลง
//   ลม แรงขึ้นบนภูเขา/หิมะ/ทะเลทราย/ชายหาด · กองไฟ/คบเพลิง ดังตามระยะใกล้ที่สุด
//   นกร้องกลางวัน · จิ้งหรีดกลางคืน (ไม่มีในหน้าหนาว) · คลื่นตามจำนวนน้ำรอบตัว · ลาวา · กบในหนองน้ำ · ถ้ำ/ดันเจี้ยน
// ลูปจะถูกโหลดเมื่อจำเป็นครั้งแรก และหยุดเล่นเมื่อเงียบสนิท (ประหยัดแรม/แบต)
use super::*;
use macroquad::audio::{set_sound_volume, stop_sound};

pub const AMB_N: usize = 11;
pub const AMB_NAMES: [&str; AMB_N] = ["rain", "rain_roof", "wind", "blizzard", "fire", "birds", "crickets", "waves", "cave", "lava", "swamp"];
const A_RAIN: usize = 0;
const A_ROOF: usize = 1;
const A_WIND: usize = 2;
const A_BLIZ: usize = 3;
const A_FIRE: usize = 4;
const A_BIRDS: usize = 5;
const A_CRICK: usize = 6;
const A_WAVES: usize = 7;
const A_CAVE: usize = 8;
const A_LAVA: usize = 9;
const A_SWAMP: usize = 10;

/// เสียงเดินสิ่งมีชีวิตอื่น (ลำดับตรงกับ Sfx.mob)
pub const MOBS: [&str; 6] = ["slime", "bone", "flap", "hoof", "paw", "stomp"];
const M_SLIME: usize = 0;
const M_BONE: usize = 1;
const M_FLAP: usize = 2;
const M_HOOF: usize = 3;
const M_PAW: usize = 4;
const M_STOMP: usize = 5;
/// ได้ยินเสียงเดินของสิ่งอื่นในรัศมีนี้ (พิกเซล ~11 ช่อง)
const HEAR_R: f32 = 176.;

/// สถานะเสียงเดินของสิ่งมีชีวิตแต่ละตัว: ตำแหน่งเฟรมก่อน + ระยะสะสม + ตัวจับเวลา (ปีก)
#[derive(Clone, Copy, Default)]
pub struct StepTrack { pub x: f32, pub y: f32, pub acc: f32, pub t: f32 }

/// พื้นผิวเสียงเดิน (ลำดับตรงกับ Sfx.steps)
pub const SURFACES: [&str; 7] = ["grass", "dirt", "sand", "snow", "stone", "wood", "mud"];

/// พื้นผิวจากไทล์
pub fn surface_of(tile: u8) -> usize {
    match tile {
        3 | 4 | 16 => 0,
        11 => 1,
        2 | 8 => 2,
        7 => 3,
        5 | 6 | 9 | 10 | 13 | 14 | 17 | 20 => 4,
        19 => 5,
        12 | 15 => 6,
        _ => 1,
    }
}

impl Game {
    /// ความดังเป้าหมายของแต่ละลูป (0..1)
    pub fn amb_targets(&self) -> [f32; AMB_N] {
        let mut a = [0f32; AMB_N];
        if self.w.dun.is_some() {
            a[A_CAVE] = 0.75;
            return a;
        }
        let (tx, ty) = (tfloor(self.p.x), tfloor(self.p.y - 3.));
        let inside = self.indoors();
        let out = if inside { 0.22 } else { 1.0 };
        let h = self.hour();
        let biome = self.w.biome_at(tx, ty);
        let rain = self.rain.clamp(0., 1.);
        // ฝน / หิมะ
        if self.weather_kind == 1 {
            if inside { a[A_ROOF] = rain * 0.85; a[A_RAIN] = rain * 0.12; } else { a[A_RAIN] = rain * 0.8; }
        } else if self.weather_kind == 2 {
            a[A_BLIZ] = rain * if inside { 0.18 } else { 0.7 };
        }
        // ลม
        let windy = match biome { 6 | 7 => 0.45, 8 | 2 | 5 | 17 => 0.33, 16 => 0.28, _ => 0.18 };
        let night = if !(6.0..19.0).contains(&h) { 0.05 } else { 0. };
        a[A_WIND] = (windy + night + rain * 0.15) * out;
        // สำรวจรอบตัว: น้ำ ลาวา หนองน้ำ
        let (mut water, mut lava, mut swamp, mut cnt) = (0, 0, 0, 0);
        for dy in (-8..=8).step_by(2) {
            for dx in (-8..=8).step_by(2) {
                cnt += 1;
                match self.w.tile_at(tx + dx, ty + dy) { 0 | 1 => water += 1, 18 => lava += 1, 15 => swamp += 1, _ => {} }
            }
        }
        let f = |c: i32| c as f32 / cnt as f32;
        a[A_WAVES] = (f(water) * 2.2).min(1.) * 0.75 * out;
        a[A_LAVA] = (f(lava) * 3.5).min(1.) * 0.8 * out;
        let swamp_k = (f(swamp) * 2.5).min(1.);
        // กองไฟ / คบเพลิง ที่ใกล้ที่สุด (ได้ยินทะลุผนังเบาลง ยกเว้นอยู่ในห้องเดียวกัน)
        let (px, py) = (self.p.x, self.p.y - 3.);
        let mut best = f32::MAX;
        for (k, &o) in self.w.placed.iter() {
            if o != 9 && o != 18 && o != 26 { continue; }
            if (k.0 - tx).abs() > 9 || (k.1 - ty).abs() > 9 { continue; }
            let d = (k.0 as f32 * 16. + 8. - px).hypot(k.1 as f32 * 16. + 8. - py) / 16. * if o == 18 { 2.2 } else { 1. };
            best = best.min(d);
        }
        if best < 8. { a[A_FIRE] = ((1. - best / 8.).powf(1.6) * 0.95).min(0.95); }
        // สัตว์: นกกลางวัน · จิ้งหรีดกลางคืน · กบในหนองน้ำ
        let green = matches!(biome, 3 | 4 | 16 | 15);
        let winter = seasons::season(self) == 3;
        let calm = 1. - rain * 0.85;
        if green && (5.5..18.5).contains(&h) {
            let dawn = if (5.5..8.5).contains(&h) { 1.35 } else { 1. }; // นกร้องหนักตอนเช้า
            a[A_BIRDS] = (if biome == 4 { 0.6 } else { 0.42 }) * dawn * calm * out * if winter { 0.35 } else { 1. };
        }
        if green && !winter && !(5.0..19.0).contains(&h) {
            a[A_CRICK] = 0.5 * calm * if inside { 0.3 } else { 1. };
        }
        if swamp_k > 0. { a[A_SWAMP] = swamp_k * 0.65 * calm * out * if (5.0..19.0).contains(&h) { 0.6 } else { 1. }; }
        a
    }

    /// เสียงเดิน: นับระยะที่เดินจริง ทุก ~18 พิกเซล = 1 ก้าว (พุ่ง = ก้าวยาวขึ้น)
    pub fn step_tick(&mut self, moved: f32) {
        if moved <= 0.01 || self.p.boat && self.on_boat() || self.sit.is_some() { self.step_d = self.step_d.min(9.); return; }
        self.step_d += moved;
        let stride = if self.rs.riding { 28. } else if self.p.dash_t > 0. { 26. } else { 18. };
        if self.step_d < stride { return; }
        self.step_d -= stride;
        if self.rs.riding {
            let bank = &self.sfx.mob[3];
            if !bank.is_empty() { play_step(&bank[gen_range(0, bank.len())], 0.5); }
            return;
        }
        let tile = self.w.tile_at(tfloor(self.p.x), tfloor(self.p.y - 2.));
        let mut s = surface_of(tile);
        // หน้าหนาว ทุ่งหญ้ามีหิมะปกคลุม
        if s == 0 && seasons::season(self) == 3 && self.w.dun.is_none() { s = 3; }
        let bank = &self.sfx.steps[s];
        if bank.is_empty() { return; }
        // สุ่มแบบไม่ซ้ำก้าวก่อน
        let mut i = gen_range(0, bank.len());
        if bank.len() > 1 && i == self.step_last { i = (i + 1) % bank.len(); }
        self.step_last = i;
        let vol = match s { 3 | 6 => 0.42, 5 | 4 => 0.36, _ => 0.32 } * if self.p.dash_t > 0. { 1.25 } else { 1. };
        play_step(&bank[i], vol);
    }
}

impl Game {
    /// เสียงเดินของชาวบ้าน มอนสเตอร์ สัตว์ (รวมสัตว์เลี้ยง) และบอส — ดังตามระยะ · ข้างนอกเบาลงถ้าเราอยู่ในบ้าน
    /// จำกัดไม่เกิน ~4 เสียงต่อ 0.15 วิ กันเสียงตีกันเมื่อมีฝูงมอน
    pub fn world_steps_tick(&mut self, dt: f32) {
        self.step_budget = (self.step_budget + dt * 26.).min(4.);
        let (px, py) = (self.p.x, self.p.y);
        let inside = self.indoors();
        let in_room = |g: &Game, x: f32, y: f32| g.room_of.contains_key(&(tfloor(x), tfloor(y - 3.)));
        let winter = seasons::season(self) == 3 && self.w.dun.is_none();
        // (ประเภท, x, y, ชนิดเสียง (-1 = พื้นผิว), ระยะต่อก้าว, ความดังพื้นฐาน, บิน?)
        let mut list: Vec<(u8, f32, f32, i32, f32, f32, bool)> = vec![];
        for n in &self.npcs { list.push((0, n.x, n.y, -1, 17., 0.55, false)); }
        for e in &self.en {
            if e.dead { list.push((1, f32::NAN, 0., 0, 1., 0., false)); continue; }
            let (m, stride, v, fly) = match e.k { EK::Slime => (M_SLIME, 20., 0.6, false), EK::Skel => (M_BONE, 15., 0.6, false), EK::Bat => (M_FLAP, 0., 0.45, true), EK::Soldier => (M_HOOF, 18., 0.5, false) };
            list.push((1, e.x, e.y, m as i32, stride, v, fly));
        }
        for a in &self.animals {
            if a.dead { list.push((2, f32::NAN, 0., 0, 1., 0., false)); continue; }
            let (m, stride, v, fly) = match a.k { A_BIRD => (M_FLAP, 0., 0.3, true), A_DEER | A_ANTE | A_BOAR => (M_HOOF, 22., 0.5, false), A_CROC => (M_PAW, 22., 0.5, false), _ => (M_PAW, 15., if a.tame { 0.45 } else { 0.4 }, false) };
            list.push((2, a.x, a.y, m as i32, stride, v, fly));
        }
        for b in &self.bosses {
            if b.dead { list.push((3, f32::NAN, 0., 0, 1., 0., false)); continue; }
            list.push((3, b.x, b.y, M_STOMP as i32, 30., 1.0, false));
        }
        let mut tr = std::mem::take(&mut self.step_tracks);
        if tr.len() != list.len() { tr = list.iter().map(|l| StepTrack { x: l.1, y: l.2, acc: 0., t: gen_range(0., 0.2) }).collect(); }
        let mut plays: Vec<(usize, usize, f32)> = vec![]; // (กลุ่มเสียง: 0..6 พื้นผิว / 10+ mob, ดัชนีแบบ, ความดัง)
        for (i, &(_, x, y, m, stride, base, fly)) in list.iter().enumerate() {
            let t = &mut tr[i];
            if x.is_nan() { continue; }
            let moved = (x - t.x).hypot(y - t.y);
            t.x = x;
            t.y = y;
            if moved > 40. { continue; } // วาร์ป/เกิดใหม่
            let d = (x - px).hypot(y - py);
            let trigger = if fly {
                t.t -= dt;
                if t.t <= 0. { t.t = gen_range(0.2, 0.3); true } else { false }
            } else {
                t.acc += moved;
                if t.acc >= stride { t.acc -= stride; true } else { false }
            };
            if !trigger || d > HEAR_R { continue; }
            let mut vol = base * (1. - d / HEAR_R).powf(1.6);
            if inside && !in_room(self, x, y) { vol *= 0.35; }
            if vol < 0.02 { continue; }
            if m < 0 {
                let mut s = surface_of(self.w.tile_at(tfloor(x), tfloor(y - 2.)));
                if s == 0 && winter { s = 3; }
                plays.push((s, 0, vol * 0.85));
            } else {
                plays.push((10 + m as usize, 0, vol));
            }
        }
        self.step_tracks = tr;
        // ดังสุดก่อน แล้วตัดตามโควตา
        plays.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        for (grp, _, vol) in plays {
            if self.step_budget < 1. { break; }
            self.step_budget -= 1.;
            let bank = if grp >= 10 { &self.sfx.mob[grp - 10] } else { &self.sfx.steps[grp] };
            if bank.is_empty() { continue; }
            play_step(&bank[gen_range(0, bank.len())], vol.min(1.));
        }
    }
}

/// ผู้จัดการลูปบรรยากาศ (เรียกจาก main loop ทุกเฟรม — โหลดแบบ async ครั้งแรกที่ต้องใช้)
pub struct AmbMgr {
    snd: [Option<Sound>; AMB_N],
    tried: [bool; AMB_N],
    playing: [bool; AMB_N],
    vol: [f32; AMB_N],
    set: [f32; AMB_N],
}

impl AmbMgr {
    pub fn new() -> Self {
        AmbMgr { snd: Default::default(), tried: [false; AMB_N], playing: [false; AMB_N], vol: [0.; AMB_N], set: [-1.; AMB_N] }
    }
    pub async fn update(&mut self, target: [f32; AMB_N], dt: f32, master: f32) {
        let on = audio::SFX_ON.load(std::sync::atomic::Ordering::Relaxed);
        for i in 0..AMB_N {
            let t = if on { target[i] * master } else { 0. };
            if t > 0.01 && !self.tried[i] {
                self.tried[i] = true;
                self.snd[i] = crate::assets::try_sound(&format!("amb_{}", AMB_NAMES[i])).await;
            }
            // เฟดเข้า ~1.5 วิ ออก ~2.5 วิ
            let rate = if t > self.vol[i] { 0.6 } else { 0.4 };
            self.vol[i] += (t - self.vol[i]).clamp(-rate * dt, rate * dt);
            let Some(s) = &self.snd[i] else { continue };
            if self.vol[i] > 0.004 {
                if !self.playing[i] {
                    play_sound(s, PlaySoundParams { looped: true, volume: self.vol[i] });
                    self.playing[i] = true;
                    self.set[i] = self.vol[i];
                } else if (self.vol[i] - self.set[i]).abs() > 0.004 {
                    set_sound_volume(s, self.vol[i]);
                    self.set[i] = self.vol[i];
                }
            } else if self.playing[i] {
                stop_sound(s);
                self.playing[i] = false;
            }
        }
    }
    pub fn stop_all(&mut self) {
        for i in 0..AMB_N {
            if let (true, Some(s)) = (self.playing[i], &self.snd[i]) { stop_sound(s); }
            self.playing[i] = false;
            self.vol[i] = 0.;
        }
    }
}
