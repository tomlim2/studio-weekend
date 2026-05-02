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
`1` / `2` / `3` pick upgrades on level-up. `R` on the game-over screen
to restart.

> macOS gotcha: the dev binary may open behind other windows. Find it via
> Mission Control (`Ctrl+↑`) or Cmd+Tab.

## Status

**MVP complete.** Stages A→H all merged to `main` (see `CONCEPT.md` Build
Stages table for commit hashes). Full loop playable: move → auto-attack →
gem pickup → magnet → level-up UI → upgrade pick → harder fights → die →
game-over screen with survival time → R to restart. SFX on hit / pickup /
level-up.

## Findings

- **Plugin-per-system pays off immediately.** Splitting `player`, `enemy`,
  `weapon`, `combat`, `gem`, `hp`, `hud`, `upgrade`, `sfx` into separate
  plugins from Stage A meant each later stage only touched 1–2 modules.
  Adding restart was a one-line `OnExit(GameOver)` system per module.
- **`SystemSet` + state gating beats per-system `run_if`.** A single
  `GameplaySet.run_if(in_state(Playing))` cleanly pauses every gameplay
  system during level-up and game-over without touching individual
  systems.
- **`OnExit(GameState)` is the correct restart hook, not a `Restart`
  message.** A message-based reset hit ordering races: `check_death`
  read not-yet-reset HP and fired a spurious game-over the same frame
  as the restart. Moving resets to `OnExit(GameOver)` removed the race
  because state transitions apply between schedules.
- **`Timer` resources don't compose with multiplicative upgrades.**
  Replacing `AttackCooldown(Timer)` with a manual `f32` accumulator
  (`since_last`, compared against `base * mult` each frame) made the
  attack-speed upgrade trivial. Timer-based cooldown would have needed
  duration mutation on every pick.
- **Bevy 0.18 ships without the `wav` decoder by default.** First Stage H
  run panicked with `UnrecognizedFormat`. Adding `features = ["wav"]` to
  the Bevy dep fixed it.
- **Procedural SFX are good enough for prototype tone.** Three short
  Python-generated sine waves with simple envelope (180 Hz hit, 900 Hz
  pickup, 500→800 Hz two-tone level-up) read as "feedback" without any
  art-direction effort.
