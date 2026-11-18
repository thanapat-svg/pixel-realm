//! เสียงสังเคราะห์สไตล์ 8-bit ด้วยโค้ดล้วน (ไม่มีไฟล์เสียง): เอฟเฟกต์ SFX + เพลงพื้นหลัง BGM แบบ chiptune
//! BGM มี 4 แบบ: 0 สำรวจ · 1 แคมป์ · 2 บอส · 3 ดันเจี้ยน/กลางคืน เปลี่ยนตามสถานการณ์

use super::*;
use macroquad::audio::stop_sound;
use std::f32::consts::TAU;
use std::sync::atomic::{AtomicBool, Ordering};

/// เปิด/ปิดเสียงเอฟเฟกต์ทั้งเกม (ตั้งจากเมนูตั้งค่า)
pub static SFX_ON: AtomicBool = AtomicBool::new(true);
const SR: u32 = 22050;
const BSR: u32 = 16000;
pub const BGM_VOL: f32 = 0.26;
pub const BGM_N: usize = 4;

fn wav_bytes(samples: &[f32], sr: u32) -> Vec<u8> {
    let n = samples.len() as u32;
    let mut d = Vec::with_capacity(44 + n as usize * 2);
    d.extend_from_slice(b"RIFF");
    d.extend_from_slice(&(36 + n * 2).to_le_bytes());
    d.extend_from_slice(b"WAVEfmt ");
    d.extend_from_slice(&16u32.to_le_bytes());
    d.extend_from_slice(&1u16.to_le_bytes());
    d.extend_from_slice(&1u16.to_le_bytes());
    d.extend_from_slice(&sr.to_le_bytes());
    d.extend_from_slice(&(sr * 2).to_le_bytes());
    d.extend_from_slice(&2u16.to_le_bytes());
    d.extend_from_slice(&16u16.to_le_bytes());
    d.extend_from_slice(b"data");
    d.extend_from_slice(&(n * 2).to_le_bytes());
    for s in samples {
        d.extend_from_slice(&((s.clamp(-1., 1.) * 32767.) as i16).to_le_bytes());
    }
    d
}

/// เสียงโน้ตเดี่ยว: kind 0 สี่เหลี่ยม · 1 ฟันเลื่อย · 2 ไซน์ · 3 นอยส์
fn tone(f0: f32, dur: f32, slide: f32, vol: f32, kind: u8) -> Vec<f32> {
    let n = (SR as f32 * dur) as usize;
    let mut out = Vec::with_capacity(n);
    let (mut ph, mut seed) = (0f32, 12345u32);
    for i in 0..n {
        let t = i as f32 / n as f32;
        ph += (f0 + slide * t) / SR as f32;
        let s = match kind {
            0 => if ph.fract() < 0.5 { 1. } else { -1. },
            1 => ph.fract() * 2. - 1.,
            2 => (ph * TAU).sin(),
            _ => {
                seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
                (seed >> 16) as f32 / 32768. - 1.
            }
        };
        out.push(s * (1. - t) * (1. - t) * vol);
    }
    out
}
/// ต่อโน้ตหลายตัวเป็นทำนองสั้นๆ (แฟนแฟร์)
fn seq(notes: &[(f32, f32)], vol: f32, kind: u8) -> Vec<f32> {
    let mut o = vec![];
    for &(f, d) in notes {
        o.extend(tone(f, d, 0., vol, kind));
    }
    o
}

#[derive(Default)]
pub struct Sfx {
    pub swing: Option<Sound>,
    pub hit: Option<Sound>,
    pub chop: Option<Sound>,
    pub rock: Option<Sound>,
    pub hurt: Option<Sound>,
    pub kill: Option<Sound>,
    pub gem: Option<Sound>,
    pub lvl: Option<Sound>,
    pub craft: Option<Sound>,
    pub eat: Option<Sound>,
    pub fire: Option<Sound>,
    pub chest: Option<Sound>,
    pub shoot: Option<Sound>,
    pub boom: Option<Sound>,
    pub buy: Option<Sound>,
    pub door: Option<Sound>,
    pub dash: Option<Sound>,
    pub crit: Option<Sound>,
    pub pickup: Option<Sound>,
    pub upgrade: Option<Sound>,
    pub roar: Option<Sound>,
    pub battk: Option<Sound>,
    pub quest: Option<Sound>,
    pub trap: Option<Sound>,
    pub fball: Option<Sound>,
    pub deny: Option<Sound>,
    pub click: Option<Sound>,
    pub bgm: [Option<Sound>; BGM_N],
}

async fn load(s: Vec<f32>) -> Option<Sound> {
    load_sound_from_bytes(&wav_bytes(&s, SR)).await.ok()
}
async fn snd(f: f32, d: f32, sl: f32, v: f32, k: u8) -> Option<Sound> {
    load(tone(f, d, sl, v, k)).await
}

pub async fn load_sfx() -> Sfx {
    let mut s = Sfx {
        swing: snd(300., 0.06, -150., 0.15, 2).await,
        hit: snd(200., 0.07, -80., 0.25, 0).await,
        chop: snd(240., 0.05, -60., 0.25, 0).await,
        rock: snd(170., 0.06, -40., 0.25, 0).await,
        hurt: snd(90., 0.18, -40., 0.35, 1).await,
        kill: snd(120., 0.18, -60., 0.3, 1).await,
        gem: snd(880., 0.18, 400., 0.2, 2).await,
        lvl: load(seq(&[(523., 0.09), (659., 0.09), (784., 0.09), (1047., 0.22)], 0.22, 0)).await,
        craft: snd(660., 0.15, 300., 0.2, 0).await,
        eat: snd(520., 0.12, 200., 0.25, 2).await,
        fire: snd(200., 0.25, 0., 0.15, 3).await,
        chest: load(seq(&[(660., 0.07), (880., 0.07), (1320., 0.2)], 0.2, 2)).await,
        shoot: snd(520., 0.1, -300., 0.15, 1).await,
        boom: snd(80., 0.4, -50., 0.4, 3).await,
        buy: snd(700., 0.12, 500., 0.2, 2).await,
        door: snd(150., 0.1, 80., 0.2, 0).await,
        dash: snd(420., 0.16, -300., 0.16, 3).await,
        crit: load(seq(&[(1100., 0.04), (1500., 0.09)], 0.22, 0)).await,
        pickup: snd(1000., 0.06, 600., 0.13, 0).await,
        upgrade: load(seq(&[(392., 0.08), (494., 0.08), (587., 0.08), (784., 0.08), (988., 0.08), (1175., 0.3)], 0.22, 0)).await,
        roar: snd(70., 0.8, -30., 0.5, 1).await,
        battk: snd(150., 0.3, -100., 0.35, 1).await,
        quest: load(seq(&[(659., 0.1), (659., 0.06), (784., 0.1), (988., 0.26)], 0.2, 0)).await,
        trap: snd(300., 0.14, -150., 0.28, 3).await,
        fball: snd(520., 0.22, -380., 0.2, 1).await,
        deny: snd(160., 0.12, -70., 0.2, 0).await,
        click: snd(800., 0.04, 0., 0.1, 0).await,
        bgm: Default::default(),
    };
    for k in 0..BGM_N {
        s.bgm[k] = load_sound_from_bytes(&wav_bytes(&compose(k), BSR)).await.ok();
    }
    s
}

pub fn play(s: &Option<Sound>) {
    if !SFX_ON.load(Ordering::Relaxed) {
        return;
    }
    if let Some(s) = s {
        play_sound(s, PlaySoundParams { looped: false, volume: 0.6 });
    }
}

// ======================= BGM (chiptune) =======================
fn mhz(m: i32) -> f32 {
    440. * 2f32.powf((m as f32 - 69.) / 12.)
}
fn hu(a: u32) -> u32 {
    let mut x = a.wrapping_mul(0x9E3779B1) ^ 0x85EBCA6B;
    x ^= x >> 15;
    x = x.wrapping_mul(0x2C1B3C6D);
    x ^= x >> 12;
    x
}
/// วาดโน้ตหนึ่งตัวลงบัฟเฟอร์ (wave 0 สี่เหลี่ยม 50% · 1 สี่เหลี่ยม 25% · 2 สามเหลี่ยม · 3 นอยส์ · 4 ไซน์)
fn put(buf: &mut [f32], start: usize, len: usize, hz: f32, wave: u8, vol: f32, decay: f32, rng: &mut u32) {
    let mut ph = 0f32;
    let dp = hz / BSR as f32;
    for i in 0..len {
        let idx = start + i;
        if idx >= buf.len() {
            break;
        }
        let t = i as f32 / len as f32;
        let env = (1. - t).powf(decay).min(i as f32 / 60.);
        let s = match wave {
            0 => if ph < 0.5 { 1. } else { -1. },
            1 => if ph < 0.25 { 1. } else { -1. },
            2 => (ph * 2. - 1.).abs() * 2. - 1.,
            3 => {
                *rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
                (*rng >> 16) as f32 / 32768. - 1.
            }
            _ => (ph * TAU).sin(),
        };
        buf[idx] += s * env * vol;
        ph += dp;
        if ph >= 1. {
            ph -= 1.;
        }
    }
}

/// สร้างเพลง 8 ห้อง (วนลูปได้) ตามประเภท
fn compose(kind: usize) -> Vec<f32> {
    // (bpm, รากคอร์ด midi, minor?) 4 คอร์ด ห้องละ 2 ห้องเพลง
    let (bpm, prog): (f32, [(i32, bool); 4]) = match kind {
        0 => (108., [(60, false), (57, true), (53, false), (55, false)]),
        1 => (84., [(53, false), (50, true), (58, false), (48, false)]),
        2 => (156., [(50, true), (46, false), (53, false), (45, false)]),
        _ => (66., [(57, true), (53, false), (50, true), (52, false)]),
    };
    let step = (BSR as f32 * 60. / (bpm * 4.)) as usize;
    let steps = 128usize;
    let mut buf = vec![0f32; step * steps];
    let mut rng = 777u32 + kind as u32;
    let pent_maj = [0, 2, 4, 7, 9];
    let pent_min = [0, 3, 5, 7, 10];
    let mut lead_until = 0usize;
    for s in 0..steps {
        let bar = s / 16;
        let sb = s % 16;
        let (root, minor) = prog[(bar / 2) % 4];
        let third = if minor { 3 } else { 4 };
        let chord = [0, third, 7, 12];
        let scale: &[i32] = if minor { &pent_min } else { &pent_maj };
        let at = s * step;
        let h = hu(s as u32 * 31 + kind as u32 * 977);
        match kind {
            0 => {
                if sb % 4 == 0 { put(&mut buf, at, step * 4, mhz(root - 12), 2, 0.30, 1.2, &mut rng); }
                if sb == 8 { put(&mut buf, at, step * 3, mhz(root - 5), 2, 0.22, 1.2, &mut rng); }
                if sb % 2 == 0 { put(&mut buf, at, step * 2, mhz(root + 12 + chord[(sb / 2) % 4]), 2, 0.10, 2.0, &mut rng); }
                if s >= lead_until && h % 100 < 46 {
                    let n = root + 12 + scale[(h / 100) as usize % 5] + if h % 7 == 0 { 12 } else { 0 };
                    let l = 2 + (h / 17) as usize % 3;
                    put(&mut buf, at, step * l, mhz(n), 1, 0.13, 1.6, &mut rng);
                    lead_until = s + l;
                }
                if sb % 4 == 2 { put(&mut buf, at, step / 2, 9000., 3, 0.05, 3., &mut rng); }
            }
            1 => {
                if sb == 0 || sb == 8 { put(&mut buf, at, step * 8, mhz(root - 12), 2, 0.28, 1.0, &mut rng); }
                if sb % 4 == 0 { put(&mut buf, at, step * 4, mhz(root + chord[(sb / 4) % 3]), 4, 0.15, 1.8, &mut rng); }
                if sb % 2 == 1 && h % 100 < 38 {
                    put(&mut buf, at, step * 2, mhz(root + 12 + scale[(h / 100) as usize % 5]), 4, 0.14, 2.2, &mut rng);
                }
            }
            2 => {
                let oct = if sb % 4 == 2 { 0 } else { -12 };
                if sb % 2 == 0 { put(&mut buf, at, step * 2, mhz(root + oct), 0, 0.17, 1.5, &mut rng); }
                if sb % 8 == 0 || sb % 8 == 6 { put(&mut buf, at, step * 3, mhz(root - 24), 2, 0.34, 1.0, &mut rng); }
                if s >= lead_until && h % 100 < 62 {
                    let n = root + 12 + scale[(h / 100) as usize % 5] + if h % 5 == 0 { 12 } else { 0 };
                    let l = 1 + (h / 17) as usize % 3;
                    put(&mut buf, at, step * l, mhz(n), 0, 0.11, 1.4, &mut rng);
                    lead_until = s + l;
                }
                if sb % 8 == 0 { put(&mut buf, at, step * 2, 90., 4, 0.55, 3.0, &mut rng); }
                if sb % 8 == 4 { put(&mut buf, at, step * 2, 5000., 3, 0.14, 2.5, &mut rng); }
                if sb % 2 == 1 { put(&mut buf, at, step / 2, 9000., 3, 0.05, 3., &mut rng); }
            }
            _ => {
                if sb == 0 { put(&mut buf, at, step * 16, mhz(root - 24), 2, 0.34, 0.5, &mut rng); }
                if sb == 0 || sb == 10 { put(&mut buf, at, step * 6, mhz(root - 12), 4, 0.12, 1.0, &mut rng); }
                if (sb == 4 || sb == 12) && h % 100 < 70 {
                    put(&mut buf, at, step * 6, mhz(root + 12 + scale[(h / 100) as usize % 5]), 4, 0.13, 2.5, &mut rng);
                }
            }
        }
    }
    for s in buf.iter_mut() {
        *s = (*s * 0.8).clamp(-0.98, 0.98);
    }
    buf
}

// ======================= ควบคุมเพลงตามสถานการณ์ =======================
impl Game {
    /// เลือกเพลงตามสถานการณ์ แล้วสลับเมื่อเปลี่ยน
    pub fn upd_bgm(&mut self) {
        let want: i32 = if !self.bgm_on {
            -1
        } else if self.bosses.iter().any(|b| !b.dead && (b.x - self.p.x).hypot(b.y - self.p.y) < 420.) {
            2
        } else if self.w.dun.is_some() {
            3
        } else if self.w.camp_near(tfloor(self.p.x), tfloor(self.p.y), 9).is_some() {
            1
        } else if self.darkv > 0.45 {
            3
        } else {
            0
        };
        if want == self.bgm_cur {
            return;
        }
        if self.bgm_cur >= 0 {
            if let Some(s) = &self.sfx.bgm[self.bgm_cur as usize] {
                stop_sound(s);
            }
        }
        if want >= 0 {
            if let Some(s) = &self.sfx.bgm[want as usize] {
                play_sound(s, PlaySoundParams { looped: true, volume: BGM_VOL });
            }
        }
        self.bgm_cur = want;
    }
}
