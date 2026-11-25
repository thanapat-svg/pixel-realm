//! ระบบสร้างฐาน: กำแพงไม้/หิน · ประตู · กล่องเก็บของ · การโต้ตอบ

use super::*;

pub struct Piece {
    pub name: &'static str,
    pub desc: &'static str,
    pub obj: u8,
    pub icon: usize,
    pub cost: &'static [(usize, i32)],
}
/// v30 จำนวนชิ้นที่โชว์ในเมนูสร้าง (ชิ้นท้ายสุด = เลือกที่ดินให้ชาวบ้าน ซ่อนไว้)
pub const BUILD_N: usize = 33;
pub const PIECES: [Piece; 35] = [
    Piece { name: "เต็นท์", desc: "ที่หลบภัยเร็วที่สุด! ผ้าใบ+เตียง 3×4 ช่อง · สร้าง 6 วินาที · กันฝน นอนดีกว่ากลางแจ้ง", obj: OBJ_TENT, icon: IC_WWALL, cost: &[(WOOD, 8)] },
    Piece { name: "กระท่อม", desc: "บ้านไม้หลังเล็ก 5×4 ช่อง มีเตียง ท่อนไม้นั่ง หน้าต่าง · สร้าง 15 วินาที", obj: OBJ_HUT, icon: IC_WWALL, cost: &[(WOOD, 22)] },
    Piece { name: "บ้านสำเร็จรูป (เล็ก)", desc: "วางทีเดียวได้ทั้งหลัง! กำแพงไม้ ประตู หน้าต่าง พื้นไม้ เตียง โคมไฟ · ประตูอยู่ช่องที่เลือก ตัวบ้านขึ้นไปด้านบน 5×5 ช่อง", obj: OBJ_HOUSE_S, icon: IC_WWALL, cost: &[(WOOD, 40), (STONE, 4)] },
    Piece { name: "บ้านสำเร็จรูป (ใหญ่)", desc: "บ้านหินพร้อมเฟอร์นิเจอร์: เตียง โต๊ะ เก้าอี้ ชั้นหนังสือ พรม กระถางต้นไม้ โคมไฟ กล่องเก็บของ · 7×6 ช่อง", obj: OBJ_HOUSE_L, icon: IC_SWALL, cost: &[(STONE, 55), (WOOD, 45), (BERRY, 3)] },
    Piece { name: "กำแพงไม้", desc: "กั้นทางได้ ตีพังได้ (ได้ไม้คืน)", obj: 10, icon: IC_WWALL, cost: &[(WOOD, 2)] },
    Piece { name: "กำแพงหิน", desc: "แข็งแรงกว่ากำแพงไม้", obj: 11, icon: IC_SWALL, cost: &[(STONE, 3)] },
    Piece { name: "ประตู", desc: "กด Q หรือปุ่ม \"ใช้\" เพื่อเปิด-ปิด", obj: 12, icon: IC_DOOR, cost: &[(WOOD, 4)] },
    Piece { name: "กล่องเก็บของ", desc: "เปิดแล้วย้ายไอเทมเข้า-ออกได้ บันทึกลงเซฟ", obj: 14, icon: IC_BOX, cost: &[(WOOD, 8), (STONE, 2)] },
    Piece { name: "ทั่งตีบวก", desc: "ตีบวกอาวุธ/เกราะ +1..+5 ด้วยแร่และชิ้นส่วนบอส", obj: 17, icon: IC_ANVIL, cost: &[(STONE, 12), (WOOD, 4)] },
    Piece { name: "คบเพลิง", desc: "ให้แสงสว่างรอบฐานตอนกลางคืน", obj: 18, icon: IC_TORCH, cost: &[(WOOD, 1), (STONE, 1)] },
    Piece { name: "หม้อทำอาหาร", desc: "ทำอาหาร/ปรุงยา จากเนื้อ เห็ด สมุนไพร (ได้บัฟชั่วคราว)", obj: 26, icon: IC_POT, cost: &[(STONE, 6), (WOOD, 4)] },
    Piece { name: "ไถดิน", desc: "แปลงทุ่งหญ้าเป็นดินปลูก (ไม่เสียของ · วางซ้ำ = คืนหญ้า)", obj: OBJ_TILL, icon: IC_HOE, cost: &[] },
    Piece { name: "เตียงนอน", desc: "นอนตอนกลางคืนเพื่อข้ามไปเช้า · ฟื้นเลือดเต็ม · เป็นจุดเกิด", obj: 33, icon: IC_BED, cost: &[(WOOD, 12), (STONE, 2)] },
    Piece { name: "โต๊ะคราฟต์ขั้นสูง", desc: "สร้างอาวุธ/เกราะขั้น 2 (ต้องมีของขั้น 1 ก่อน)", obj: 34, icon: IC_ATABLE, cost: &[(STONE, 20), (WOOD, 10), (GEM, 4)] },
    Piece { name: "ปราสาท", desc: "ศูนย์กลางอาณาจักร (ต้องมี + กำแพงเมือง 12 ชิ้น + ชาวบ้าน >10 = เก็บภาษีได้)", obj: 37, icon: IC_ATABLE, cost: &[(STONE, 80), (WOOD, 40), (GEM, 10)] },
    Piece { name: "โอ่งน้ำ (ส่วนตัว)", desc: "น้ำดื่มสำหรับคนเดียว · ชาวบ้าน 1 คนที่เตียงอยู่ใกล้ (≤4 ช่อง) มีน้ำใช้ · ถูกและเล็ก", obj: O_WJAR, icon: IC_POT, cost: &[(STONE, 3), (WOOD, 1)] },
    Piece { name: "บ่อน้ำครัวเรือน", desc: "น้ำดื่มสะอาด (ดื่ม +45) · ใช้ได้ 6 คนในรัศมี 12 ช่อง", obj: 39, icon: IC_POT, cost: &[(STONE, 10), (WOOD, 4)] },
    Piece { name: "บ่อน้ำกลางเมือง", desc: "ศูนย์กลางเมือง · ทุกคนในรัศมี 30 ช่องมีน้ำใช้ ไม่จำกัดคน · ชาวบ้านมีความสุข +1/วัน", obj: O_BIGWELL, icon: IC_POT, cost: &[(STONE, 40), (WOOD, 12), (GEM, 1)] },
    Piece { name: "กำแพงเมือง", desc: "กำแพงหินสูง ทนทานกว่ากำแพงหิน (นับรวมเป็นกำแพงเมือง)", obj: 38, icon: IC_SWALL, cost: &[(STONE, 8)] },
    Piece { name: "ปลูกต้นกล้า", desc: "ปลูกต้นไม้ (ใช้เมล็ดจากผลไม้) · 4 วันโตเป็นต้นไม้ตามไบโอม · ฤดูหนาวไม่โต", obj: T_SAPLING, icon: WOOD, cost: &[(WOOD, 1), (BERRY, 1)] },
    Piece { name: "พื้นไม้", desc: "ปูพื้นในบ้าน (ไม้ 1/ช่อง) · วางซ้ำที่เดิม = รื้อคืนไม้", obj: OBJ_FLOOR_W, icon: IC_WWALL, cost: &[(WOOD, 1)] },
    Piece { name: "พื้นหิน", desc: "ปูพื้นหิน (หิน 1/ช่อง) · วางซ้ำที่เดิม = รื้อคืนหิน", obj: OBJ_FLOOR_S, icon: IC_SWALL, cost: &[(STONE, 1)] },
    Piece { name: "หน้าต่าง", desc: "ใช้แทนกำแพงได้ (นับเป็นผนังห้อง) · ในบ้านเห็นท้องฟ้า แดดลอดเข้ามา · +ความน่าอยู่", obj: O_WINDOW, icon: IC_WWALL, cost: &[(WOOD, 3), (STONE, 1)] },
    Piece { name: "โต๊ะ", desc: "เฟอร์นิเจอร์ · +ความน่าอยู่", obj: O_TABLE, icon: IC_WWALL, cost: &[(WOOD, 4)] },
    Piece { name: "เก้าอี้", desc: "เฟอร์นิเจอร์ (เดินผ่านได้) · +ความน่าอยู่", obj: O_CHAIR, icon: IC_WWALL, cost: &[(WOOD, 2)] },
    Piece { name: "พรม", desc: "ปูพื้นสวยๆ (เดินผ่านได้ · ย้อมด้วยผลเบอร์รี่) · +ความน่าอยู่", obj: O_RUG, icon: IC_WWALL, cost: &[(WOOD, 2), (BERRY, 2)] },
    Piece { name: "ชั้นหนังสือ", desc: "เฟอร์นิเจอร์ · +ความน่าอยู่", obj: O_SHELF, icon: IC_WWALL, cost: &[(WOOD, 6)] },
    Piece { name: "ตู้เสื้อผ้า", desc: "เฟอร์นิเจอร์ · +ความน่าอยู่", obj: O_WARDROBE, icon: IC_WWALL, cost: &[(WOOD, 6)] },
    Piece { name: "กระถางต้นไม้", desc: "ต้นไม้ในบ้าน · +ความน่าอยู่", obj: O_PLANT, icon: IC_WWALL, cost: &[(STONE, 2), (BERRY, 1)] },
    Piece { name: "วงกองไฟ", desc: "กองไฟ + ท่อนไม้นั่ง 4 ท่อน · ตอนเย็นชาวบ้านมานั่งคุยกัน (นั่งด้วยได้!) · สร้าง 8 วินาที", obj: OBJ_FIRERING, icon: IC_WWALL, cost: &[(WOOD, 10), (STONE, 3)] },
    Piece { name: "ท่อนไม้นั่ง", desc: "แตะเพื่อนั่ง · วางใกล้กองไฟให้ชาวบ้านมานั่งคุยกัน", obj: O_LOG, icon: IC_WWALL, cost: &[(WOOD, 2)] },
    Piece { name: "แผงตลาดนัด", desc: "วางใกล้กองไฟกลางเมือง · ทุกเย็น 15:30–17:30 ชาวบ้านมาเจอกัน คนที่หิวซื้ออาหารจากกล่องในเมือง (ค่าแผงเข้าคลังเมือง)", obj: O_STALL, icon: IC_WWALL, cost: &[(WOOD, 6)] },
    Piece { name: "โคมไฟ", desc: "ไฟสว่างในบ้านตอนกลางคืน (เป็นเขตปลอดภัยเหมือนคบเพลิง) · +ความน่าอยู่", obj: O_LAMP, icon: IC_TORCH, cost: &[(WOOD, 1), (STONE, 2)] },
    Piece { name: "ย้ายของ", desc: "ย้ายของที่สร้างแล้วโดยไม่ต้องรื้อ (ฟรี) · แตะของ 2 ครั้ง = ยก · แตะที่ใหม่ 2 ครั้ง = วาง · แตะผนัง/ประตู/พื้นบ้าน = ย้ายทั้งหลัง (หมุนได้) · ของในกล่องและเจ้าของเตียงตามไปด้วย · ต้นไม้/หินที่ขวางตัดให้เอง · มีเส้นไกด์ดูดเข้าแนวบ้านข้างๆ", obj: OBJ_MOVE, icon: IC_BOX, cost: &[] },
    Piece { name: "ให้ที่ดิน", desc: "เลือกที่ให้ชาวบ้านที่มาขอ (ทำไร่/สร้างบ้าน)", obj: OBJ_PERMIT, icon: IC_BOX, cost: &[] },
];

/// สิ่งที่ Game ขอให้ UI เปิด
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Req {
    Chest((i32, i32)),
    Shop,
    Forge,
    Cook,
    Adv,
    Npc(u32),
    /// v29 เปิดแท็บชุมชนที่หน้าเลือกคนให้ทำอาชีพ
    RolePick(u8),
    /// เมือง: (ดัชนีเมือง, 0 ตลาด 1 ปราสาท 2 คอกม้า 3 รถม้า)
    City(usize, u8),
    /// เปิดแท็บเมนู (จากปุ่มของที่ปรึกษา)
    Tab(u8),
}

impl Game {
    pub fn can_till(&self, tx: i32, ty: i32) -> bool {
        let (t, o) = (self.w.tile_at(tx, ty), self.w.obj_at(tx, ty));
        ((t == T_SOIL || t == T_WET) && (o == 0 || (27..=32).contains(&o))) || (self.tillable_ground(tx, ty) && (o == 0 || o == 3 || o == O_DRYGRASS))
    }
    /// v30 ดินที่ไถได้: ทุ่งหญ้า ป่า สะวันนา หนองน้ำ · ทราย/ชายหาดได้เฉพาะริมน้ำ (≤4 ช่อง) — เมืองกลางทะเลทรายทำไร่ริมโอเอซิสได้
    pub fn tillable_ground(&self, tx: i32, ty: i32) -> bool {
        match self.w.tile_at(tx, ty) {
            3 | 4 | T_SAVANNA | T_SWAMP => true,
            2 | 8 => (-4..=4).any(|dy| (-4..=4).any(|dx| self.w.tile_at(tx + dx, ty + dy) < 2)),
            _ => false,
        }
    }
    pub fn player_in_tile(&self, tx: i32, ty: i32) -> bool {
        let p = &self.p;
        [(p.x - 4., p.y - 4.), (p.x + 4., p.y - 4.), (p.x - 4., p.y), (p.x + 4., p.y)].iter().any(|&(x, y)| tfloor(x) == tx && tfloor(y) == ty)
    }
    pub fn can_place_at(&self, tx: i32, ty: i32) -> bool {
        let o = self.w.obj_at(tx, ty);
        !SOLIDT[self.w.tile_at(tx, ty) as usize] && (o == 0 || o == 3) && !self.player_in_tile(tx, ty) && !self.site_reserved((tx, ty))
            && self.w.realm.city_at(tx, ty).map_or(true, |ci| self.city_owner(ci) == PLAYER_OWN)
    }
    /// v40 ผู้เล่นสร้างได้ไหม: เหมือน can_place_at แต่ต้นไม้/พุ่ม/หิน/หญ้าธรรมชาติไม่บัง (ถางให้ตอนวาง)
    pub fn can_build_at(&self, tx: i32, ty: i32) -> bool {
        let k = (tx, ty);
        let o = self.w.obj_at(tx, ty);
        let ok_obj = o == 0 || o == 3 || (!self.w.placed.contains_key(&k) && mover::nat_cuttable(o));
        !SOLIDT[self.w.tile_at(tx, ty) as usize] && ok_obj && !self.player_in_tile(tx, ty) && !self.site_reserved(k)
            && self.w.realm.city_at(tx, ty).map_or(true, |ci| self.city_owner(ci) == PLAYER_OWN)
    }
    /// v40 ถางของธรรมชาติตรงช่องนี้ก่อนสร้างทับ (ได้วัสดุคืน)
    pub fn cut_for_build(&mut self, k: (i32, i32)) {
        if self.w.placed.contains_key(&k) { return; }
        let o = self.w.obj_at(k.0, k.1);
        if o == 0 || !mover::nat_cuttable(o) { return; }
        self.w.removed.insert(k);
        self.obj_dmg.remove(&k);
        if let Some((it, q)) = mover::cut_refund(o) { self.p.inv[it] += q; }
    }
    pub fn place_piece(&mut self) {
        if self.w.dun.is_some() {
            self.toast("สร้างในดันเจี้ยนไม่ได้");
            return;
        }
        let Some(sel) = self.build_sel else {
            self.toast("เลือกสิ่งปลูกสร้างจากเมนูสร้าง (B) ก่อน");
            return;
        };
        let pc = &PIECES[sel];
        if pc.obj == OBJ_PERMIT { let k = self.build_target(); self.permit_place(k); return; }
        if pc.obj == OBJ_MOVE {
            // v28 ย้ายของ: ยังไม่ได้ยก = ยก · ยกอยู่ = วาง
            let k = self.build_target();
            if self.rs.carry.is_none() {
                if !self.move_reach(k) { self.toast("ไกลเกินไป — เดินเข้าไปใกล้อีกหน่อย"); return; }
                self.move_pick(k);
            } else { self.move_drop(k); }
            return;
        }
        if !self.build_reach(self.build_target()) { self.toast("ไกลเกินไป — เดินเข้าไปใกล้อีกหน่อย"); return; }
        if pc.obj == OBJ_TILL {
            let k = self.build_target();
            if self.till_at(k) {
                self.save();
            }
            return;
        }
        if is_prefab(pc.obj) {
            let k = self.build_target();
            if self.place_prefab(pc.obj, k, self.build_dir, pc.cost) { self.save(); }
            return;
        }
        if pc.obj == OBJ_FLOOR_W || pc.obj == OBJ_FLOOR_S {
            let k = self.build_target();
            if !self.build_reach(k) { self.toast("ไกลเกินไป"); return; }
            if self.place_floor(k, pc.obj) {
                play(&self.sfx.door);
                self.room_t = 0.;
                self.save();
            }
            return;
        }
        if !can_pay(&self.p, pc.cost) {
            self.toast("วัสดุไม่พอ");
            return;
        }
        let (tx, ty) = self.build_target();
        if !self.can_build_at(tx, ty) {
            self.toast("วางตรงนี้ไม่ได้");
            return;
        }
        if pc.obj != T_SAPLING { self.cut_for_build((tx, ty)); }
        if pc.obj == T_SAPLING {
            if !self.plant_sapling((tx, ty)) { return; }
            for &(it, n) in pc.cost { self.p.inv[it] -= n; }
            play(&self.sfx.eat);
            self.save();
            return;
        }
        for &(it, n) in pc.cost {
            self.p.inv[it] -= n;
        }
        self.w.placed.insert((tx, ty), pc.obj);
        self.room_t = 0.;
        if pc.obj == O_LAMP { self.refresh_torches(); }
        self.ev[3] += 1;
        if pc.obj == 14 {
            self.chests.insert((tx, ty), [0; NI]);
        }
        self.obj_dmg.remove(&(tx, ty));
        play(&self.sfx.door);
        self.burst(tx as f32 * 16. + 8., ty as f32 * 16. + 10., hx(0xc8a070), 8, 40.);
        self.save();
    }

    /// วัตถุที่โต้ตอบได้ใกล้ตัวที่สุด (ประตู/กล่อง/พ่อค้า)
    pub fn interact_target(&self) -> Option<((i32, i32), u8)> {
        let (ptx, pty) = (tfloor(self.p.x), tfloor(self.p.y - 3.));
        let mut best: Option<(f32, (i32, i32), u8)> = None;
        for ty in pty - 2..=pty + 2 {
            for tx in ptx - 2..=ptx + 2 {
                let o = self.w.obj_at(tx, ty);
                if !matches!(o, 12 | 13 | 14 | 15 | 17 | 19 | 20 | 24 | 25 | 26 | 33 | 34 | 36 | 37 | 39 | O_WJAR | O_BIGWELL | 41 | 42 | O_ALTAR | O_GIANT | O_CHAIR | O_LOG | O_STALL | O_STABLE | O_COACH) {
                    continue;
                }
                if (o == 41 || o == 42) && !self.fruit_ready((tx, ty)) { continue; }
                if is_seat(o) && self.sit == Some((tx, ty)) { continue; }
                if o == 37 && self.w.realm.city_at(tx, ty).is_none() { continue; }
                let d = (tx as f32 * 16. + 8. - self.p.x).hypot(ty as f32 * 16. + 8. - (self.p.y - 3.));
                if d < 28. && best.map_or(true, |b| d < b.0) {
                    best = Some((d, (tx, ty), o));
                }
            }
        }
        best.map(|b| (b.1, b.2))
    }
    pub fn interact(&mut self) {
        if self.fish.is_some() {
            self.fish_press();
            return;
        }
        if self.try_tame() {
            return;
        }
        if self.npc_interact() {
            return;
        }
        if self.caravan_use() {
            return;
        }
        if let Some(i) = self.citizen_near() {
            self.citizen_talk(i);
            return;
        }
        if self.try_farm() {
            return;
        }
        let Some((k, o)) = self.interact_target() else {
            if self.drink_ahead() { return; }
            if !self.start_fish() {
                self.toast("ไม่มีอะไรให้ใช้ตรงนี้");
            }
            return;
        };
        let city = self.w.realm.city_at(k.0, k.1);
        match o {
            O_STALL | O_STABLE | O_COACH | 37 if city.is_some() => {
                self.req = Some(Req::City(city.unwrap(), match o { O_STALL => 0, 37 => 1, O_STABLE => 2, _ => 3 }));
            }
            O_CHAIR | O_LOG => self.sit_on(k),
            O_STALL => {
                // v42 แผงตลาดนัดในเมืองเรา
                let n = self.rs.ar.mkt_seen.len();
                self.toast(if self.market_open() { format!("ตลาดนัดกำลังเปิด (15:30–17:30) · ชาวบ้านมาแล้ว {} คน — คนที่หิวซื้ออาหารจากกล่องในเมืองได้ ค่าแผงเข้าคลังเมือง", n) } else { "แผงตลาดนัด: ทุกเย็น 15:30–17:30 ชาวบ้านมาเจอกัน แลกซื้ออาหารกันเอง".to_string() });
            }
            12 => {
                self.auto_doors.remove(&k);
                self.w.placed.insert(k, 13);
                play(&self.sfx.door);
                self.save();
            }
            13 => {
                if self.player_in_tile(k.0, k.1) {
                    self.toast("ปิดไม่ได้ มีคนยืนอยู่ในประตู");
                } else {
                    self.auto_doors.remove(&k);
                    self.w.placed.insert(k, 12);
                    play(&self.sfx.door);
                    self.save();
                }
            }
            14 => {
                self.chests.entry(k).or_insert([0; NI]);
                play(&self.sfx.chest);
                self.req = Some(Req::Chest(k));
            }
            17 => {
                play(&self.sfx.click);
                self.req = Some(Req::Forge);
            }
            25 => {
                self.big_map = true;
                self.mini_t = 99.;
                self.toast("แตะสัญลักษณ์ W บนแผนที่เพื่อวาร์ป");
            }
            26 => {
                play(&self.sfx.click);
                self.req = Some(Req::Cook);
            }
            33 => self.sleep(k),
            39 | O_WJAR | O_BIGWELL => self.drink_well(),
            41 | 42 => { self.pick_fruit(k, o); }
            O_ALTAR => { self.shrine_pray(k); }
            O_GIANT => { self.giant_gather(k); }
            34 => {
                play(&self.sfx.click);
                self.req = Some(Req::Adv);
            }
            19 | 36 => self.enter_dungeon(k),
            20 => self.exit_dungeon(),
            24 => self.open_rare_chest(k),
            _ => {
                play(&self.sfx.buy);
                self.shop_camp = Some(k);
                self.req = Some(Req::Shop);
            }
        }
    }
    /// ย้ายไอเทมระหว่างกระเป๋า <-> กล่อง
    pub fn chest_move(&mut self, k: (i32, i32), item: usize, n: i32, into: bool) {
        let c = self.chests.entry(k).or_insert([0; NI]);
        if into {
            let m = n.min(self.p.inv[item]);
            c[item] += m;
            self.p.inv[item] -= m;
        } else {
            let m = n.min(c[item]);
            c[item] -= m;
            self.p.inv[item] += m;
        }
        self.save();
    }
}

/// ข้อความช่วยเหลือเมื่ออยู่ใกล้สิ่งที่โต้ตอบได้
pub fn interact_hint(o: u8, touch: bool) -> String {
    let key = if touch { "ปุ่ม \"ใช้\"" } else { "Q" };
    match o {
        12 => format!("{}: เปิดประตู", key),
        O_CHAIR | O_LOG => format!("{}: นั่ง (นั่งกินอิ่มขึ้น · ใกล้กองไฟคุยกับชาวบ้านได้)", key),
        O_STALL => format!("{}: ตลาด (ซื้อ/ขาย)", key),
        O_STABLE => format!("{}: คอกม้า", key),
        O_COACH => format!("{}: ท่ารถม้า (เดินทางไปเมืองอื่น)", key),
        37 => format!("{}: เข้าเฝ้าผู้ปกครอง (การทูต)", key),
        13 => format!("{}: ปิดประตู", key),
        14 => format!("{}: เปิดกล่องเก็บของ", key),
        17 => format!("{}: ใช้ทั่งตีบวก", key),
        25 => format!("{}: เปิดแผนที่วาร์ป", key),
        26 => format!("{}: ทำอาหาร / ปรุงยา", key),
        19 => format!("{}: เข้าดันเจี้ยนลับ (อันตราย!)", key),
        36 => format!("{}: เข้าถ้ำใต้ดิน (มืด · มีแร่ · มอนสเตอร์ดุ!)", key),
        33 => format!("{}: นอน (ข้ามไปเช้า)", key),
        39 | O_BIGWELL => format!("{}: ดื่มน้ำจากบ่อ", key),
        O_WJAR => format!("{}: ดื่มน้ำจากโอ่ง", key),
        41 => format!("{}: เก็บแอปเปิล (ไม่ต้องโค่นต้น)", key),
        42 => format!("{}: ดื่มน้ำมะพร้าว", key),
        34 => format!("{}: ใช้โต๊ะคราฟต์ขั้นสูง", key),
        20 => format!("{}: ออกจากดันเจี้ยน", key),
        24 => format!("{}: เปิดหีบหายาก", key),
        _ => format!("{}: คุยกับพ่อค้า", key),
    }
}
