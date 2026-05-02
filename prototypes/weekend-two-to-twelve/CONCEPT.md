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

- [ ] 캐릭터가 WSAD로 이동, 두 문이 화면에 보임
- [ ] 문에 닿으면 정답/오답 분기
- [ ] 시드 기반 정답 경로 12개 동일하게 재생성됨
- [ ] 12층 통과 = 승리 화면
- [ ] 오답 = try again 화면 → 클릭 → 1층 리셋
- [ ] 진행 카운터 표시
- [ ] localStorage best floor 저장 + 표시

## Build Stages

| 단계 | 목표 | 검증 | 상태 |
|---|---|---|---|
| A | scene + 320×180 픽셀 후처리 + 캐릭터 + WSAD + 두 문 (스프라이트만) | 화면에 도트 룩으로 캐릭터/문 보이고 WSAD로 움직임 | — |
| B | 문 충돌 판정 + 시드 PRNG 12개 정답 경로 + 정답/오답 콘솔 로그 | 같은 시드로 같은 경로, 콘솔에 분기 출력 | — |
| C | 룸 진행 (정답 = 다음 룸으로 reset, 캐릭터 위치 초기화) + 12층 승리 화면 | 12층까지 진행 가능 | — |
| D | "try again!" 오답 화면 + 클릭 리셋 + 진행 카운터 HUD | 죽음 → 클릭 → 1층 시작, 카운터 동작 | — |
| E | 약올림 연출 — 컨피티, 웃는 NPC 스프라이트, "byebye~" 텍스트 애니 | 죽으면 신나서 약오름 | — |
| F | BGM 1트랙 + best floor localStorage + 그 외 폴리시 | 전체 톤 일관, best 표시됨 | — |

## Architecture (계획)

단일 HTML 파일 안에서 ES module pattern. 모듈 분리 가능하면 별도 .js로 빼지만 "빠른 구현" 원칙 우선.

```
index.html
  <script type="importmap"> three from CDN </script>
  <script type="module">
    // scene setup
    // pixel render target + upscale post pass
    // input
    // game state (currentFloor, path[12])
    // tick loop
  </script>
```

상태:
- `currentFloor: 0..11`
- `path: [0|1] × 12` (시드로 생성, 정답 경로)
- `phase: 'playing' | 'failed' | 'won'`
- `bestFloor: number` (localStorage)
