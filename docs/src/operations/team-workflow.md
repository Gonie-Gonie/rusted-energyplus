# Team Workflow

작업은 의미 있는 단위로 끊어서 진행한다.

## 기본 원칙

- setup, architecture, feature, test, docs 중 하나의 목적이 닫히는 단위로
  commit한다.
- commit 전 관련 local script를 실행한다.
- commit 후 remote가 허용하면 push한다.
- unrelated refactor와 oracle/compatibility 변경을 섞지 않는다.
- 새 기능은 구현, test, docs, porting-map, supported object coverage를 함께
  갱신한다.

## 현재 v0.1.0 단위

첫 commit 후보:

```text
Initialize reproducible setup skeleton
```

포함 범위:

- Rust workspace skeleton
- setup/check/oracle smoke scripts
- EnergyPlus 26.1.0 oracle lock
- copied development plan
- docs skeleton
