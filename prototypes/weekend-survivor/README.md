# Weekend Survivor

Single-screen Vampire Survivors–style auto-shooter prototype built in
Bevy 0.18. Geometric shapes only (no art assets). Weekend scope.

## Hypothesis

Bevy 0.18 + ECS can deliver a satisfying Vampire Survivors core loop
(move / chase / auto-attack / level / upgrade / die) within a weekend,
with module-separated architecture from day one (Plugin-per-system).

The pivot rationale (away from a rhythm game, toward this ECS-native
genre) is captured in `~/Library/.../claude/projects/project-weekend/days/devlog-2026-04-26.md`.

## How to Run

```bash
cargo run
```

First build pulls Bevy + dependencies (~3–4 min on M-series). Subsequent
runs compile in ~2 seconds thanks to the dev-profile opt-level trick
(own crate `opt-level = 1`, deps `opt-level = 3`).

Controls: WASD or arrow keys to move. Auto-attack fires on cooldown.

> macOS gotcha: the dev binary may open behind other windows. Find it via
> Mission Control (`Ctrl+↑`) or Cmd+Tab.

## Status

In-progress. Stage D of H complete (see `CONCEPT.md` Build Stages table).
Currently playable: a cyan circle moves around, red squares converge from
off-screen and chase, yellow projectiles auto-fire at the nearest enemy
within 500 px and one-shot kill on hit. No XP, no upgrades, no death yet.

## Findings (populated as stages conclude)

- TBD
