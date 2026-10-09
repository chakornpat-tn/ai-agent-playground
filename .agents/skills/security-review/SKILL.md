---
name: code-review
description: >
  Review a branch, commit range, pull request, or working-tree diff for correctness bugs,
  security vulnerabilities, regressions, broken edge cases, and risky changes. Use this skill
  whenever the user asks to review code, review a PR, inspect a diff, find bugs before merging,
  or run a code review. Report actionable findings with file and line references; do not change
  code unless the user explicitly requests fixes or passes --fix.
compatibility: Requires Git and the repository's available source files. GitHub CLI is optional and only needed for PR metadata or posting comments.
---

# Code Review

Review the requested change in the context of the surrounding codebase. Prioritize correctness
and production risk over formatting preferences: identify issues that can break behavior, leak
data, create security vulnerabilities, regress existing flows, or make rollback unsafe.

This skill is based on the local review workflow described in the
[Claude Code Code Review documentation](https://code.claude.com/docs/en/code-review).

## Select the review target

Resolve the target before inspecting code:

- With no target, review commits on the current branch that are ahead of its upstream plus
  uncommitted changes.
- A file path reviews that file and its relevant callers and tests.
- A branch, commit, PR number, or ref range such as `main...feature` reviews that target.
- For a GitHub PR, use `gh` only when PR metadata, comments, or checks are explicitly in scope.

Inspect repository guidance (`AGENTS.md`, `CLAUDE.md`, README files, and project review rules)
before judging the change. Use `git diff`, `git diff --check`, and focused source/test reads;
do not rely on filenames or the diff alone when behavior depends on callers, schemas, routes,
configuration, or generated contracts.

## Review workflow

1. Establish the exact diff and its base. Separate pre-existing problems from issues introduced
   by the change.
2. Trace changed behavior through its consumers, error paths, persistence boundaries, auth or
   permission checks, and user-visible flows.
3. Check high-risk areas first: data loss, injection, secret or PII exposure, authorization,
   concurrency, validation, migrations, backwards compatibility, API contracts, and broken
   tests or build assumptions.
4. Verify suspected findings against actual code. Prefer a focused test, type check, lint,
   reproducer, or source citation over an inference from naming.
5. Check whether the change has adequate regression coverage when missing coverage would hide a
   likely correctness issue. Do not report coverage merely as a style preference.
6. Run only relevant, safe verification commands unless the user asks for a broader test suite.
   Report commands that could not run and why.

## Severity

Use these labels:

- `🔴 Important` — a bug, security issue, data problem, or regression that should be fixed before
  merging.
- `🟡 Nit` — a minor issue worth fixing but not merge-blocking.
- `🟣 Pre-existing` — a real issue found in the inspected area that was not introduced by this
  change.

Do not report speculative concerns as findings. If evidence is insufficient, ask a focused
question or omit it. Keep the number of low-value nits small and lead with important findings.

## Report format

Start with a short summary and the verification performed. Then list findings in descending
severity. Every finding must include:

- file path and exact line or range;
- severity and a concise problem statement;
- why the behavior matters, when it is not obvious;
- a concrete fix or next action.

Use this format:

```text
🔴 Important — `path/to/file.ts:42`
Problem: concise statement of the defect.
Why: concrete runtime, security, or maintenance impact.
Fix: specific change that would address it.
Evidence: test, command, or relevant source path when useful.
```

If no actionable issues are found, say so explicitly and list the checks that were run. Do not
claim that a review is comprehensive when tests or source access were unavailable.

## Fix and posting boundaries

- Review is read-only by default.
- Apply fixes only when the user explicitly asks for them or supplies `--fix`; preserve unrelated
  worktree changes and re-run focused verification after edits.
- Do not post GitHub or GitLab comments, approve a PR, or alter branch protection without an
  explicit request and the required CLI/account access.
- A local review does not automatically read a repository `REVIEW.md`; use it only if the active
  host workflow explicitly supports that file. Repository-wide instructions such as `AGENTS.md`
  and `CLAUDE.md` remain relevant project guidance.

## Supported options

When the user uses the Claude Code-style command, interpret it as follows:

- `/code-review` or `/review`: review the current branch diff and uncommitted changes;
- `/code-review <target>`: review the supplied path, PR, branch, commit, or ref range;
- `--fix`: apply verified findings, then report the edits and verification;
- `--comment`: post findings to the specified PR/MR only when explicitly authorized;
- `--max-findings <n|all|default>`: cap findings or restore the default cap;
- effort levels from `low` through `max`: trade review breadth for confidence, with higher effort
  allowing broader but potentially lower-confidence findings.
