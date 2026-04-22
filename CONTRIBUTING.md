# Contributing to scaffolding

Thank you for your interest in contributing to scaffolding!

## Getting Started

1. Fork the repository.
2. Create a feature branch from the default branch (usually `main`).
3. Make your changes.
4. Submit a pull request.

## Commits

The commits are used to generate the changelog upon a release, therefore keep
them clean. To help with keeping them clean, follow these principles:

- Use [conventional commits](https://www.conventionalcommits.org/) for commit
  messages. You can also use Markdown, especially in the body.
- Keep commits atomic, meaning that each commit should build and pass all unit
  tests. If a new feature is added or a bug is fixed, then fix the tests in the
  same commit.
- In case of changes on the origin branch, always rebase, **NEVER** merge.
- Avoid referencing the issue or PR number in the commit message, as the
  automated changelog will add this reference for you.

## Pull Requests

- All changes require a pull request and at least one approving review.
- Add a short, clear, and hand-written description of the change proposed by
  the PR. AI tooling may add an in-depth summary below your description, so
  keep it high-level, but ensure that you include one.
- PRs must pass CI checks before merging.
- Keep PRs focused — one logical change per PR.
- Add comments to your own PR before requesting review to explain the approach
  or to add questions for the reviewers.
- Ensure all review threads are resolved before merging.
- Use `fixup!` commits to address review comments and make additional changes
  once the PR is open, as this allows the reviewers to see what has changed
  between reviews.
- When addressing a review comment, link to the commit that addressed the
  comment, even if it is a `fixup!` commit that will be squashed later.
- After an approval, rebase on the base branch. If this is done via the GitHub
  UI, no re-approval will be needed. However, if this is done locally, then a
  re-approval is required.
- Squash all `fixup!` commits and clean up the commit history before merging
  into the base branch. You can do this at the same time as rebasing with
  `git rebase --autosquash <base_branch>`. For older versions of git, you may
  need to use `git rebase --autosquash --interactive <base_branch>`, which will
  perform an interactive rebase with all of the `fixup!` commits rearranged and
  marked as fixup, you can then accept this interactive rebase.
- We will reject PRs that are purely cosmetic and appear to have been automated
  with tooling, such as correcting spelling and grammatical mistakes. If you
  believe that such a change is critical in making the documentation or code
  more readable, or if the change corrects a real logical mistake in the text,
  then submit such a PR with a description explaining the critical correction.
- All AI-generated PRs must have been self-reviewed. If a PR is judged to be
  AI-generated, not checked by the author, and needs a lot of work to be
  consistent with the existing codebase and/or it does not solve the problem it
  claims to, then we will close the PR without comment.

## Reporting Issues

- Use GitHub Issues to report bugs or request features.
- Search existing issues before creating a new one.
- Include reproduction steps for bug reports.

## Development Setup

Refer to the repository's README for specific setup instructions.

## Code of Conduct

We are committed to providing a welcoming and inclusive experience for
everyone. Please be respectful and constructive in all interactions.

## License

By contributing, you agree that your contributions will be licensed under the
same license as the project.
