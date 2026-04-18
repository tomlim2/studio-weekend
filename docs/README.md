---
title: "Studio Weekend — Hallucination Zoo"
tags:
  - studio-weekend
  - spec
date: 2026-04-18
source: claude
status: committed
---

# Hallucination Zoo (가제)

> Placid Plastic Duck Simulator × Coconut Simulator × 환각. 허무 수집 시뮬.

"펜" 누르면 AI가 기괴한 생물 1마리 생성. 도감 등록. 정원에서 떠다님. 끝.

---

## 장르

Ambient Collection Sim · Joke Game · 허무겜.

## 레퍼런스

- Placid Plastic Duck Simulator ($2.99, 27k+ 리뷰, 95% 긍정)
- Coconut Simulator
- Rock Simulator
- Unpacking (수집 쾌감 이식)

## 코어

- 생성 = 프로시듀럴 (명사 × 형용사 × 직업 조합). 실제 LLM 호출 X.
- 스프라이트 = 파츠 조합 (몸 20 × 머리 30 × 눈 50).
- 수집 = 도감 · 희귀도 · 도전과제.
- 입력 = 클릭 몇 개. 머리 비움.

## Non-Goals

- 전투, 밸런스, 멀티, 스토리, 3D, 장편 캠페인.

## 주차 로드맵 (5주)

- [ ] **W1**: 생성 시스템 — 명사/형용사 DB 300개, 설명문 템플릿 10개, CLI로 무작위 생성 100마리 뽑아 보기
- [ ] **W2**: 픽셀 파츠 시스템 — 몸/머리/눈/악세 각 10~20개, Godot 씬에서 조합 렌더
- [ ] **W3**: 도감 UI + 정원 씬 (생물 떠다님) + BGM
- [ ] **W4**: 희귀도 티어, 도전과제, Steam 연동 (업적만)
- [ ] **W5**: 폴리시 + Steam 페이지 + 트레일러

## 스코프 하드 캡

- 초기 명사 300 · 형용사 300 · 직업 100
- 파츠 총 ~80
- 도감 슬롯 ~1000
- 도전과제 20개
- 가격 $2.99~$4.99 (Early Access 없이 그냥 출시)

## 네이밍 (결정 필요)

"Hallucination Zoo"는 일단 가제. AI 티 덜 나는 후보:
- *Weird Creature Garden*
- *환각 식물원*
- *이상한 도감*
- *Where the Bananas Are Professors*

## Next

코드 시작 전 단 하나 — **명사/형용사/직업 시드 리스트 100개씩만 먼저 뽑아보기** (이게 재밌나 직관 체크).
