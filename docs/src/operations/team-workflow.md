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

## Public Version Rule

public `vX.Y.Z` tag는 build artifact와 사용자-visible command가 있는 경우에만 만든다.
setup-only, docs-only, oracle-only 단계는 foundation checkpoint로 관리한다.

## 첫 public release 단위

첫 public release:

```text
v0.1.0 RawModel inspection CLI + typed compile preview
```

포함 범위:

- eplus-rs.exe build
- model inspect command
- model compile preview command
- epJSON RawModel parse
- release zip artifact
- Git tag push triggers `.github/workflows/release.yml`
- GitHub Release and zip asset are created by the workflow

## Release Publishing

Normal release path:

```text
commit -> push main -> annotated vX.Y.Z tag -> push tag -> GitHub Actions release workflow
```

Manual fallback:

```text
scripts\dev.cmd github-release
```

The manual fallback requires `GH_TOKEN` or `GITHUB_TOKEN` and should only be
used when the workflow path is unavailable.
