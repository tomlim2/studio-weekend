---
title: "Design Criteria — Scoring Rubric for AI Studio Tycoon"
tags:
  - research
  - game-dev
  - tycoon
date: 2026-04-18
source: claude
---

# Design Criteria — Scoring Rubric for AI Studio Tycoon

Research for: AI Studio Tycoon | Produced: 2026-04-18 | Consumer: creative-director + game-designer

A rubric for scoring design decisions. Each axis cites reference anchors from `similar-games.md`. The "Our target band" column is intentionally blank — the designer fills it during the planning meeting. "Risk if too short / too long" tells you what failure mode each extreme drifts toward.

---

## How to use this

1. Read each axis with its reference anchors.
2. Fill the **Target band** column based on the AI Studio Tycoon vision.
3. Re-score every 2 weeks; if a decision is drifting outside the band, flag it.
4. Treat the **Risk** columns as concrete failure modes seen in shipped games — not abstractions.

---

## The rubric

### Axis 1: Core loop length (one "ship")

| Field | Value |
|-------|-------|
| Reference anchors | GDT: ~90 sec per project. Mad Games Tycoon 2: ~2–4 min. Two Point Hospital: 5–10 min per "satisfaction milestone." Big Pharma: continuous (no discrete loop). Startup Company: ~30–60 sec per component task. |
| Our target band | **___** (designer fill) |
| Risk if too short (<30s) | Pure click-grind; no time for narrative beats or hallucination events to land. |
| Risk if too long (>5 min) | Players bounce in first session; streamer-unfriendly. |

---

### Axis 2: Systems count at launch

| Field | Value |
|-------|-------|
| Reference anchors | GDT launch: 5 systems (genre, topic, platform, staff, R&D). Software Inc.: 12+. Mad Games Tycoon 2: 20+ (criticized as cluttered). Two Point Hospital: 6 main systems, gimmick-of-the-level for variety. |
| Our target band | **___** |
| Risk if too few (<5) | Game runs out of decisions by hour 3; reviews call it shallow (GDT post-year-35 problem). |
| Risk if too many (>15) | UI cluttered, onboarding hostile (MGT2's #1 complaint); mainstream press dismisses as "spreadsheet game." |

---

### Axis 3: Art scope

| Field | Value |
|-------|-------|
| Reference anchors | GDT: 2D pixel/cartoon, ~1 artist. Software Inc.: 3D isometric, low-poly, free-form construction. Two Point Hospital: 3D isometric with full character animation, full studio team. Hollywood Animal: stylized 2D portraits + period-UI, mid-size team. SLOP Factory: mobile-2D, single artist. |
| Our target band | **___** |
| Risk if too small | Looks like an asset-flip; loses against $5 GDT clones on Steam visibility. |
| Risk if too large | Burns budget before mechanics ship; Hollywood Animal-style EA-slipping risk if polish-before-systems. |

---

### Axis 4: UI density

| Field | Value |
|-------|-------|
| Reference anchors | GDT: low (one main panel). Software Inc.: medium (room view + assignment panel). MGT2: high (dense tabs, oversized buttons — criticized). Two Point Hospital: medium (clear iconography). |
| Our target band | **___** |
| Risk if too sparse | Hardcore management-sim audience (the buyers) feel under-served. |
| Risk if too dense | Mobile-port-feel reviews (MGT2 case); mainstream press dismisses. Lose Twitch/YouTube readability. |

---

### Axis 5: Time progression model

| Field | Value |
|-------|-------|
| Reference anchors | Real-time + pause: Two Point Hospital, Software Inc., Hollywood Animal (modern default). Project-tick: GDT, MGT2 (each ship is a turn). Real-time-only: Project Highrise, Startup Company (Highrise's midnight-tick is the loudest pacing complaint in the set). |
| Our target band | **___** |
| Risk if too "real-time" | Forces idle waiting (Highrise's documented failure). |
| Risk if too "turn-based" | Loses the sandbox feel; harder to layer ambient events (hallucination triggers, burnout) on top. |

---

### Axis 6: Meta-progression depth

| Field | Value |
|-------|-------|
| Reference anchors | GDT: linear era unlock (5 eras, ~35 in-game years, runs out). MGT2: era + console manufacturing branch (sandbox to year 2050). Two Point Hospital: linear hospital chain (15 hospitals, then sandbox). Software Inc.: open-ended sandbox from start. |
| Our target band | **___** |
| Risk if too shallow | Player hits ceiling at 10–15 hours; review tail erodes (GDT post-year-35). |
| Risk if too deep | Onboarding becomes hostile; first-session conversion drops. |

---

### Axis 7: Monetization / pricing tier

| Field | Value |
|-------|-------|
| Reference anchors | $5 (Game Builder Tycoon, lite GDT-clones, ~76% positive). $9.99 (GDT, classic indie). $15–$20 (modern indie management-sim default: Big Pharma $15, Project Highrise $17, MGT2 $19, Software Inc. $19, Computer Tycoon $19.99, Arcade Paradise $19.99, Hollywood Animal $20). $30–$40 (Two Point Hospital, publisher-backed). |
| Our target band | **___** |
| Risk if too low ($<10) | Perceived as a clone; review-count multiplier doesn't pay back marketing. |
| Risk if too high ($>25) | Wishlist-to-purchase conversion craters without publisher backing or comedy-IP brand. |

Notes: median indie-game price is $10.99 ([HowToMarketAGame](https://howtomarketagame.com/2022/08/23/4-tips-to-help-you-price-your-indie-game/)). Steam discount-driven sales reward 20–30% headroom over target take.

---

### Axis 8: Replayability mechanism

| Field | Value |
|-------|-------|
| Reference anchors | Mod support: GDT (860 Workshop mods — extends life by years). Sandbox + competitor AI: MGT2 (up to 100 rivals), Software Inc. Scenario chain: Two Point Hospital (15 hospitals). Procedural: nobody nails it in this set. |
| Our target band | **___** |
| Risk if too thin | Game has a 10-hour ceiling; nothing for content creators to keep playing. |
| Risk if too "infinite sandbox" | First 2 hours lack direction; players who wanted a campaign churn. |

---

### Axis 9: Onboarding curve

| Field | Value |
|-------|-------|
| Reference anchors | GDT garage = "one-screen first hour" (works). Two Point Hospital first hospital is failable-but-recoverable (works). MGT2 throws full surface day one (criticized: "core mechanic is not explained"). Software Inc. requires forum-reading (forgiven because solo dev, not for studios). |
| Our target band | **___** |
| Risk if too gentle | Insiders feel patronized; "tutorial too long" reviews. |
| Risk if too steep | First-session bounce; mainstream press cannot grade the game. |

---

### Axis 10: Failure pressure

| Field | Value |
|-------|-------|
| Reference anchors | Two Point Hospital: very low — "doesn't punish mistakes" (trade-off: cozy and broad reach, but no tension). Startup Panic: very high — "constantly punishes progress" / "death spiral" (criticized as un-fun). GDT: medium — bankruptcy possible but rare. |
| Our target band | **___** |
| Risk if too low | Wins feel meaningless; no tension; satire lands flat (need real stakes for satire to bite). |
| Risk if too high | Startup Panic's "death spiral" — every win triggers more problems, players quit out of fatigue. |

---

### Axis 11: Tone commitment

| Field | Value |
|-------|-------|
| Reference anchors | The Founder: 100% committed satire (browser-game viral). Two Point Hospital: 100% committed cartoon comedy. SLOP Factory: 100% committed satire (mobile). Startup Panic: 50% — "doesn't commit hard enough to satire" (mushy middle, criticized). Hollywood Animal: split (drama + management) — works because both halves are strong. |
| Our target band | **___** |
| Risk if too earnest | Loses the "AI satire" hook that's the differentiator vs. TechDev Tycoon. |
| Risk if too satirical | Player can't take the management sim seriously; The Founder's flat replay tail. |

---

### Axis 12: Causality visibility (why did the score change)

| Field | Value |
|-------|-------|
| Reference anchors | GDT: opaque review algo — single largest negative-review theme in the genre. MGT2: opaque graphics/sound scoring — second-most-cited. Software Inc.: visible per-component scores (loved). Two Point Hospital: visible patient-by-patient feedback (loved). |
| Our target band | **High by default** — every score change must trace to a visible cause. |
| Risk if too opaque | Single biggest killer of long-tail reviews in the entire reference set. Don't ship without this. |
| Risk if too verbose | Numbers-on-screen overload; novice players overwhelmed. Solve with a "show details" toggle. |

---

## Quick-reference scoring sheet

| # | Axis | Anchor low | Anchor high | Our target |
|---|------|------------|-------------|-----------|
| 1 | Core loop length | 30s (Startup Co. task) | 5 min (Two Point Hospital milestone) | _ |
| 2 | Systems at launch | 5 (GDT) | 20+ (MGT2 — too many) | _ |
| 3 | Art scope | 1 artist 2D (GDT) | Studio team 3D (Two Point Hospital) | _ |
| 4 | UI density | Low (GDT) | High (MGT2 — too dense) | _ |
| 5 | Time model | Real-time + pause (Two Point Hospital) | Project-turn (GDT) | _ |
| 6 | Meta-progression | Linear unlock (GDT) | Open sandbox (Software Inc.) | _ |
| 7 | Price | $5 (lite-clone) | $20 (modern indie) | _ |
| 8 | Replay mechanism | Scenario chain | Mod-supported sandbox (GDT) | _ |
| 9 | Onboarding | One-screen first hour (GDT) | Open feature surface (MGT2 — too steep) | _ |
| 10 | Failure pressure | Low (Two Point Hospital) | High (Startup Panic — too high) | _ |
| 11 | Tone | 100% satire (The Founder) | Earnest competence (Software Inc.) | _ |
| 12 | Causality visibility | Opaque (GDT — DON'T) | Per-component (Software Inc. — DO) | **HIGH (mandate)** |

All anchors trace to entries in `similar-games.md`.
