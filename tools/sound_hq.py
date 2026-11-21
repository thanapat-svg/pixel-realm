#!/usr/bin/env python3
"""v15: คลังเสียงสมจริง สังเคราะห์ด้วยฟิสิกส์ของเสียง (ไม่มีไฟล์ต้นฉบับ ไม่ติดลิขสิทธิ์)

เทคนิคหลัก
  - modal synthesis: เสียงกระทบ = ผลรวมไซน์ที่สั่นแล้วจางตามความถี่ธรรมชาติของวัสดุ (ไม้/หิน/โลหะ)
  - granular synthesis: เสียงเม็ดเล็กจำนวนมาก (หิมะ กรวด ใบไม้ ไฟปะทุ หยดฝน)
  - filtered noise + LFO: ลม ทะเล ไฟ (ลมกระโชก คลื่นซัด เปลวไฟพลิ้ว)
  - formant / source-filter: เสียงร้อง (ผู้เล่นเจ็บ บอสคำราม กบ)
  - Karplus–Strong: สายธนู
  - convolution reverb กับ impulse response สังเคราะห์ (ห้อง/ถ้ำ/ภายนอก) สเตอริโอแยกซ้ายขวา
ผลลัพธ์ (44.1 kHz 16-bit)
  sfx_<ชื่อ>.wav      เอฟเฟกต์ 30 เสียง (สเตอริโอ)
  step_<พื้น>_<n>.wav เสียงเดิน 7 พื้นผิว × 5 แบบ (โมโน)
  amb_<ชื่อ>.wav      เสียงบรรยากาศวนลูปไร้รอยต่อ 11 แบบ (สเตอริโอ 12 วินาที)
"""
import os, wave
import numpy as np
from scipy import signal

SR = 44100
rng = np.random.default_rng(2026)


# ------------------------------------------------------------------ พื้นฐาน
def T(d):
    return np.arange(int(d * SR)) / SR


def sos_filt(x, kind, f, order=2):
    nyq = SR / 2
    if kind == "band":
        lo, hi = max(10, f[0]), min(nyq * 0.98, f[1])
        sos = signal.butter(order, [lo / nyq, hi / nyq], "bandpass", output="sos")
    else:
        sos = signal.butter(order, min(f, nyq * 0.98) / nyq, kind, output="sos")
    return signal.sosfilt(sos, x)


def lp(x, f, o=2): return sos_filt(x, "lowpass", f, o)
def hp(x, f, o=2): return sos_filt(x, "highpass", f, o)
def bp(x, lo, hi, o=2): return sos_filt(x, "band", (lo, hi), o)


def white(d): return rng.standard_normal(int(d * SR))


def tv_band(x, centers, q=1.2, floor=0.0):
    """แบนด์พาสที่ความถี่กลางเปลี่ยนตามเวลา แบบไม่มีเสียงคลิก (กรองในโดเมน STFT ด้วยหน้าต่างเกาส์เซียนบนแกน log-ความถี่)
    centers = อาเรย์ความถี่กลางยาวเท่า x · q = ความกว้างแถบ (อ็อกเทฟ)"""
    nper = 1024
    f, t, Z = signal.stft(x, SR, nperseg=nper, noverlap=nper * 3 // 4)
    idx = np.clip((t * SR).astype(int), 0, len(x) - 1)
    c = np.asarray(centers)[idx]
    lf = np.log2(np.maximum(f, 1.0))[:, None]
    lc = np.log2(np.maximum(c, 1.0))[None, :]
    M = np.exp(-0.5 * ((lf - lc) / (q / 2.355)) ** 2) + floor
    _, y = signal.istft(Z * M, SR, nperseg=nper, noverlap=nper * 3 // 4)
    return pad(y, len(x) / SR)[:len(x)]


def pink(n):
    """pink noise (1/f) ด้วยการกรองในโดเมนความถี่"""
    X = np.fft.rfft(rng.standard_normal(n))
    f = np.fft.rfftfreq(n, 1 / SR)
    f[0] = 1
    X /= np.sqrt(f)
    y = np.fft.irfft(X, n)
    return y / (np.abs(y).max() + 1e-9)


def brown(n):
    X = np.fft.rfft(rng.standard_normal(n))
    f = np.fft.rfftfreq(n, 1 / SR)
    f[0] = 1
    X /= f
    y = np.fft.irfft(X, n)
    return y / (np.abs(y).max() + 1e-9)


def smooth_rand(n, rate, lo=0.0, hi=1.0):
    """สัญญาณสุ่มเปลี่ยนช้าๆ (LFO แบบไม่เป็นคาบ) — rate = จุดควบคุมต่อวินาที"""
    k = max(4, int(n / SR * rate) + 4)
    pts = rng.uniform(lo, hi, k)
    x = np.linspace(0, k - 3, n)
    i = np.floor(x).astype(int)
    fr = x - i
    # Catmull-Rom ให้เรียบ
    p0, p1, p2, p3 = pts[i], pts[i + 1], pts[i + 2], pts[np.minimum(i + 3, k - 1)]
    return 0.5 * ((2 * p1) + (-p0 + p2) * fr + (2 * p0 - 5 * p1 + 4 * p2 - p3) * fr ** 2 + (-p0 + 3 * p1 - 3 * p2 + p3) * fr ** 3)


def adsr(n, a=0.002, d=0.05, s=0.0, r=0.0, curve=4.0):
    t = np.arange(n) / SR
    e = np.where(t < a, t / max(a, 1e-6), 1.0)
    dec = np.exp(-np.maximum(t - a, 0) * curve / max(d, 1e-6))
    return e * (s + (1 - s) * dec)


def expdec(n, tau, att=0.001):
    t = np.arange(n) / SR
    return np.minimum(1, t / max(att, 1e-6)) * np.exp(-t / tau)


def modal(freqs, taus, amps, d, jitter=0.0, att=0.0005):
    """เสียงกระทบวัสดุ: ไซน์หลายความถี่ จางคนละอัตรา"""
    t = T(d)
    y = np.zeros_like(t)
    for f, tau, a in zip(freqs, taus, amps):
        f = f * (1 + rng.uniform(-jitter, jitter))
        if f >= SR * 0.45:
            continue
        y += a * np.sin(2 * np.pi * f * t + rng.uniform(0, 2 * np.pi)) * np.exp(-t / tau)
    y *= np.minimum(1, t / att)
    k = max(2, int(len(t) * 0.2))
    y[-k:] *= np.cos(np.linspace(0, np.pi / 2, k)) ** 2  # จางท้ายให้สนิท
    return y


def sweep(d, f0, f1, curve="exp"):
    t = T(d)
    f = f0 * (f1 / f0) ** (t / d) if curve == "exp" else f0 + (f1 - f0) * t / d
    return np.sin(2 * np.pi * np.cumsum(f) / SR)


def put(dst, src, at, gain=1.0):
    i = int(at * SR)
    if i >= len(dst) or i + len(src) <= 0:
        return dst
    s0 = max(0, -i)
    i = max(0, i)
    m = min(len(src) - s0, len(dst) - i)
    seg = src[s0:s0 + m] * gain
    # เรียวปลาย 3 ms กันเสียงคลิกจากชิ้นเสียงที่ถูกตัดกลางคลื่น
    k = min(len(seg), int(0.003 * SR))
    if k > 1:
        seg = seg.copy()
        tap = np.cos(np.linspace(0, np.pi / 2, k)) ** 2
        seg[-k:] *= tap[:, None] if seg.ndim == 2 else tap
    dst[i:i + m] += seg
    return dst


def pad(x, d):
    n = int(d * SR)
    return np.concatenate([x, np.zeros(max(0, n - len(x)))])[:n] if n else x


def mix(*xs):
    n = max(len(x) for x in xs)
    o = np.zeros(n)
    for x in xs:
        o[:len(x)] += x
    return o


def softclip(x, drive=1.5):
    return np.tanh(x * drive) / np.tanh(drive)


def fade(x, fin=0.002, fout=0.02):
    n = len(x)
    a, b = int(fin * SR), int(fout * SR)
    e = np.ones(n)
    if a: e[:a] = np.linspace(0, 1, a)
    if b: e[-b:] *= np.linspace(1, 0, b) ** 2
    return x * (e[:, None] if x.ndim == 2 else e)


# ------------------------------------------------------------------ reverb
_IR = {}


def ir(kind):
    """impulse response สังเคราะห์: สะท้อนต้นๆ + หางที่จางแบบเอกซ์โปเนนเชียล ซ้าย/ขวาไม่เหมือนกัน"""
    if kind in _IR:
        return _IR[kind]
    rt, pre, bright, er = {"small": (0.35, 0.004, 6000, 6), "room": (0.8, 0.008, 5000, 10), "hall": (1.8, 0.015, 4500, 14),
                           "cave": (3.2, 0.03, 2500, 18), "out": (0.6, 0.02, 7000, 4)}[kind]
    n = int((rt * 1.3 + pre) * SR)
    out = []
    for ch in range(2):
        t = np.arange(n) / SR
        tail = rng.standard_normal(n) * np.exp(-t * 6.9 / rt)
        tail = lp(tail, bright) * (t > pre)
        # ความถี่สูงจางเร็วกว่า: ผสมหางที่กรองต่ำลงเรื่อยๆ
        tail = tail * 0.6 + lp(tail, bright * 0.35) * 0.4
        for k in range(er):
            put(tail, np.array([rng.uniform(0.3, 0.9) * (-1) ** rng.integers(2)]), pre + rng.uniform(0.002, 0.06 + rt * 0.02))
        tail[0] = 0
        out.append(tail / np.abs(tail).sum() * 6)
    _IR[kind] = out
    return out


def reverb(x, kind="room", wet=0.25):
    """คืนสเตอริโอ (n,2)"""
    if x.ndim == 1:
        x = np.stack([x, x], 1)
    L, R = ir(kind)
    n = len(x)
    wl = signal.fftconvolve(x[:, 0], L)[:n + len(L)]
    wr = signal.fftconvolve(x[:, 1], R)[:n + len(R)]
    m = len(wl)
    dry = np.zeros((m, 2))
    dry[:n] = x
    return dry * (1 - wet * 0.5) + np.stack([wl, wr], 1) * wet


def stereo(x, width=0.0006):
    """โมโน → สเตอริโอ (หน่วงช่องขวานิดเดียวให้กว้างขึ้น)"""
    d = int(width * SR)
    R = np.concatenate([np.zeros(d), x])[:len(x)]
    return np.stack([x, R], 1)


def dcfix(x):
    return hp(x, 22, 2) if x.ndim == 1 else np.stack([hp(x[:, c], 22, 2) for c in range(x.shape[1])], 1)


def norm(x, peak=0.89):
    x = dcfix(x)
    m = np.abs(x).max()
    return x / m * peak if m > 0 else x


def trim(x, thresh=1e-4, tail=0.02):
    """ตัดความเงียบท้ายไฟล์"""
    a = np.abs(x if x.ndim == 1 else x.max(1))
    idx = np.where(a > thresh * a.max())[0]
    if len(idx) == 0:
        return x
    e = min(len(x), idx[-1] + int(tail * SR))
    return x[:e]


# ------------------------------------------------------------------ ชิ้นส่วนเสียง
def thump(f0=90, f1=45, d=0.12, tau=0.04):
    """แรงกระแทกต่ำ (ส้นเท้า ตัวกระแทก)"""
    return sweep(d, f0, f1) * expdec(int(d * SR), tau, 0.001)


def click(d=0.006, lo=2000, hi=12000):
    n = int(d * SR)
    return bp(rng.standard_normal(n), lo, hi) * np.exp(-np.arange(n) / (n / 4 + 1))


def grains(d, rate, lo, hi, gdur=(0.001, 0.004), shape=None, amp=(0.3, 1.0), order=2):
    """เม็ดเสียงเล็กๆ จำนวนมาก (หิมะ ใบไม้ ไฟปะทุ หยดน้ำ) — rate = เม็ด/วินาที, shape = ความหนาแน่นตามเวลา"""
    n = int(d * SR)
    y = np.zeros(n)
    if shape is None:
        shape = np.ones(n)
    k = int(rate * d * 1.5) + 1
    pos = rng.uniform(0, n, k).astype(int)
    keep = rng.uniform(0, 1, k) < shape[np.minimum(pos, n - 1)] / max(shape.max(), 1e-9)
    for p in pos[keep]:
        g = int(rng.uniform(*gdur) * SR) + 2
        dens = shape[min(p, n - 1)] / max(shape.max(), 1e-9)
        w = rng.standard_normal(g) * np.hanning(g) * rng.uniform(*amp) * (0.35 + 0.65 * dens)
        put(y, w, p / SR)
    return bp(y, lo, hi, order)


def karplus(f, d, damp=0.996, bright=0.5):
    n = int(d * SR)
    p = int(SR / f)
    buf = rng.uniform(-1, 1, p)
    buf = lp(buf, 2000 + bright * 8000)
    y = np.zeros(n)
    for i in range(n):
        y[i] = buf[i % p]
        buf[i % p] = damp * 0.5 * (buf[i % p] + buf[(i + 1) % p])
    return y


def formant_voice(d, f0_curve, formants, breath=0.15, jitter=0.01):
    """แหล่งเสียงกล่องเสียง (พัลส์) ผ่านฟอร์แมนต์ — เสียงร้อง"""
    n = int(d * SR)
    f0 = np.interp(np.linspace(0, 1, n), np.linspace(0, 1, len(f0_curve)), f0_curve)
    f0 = f0 * (1 + jitter * smooth_rand(n, 30, -1, 1))
    ph = np.cumsum(f0 / SR)
    src = (ph % 1.0) * 2 - 1  # ฟันเลื่อย ~ พัลส์กล่องเสียง
    src = src - lp(src, 80)
    src += breath * rng.standard_normal(n)
    y = np.zeros(n)
    for fc, bw, g in formants:
        y += g * bp(src, fc - bw / 2, fc + bw / 2, 2)
    return y


def bubbles(d, count, f_lo=300, f_hi=1500, dur=(0.01, 0.04)):
    y = np.zeros(int(d * SR))
    for _ in range(count):
        bd = rng.uniform(*dur)
        f = rng.uniform(f_lo, f_hi)
        b = sweep(bd, f, f * rng.uniform(1.3, 2.2)) * expdec(int(bd * SR), bd / 3)
        put(y, b, rng.uniform(0, d - bd), rng.uniform(0.2, 1))
    return y


def loopify(x, xf=1.5):
    """ทำให้วนลูปไร้รอยต่อ: เอาท้ายมาเฟดซ้อนกับต้น (equal power)"""
    n = int(xf * SR)
    body = x[:-n].copy()
    tail = x[-n:]
    a = np.linspace(0, np.pi / 2, n)
    if x.ndim == 2:
        a = a[:, None]
    body[:n] = body[:n] * np.sin(a) + tail * np.cos(a)
    return body


# ------------------------------------------------------------------ SFX
def sfx_bank():
    B = {}
    # ฟันดาบ: ลมวูบ (แบนด์พาสกวาดขึ้นลง) + เสียงโลหะแว้บ
    d = 0.26
    n = int(d * SR)
    w = white(d)
    t = T(d)
    env = np.sin(np.pi * np.clip(t / d, 0, 1)) ** 2.2
    cen = 450 + 1900 * np.sin(np.pi * np.clip(t / d * 1.1, 0, 1))
    y = tv_band(w, cen, 1.1)
    B["swing"] = reverb(mix(y * env, modal([3150, 4720], [0.08, 0.05], [0.04, 0.02], d) * env), "small", 0.15)

    # โดนศัตรู: ตุบเนื้อ + แปะ + ดาบสั่นเบาๆ
    B["hit"] = reverb(mix(thump(140, 60, 0.18, 0.05) * 0.9, bp(white(0.05), 300, 1800) * expdec(int(0.05 * SR), 0.012) * 0.7,
                          click(0.01, 2000, 9000) * 0.5, modal([2380, 3720], [0.12, 0.07], [0.05, 0.03], 0.3)), "small", 0.2)

    # ตัดไม้: ขวานสับไม้ — โหมดไม้ + เสียงแตกของเสี้ยน
    B["chop"] = reverb(mix(modal([180, 420, 980, 1650, 2400], [0.06, 0.05, 0.03, 0.02, 0.015], [0.6, 1.0, 0.6, 0.35, 0.2], 0.4, 0.03),
                           click(0.004, 3000, 12000) * 0.8, grains(0.12, 600, 1500, 7000, shape=np.exp(-T(0.12) * 30)) * 0.4), "out", 0.2)

    # ขุดหิน: ปลายพลั่วโลหะกระทบหิน + เศษหินร่วง
    crumble = np.zeros(int(0.5 * SR))
    put(crumble, grains(0.35, 300, 1200, 6000, (0.002, 0.008), np.exp(-T(0.35) * 9)), 0.03, 0.5)
    B["rock"] = reverb(mix(modal([2100, 3350, 4800, 6900], [0.05, 0.035, 0.025, 0.015], [0.8, 0.6, 0.4, 0.2], 0.4, 0.02),
                           thump(110, 60, 0.12, 0.03) * 0.6, click(0.004, 4000, 14000), crumble), "out", 0.22)

    # ผู้เล่นเจ็บ: ตัวกระแทก + เสียง "อึ๊ก"
    v = formant_voice(0.28, [175, 190, 150, 120], [(650, 160, 1.0), (1150, 200, 0.6), (2500, 300, 0.25)], 0.2) * adsr(int(0.28 * SR), 0.01, 0.12, 0, 0, 3)
    B["hurt"] = reverb(mix(thump(120, 55, 0.15, 0.04) * 0.8, v * 0.9), "small", 0.18)

    # มอนตาย: พรึ่บเป็นควัน + เสียงหยุ่นๆ + ประกายรางวัล
    puff = lp(white(0.5), 1800) * adsr(int(0.5 * SR), 0.01, 0.18, 0, 0, 4)
    squish = bp(white(0.25), 200, 900) * (0.6 + 0.4 * np.sin(2 * np.pi * 22 * T(0.25))) * expdec(int(0.25 * SR), 0.07)
    sparkle = np.zeros(int(0.9 * SR))
    for i, f in enumerate([1760, 2349, 2794]):
        put(sparkle, modal([f, f * 2.76], [0.25, 0.1], [0.15, 0.05], 0.5), 0.12 + i * 0.06)
    B["kill"] = reverb(mix(puff, squish * 0.7, bubbles(0.3, 8, 250, 700) * 0.3, sparkle), "room", 0.22)

    # อัญมณี: กระดิ่งแก้ว (สัดส่วนฮาร์โมนิกแบบระฆัง) 2 โน้ต
    bell = lambda f, d=1.2, a=1.0: modal([f, f * 2.76, f * 5.40, f * 8.93], [0.6, 0.35, 0.18, 0.08], [a, a * 0.45, a * 0.25, a * 0.12], d, 0.002)
    g = np.zeros(int(1.6 * SR))
    put(g, bell(1568), 0)
    put(g, bell(2093), 0.09)
    put(g, bell(3136, 1.0, 0.4), 0.18)
    B["gem"] = reverb(g, "hall", 0.35)

    # เลเวลอัป: อาร์เปจจิโอระฆัง + คอร์ดเครื่องเป่า
    def brass(f, d, a=0.3):
        tt = T(d)
        y = sum((1 / h) * np.sin(2 * np.pi * f * h * tt * (1 + 0.003 * np.sin(2 * np.pi * 5 * tt))) for h in range(1, 9))
        return lp(y, 2500) * adsr(int(d * SR), 0.04, d, 0.6, 0, 2) * a * fade(np.ones(int(d * SR)), 0.01, 0.25)
    lv = np.zeros(int(2.4 * SR))
    for i, f in enumerate([523.25, 659.25, 783.99, 1046.5]):
        put(lv, bell(f, 1.2, 0.8), i * 0.09)
    for f in [523.25, 659.25, 783.99]:
        put(lv, brass(f, 1.2, 0.18), 0.38)
    B["lvl"] = reverb(lv, "hall", 0.3)

    # คราฟต์: ค้อนเคาะ 2 ที + กระดิ่งสำเร็จ
    hit_m = lambda: mix(modal([880, 2350, 3900, 5600], [0.15, 0.08, 0.05, 0.03], [0.5, 0.7, 0.4, 0.2], 0.5, 0.02), click(0.003, 3000, 12000) * 0.6, thump(150, 80, 0.08, 0.02) * 0.4)
    cr = np.zeros(int(1.3 * SR))
    put(cr, hit_m(), 0)
    put(cr, hit_m(), 0.17, 0.9)
    put(cr, bell(1318.5, 1.0, 0.6), 0.36)
    B["craft"] = reverb(cr, "room", 0.22)

    # กิน: เคี้ยวกรุบ 3 ครั้ง + กลืน
    e = np.zeros(int(0.85 * SR))
    for i in range(3):
        bite = mix(grains(0.09, 1800, 1200, 7000, (0.0005, 0.002), np.exp(-T(0.09) * 25)), lp(white(0.08), 600) * expdec(int(0.08 * SR), 0.02) * 0.4)
        put(e, bite, i * 0.16 + rng.uniform(0, 0.02), 0.9 - i * 0.15)
    gulp = formant_voice(0.14, [260, 160], [(350, 150, 1.0), (900, 200, 0.4)], 0.05) * adsr(int(0.14 * SR), 0.01, 0.06, 0, 0, 3)
    put(e, gulp, 0.6, 0.5)
    B["eat"] = reverb(e, "small", 0.12)

    # จุดไฟ: เสียงขีด + วูบ + ปะทุ
    fl = np.zeros(int(1.2 * SR))
    put(fl, hp(white(0.07), 2500) * expdec(int(0.07 * SR), 0.02), 0, 0.6)
    tt = T(0.7)
    wh = bp(white(0.7), 250, 1400) * np.sin(np.pi * np.clip(tt / 0.7, 0, 1)) ** 1.5 * (0.7 + 0.3 * smooth_rand(len(tt), 25))
    put(fl, wh, 0.05)
    put(fl, grains(1.0, 25, 1500, 7000, (0.0008, 0.003), np.linspace(1, 0.3, int(SR))) * 1.5, 0.15)
    put(fl, lp(brown(int(0.9 * SR)), 300) * np.linspace(0, 1, int(0.9 * SR)) ** 0.5 * 0.5, 0.2)
    B["fire"] = reverb(fl, "out", 0.15)

    # เปิดหีบ: บานพับไม้เอี๊ยด + กลอนคลิก + ประกาย
    def creak(d, f0=40, f1=70, res=(700, 1800)):
        n = int(d * SR)
        rate = np.interp(np.arange(n), [0, n // 2, n], [f0, f1, f0 * 0.8]) * (1 + 0.15 * smooth_rand(n, 12, -1, 1))
        ph = np.cumsum(rate / SR)
        pulses = (np.diff(np.floor(ph), prepend=0) > 0).astype(float) * rng.uniform(0.5, 1, n)
        y = sum(bp(pulses, r * 0.85, r * 1.15) for r in res)
        return y * np.sin(np.pi * np.clip(np.arange(n) / n, 0, 1))
    ch = np.zeros(int(1.6 * SR))
    put(ch, creak(0.45) * 3, 0)
    put(ch, click(0.008, 1500, 6000) * 0.8, 0.46)
    put(ch, modal([210, 480, 900], [0.06, 0.04, 0.02], [0.6, 0.4, 0.2], 0.2) * 0.6, 0.47)
    for i, f in enumerate([880, 1108.7, 1318.5, 1760]):
        put(ch, bell(f, 0.9, 0.35), 0.55 + i * 0.07)
    B["chest"] = reverb(ch, "room", 0.25)

    # ยิงธนู: สายดีด + ลูกศรแหวกอากาศ
    tw = karplus(196, 0.35, 0.985, 0.4) * expdec(int(0.35 * SR), 0.1)
    tt = T(0.25)
    whz = bp(white(0.25), 2000, 7000) * np.exp(-((tt - 0.06) / 0.05) ** 2)
    B["shoot"] = reverb(mix(tw * 0.8, whz * 0.5, click(0.003, 2000, 9000) * 0.5), "out", 0.15)

    # ระเบิด: แตกเปรี้ยง + เนื้อเสียงต่ำ + ซับเบส + เศษตก
    bm = np.zeros(int(3.0 * SR))
    put(bm, hp(white(0.03), 800) * expdec(int(0.03 * SR), 0.008), 0, 1.0)
    body = lp(white(2.0), 900) * expdec(int(2.0 * SR), 0.35, 0.004)
    put(bm, body, 0.005, 1.2)
    sub = lp(brown(int(2.5 * SR)), 80) * expdec(int(2.5 * SR), 0.6, 0.01) * 2.5
    put(bm, sub, 0)
    put(bm, grains(1.6, 40, 800, 5000, (0.002, 0.01), np.exp(-T(1.6) * 2.5)) * 0.8, 0.25)
    B["boom"] = reverb(softclip(bm * 1.2, 1.3), "hall", 0.3)

    # ซื้อของ: เหรียญกระทบกัน + กระดิ่ง
    by = np.zeros(int(1.2 * SR))
    for i in range(6):
        cf = rng.uniform(2800, 4200)
        put(by, modal([cf, cf * 1.47, cf * 2.09, cf * 2.64], [0.12, 0.08, 0.05, 0.03], [0.6, 0.4, 0.3, 0.15], 0.3, 0.02), 0.03 * i + rng.uniform(0, 0.025), rng.uniform(0.4, 0.9))
    put(by, bell(1760, 1.0, 0.5), 0.22)
    B["buy"] = reverb(by, "room", 0.2)

    # ประตู/วางของ: เอี๊ยดสั้น + ตึ๊กไม้ + กลอน
    dr = np.zeros(int(0.9 * SR))
    put(dr, creak(0.22, 55, 85, (600, 1500)) * 2.2, 0)
    put(dr, modal([140, 310, 520, 870], [0.09, 0.06, 0.04, 0.02], [0.9, 0.6, 0.35, 0.2], 0.45, 0.02), 0.2)
    put(dr, thump(100, 60, 0.1, 0.03) * 0.6, 0.2)
    put(dr, click(0.005, 2000, 8000) * 0.5, 0.21)
    B["door"] = reverb(dr, "room", 0.2)

    # พุ่งตัว: ลมวูบ + ผ้าสะบัด + เท้าครูด
    tt = T(0.32)
    wv = bp(white(0.32), 400, 2600) * np.exp(-((tt - 0.08) / 0.07) ** 2) + bp(white(0.32), 150, 500) * np.exp(-((tt - 0.07) / 0.09) ** 2) * 0.6
    cloth = grains(0.2, 900, 1500, 6000, (0.001, 0.003), np.exp(-T(0.2) * 15)) * 0.4
    B["dash"] = reverb(mix(wv, cloth), "out", 0.15)

    # คริติคอล: เสียงดาบ "ชิ้ง" ใสยาว + กระแทกหนัก
    B["crit"] = reverb(mix(modal([3100, 4650, 7300, 9800], [0.5, 0.3, 0.15, 0.08], [0.5, 0.35, 0.2, 0.1], 0.9, 0.005),
                           thump(160, 50, 0.2, 0.05), hp(white(0.04), 3000) * expdec(int(0.04 * SR), 0.01) * 0.7), "room", 0.25)

    # เก็บของ: ป๊อปนุ่มๆ + ประกายเล็ก
    pk = mix(sweep(0.12, 520, 1150) * expdec(int(0.12 * SR), 0.04, 0.002) * 0.7, pad(np.zeros(1), 0.04))
    pk2 = np.zeros(int(0.5 * SR))
    put(pk2, pk, 0)
    put(pk2, bell(2637, 0.4, 0.2), 0.05)
    B["pickup"] = reverb(pk2, "small", 0.15)

    # อัปเกรด: ทั่งดัง + ประกายไล่ขึ้น
    up = np.zeros(int(2.4 * SR))
    put(up, hit_m(), 0)
    for i, f in enumerate([783.99, 987.77, 1174.66, 1567.98, 1975.5, 2349.3]):
        put(up, bell(f, 1.0, 0.45), 0.2 + i * 0.07)
    tt = T(1.2)
    put(up, hp(white(1.2), 5000) * np.sin(np.pi * tt / 1.2) * 0.025, 0.2)
    B["upgrade"] = reverb(up, "hall", 0.32)

    # บอสคำราม: ฟันเลื่อยต่ำสั่นไม่สม่ำเสมอ ผ่านฟอร์แมนต์ "อ้า" + ลมหายใจ + ซับ
    n = int(1.8 * SR)
    f0c = [70, 95, 110, 100, 80, 60]
    rv = formant_voice(1.8, f0c, [(500, 200, 1.0), (900, 250, 0.8), (2200, 400, 0.3)], 0.6, 0.06)
    am = 0.75 + 0.25 * np.sin(2 * np.pi * 27 * T(1.8))
    rv = rv * am * adsr(n, 0.15, 1.2, 0.5, 0, 1.5) * fade(np.ones(n), 0.01, 0.5)
    B["roar"] = reverb(fade(softclip(mix(rv, lp(brown(n), 120) * adsr(n, 0.1, 1.5, 0.4, 0, 2) * 0.8), 2.2), 0.01, 0.4), "cave", 0.35)

    # บอสฟาด: กระแทกพื้นหนัก + พื้นแตก + ฝุ่น
    ba = np.zeros(int(1.8 * SR))
    put(ba, thump(70, 32, 0.6, 0.18) * 1.4, 0)
    put(ba, lp(white(0.6), 1200) * expdec(int(0.6 * SR), 0.12), 0, 0.9)
    put(ba, grains(1.0, 60, 600, 4000, (0.003, 0.012), np.exp(-T(1.0) * 4)) * 0.7, 0.05)
    B["battk"] = reverb(softclip(ba, 1.4), "hall", 0.28)

    # เควสต์: ระนาดไม้ (โหมดแท่งไม้)
    mar = lambda f, a=0.7: modal([f, f * 3.93, f * 9.2], [0.35, 0.08, 0.03], [a, a * 0.3, a * 0.1], 0.8, 0.002)
    q = np.zeros(int(1.6 * SR))
    for at, f in [(0, 783.99), (0.12, 783.99), (0.24, 987.77), (0.42, 1174.66)]:
        put(q, mar(f), at)
    put(q, bell(1174.66, 1.0, 0.25), 0.42)
    B["quest"] = reverb(q, "room", 0.25)

    # กับดัก: สปริงดีด + โลหะสั่น
    tr = np.zeros(int(0.9 * SR))
    put(tr, click(0.006, 1500, 12000) * 1.2, 0)
    tt = T(0.6)
    spring = np.sin(2 * np.pi * np.cumsum(650 + 250 * np.sin(2 * np.pi * 30 * tt) * np.exp(-tt * 8)) / SR) * expdec(len(tt), 0.12)
    put(tr, spring * 0.5, 0.005)
    put(tr, modal([1450, 2900, 4300], [0.15, 0.08, 0.05], [0.5, 0.3, 0.2], 0.5), 0.005, 0.6)
    B["trap"] = reverb(tr, "small", 0.18)

    # ลูกไฟ: เปลวไฟพุ่ง (นอยส์สั่นแบบเปลวไฟ) + ปะทุ
    tt = T(0.7)
    flame = bp(white(0.7), 300, 2500) * (0.6 + 0.4 * smooth_rand(len(tt), 40)) * np.sin(np.pi * np.clip(tt / 0.7, 0, 1)) ** 1.2
    flame += lp(brown(len(tt)), 250) * np.sin(np.pi * np.clip(tt / 0.7, 0, 1)) * 0.8
    B["fball"] = reverb(mix(flame, grains(0.7, 30, 1800, 7000, (0.0008, 0.003)) * 1.2), "out", 0.2)

    # ปฏิเสธ: ตอกไม้ 2 เสียงลง (นุ่ม ไม่บาดหู)
    dn = np.zeros(int(0.6 * SR))
    put(dn, mar(523.25, 0.6), 0)
    put(dn, mar(392.0, 0.6), 0.11)
    B["deny"] = reverb(dn, "small", 0.15)

    # คลิก UI: ติ๊กนุ่ม
    B["click"] = reverb(mix(modal([2200, 4100], [0.012, 0.006], [0.6, 0.3], 0.06), click(0.002, 3000, 10000) * 0.3), "small", 0.08)

    # ---- ใหม่
    # ฟ้าร้อง: เปรี้ยง + ครืนยาวกลิ้งไปมา
    th = np.zeros(int(7.0 * SR))
    put(th, hp(white(0.15), 600) * expdec(int(0.15 * SR), 0.04, 0.002) * 0.8, 0.0)
    n = int(6.5 * SR)
    rumble = lp(brown(n), 160) * 1.6
    rolls = np.zeros(n)
    for k in range(7):
        c = rng.uniform(0.1, 4.5)
        rolls += rng.uniform(0.3, 1) * np.exp(-((T(6.5) - c) / rng.uniform(0.25, 0.8)) ** 2)
    rolls = np.maximum(rolls, 0.15) * expdec(n, 2.5, 0.05)
    put(th, rumble * rolls + bp(white(6.5), 150, 900) * rolls * 0.25, 0.05)
    B["thunder"] = reverb(softclip(th, 1.2), "hall", 0.45)

    # น้ำกระเซ็น
    sp = np.zeros(int(0.9 * SR))
    put(sp, bp(white(0.35), 600, 6000) * expdec(int(0.35 * SR), 0.08, 0.003), 0)
    put(sp, bubbles(0.6, 22, 350, 1800), 0.03, 0.45)
    put(sp, lp(white(0.2), 400) * expdec(int(0.2 * SR), 0.05) * 0.5, 0)
    B["splash"] = reverb(sp, "out", 0.2)

    # ค้อนตอกตะปู (ตอนก่อสร้าง)
    hm = mix(modal([260, 610, 1180, 2050], [0.07, 0.05, 0.03, 0.02], [0.8, 0.5, 0.3, 0.15], 0.35, 0.03),
             modal([2950, 5100], [0.09, 0.05], [0.25, 0.12], 0.3), click(0.003, 2500, 12000) * 0.7, thump(130, 70, 0.08, 0.02) * 0.5)
    B["hammer"] = reverb(hm, "out", 0.2)

    out = {}
    for k, v in B.items():
        v = trim(v if v.ndim == 2 else stereo(v))
        out[k] = norm(fade(v, 0.0005, 0.03), 0.85)
    return out


# ------------------------------------------------------------------ เสียงเดิน
def footstep(surface, i):
    d = 0.42
    y = np.zeros(int(d * SR))
    heel = thump(rng.uniform(90, 130), rng.uniform(45, 60), 0.12, rng.uniform(0.02, 0.035))
    if surface == "grass":
        put(y, heel * 0.5, 0)
        sh = np.exp(-((T(0.25) - 0.04) / 0.05) ** 2)
        put(y, grains(0.25, 3500, 900, 6500, (0.0006, 0.0025), sh, order=2) * 0.9, 0.0)
        put(y, bp(white(0.15), 500, 2500) * np.exp(-((T(0.15) - 0.05) / 0.04) ** 2) * 0.3, 0)
    elif surface == "dirt":
        put(y, heel * 0.8, 0)
        put(y, bp(white(0.12), 300, 2500) * expdec(int(0.12 * SR), 0.025) * 0.5, 0.005)
        put(y, grains(0.15, 600, 800, 4000, (0.001, 0.004), np.exp(-T(0.15) * 18)) * 0.4, 0.01)
    elif surface == "sand":
        put(y, heel * 0.35, 0)
        sh = np.exp(-((T(0.32) - 0.08) / 0.07) ** 2)
        put(y, grains(0.32, 6000, 1800, 8000, (0.0003, 0.0012), sh) * 0.7, 0)
        put(y, bp(white(0.32), 600, 4000) * sh * 0.3, 0)
    elif surface == "snow":
        put(y, lp(heel, 300) * 0.5, 0)
        # หิมะถูกบีบอัด: เม็ดกรอบถี่ขึ้นแล้วซาลง (crunch)
        sh = np.interp(T(0.3), [0, 0.03, 0.12, 0.3], [0, 1, 0.6, 0])
        put(y, grains(0.3, 2500, 600, 4500, (0.001, 0.005), sh, (0.2, 1.0), order=4) * 1.3, 0.005)
        put(y, lp(white(0.25), 1200) * sh[:int(0.25 * SR)] * 0.25, 0.005)
    elif surface == "stone":
        put(y, heel * 0.7, 0)
        put(y, click(0.004, 2500, 10000) * 0.6, 0.002)
        put(y, modal([rng.uniform(900, 1300), rng.uniform(2200, 2800)], [0.02, 0.012], [0.25, 0.12], 0.08), 0.002)
        put(y, grains(0.08, 400, 2000, 8000, (0.0008, 0.003), np.exp(-T(0.08) * 30)) * 0.3, 0.01)
    elif surface == "wood":
        put(y, modal([rng.uniform(150, 175), rng.uniform(320, 360), rng.uniform(580, 650), rng.uniform(1050, 1200)], [0.06, 0.04, 0.025, 0.015], [0.8, 0.6, 0.35, 0.18], 0.3, 0.02), 0)
        put(y, click(0.003, 2500, 9000) * 0.45, 0)
        if i % 3 == 0:
            n2 = int(0.18 * SR)
            rate = 60 + 30 * smooth_rand(n2, 10, -1, 1)
            ph = np.cumsum(rate / SR)
            pul = (np.diff(np.floor(ph), prepend=0) > 0).astype(float)
            put(y, bp(pul, 900, 1300) * np.sin(np.pi * np.arange(n2) / n2) * 0.8, 0.05)
    elif surface == "mud":
        put(y, lp(heel, 250) * 0.6, 0)
        n2 = int(0.3 * SR)
        tt = np.arange(n2) / SR
        suck = tv_band(rng.standard_normal(n2), 300 + 700 * tt / 0.3, 0.6)
        put(y, suck * np.sin(np.pi * np.clip(tt / 0.3, 0, 1)) * 0.9, 0.02)
        put(y, bubbles(0.25, 5, 150, 500, (0.02, 0.05)) * 0.6, 0.05)
    # เท้าย่ำพื้นจริงพลังงานส่วนใหญ่อยู่ต่ำกว่า ~6 kHz — ตัดความซ่าแหลมออก (ยกเว้นหินที่มีเสียงคลิกส้นรองเท้า)
    y = lp(y, 9000 if surface == "stone" else 6500, 2)
    y = trim(y)
    y = reverb(y, "small", 0.06)[:, 0]
    return norm(fade(y, 0.0005, 0.03), 0.8)


SURFACES = ["grass", "dirt", "sand", "snow", "stone", "wood", "mud"]


# ------------------------------------------------------------------ เสียงเดินของมอนสเตอร์/สัตว์ (v15.2)
MOBS = ["slime", "bone", "flap", "hoof", "paw", "stomp"]


def mob_step(kind, i):
    if kind == "slime":
        # สไลม์กระดึ๊บ: ก้อนวุ้นกระแทกพื้น (ไซน์ต่ำสั่นยวบ) + เสียงแฉะ + ฟองเล็ก
        d = 0.32
        t = T(d)
        f = rng.uniform(150, 210) * (1 - 0.45 * t / d) * (1 + 0.12 * np.sin(2 * np.pi * rng.uniform(16, 24) * t))
        blob = np.sin(2 * np.pi * np.cumsum(f) / SR) * expdec(len(t), 0.07, 0.004)
        sq = bp(white(d), 300, 1400) * expdec(len(t), 0.04, 0.003) * (0.6 + 0.4 * np.sin(2 * np.pi * 30 * t))
        y = mix(fade(blob, 0, 0.06), fade(sq, 0, 0.06) * 0.5, bubbles(0.2, 3, 300, 800, (0.015, 0.03)) * 0.25)
        room = "small"
    elif kind == "bone":
        # โครงกระดูก: กระดูกกระทบกันกรุกกริก 3–4 ครั้ง + เท้ากระดูกแตะพื้น
        y = np.zeros(int(0.25 * SR))
        put(y, thump(160, 90, 0.06, 0.012) * 0.5, 0)
        for k in range(rng.integers(3, 5)):
            f = rng.uniform(1600, 3200)
            put(y, modal([f, f * 1.83, f * 2.7], [0.012, 0.008, 0.005], [0.8, 0.4, 0.2], 0.05, 0.05), rng.uniform(0, 0.07), rng.uniform(0.4, 1))
        put(y, click(0.003, 2500, 9000) * 0.6, 0)
        room = "small"
    elif kind == "flap":
        # ปีกค้างคาว/นก: ลมตีพั่บ 2 จังหวะ (หนังปีก)
        y = np.zeros(int(0.22 * SR))
        for k, at in enumerate([0, rng.uniform(0.07, 0.1)]):
            n = int(0.07 * SR)
            tt = np.arange(n) / SR
            w = bp(rng.standard_normal(n), 250, 1600) * np.exp(-((tt - 0.02) / 0.012) ** 2)
            put(y, w, at, 1 - 0.35 * k)
        room = "out"
    elif kind == "hoof":
        # กีบ (กวาง/แอนทิโลป/หมูป่า): ต๊อกแข็ง 2 ที เหมือนเท้าหน้า-หลัง
        y = np.zeros(int(0.25 * SR))
        for k, at in enumerate([0, rng.uniform(0.06, 0.09)]):
            f = rng.uniform(380, 520)
            put(y, mix(modal([f, f * 2.3, f * 4.1], [0.025, 0.015, 0.008], [0.8, 0.4, 0.2], 0.08), click(0.003, 1500, 7000) * 0.5, thump(120, 70, 0.06, 0.015) * 0.5), at, 1 - 0.3 * k)
        room = "out"
    elif kind == "paw":
        # อุ้งเท้านุ่ม (หมาป่า/กระต่าย/จระเข้/สัตว์เลี้ยง): ตุบเบาๆ + ใบไม้/ดินกรุบนิดๆ
        y = mix(thump(rng.uniform(100, 140), 60, 0.08, 0.018) * 0.7, grains(0.1, 1500, 700, 4500, (0.0005, 0.002), np.exp(-T(0.1) * 30)) * 0.4)
        room = "out"
    else:
        # ย่ำหนัก (บอส): ตึงต่ำ + พื้นสะเทือน + เศษหินกรุ
        y = np.zeros(int(1.0 * SR))
        put(y, thump(rng.uniform(55, 70), 28, 0.5, 0.14) * 1.2, 0)
        put(y, lp(white(0.3), 700) * expdec(int(0.3 * SR), 0.06), 0, 0.6)
        put(y, grains(0.5, 50, 700, 4000, (0.002, 0.008), np.exp(-T(0.5) * 7)) * 0.6, 0.04)
        room = "room"
    y = lp(y, 9000)
    y = reverb(trim(y), room, 0.12)[:, 0]
    return norm(fade(trim(y), 0.0005, 0.03), 0.8)


# ------------------------------------------------------------------ บรรยากาศ (วนลูป)
def amb_bank(secs=12.0):
    D = secs + 1.5
    n = int(D * SR)
    A = {}

    def st(fnL, fnR=None):
        L = fnL()
        R = fnR() if fnR else fnL()
        return np.stack([L, R], 1)

    # ฝน: หยดฝนเป็นพันเม็ด (แบ่งกลุ่มตามระดับเสียง) + เสียงซ่าพื้นหลัง
    def rain_ch(heavy=1.0, roof=False):
        y = np.zeros(n)
        groups = 10
        for gi in range(groups):
            imp = np.zeros(n)
            k = int((900 if not roof else 260) * D / groups * heavy)
            pos = rng.integers(0, n, k)
            imp[pos] = rng.uniform(0.1, 1, k) ** 2
            if roof:
                f = rng.uniform(500, 2200)
                ker = modal([f, f * 1.7], [0.012, 0.007], [1, 0.4], 0.05)
            else:
                f = rng.uniform(2500, 9000)
                kd = 0.008
                ker = (sweep(kd, f, f * 1.4) * expdec(int(kd * SR), 0.002) + rng.standard_normal(int(kd * SR)) * expdec(int(kd * SR), 0.0008) * 0.5)
            y += signal.fftconvolve(imp, ker)[:n]
        bed = pink(n)
        bed = bp(bed, 300, 2800) if roof else bp(bed, 400, 6500)
        mod = 0.8 + 0.2 * smooth_rand(n, 0.4)
        y = y / (np.abs(y).max() + 1e-9) * (0.55 if not roof else 0.7) + bed / (np.abs(bed).max() + 1e-9) * (0.6 if not roof else 0.45) * mod
        y = lp(y, 6500, 4) if not roof else y
        if roof:
            y = lp(y, 3500)
            # รางน้ำ: เสียงน้ำไหลเป็นสายเบาๆ
            y += bubbles(D, int(14 * D), 600, 2200, (0.006, 0.02)) * 0.15
        return y
    A["rain"] = st(lambda: rain_ch())
    A["rain_roof"] = st(lambda: rain_ch(1.0, True))

    # ลม: นอยส์สีน้ำตาล/ชมพู ผ่านแบนด์พาสที่ความถี่กลางเลื่อนช้าๆ + กระโชก
    def wind_ch(strength=1.0, howl=False):
        base = pink(n) * 0.5 + brown(n) * 0.8
        gust = smooth_rand(n, 0.25, 0.25, 1.0) ** 1.5
        cen = 250 + 550 * smooth_rand(n, 0.12, 0, 1) * strength
        y = tv_band(base, cen, 1.6)
        y = y / (np.abs(y).max() + 1e-9) * gust
        if howl:
            cen2 = 450 + 600 * smooth_rand(n, 0.2, 0, 1)
            hw = tv_band(rng.standard_normal(n), cen2, 0.15)
            hw = hw / (np.abs(hw).max() + 1e-9) * gust ** 2 * 0.5
            y += hw + bp(rng.standard_normal(n), 3500, 8000) * 0.02 * gust
        return y
    A["wind"] = st(lambda: wind_ch(0.8))
    A["blizzard"] = st(lambda: wind_ch(1.4, True))

    # กองไฟ: ปะทุ (ปัวซอง เป็นชุดๆ) + ซ่า + เสียงไฟลุกต่ำ
    def fire_ch():
        y = np.zeros(n)
        burst = smooth_rand(n, 1.5, 0.2, 1.0) ** 2
        y += grains(D, 18, 1200, 7000, (0.0005, 0.0025), burst, (0.2, 1.0)) * 3.5
        pops = np.zeros(n)
        for _ in range(int(2.5 * D)):
            f = rng.uniform(600, 2500)
            pp = mix(click(0.004, 1500, 9000), modal([f], [0.01], [0.6], 0.04))
            put(pops, pp, rng.uniform(0, D - 0.05), rng.uniform(0.3, 1))
        y += pops * 0.9
        roar = lp(brown(n), 300) * (0.7 + 0.3 * smooth_rand(n, 6))
        y += roar / (np.abs(roar).max() + 1e-9) * 0.35
        y += hp(pink(n), 3000) * 0.04
        return y
    A["fire"] = st(fire_ch)

    # นกร้อง: หลายชนิด (จิ๊บ เสียงหวีด เสียงรัว) กระจายซ้ายขวา + ใบไม้ไหว
    def bird_call(kind):
        if kind == 0:   # จิ๊บๆ สั้น
            c = np.zeros(int(0.6 * SR))
            f = rng.uniform(3500, 5500)
            for j in range(rng.integers(2, 5)):
                cd = rng.uniform(0.04, 0.07)
                put(c, sweep(cd, f, f * rng.uniform(1.2, 1.6)) * np.hanning(int(cd * SR)), j * rng.uniform(0.09, 0.13))
            return c
        if kind == 1:   # หวีดยาวขึ้น-ลง
            cd = rng.uniform(0.35, 0.6)
            t = T(cd)
            f = rng.uniform(2200, 3200) + 900 * np.sin(np.pi * t / cd) + 120 * np.sin(2 * np.pi * 28 * t)
            return np.sin(2 * np.pi * np.cumsum(f) / SR) * np.hanning(len(t))
        cd = rng.uniform(0.4, 0.8)  # รัวเร็ว
        t = T(cd)
        f = rng.uniform(4000, 6000) - 1200 * t / cd
        am = (np.sin(2 * np.pi * rng.uniform(18, 30) * t) > 0.2).astype(float)
        return np.sin(2 * np.pi * np.cumsum(f) / SR) * lp(am, 400) * np.hanning(len(t))

    def birds_st():
        L = np.zeros(n)
        R = np.zeros(n)
        for _ in range(int(2.2 * D)):
            c = bird_call(rng.integers(0, 3)) * rng.uniform(0.2, 1)
            at = rng.uniform(0, D - 1)
            pan = rng.uniform(-1, 1)
            a = (pan + 1) * np.pi / 4
            put(L, c, at, np.cos(a))
            put(R, c, at + abs(pan) * 0.0005, np.sin(a))
        leaves = lambda: bp(pink(n), 1500, 7000) * smooth_rand(n, 0.5, 0.1, 1) * 0.08
        st_ = np.stack([L + leaves(), R + leaves()], 1)
        return reverb(st_, "out", 0.25)[:n]
    A["birds"] = birds_st()

    # จิ้งหรีด: พัลส์ที่ 4–5 kHz หลายตัว คนละจังหวะ + ลมกลางคืนเบาๆ
    def crickets_st():
        L = np.zeros(n)
        R = np.zeros(n)
        for c in range(6):
            f = rng.uniform(4200, 5200)
            period = rng.uniform(0.6, 1.1)
            pr = rng.uniform(28, 40)
            pan = rng.uniform(-1, 1)
            a = (pan + 1) * np.pi / 4
            vol = rng.uniform(0.3, 1)
            t = T(0.16)
            ch = np.sin(2 * np.pi * f * t) * (0.5 + 0.5 * np.sin(2 * np.pi * pr * t)) ** 4
            ch = lp(ch * np.hanning(len(t)), 9000)
            at = rng.uniform(0, period)
            while at < D - 0.2:
                put(L, ch, at, vol * np.cos(a))
                put(R, ch, at, vol * np.sin(a))
                at += period * rng.uniform(0.95, 1.05)
        bed = lp(pink(n), 500) * 0.06
        return reverb(np.stack([L + bed, R + bed], 1), "out", 0.3)[:n]
    A["crickets"] = crickets_st()

    # คลื่นทะเล: คลื่นซัดเป็นรอบ ~6 วิ (ซ่าโฟมตามหลัง) + ครืนต่ำ
    def waves_ch(off):
        y = np.zeros(n)
        t = T(D)
        env = np.zeros(n)
        at = off
        while at < D:
            per = rng.uniform(5, 7.5)
            env += np.exp(-((t - at - per * 0.35) / (per * 0.18)) ** 2) * rng.uniform(0.6, 1)
            at += per
        foam = np.roll(env, int(0.6 * SR))
        y = lp(brown(n), 500) * env * 1.4 + bp(pink(n), 700, 6000) * foam * 0.6 + bp(white(D), 5000, 9000) * foam * 0.02
        return y
    A["waves"] = np.stack([waves_ch(0), waves_ch(1.3)], 1)

    # ถ้ำ: เสียงต่ำอึมครึม + หยดน้ำก้อง
    def cave_st():
        drone = lp(brown(n), 120) * (0.8 + 0.2 * smooth_rand(n, 0.3))
        drips = np.zeros(n)
        at = 0.3
        while at < D - 1:
            f = rng.uniform(900, 1800)
            dd = 0.03
            dp = sweep(dd, f, f * 2.2) * expdec(int(dd * SR), 0.008, 0.002)
            put(drips, dp, at, rng.uniform(0.3, 1))
            at += rng.uniform(0.7, 2.5)
        x = np.stack([drone * 0.5 + drips, drone * 0.5 + drips], 1)
        return reverb(x, "cave", 0.6)[:n]
    A["cave"] = cave_st()

    # ลาวา: ฟองเดือดปุดๆ ต่ำ + ครืน + ซ่าความร้อน
    def lava_ch():
        y = lp(brown(n), 90) * 0.8
        for _ in range(int(3 * D)):
            bd = rng.uniform(0.05, 0.15)
            f = rng.uniform(70, 220)
            b = sweep(bd, f, f * rng.uniform(1.5, 2.5)) * expdec(int(bd * SR), bd / 2.5)
            put(y, b, rng.uniform(0, D - 0.2), rng.uniform(0.3, 1))
        y += grains(D, 6, 1200, 4500, (0.001, 0.004)) * 0.35 + bp(pink(n), 1500, 5000) * 0.015
        return y
    A["lava"] = st(lava_ch)

    # หนองน้ำ: กบร้อง (พัลส์ผ่านฟอร์แมนต์) + แมลงหึ่ง + น้ำกระเพื่อม
    def swamp_st():
        L = np.zeros(n)
        R = np.zeros(n)
        for _ in range(int(1.8 * D)):
            cd = rng.uniform(0.15, 0.35)
            f0 = rng.uniform(90, 160)
            cr = formant_voice(cd, [f0, f0 * 1.1, f0 * 0.9], [(rng.uniform(400, 700), 150, 1.0), (rng.uniform(1100, 1500), 200, 0.4)], 0.05)
            cr *= (0.5 + 0.5 * (np.sin(2 * np.pi * rng.uniform(15, 30) * T(cd)) > 0)) * np.hanning(int(cd * SR))
            at = rng.uniform(0, D - 0.5)
            pan = rng.uniform(-1, 1)
            a = (pan + 1) * np.pi / 4
            v = rng.uniform(0.2, 1)
            put(L, cr, at, v * np.cos(a))
            put(R, cr, at, v * np.sin(a))
        buzz = bp(white(D), 5500, 8000) * (0.5 + 0.5 * smooth_rand(n, 2)) * 0.012
        lap = bubbles(D, int(4 * D), 200, 700, (0.02, 0.06)) * 0.2
        return reverb(np.stack([L + buzz + lap, R + buzz + lap], 1), "out", 0.3)[:n]
    A["swamp"] = swamp_st()

    out = {}
    for k, v in A.items():
        v = v[:n]
        v = dcfix(loopify(v, 1.5))
        # บรรยากาศ: ปรับตามความดังเฉลี่ย (RMS) ไม่ใช่ยอดคลื่น เพื่อให้แต่ละลูปดังพอๆ กัน แล้วลิมิตยอดแบบนุ่ม
        r = np.sqrt((v ** 2).mean()) + 1e-9
        v = v * (0.11 / r)
        out[k] = np.tanh(v / 0.8) * 0.8
    return out


def write_wav(path, x):
    x = np.asarray(x)
    if x.ndim == 1:
        ch = 1
        data = x
    else:
        ch = x.shape[1]
        data = x.reshape(-1)
    pcm = np.clip(data * 32767, -32768, 32767).astype("<i2")
    with wave.open(path, "wb") as w:
        w.setnchannels(ch)
        w.setsampwidth(2)
        w.setframerate(SR)
        w.writeframes(pcm.tobytes())


def generate(out_dir):
    os.makedirs(out_dir, exist_ok=True)
    total = 0
    B = sfx_bank()
    for k, v in B.items():
        p = os.path.join(out_dir, f"sfx_{k}.wav")
        write_wav(p, v)
        total += os.path.getsize(p)
    for s in SURFACES:
        for i in range(5):
            p = os.path.join(out_dir, f"step_{s}_{i}.wav")
            write_wav(p, footstep(s, i))
            total += os.path.getsize(p)
    for m in MOBS:
        for i in range(4):
            p = os.path.join(out_dir, f"mob_{m}_{i}.wav")
            write_wav(p, mob_step(m, i))
            total += os.path.getsize(p)
    A = amb_bank()
    for k, v in A.items():
        p = os.path.join(out_dir, f"amb_{k}.wav")
        write_wav(p, v)
        total += os.path.getsize(p)
    print(f"HQ: sfx {len(B)} · เดิน {len(SURFACES) * 5} + สัตว์/มอน {len(MOBS) * 4} · บรรยากาศ {len(A)} · {total / 1e6:.1f} MB (WAV)")
    return B, A


if __name__ == "__main__":
    import sys
    generate(sys.argv[1] if len(sys.argv) > 1 else "build/hq_audio")
