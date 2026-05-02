# Weekend 2-to-the-12

2개의 문, 12층, 한 번 틀리면 처음부터. 시드 고정 4096경로의 한 가지가 정답.

## Player Fantasy

처음 죽을 때까지는 가벼운 호기심. 5층쯤부터는 짜증. 11층에서 죽으면 비명. 약올리는 톤이 그 비명을 부추긴다.

## Core Loop

룸 입장 → 두 문 중 하나 걸어 들어가기 → 정답 = 다음 룸 / 오답 = "try again!" → 클릭 → 1층부터 다시.

## MVP Scope

- 단일 화면, 두 문 (좌/우), WSAD로 캐릭터 이동
- 정답 / 오답 판정 (시드 기반 12개 0/1 경로)
- 12층 도달 = 승리 화면
- 오답 = "try again!" 화면, 클릭으로 1층 리셋
- 약올림 연출: 신나는 "byebye" 애니메이션, 컨피티, 죽어도 계속되는 신나는 BGM
- 진행층 카운터 표시
- localStorage에 best floor 저장

## Tone

- 매우 밝고 즐거움 + 빡치게 만드는 약올림
- 죽음이 슬프지 않고 신남 → 분노 가속

## Visual

- 도트 룩 + 3D 움직임 (Three.js + 저해상도 RenderTarget + nearest 업스케일)
- 캐릭터/문 = 빌보드 스프라이트, 바닥 = mesh
- 렌더 해상도: **320×180**, 화면에 nearest로 4~6배 업스케일
- 팔레트 8색 (Sweetie16 서브셋):

| Hex | 용도 |
|---|---|
| `#1a1c2c` | 깊은 그림자, 바깥 |
| `#5d275d` | 그림자, 문 외곽 |
| `#38b764` | GB 짙은 녹색 (바닥) |
| `#a7f070` | GB 밝은 녹색 (악센트) |
| `#b13e53` | 핑크-레드 (오답/약올림) |
| `#ef7d57` | 따뜻한 주황 |
| `#ffcd75` | 명랑한 노랑 (캐릭터/하이라이트) |
| `#f4f4f4` | 흰색 (텍스트/하이라이트) |

## Tech Stack

- **Three.js** (단일 HTML, importmap via esm.sh CDN, no build)
- ES modules
- 의존성: three.js만
- 빌드 0, 그냥 `open index.html`

## Out of Scope

- 무한 모드 / 일일 시드
- 배경 변화 / 룸별 테마
- 효과음 (Stretch)
- 멀티플레이 / 리더보드

## Acceptance Criteria

- [x] 카메라가 WSAD로 이동, 두 문이 화면에 보임 (1인칭으로 변경 — 캐릭터 스프라이트 없음)
- [x] 문에 닿으면 (z=-2.0 평면 통과) 정답/오답 분기
- [x] 시드 기반 정답 경로 12개 동일하게 재생성됨 (`mulberry32(0xC0FFEE)`)
- [x] 12층 통과 = 승리 화면 (녹색 톤 오버레이)
- [x] 오답 = try again 화면 → 클릭 또는 아무 키 → 1층 리셋
- [x] 진행 카운터 표시 (Floor N / 12)
- [x] localStorage best floor 저장 + HUD/오버레이 표시

## Build Stages

| 단계 | 목표 | 검증 | 상태 |
|---|---|---|---|
| A | scene + 320×180 픽셀 후처리 + 캐릭터 + WSAD + 두 문 | 도트 룩 + WSAD 이동 | `8ec8415` (3인칭) → `c615755` (1인칭으로 변경) |
| B | 문 충돌 판정 + 시드 PRNG + 정답/오답 분기 | 같은 시드 = 같은 경로, 콘솔 분기 | `2a82dc5` |
| C+D | 클릭 재시작 오버레이 + 진행 카운터 + 층별 시각 큐 | 죽음 → 클릭 → 1층 재시작 | `ef30bc0` |
| E | 약올림 연출 — 컨피티, 웃는 NPC 스프라이트, "byebye~" 애니 | 죽으면 신나는 톤으로 약올림 | `cd58979` |
| F | BGM 1트랙 + best floor localStorage + 아무 키 재시작 | 죽어도 음악 계속, best 누적 | `c596c04` |

전 단계 `main` 머지 완료. MVP Acceptance Criteria 7/7 충족.

## Architecture (final)

전체 게임이 단일 `index.html` 안에 ES module로 인라인. Three.js만 importmap으로 esm.sh CDN에서 로드. 모듈 분리하지 않음 — "빠른 구현" 원칙 + 한 화면짜리 게임이라 응집도 충분.

| 영역 | 위치 |
|---|---|
| 팔레트 (8색 hex) | `PAL` 객체 |
| 저해상도 RenderTarget + nearest 업스케일 | `target` + `fsScene` 풀스크린 쿼드 |
| 1인칭 카메라 (FOV 70°, 머리 높이 1.6) | `camera`, `SPAWN` |
| Floor mesh, back wall mesh | `floor`, `wall` (벽 색이 floor마다 순환) |
| 도어 빌보드 (좌/우, 16×32 캔버스 → texture) | `doorL`, `doorR` |
| Input (e.code 기반) | `keys`, `keydown`/`keyup` |
| Seeded PRNG (mulberry32) + path | `rand`, `PATH` |
| Game state | `state.{floor, phase}` |
| Door collision + branching | `commitChoice()` (z<-2.0 평면 통과) |
| Overlay UI (DOM, fail/win 공용) | `#overlay`, `showOverlay()` |
| 약올림 연출 (NPC, 컨피티, wiggle/bounce/pulse) | CSS keyframes + `spawnConfetti()` |
| BGM (procedural chiptune, 4-bar loop) | `BGM` IIFE — Web Audio square+triangle |
| Best floor (localStorage) | `bestFloor`, `recordIfBetter()` |

상태:
- `state.floor: 0..11` — 현재 층 인덱스
- `state.phase: 'playing' | 'failed' | 'won'`
- `bestFloor: number` — localStorage `w212-best`

모든 게임플레이 흐름이 `commitChoice()` 한 함수로 모임. floor 진행 = 카메라를 SPAWN으로 리셋 + 벽 색 갱신.
