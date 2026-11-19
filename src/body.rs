// body.rs — รูปร่างหน้าตาแบบชีวิตจริง (v7c): สีผิว · สีผม · ทรงผม · สีเสื้อ · ส่วนสูง (ซม.) · น้ำหนัก (กก.) · อายุ
//  - ผู้เล่นเลือกเองตอนเริ่มเกมใหม่ · ชาวบ้านสุ่มแบบกำหนดแน่นอนจาก seed (ผู้ชายสูงเฉลี่ยกว่า ผู้หญิง · คนสูงอายุผมเทา/ขาว/ล้านบ่อยขึ้น)
//  - ลูกได้สีผิวผสมจากพ่อแม่ (ผู้เล่น + คู่สมรส) · ร่างใหญ่เผาผลาญอาหารมากกว่า · ตัวสูงก้าวยาวกว่า · น้ำหนักเกินเดินช้าลงเล็กน้อย
use super::*;

pub const SKIN_N: usize = 8;
pub const SKIN_TONES: [u32; SKIN_N] = [0xfbe3cf, 0xf2c79a, 0xe3ad80, 0xc98e5a, 0xa8703f, 0x8a552b, 0x6b3f20, 0x4a2a16];
pub const SKIN_NAME: [&str; SKIN_N] = ["ขาวซีด", "ขาว", "ขาวอมเหลือง", "แทนอ่อน", "แทน", "น้ำตาล", "น้ำตาลเข้ม", "เข้มมาก"];
pub const HAIR_N: usize = 8;
pub const HAIR_COLS: [u32; HAIR_N] = [0x1a1512, 0x3a2414, 0x5a3a1e, 0x8a4a22, 0xd8b457, 0xb8541e, 0x8d8d93, 0xe9e9ee];
pub const HAIR_NAME: [&str; HAIR_N] = ["ดำ", "น้ำตาลเข้ม", "น้ำตาล", "น้ำตาลแดง", "บลอนด์", "แดงส้ม", "เทา", "ขาว"];
pub const STYLE_N: usize = 5;
pub const STYLE_NAME: [&str; STYLE_N] = ["สั้น", "เกรียน", "ยาว", "หางม้า", "ล้าน"];
pub const CLOTH_N: usize = 10;
pub const CLOTH_COLS: [u32; CLOTH_N] = [0x3a6fd8, 0xd0504a, 0x3f9a58, 0x8a5ac8, 0xe0883a, 0x2fa0a0, 0x8a6038, 0x6f7388, 0xd878a8, 0xd8c040];
pub const CLOTH_NAME: [&str; CLOTH_N] = ["ฟ้า", "แดง", "เขียว", "ม่วง", "ส้ม", "เขียวน้ำทะเล", "น้ำตาล", "เทา", "ชมพู", "เหลือง"];
pub const CM_MIN: i32 = 140;
pub const CM_MAX: i32 = 205;
pub const KG_MIN: i32 = 38;
pub const KG_MAX: i32 = 140;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Body { pub skin: u8, pub hair: u8, pub style: u8, pub cloth: u8, pub cm: u16, pub kg: u16, pub age: u8 }

impl Default for Body {
    fn default() -> Self { Body { skin: 1, hair: 2, style: 0, cloth: 0, cm: 172, kg: 68, age: 25 } }
}

fn rnd(seed: u32, k: i32) -> f64 { hash(seed as i32, k, 51_517) }
/// ค่าเกือบปกติ (เฉลี่ย 0 ส่วนเบี่ยงเบน ~1) จากเลขสุ่มสามตัว
fn gauss(seed: u32, k: i32) -> f64 { (rnd(seed, k) + rnd(seed, k + 1) + rnd(seed, k + 2) - 1.5) * 2. }
fn lighten(c: u32, t: f32) -> Color {
    let ch = |s: u32| (((c >> s) & 255) as f32 / 255.) * (1. - t) + t;
    Color::new(ch(16), ch(8), ch(0), 1.)
}

impl Body {
    pub fn clamp(mut self) -> Self {
        self.skin = self.skin.min(SKIN_N as u8 - 1);
        self.hair = self.hair.min(HAIR_N as u8 - 1);
        self.style = self.style.min(STYLE_N as u8 - 1);
        self.cloth = self.cloth.min(CLOTH_N as u8 - 1);
        self.cm = self.cm.clamp(if self.age < 18 { 80 } else { CM_MIN as u16 }, CM_MAX as u16);
        self.kg = self.kg.clamp(if self.age < 18 { 10 } else { KG_MIN as u16 }, KG_MAX as u16);
        self
    }
    pub fn bmi(&self) -> f32 { let m = self.cm as f32 / 100.; self.kg as f32 / (m * m) }
    pub fn is_child(&self) -> bool { self.age < 13 }
    pub fn skin_col(&self) -> Color { hx(SKIN_TONES[self.skin as usize]) }
    pub fn hair_col(&self) -> Color { hx(HAIR_COLS[self.hair as usize]) }
    pub fn cloth_col(&self) -> Color { hx(CLOTH_COLS[self.cloth as usize]) }
    pub fn cloth_hi(&self) -> Color { lighten(CLOTH_COLS[self.cloth as usize], 0.55) }

    /// สัดส่วนวาด (กว้าง, สูง) เทียบกับสไปรต์มาตรฐาน (172 ซม. / BMI 23)
    pub fn scale(&self) -> (f32, f32) {
        let sy = (self.cm as f32 / 172.).clamp(0.5, 1.15);
        let bf = (self.bmi() / 22.5).max(0.1).sqrt().clamp(0.84, 1.22);
        ((sy.powf(0.6) * bf).clamp(0.5, 1.25), sy)
    }
    /// ร่างใหญ่เผาผลาญมากกว่า (ตัวคูณความหิว) — เด็กกินน้อยกว่าเล็กน้อย
    pub fn hunger_mul(&self) -> f32 { (self.kg as f32 / 68.).sqrt().clamp(0.85, 1.2) }
    /// ก้าวยาวตามส่วนสูง · น้ำหนักเกินเดินช้าลงนิด ๆ
    pub fn speed_mul(&self) -> f32 {
        let leg = (self.cm as f32 / 172.).sqrt().clamp(0.96, 1.04);
        let heft = 1. - ((self.bmi() - 27.).max(0.) * 0.012).min(0.06);
        (leg * heft).clamp(0.92, 1.04)
    }
    pub fn build_word(&self) -> &'static str {
        let b = self.bmi();
        if b < 18.5 { "ผอม" } else if b < 25. { "สมส่วน" } else if b < 30. { "ท้วม" } else { "อ้วน" }
    }
    pub fn height_word(&self) -> &'static str {
        if self.is_child() { return "ตัวเล็ก"; }
        match self.cm { 0..=159 => "ค่อนข้างเตี้ย", 160..=179 => "สูงปานกลาง", 180..=189 => "สูง", _ => "สูงมาก" }
    }
    pub fn describe(&self) -> String {
        format!("อายุ {} ปี · สูง {} ซม. · หนัก {} กก. ({})", self.age, self.cm, self.kg, self.build_word())
    }
    pub fn look_words(&self) -> String {
        format!("ผิว{} · ผม{} ทรง{}", SKIN_NAME[self.skin as usize], HAIR_NAME[self.hair as usize], STYLE_NAME[self.style as usize])
    }

    /// สร้างชาวบ้านแบบกำหนดแน่นอนจาก seed (ค่า look ที่เก็บในเซฟ) · fem = เพศตามชื่อ
    pub fn generate(seed: u32, fem: bool) -> Body {
        let r = |k: i32| rnd(seed, k);
        // ผิว: การกระจายครบทุกโทน
        const W: [f64; SKIN_N] = [0.08, 0.17, 0.17, 0.16, 0.14, 0.12, 0.10, 0.06];
        let mut x = r(1);
        let mut skin = SKIN_N - 1;
        for (i, w) in W.iter().enumerate() { if x < *w { skin = i; break; } x -= *w; }
        // อายุ: เอนไปทางวัยทำงาน 18–75
        let age = (18. + r(2).powf(1.35) * 58.) as i32;
        // ส่วนสูง: ชาย ~175 หญิง ~162 (ส่วนเบี่ยงเบน ~7) · สูงอายุเตี้ยลง
        let mut cm = (if fem { 162. } else { 175. }) + gauss(seed, 3) * 7. - ((age - 55).max(0) as f64) * 0.25;
        cm = cm.clamp(148., 198.);
        // BMI: เฉลี่ย ~23 อายุมากขึ้นอ้วนขึ้นเล็กน้อย
        let bmi = (22.6 + gauss(seed, 6) * 3.4 + (age as f64 - 30.) * 0.04 - if fem { 0.6 } else { 0. }).clamp(17., 36.);
        let kg = bmi * (cm / 100.) * (cm / 100.);
        // ผม: สีเข้มพบมากในผิวเข้ม · คนสูงอายุผมหงอก
        let hp = r(9);
        let natural: &[u8] = if skin >= 5 { &[0, 0, 0, 0, 1, 1, 2] } else if skin >= 3 { &[0, 0, 1, 1, 2, 2, 3] } else { &[0, 1, 2, 2, 3, 4, 4, 5] };
        let mut hair = natural[((hp * natural.len() as f64) as usize).min(natural.len() - 1)];
        if age >= 60 { hair = if r(10) < 0.55 { 6 } else { 7 }; } else if age >= 45 && r(10) < 0.30 { hair = 6; }
        // ทรงผม
        let sp = r(11);
        let style = if fem {
            if sp < 0.42 { 2 } else if sp < 0.68 { 3 } else if sp < 0.96 { 0 } else { 1 }
        } else {
            let bald = if age >= 50 { 0.30 } else if age >= 35 { 0.12 } else { 0.03 };
            if sp < bald { 4 } else if sp < bald + 0.42 { 0 } else if sp < bald + 0.67 { 1 } else if sp < bald + 0.82 { 2 } else { 3 }
        };
        Body { skin: skin as u8, hair, style, cloth: (r(12) * CLOTH_N as f64) as u8 % CLOTH_N as u8, cm: cm.round() as u16, kg: kg.round() as u16, age: age as u8 }.clamp()
    }
    /// เด็ก: อายุ 5–12 · ได้สีผิวผสมจากพ่อแม่ (เฉลี่ยโทน ±1) และสีผมของพ่อหรือแม่
    pub fn child_of(seed: u32, fem: bool, a: &Body, b: &Body) -> Body {
        let r = |k: i32| rnd(seed, 40 + k);
        let age = 5 + (r(1) * 8.) as i32;
        let mix = (a.skin as i32 + b.skin as i32 + 1) / 2;
        let jitter = if r(2) < 0.25 { -1 } else if r(2) > 0.75 { 1 } else { 0 };
        let skin = (mix + jitter).clamp(0, SKIN_N as i32 - 1) as u8;
        let mut hair = if r(3) < 0.5 { a.hair } else { b.hair };
        if hair >= 6 { hair = 2; } // เด็กไม่ผมหงอก
        let cm = 80. + age as f64 * 5.6 + gauss(seed, 44) * 3.;
        let kg = 11. + (age - 3) as f64 * 2.9 + gauss(seed, 47) * 1.5;
        let style = if fem { if r(4) < 0.5 { 3 } else { 0 } } else if r(4) < 0.2 { 1 } else { 0 };
        Body { skin, hair, style, cloth: (r(5) * CLOTH_N as f64) as u8 % CLOTH_N as u8, cm: cm.round() as u16, kg: kg.round() as u16, age: age as u8 }.clamp()
    }
    /// สุ่มรูปร่างให้ผู้เล่น (ปุ่ม "สุ่ม" ตอนสร้างตัวละคร)
    pub fn random_player() -> Body {
        let seed = gen_range(1u32, 1_000_000);
        let mut b = Body::generate(seed, gen_range(0, 2) == 1);
        b.age = 25 + (b.age % 20);
        if b.age >= 45 { b.age = 30; }
        if b.hair >= 6 { b.hair = 2; }
        b.clamp()
    }

    pub fn save_str(&self) -> String {
        format!("body {} {} {} {} {} {} {}\n", self.skin, self.hair, self.style, self.cloth, self.cm, self.kg, self.age)
    }
    pub fn load(f: &[f32]) -> Body {
        if f.len() < 7 { return Body::default(); }
        Body { skin: f[0] as u8, hair: f[1] as u8, style: f[2] as u8, cloth: f[3] as u8, cm: f[4] as u16, kg: f[5] as u16, age: f[6] as u8 }.clamp()
    }
}

impl Game {
    /// รูปร่างของชาวบ้าน: ลูก = ผสมจากผู้เล่นกับคู่สมรสคนแรก · คนอื่นสุ่มจาก look + เพศตามชื่อ
    pub fn npc_body(&self, n: &Npc) -> Body {
        let fem = NPC_FEM[n.name as usize % NPC_FEM.len()];
        if n.child {
            let spouse = self.npcs.iter().find(|m| m.married).map(|m| Body::generate(m.look as u32, NPC_FEM[m.name as usize % NPC_FEM.len()])).unwrap_or(self.p.body);
            return Body::child_of(n.look as u32 + n.id * 31, fem, &self.p.body, &spouse);
        }
        Body::generate(n.look as u32, fem)
    }
}
