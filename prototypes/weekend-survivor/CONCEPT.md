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
- 오디오: `bevy_audio` + .ogg 3종
- 의존성: `bevy`, `rand` 만

## Acceptance Criteria

- [ ] 윈도우 열리고 플레이어가 WASD로 이동
- [ ] 적 ≥10마리가 동시에 화면에서 플레이어 추격
- [ ] 자동공격이 가장 가까운 적을 맞히고 처치
- [ ] XP 픽업 → 레벨업 UI 표시 → 업그레이드 1개 적용 확인
- [ ] HP 0에서 게임오버 + 생존 시간 표시
- [ ] 60 FPS 유지하며 30초 이상 플레이 가능

## Build Stages

| 단계 | 목표 | 검증 |
|---|---|---|
| A | 윈도우 + 카메라 + 플레이어 도형 + WASD 이동 | 화면 안에서 움직임 확인 |
| B | 적 스폰 + 플레이어 추격 AI | 적 10마리가 모여듦 |
| C | 자동공격 (cooldown + nearest-target) | 투사체가 적 향해 발사 |
| D | 충돌 감지 + 데미지 + 적 사망 | 적이 맞으면 사라짐 |
| E | XP 젬 드롭 + 자석 흡수 + 레벨업 | 레벨업 트리거 동작 |
| F | 업그레이드 선택 UI + 적용 | 픽 후 스탯 변화 |
| G | HP + 게임오버 + 생존 타이머 | 죽으면 게임오버 화면 |
| H | SFX 3종 | 피격/픽업/레벨업 소리 들림 |
