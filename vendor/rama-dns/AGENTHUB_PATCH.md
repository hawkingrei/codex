# AgentHub security patch

This directory contains the published `rama-dns` 0.3.0-alpha.4 sources with a
minimal compatibility update for `hickory-resolver` 0.26.1.

The patch exists because Codex 0.147 still depends on the Rama alpha API while
`hickory-proto` 0.25.x is affected by a denial-of-service advisory. Upgrading
the full Rama stack would require unrelated API migrations. The local changes
are limited to:

- selecting `hickory-resolver` 0.26.1;
- adapting resolver construction and answer iteration to the 0.26 API; and
- preserving the existing `rama-dns` public API used by `rama-net`.

Remove this directory and the workspace patch when the upstream Codex/Rama
dependency graph no longer resolves an affected `hickory-proto` release.
