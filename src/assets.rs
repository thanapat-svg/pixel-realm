//! จุดต่อสำหรับ asset ภาพ/เสียงคุณภาพสูงในอนาคต
//!
//! เกมนี้วาดสไปรต์และสังเคราะห์เสียงด้วยโค้ดทั้งหมด (ไม่ต้องมีไฟล์ใดๆ ก็เล่นได้)
//! ถ้าวางไฟล์ตามชื่อด้านล่างลงในโฟลเดอร์ `assets/` เกมจะโหลดมาใช้แทนของเดิมโดยอัตโนมัติ
//! ไฟล์ที่ไม่มีจะถูกข้ามไป ไม่ทำให้เกมพัง — ดูรายละเอียดใน assets/README.md
//!
//! เสียง: assets/audio/sfx_<ชื่อ>.ogg|wav   (ชื่อ = ฟิลด์ใน Sfx เช่น sfx_swing, sfx_hit, sfx_chest)
//!        assets/audio/bgm_<เลข>.ogg|wav    (เลข 0..BGM_N-1)
//! ภาพ:   assets/sprites/tile_<id>.png      (ไทล์พื้น id ตาม TN เช่น tile_3 = ทุ่งหญ้า)
//!        assets/sprites/obj_<id>.png       (วัตถุ id ตามตารางใน main.rs เช่น obj_26 = หม้อทำอาหาร)
//!        assets/sprites/icon_<n>.png       (ไอคอนไอเทม/เครื่องมือ n ตามลำดับ icons)
//! ภาพจะถูกย่อ/ขยายให้ได้ขนาดเท่าสไปรต์เดิม (ไทล์ 16x16, ไอคอน 8x8, วัตถุตามขนาดเดิม)
//! บน Android ต้องมี `assets = "assets"` ใน Cargo.toml (ตั้งไว้แล้ว) เพื่อแพ็กไฟล์เข้า APK

use super::*;
use macroquad::file::load_file;

async fn first_bytes(base: &str, exts: &[&str]) -> Option<Vec<u8>> {
    for e in exts {
        if let Ok(b) = load_file(&format!("assets/{}.{}", base, e)).await {
            if !b.is_empty() {
                return Some(b);
            }
        }
    }
    None
}

/// โหลดเสียงจาก assets/audio/<name>.ogg|wav (None ถ้าไม่มีไฟล์)
pub async fn try_sound(name: &str) -> Option<Sound> {
    let b = first_bytes(&format!("audio/{}", name), &["ogg", "wav"]).await?;
    load_sound_from_bytes(&b).await.ok()
}

/// แทนเสียงสังเคราะห์ด้วยไฟล์ที่มี
pub async fn override_audio(s: &mut Sfx) {
    macro_rules! ov {
        ($($f:ident),* $(,)?) => { $( if let Some(x) = try_sound(concat!("sfx_", stringify!($f))).await { s.$f = Some(x); } )* };
    }
    ov!(swing, hit, chop, rock, hurt, kill, gem, lvl, craft, eat, fire, chest, shoot, boom, buy, door, dash, crit, pickup, upgrade, roar, battk, quest, trap, fball, deny, click);
    for k in 0..BGM_N {
        if let Some(x) = try_sound(&format!("bgm_{}", k)).await {
            s.bgm[k] = Some(x);
        }
    }
}

/// ภาพที่โหลดจากไฟล์ (รอแทนสไปรต์ที่วาดด้วยโค้ด)
#[derive(Default)]
pub struct SpriteOv {
    pub tiles: Vec<(usize, Image)>,
    pub objs: Vec<(usize, Image)>,
    pub icons: Vec<(usize, Image)>,
}

async fn try_image(name: &str) -> Option<Image> {
    let b = first_bytes(&format!("sprites/{}", name), &["png"]).await?;
    Image::from_file_with_format(&b, None).ok()
}

pub async fn load_sprite_overrides() -> SpriteOv {
    let mut o = SpriteOv::default();
    for i in 0..TN.len() {
        if let Some(im) = try_image(&format!("tile_{}", i)).await {
            o.tiles.push((i, im));
        }
    }
    for i in 1..OSOLID.len() {
        if let Some(im) = try_image(&format!("obj_{}", i)).await {
            o.objs.push((i, im));
        }
    }
    for i in 0..INAME.len() + 20 {
        if let Some(im) = try_image(&format!("icon_{}", i)).await {
            o.icons.push((i, im));
        }
    }
    o
}

/// ย่อ (เฉลี่ยกล่อง) หรือขยาย (nearest) ภาพให้ได้ขนาด tw x th
pub fn fit(src: &Image, tw: u16, th: u16) -> Image {
    if src.width == tw && src.height == th {
        return src.clone();
    }
    let (sw, sh) = (src.width as u32, src.height as u32);
    let mut o = Image::gen_image_color(tw, th, BLANK);
    for y in 0..th as u32 {
        for x in 0..tw as u32 {
            let x0 = x * sw / tw as u32;
            let x1 = (((x + 1) * sw / tw as u32).max(x0 + 1)).min(sw);
            let y0 = y * sh / th as u32;
            let y1 = (((y + 1) * sh / th as u32).max(y0 + 1)).min(sh);
            let (mut r, mut g, mut b, mut a, mut n) = (0., 0., 0., 0., 0.);
            for yy in y0..y1 {
                for xx in x0..x1 {
                    let c = src.get_pixel(xx, yy);
                    r += c.r * c.a;
                    g += c.g * c.a;
                    b += c.b * c.a;
                    a += c.a;
                    n += 1.;
                }
            }
            if a > 0. {
                o.set_pixel(x, y, Color::new(r / a, g / a, b / a, a / n));
            }
        }
    }
    o
}

impl SpriteOv {
    pub fn is_empty(&self) -> bool {
        self.tiles.is_empty() && self.objs.is_empty() && self.icons.is_empty()
    }
    /// ใส่ภาพที่โหลดได้ลง atlas แล้วแทนที่ Rect ของสไปรต์เดิม
    pub fn apply(&self, at: &mut Atlas, tiles: &mut Vec<Vec<Rect>>, obj: &mut Vec<Rect>, icons: &mut Vec<Rect>) {
        for (i, im) in &self.tiles {
            if *i < tiles.len() {
                let r = at.add(&fit(im, 16, 16));
                tiles[*i] = vec![r; 4];
            }
        }
        for (i, im) in &self.objs {
            if *i < obj.len() {
                let old = obj[*i];
                let (w, h) = if old.w > 1. { (old.w as u16, old.h as u16) } else { (16, 16) };
                obj[*i] = at.add(&fit(im, w, h));
            }
        }
        for (i, im) in &self.icons {
            if *i < icons.len() {
                obj_icon(at, icons, *i, im);
            }
        }
    }
}
fn obj_icon(at: &mut Atlas, icons: &mut Vec<Rect>, i: usize, im: &Image) {
    icons[i] = at.add(&fit(im, 8, 8));
}
