#!/usr/bin/env bash
# เตรียม assets (เสียง + ภาพ) ก่อน build APK — เรียกจาก GitHub Actions ทุกครั้ง
# ไม่ทำให้ build ล้ม: ถ้าขั้นภาพพัง เกมจะใช้สไปรต์ที่วาดในโค้ด; ถ้าขั้นเสียงพัง เกมใช้ chiptune ในโค้ด
set -u
cd "$(dirname "$0")/.."
python3 -m pip install --quiet numpy pillow 2>&1 | tail -1 || true
python3 tools/gen_audio.py --out assets/audio || echo "::warning::gen_audio ล้มเหลว (เกมใช้เสียงในโค้ดแทน)"
python3 tools/fetch_sprites.py --out assets/sprites --preview build/sprite_preview || true
du -sh assets/audio assets/sprites 2>/dev/null || true
