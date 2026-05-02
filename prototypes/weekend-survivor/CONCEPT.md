# Weekend Survivor

Single-screen Vampire Survivors–style auto-shooter built in Bevy 0.18.
Geometric shapes only. Weekend scope. Target: vertical slice in 2 days.

## Player Fantasy

한 점이 도형 무리에 둘러싸여서 살아남는다.
긴장감 있는 생존, 점점 카오스, 1분에 한 번 의사결정(레벨업 픽).

## Core Loop

이동 → 적이 모여듦 → 자동공격이 솎아냄 → XP 젬 흡수 →
레벨업 → 업그레이드 1개 선택 → 더 강한 적 → 죽을 때까지 반복.

## MVP Scope (must-have)

- 탑다운 플레이어(원), WASD 이동
- 적(빨간 사각형) 플레이어 추격, 시간 갈수록 스폰 빈도 증가
- 자동공격 1종: 가장 가까운 적에게 투사체(작은 원) 발사 (쿨다운제)
- 적 처치 → XP 젬 드롭 → 자석 범위 내 자동 흡수
- 레벨업 → 3개 업그레이드 중 1개 선택 (공격속도 / 이동속도 / 투사체 수)
- HP 0 → 게임오버 화면, 생존 시간 표시
- SFX 3종: 피격, 픽업, 레벨업

## Stretch (시간 남으면)

- 2번째 무기, 5분 보스, 메인 메뉴, BGM 1트랙

## Out of Scope

- 세이브/로드, 설정 화면, 멀티 캐릭터, 멀티 스테이지, 도전과제,
  스프라이트 에셋, 셰이더, 멀티플레이

## Tech Stack

- Bevy 0.18 (회사 shotloom 버전 매칭)
- Rust edition 2021
- 단일 바이너리 크레이트
- 비주얼: `Mesh2d` 프리미티브 (`Circle`, `Rectangle`)
- 오디오: `bevy_audio` (feature `wav`) + .wav 3종 (procedural sine)
- 의존성: `bevy`, `rand` 만

## Acceptance Criteria

- [x] 윈도우 열리고 플레이어가 WASD로 이동
- [x] 적 ≥10마리가 동시에 화면에서 플레이어 추격
- [x] 자동공격이 가장 가까운 적을 맞히고 처치
- [x] XP 픽업 → 레벨업 UI 표시 → 업그레이드 1개 적용 확인
- [x] HP 0에서 게임오버 + 생존 시간 표시
- [x] 60 FPS 유지하며 30초 이상 플레이 가능 (M2 Max에서 28~29초 평균 데스 시점까지 안정)

## Build Stages

| 단계 | 목표 | 검증 | 상태 |
|---|---|---|---|
| A | 윈도우 + 카메라 + 플레이어 도형 + WASD 이동 | 화면 안에서 움직임 확인 | `ebf4cc3` |
| B | 적 스폰 + 플레이어 추격 AI | 적 10마리가 모여듦 | `431910c` |
| C | 자동공격 (cooldown + nearest-target) | 투사체가 적 향해 발사 | `7204c38` |
| D | 충돌 감지 + 데미지 + 적 사망 | 적이 맞으면 사라짐 | `7c2e5fa` |
| E | XP 젬 드롭 + 자석 흡수 + 레벨업 | 레벨업 트리거 동작 | `6070822` |
| F | 업그레이드 선택 UI + 적용 | 픽 후 스탯 변화 | `bb7efbc` |
| G | HP + 게임오버 + 생존 타이머 + 리스타트 | 죽으면 게임오버 → R 재시작 | `e63b58f` |
| H | SFX 3종 | 피격/픽업/레벨업 소리 들림 | `6b71164` |

전 단계 `main` 머지 완료. MVP Acceptance Criteria 모두 충족.

## Architecture (final)

각 단계가 Plugin-per-system 분리로 들어감. 모든 게임플레이 시스템은
`GameplaySet.run_if(in_state(Playing))`으로 묶여서 LevelingUp / GameOver
state 진입 시 자동 일시정지.

| 모듈 | 역할 |
|---|---|
| `camera.rs` | 카메라 + 플레이어 추적 |
| `player.rs` | Player/Xp/Level 컴포넌트, WASD 이동 (`Upgrades.move_speed_mult` 적용) |
| `enemy.rs` | 스폰 (cap 100, 0.5s 간격), 추격 AI |
| `weapon.rs` | nearest-target 자동공격, projectile_count 분산 (8°/발), `Upgrades.attack_cooldown_mult` |
| `combat.rs` | projectile↔enemy 충돌, `EnemyKilled { pos }` Message emit |
| `gem.rs` | `EnemyKilled` 리스닝 → 젬 드롭, 자석 흡수, XP 누적, `LevelUp` Message emit |
| `hp.rs` | Hp/IFrame/SurvivalTime, 적 접촉 데미지 (10/hit, 0.5s i-frame), 사망 → GameOver |
| `hud.rs` | HP 텍스트(좌상), 생존 타이머(중상, mm:ss), 게임오버 오버레이, R 키 재시작 |
| `upgrade.rs` | GameState (Playing / LevelingUp / GameOver), Upgrades resource, 레벨업 UI + 픽(1/2/3) |
| `sfx.rs` | Sfx resource (hit/pickup/levelup 핸들), `play_oneshot` helper |

리스타트는 `OnExit(GameState::GameOver)` 트리거로 각 모듈이 자기 상태를
초기화 — `RestartGame` 메시지 패턴은 ordering race 발생해서 폐기됨
(`reset_*` 시스템과 `check_death`가 같은 frame에 돌면서 reset 전 HP를
읽는 케이스).

수치 상수는 각 모듈 상단의 `const`에 박혀있음. 튜닝은 그쪽 직접 수정.
