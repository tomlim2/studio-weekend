# Weekend 2-to-the-12

2 doors, 12 floors, one wrong step and you're back to the start.
4096 paths, only one is right.

## Hypothesis

A purely punishing trial-and-error mechanic can be made *fun* if the
failure tone is cheerful enough that the player rage-laughs and goes
again. The taunting bright-pixel aesthetic is the actual game; the
mechanic is just the fuel.

## How to Run

```bash
open index.html
```

Single HTML, no build, ES modules via CDN importmap. Modern browser
required (Chrome / Safari / Firefox recent).

Controls: WSAD to move. Walk into either door to commit a choice.
Click on the "try again!" screen to restart from floor 1.

## Status

**MVP complete.** Stages A→F all merged to `main` (see `CONCEPT.md` Build
Stages table). Full loop playable: WSAD walk → walk through one of two
doors → correct = next floor (room wall color cycles), wrong = taunting
overlay (waving NPC, confetti, "byebye~"). Click or any key to retry.
Procedural chiptune BGM never stops. Best floor persists in
localStorage.

## Findings

- **Three.js + low-res RenderTarget + nearest upscale** is the cheapest
  way to get a "fake pixel" look while keeping full 3D camera/movement.
  Integer-scale CSS positioning with `image-rendering: pixelated` keeps
  pixels sharp at any window size.
- **`e.code` (physical key) instead of `e.key` (character)** — Korean IME
  silently breaks WSAD because `e.key` returns 'ㅈ', 'ㅁ', etc. Using
  `e.code` ('KeyW', 'KeyA', …) is layout/IME-immune.
- **First-person trumped third-person here.** A visible character sprite
  needed careful camera framing (frustum kept clipping the player).
  Switching to FP killed the framing problem and matched the "you are
  in the room" feel of the choice mechanic.
- **`AudioContext` start must be gated behind a user gesture.** Browsers
  block `new AudioContext()` from autoplaying audio until the user
  presses a key or clicks. Binding `BGM.start()` to the first keydown
  is enough.
- **Self-rescheduling chiptune via Web Audio is two screens of code.**
  No samples, no audio assets, no decoder gotchas. A 4-bar C-major loop
  with square + triangle oscillators is enough cheerful taunt fuel.
- **Taunt tone is the actual game.** The mechanic is one binary choice
  repeated 12×, brute-force unsolvable (1/4096). Without the cheerful
  "byebye~" + confetti + bouncy NPC, every death would feel cruel.
  With it, it feels like the game is laughing *with* you, not at you.
