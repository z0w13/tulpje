#!/usr/bin/env python3

from dataclasses import dataclass
from collections import defaultdict
from typing import Iterable, NamedTuple, Optional, Self
from fnmatch import fnmatchcase
from graphlib import TopologicalSorter
import tomllib
import subprocess
import json
import argparse
import sys
import os
import re

import tomlkit
from tomlkit.items import AoT


def argparser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser()
    parser.add_argument("crate", default=None, nargs="?")
    parser.add_argument("--execute", action="store_true")

    return parser


def find_file_upwards(path: str, name: str) -> Optional[str]:
    full_path = os.path.join(path, name)
    if os.path.isfile(full_path):
        return full_path
    else:
        if os.path.dirname(path) == path:
            return None
        else:
            return find_file_upwards(os.path.dirname(path), name)


class CrateInfo(NamedTuple):
    name: str
    path: str
    independent: bool
    workspace_dependencies: frozenset[str]
    lock_file: str

    @property
    def manifest(self) -> str:
        return os.path.join(self.path, "Cargo.toml")

    @classmethod
    def from_manifest(cls, path: str) -> Self:
        try:
            independent = not (
                tomllib.load(open(path, "rb"))["package"]["version"]["workspace"]
                == True
            )
        except (IndexError, TypeError):
            independent = True

        manifest_json = process_run(
            ["cargo", "read-manifest", "--frozen", "--manifest-path", path]
        )
        parsed_manifest = json.loads(manifest_json)
        workspace_dependencies = {
            d["name"] for d in parsed_manifest["dependencies"] if "path" in d
        }

        lock_file = find_file_upwards(path, "Cargo.lock")
        if lock_file is None:
            raise Exception("Couldn't find Cargo.lock")

        return cls(
            name=parsed_manifest["name"],
            path=os.path.dirname(path),
            independent=independent,
            workspace_dependencies=frozenset(workspace_dependencies),
            lock_file=lock_file,
        )


class Version(NamedTuple):
    # X.Y.Z
    REGEX = re.compile(r"^([0-9]+)\.([0-9]+)\.([0-9]+)$")

    major: int
    minor: int
    patch: int

    def dot_zero(self) -> bool:
        return self.major == 0

    def __str__(self) -> str:
        return f"{self.major}.{self.minor}.{self.patch}"

    def bumped(self, feature: bool, breaking: bool) -> "Version":
        if self.dot_zero():
            if breaking:
                return Version(self.major, self.minor + 1, 0)
            else:
                return Version(self.major, self.minor, self.patch + 1)
        else:
            if breaking:
                return Version(self.major + 1, 0, 0)
            elif feature:
                return Version(self.major, self.minor + 1, 0)
            else:
                return Version(self.major, self.minor, self.patch + 1)

    @classmethod
    def parse(cls, version: str) -> Self:
        match = cls.REGEX.match(version)
        assert match is not None

        groups = match.groups()
        return cls(int(groups[0]), int(groups[1]), int(groups[2]))


def sort_versions(versions: list[str]) -> list[str]:
    return [
        ".".join(map(str, v))
        for v in sorted([Version.parse(v) for v in versions], reverse=True)
    ]


def latest_version(versions: list[str]) -> Optional[str]:
    versions = sort_versions(versions)
    return versions[0] if len(versions) > 0 else None


def git_tags_with_prefix(prefix: str = "") -> list[str]:
    return [
        tag
        for tag in process_run(["git", "tag", "--list"], encoding="utf-8").splitlines()
        if tag.startswith(f"{prefix}v")
    ]


def get_latest_tag(prefix: str = "") -> Optional[str]:
    tags = git_tags_with_prefix(prefix)
    latest = latest_version([t.removeprefix(f"{prefix}v") for t in tags])
    if latest is None:
        return None

    return f"{prefix}v{latest}"


class CommitInfo(NamedTuple):
    # type(scope)!: subject
    SUBJECT_REGEX = re.compile(r"^(.*?)(\(.*\))?(!)?: (.*)$")

    sha: str

    subject: str
    typ: str
    scope: str
    breaking: bool

    raw_subject: str
    files: list[str]

    @classmethod
    def create(cls, sha: str, raw_subject: str, files: list[str]):
        match = cls.SUBJECT_REGEX.match(raw_subject)
        assert match is not None

        typ, scope, breaking, subject = match.groups()
        return cls(sha, subject, typ, scope, breaking == "!", raw_subject, files)


def get_commits_since_ref(ref: str) -> list[CommitInfo]:
    def create_commit_info(sha, subject) -> CommitInfo:
        return CommitInfo.create(sha, subject, get_commit_files(sha))

    return [
        create_commit_info(*commit.split(" ", 1))
        for commit in process_run(
            ["git", "log", "--format=%H %s", f"{ref}..HEAD"], encoding="utf-8"
        ).splitlines()
    ]


def get_commit_files(sha: str) -> list[str]:
    return process_run(
        ["git", "diff-tree", "--no-commit-id", "--name-only", "-r", sha],
        encoding="utf-8",
    ).splitlines()


class SemverCheckResult(NamedTuple):
    breaking: bool
    output: str


def cargo_semver_checks(
    baseline_rev: str, crate: Optional[str] = None
) -> SemverCheckResult:
    result = subprocess.run(
        ["cargo", "semver-checks", "--baseline-rev", baseline_rev]
        + ([] if crate is None else ["--package", crate]),
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        encoding="utf-8",
    )
    return SemverCheckResult(result.returncode != 0, result.stdout)


def filename_match(filename: str, matchlist: set[str]) -> bool:
    def match_entry(filename: str, entry: str) -> bool:
        if entry.startswith("!"):
            return fnmatchcase(filename, entry[1:]) or fnmatchcase(
                filename, f"*/{entry[1:]}"
            )
        else:
            return fnmatchcase(filename, entry) or fnmatchcase(filename, f"*/{entry}")

    return any(match_entry(filename, entry) for entry in matchlist)


def should_create_release(
    commits: list[CommitInfo], matchlist: set[str], path: Optional[str]
) -> bool:
    changed_files = {
        file
        for commit in commits
        for file in commit.files
        if path is None or file.startswith(path)
    }
    return any(filename_match(file, matchlist) for file in changed_files)


def filter_commits_by_path(
    commits: list[CommitInfo], paths: Iterable[str], invert=False
) -> list[CommitInfo]:
    def check(file: str, paths: Iterable[str], invert: bool) -> bool:
        if invert:
            return any(not file.startswith(path) for path in paths)
        else:
            return any(file.startswith(path) for path in paths)

    return [
        commit
        for commit in commits
        if any(check(file, paths, invert) for file in commit.files)
    ]


RELEASE_FILENAME_MATCHLIST = {
    "*.rs",
    "Cargo.toml",
}
RELEASE_FILENAME_MATCHLIST_WORKSPACE = {
    ".sqlx/*",
    "*.sql",
    "Cargo.lock",
    "compose.*.yml",
    "Dockerfile*",
}


@dataclass
class ReleaseInfo:
    crates: tuple[CrateInfo, ...]

    prev_version: Version
    curr_version: Version

    commits: tuple[CommitInfo, ...]
    changelog: str
    should_release: bool

    has_feature_commit: bool
    has_breaking_commit: bool
    is_breaking_semver_checks: bool

    def breaking_with_reason(self) -> str:
        if not self.is_breaking:
            return "False"
        elif self.has_breaking_commit:
            return "True (Breaking Commit)"
        elif self.is_breaking_semver_checks:
            return "True (`cargo semver-checks` failed)"

        raise Exception(
            "did we add a new conditional for breaking changes, shouldn't reach this"
        )

    @property
    def changed(self) -> bool:
        return self.prev_version != self.curr_version

    @property
    def single_crate(self) -> bool:
        return len(self.crates) == 1

    @property
    def prefix(self) -> str:
        if self.single_crate:
            return f"{self.crates[0].path}-"
        else:
            return ""

    @property
    def is_breaking(self) -> bool:
        return self.has_breaking_commit or self.is_breaking_semver_checks

    @property
    def dir(self) -> str:
        return f"{self.crates[0].path}" if self.single_crate else ""

    @property
    def prev_tag(self) -> str:
        return f"{self.prefix}v{self.prev_version}"

    @property
    def curr_tag(self) -> str:
        return f"{self.prefix}v{self.curr_version}"

    @property
    def commit_count(self) -> int:
        return len(self.commits)

    @property
    def changelog_path(self) -> str:
        return f"{self.dir}/CHANGELOG.md" if self.single_crate else "CHANGELOG.md"

    @property
    def tag_changelog(self) -> str:
        return re.sub(
            r"\[.*?\]\((.*?)\)",
            r"\1",
            "\n".join(
                self.changelog.replace(
                    "\n<details><summary>view details</summary>\n", ""
                )
                .replace("</details>", "")
                .splitlines()[2:]
            ).strip(),
        )


def create_changelog_update(
    prefix: str,
    new_version: Version,
    independent_crate: bool,
    independent_crates: Iterable[CrateInfo],
) -> str:
    if independent_crate:
        extra_args = ["--include-path", f"{prefix}/**/*"]
    else:
        extra_args = sum(
            [["--exclude-path", f"{crate.path}/**/*"] for crate in independent_crates],
            [],
        )

    command = (
        ["git-cliff"]
        + extra_args
        + ["--strip", "all", "--unreleased", "--tag", str(new_version)]
    )
    return process_run(command, encoding="utf-8").strip()


def gather_release(
    crates: list[CrateInfo], independent_crates: list[CrateInfo]
) -> ReleaseInfo:
    independent_crate = len(crates) == 1
    prefix = "" if not independent_crate else crates[0].name.removeprefix("tulpje-")

    if independent_crate:
        file_whitelist = RELEASE_FILENAME_MATCHLIST
    else:
        file_whitelist = RELEASE_FILENAME_MATCHLIST.union(
            RELEASE_FILENAME_MATCHLIST_WORKSPACE
        ).union({f"!{crate.path}/**/*" for crate in independent_crates})

    latest_tag = get_latest_tag(f"{prefix}-" if len(prefix) > 0 else "")
    # fall back to main tag if there's none for the prefix yet
    if len(prefix) > 0 and latest_tag is None:
        latest_tag = get_latest_tag()

    if latest_tag is None:
        raise Exception("ERR: couldn't find the previous tag")

    if independent_crate:
        commits = filter_commits_by_path(get_commits_since_ref(latest_tag), [prefix])
    else:
        commits = filter_commits_by_path(
            get_commits_since_ref(latest_tag),
            [crate.path for crate in independent_crates],
            True,
        )

    has_feature_commit = any(commit.subject.startswith("feat") for commit in commits)
    has_breaking_change_commit = any(commit.breaking for commit in commits)
    has_breaking_change_semver_checks, _ = cargo_semver_checks(
        latest_tag, crates[0].name if independent_crate else None
    )
    should_release = should_create_release(commits, file_whitelist, prefix)
    old_version = Version.parse(latest_tag.removeprefix(prefix).removeprefix("v"))
    new_version = old_version.bumped(
        has_feature_commit,
        has_breaking_change_commit or has_breaking_change_semver_checks,
    )

    new_changelog = create_changelog_update(
        prefix, new_version, independent_crate, independent_crates
    )

    return ReleaseInfo(
        crates=tuple(crates),
        commits=tuple(commits),
        prev_version=old_version,
        curr_version=new_version,
        changelog=new_changelog,
        should_release=should_release,
        has_feature_commit=has_feature_commit,
        has_breaking_commit=has_breaking_change_commit,
        is_breaking_semver_checks=has_breaking_change_semver_checks,
    )


def process_run(*args, **kwargs) -> str:
    """run a process and print it's output if it errors, otherwise return output"""
    output_on_error = kwargs.pop("output_on_error", True)
    try:
        return subprocess.check_output(*args, **kwargs)
    except subprocess.CalledProcessError as e:
        if output_on_error:
            print(e.output)
        raise e


def lock_bump_version(crate: CrateInfo, new_version: Version):
    with open(crate.lock_file) as manifest_file:
        manifest = tomlkit.load(manifest_file)

    packages = manifest["package"]
    assert isinstance(packages, AoT)

    package = next(filter(lambda pkg: pkg["name"] == crate.name, packages))
    if package is None:
        raise Exception("Couldn't find package `{crate.name}` in `{lock_file}`")

    package["version"] = str(new_version)

    with open(crate.lock_file, "w") as manifest_file:
        tomlkit.dump(manifest, manifest_file)


def workspace_bump_version(
    manifest_path: str, crates: Iterable[CrateInfo], version: Version
):
    with open(manifest_path) as manifest_file:
        manifest = tomlkit.load(manifest_file)
        manifest["workspace"]["package"][  # pyright: ignore[reportIndexIssue]
            "version"
        ] = str(version)

    with open(manifest_path, "w") as manifest_file:
        tomlkit.dump(manifest, manifest_file)

    for crate in crates:
        lock_bump_version(crate, version)


def manifest_bump_version(crate: CrateInfo, version: Version):
    with open(crate.manifest) as manifest_file:
        manifest = tomlkit.load(manifest_file)
        if not isinstance(
            manifest["package"]["version"], str  # pyright: ignore[reportIndexIssue]
        ):

            raise Exception(
                f"Crate {manifest["name"]} doesn't have a string version, likely workspace crate"
            )

        manifest["package"]["version"] = str(  # pyright: ignore[reportIndexIssue]
            version
        )
    with open(crate.manifest, "w") as manifest_file:
        tomlkit.dump(manifest, manifest_file)

    lock_bump_version(crate, version)


def workspace_update_dependency(
    crate: CrateInfo, dependency: CrateInfo, new_version: Version
):
    """Update workspace `crate`'s dependency on `dependency` to `new_version`"""
    with open(crate.manifest) as manifest_file:
        manifest = tomlkit.load(manifest_file)
        manifest["dependencies"][dependency.name][  # pyright: ignore[reportIndexIssue]
            "version"
        ] = str(new_version)
    with open(crate.manifest, "w") as manifest_file:
        tomlkit.dump(manifest, manifest_file)


def sort_releases_by_deps(releases: list[ReleaseInfo]) -> list[ReleaseInfo]:
    # crate.name -> ReleaseInfo
    releases_by_crate: dict[str, ReleaseInfo] = {}
    for release in releases:
        for crate in release.crates:
            releases_by_crate[crate.name] = release

    # create an order of which dependencies to evaluate first
    dependency_order = [
        *TopologicalSorter(
            {
                crate.name: crate.workspace_dependencies
                for release in releases
                for crate in release.crates
            }
        ).static_order()
    ]
    releases_by_deps: list[ReleaseInfo] = []
    for crate in dependency_order:
        release = releases_by_crate[crate]
        if not release in releases_by_deps:
            releases_by_deps.append(releases_by_crate[crate])

    return releases_by_deps


def process_dependencies(releases_by_deps: list[ReleaseInfo]) -> list[ReleaseInfo]:
    releases_copy = [ReleaseInfo(**release.__dict__) for release in releases_by_deps]

    # crate.name -> ReleaseInfo
    releases_by_crate: dict[str, ReleaseInfo] = {}
    for release in releases_copy:
        for crate in release.crates:
            releases_by_crate[crate.name] = release

    # reverse dependency lookup
    depended_on_by: dict[str, set[CrateInfo]] = defaultdict(set)
    for release in releases_by_deps:
        for crate in release.crates:
            for dependency in crate.workspace_dependencies:
                depended_on_by[dependency].add(crate)

    # iterate through dependencies and update versions where required
    for release in releases_copy:
        if release.should_release:
            for crate in release.crates:
                for depended_crate in depended_on_by[crate.name]:
                    crate_release = releases_by_crate[depended_crate.name]
                    crate_release.should_release = True
                    if not crate_release.changed:
                        crate_release.curr_version = crate_release.curr_version.bumped(
                            False, False
                        )
    return [release for release in releases_copy if release.should_release]


# TODO: Enforce independent crates not depending on workspace crates
def do_releases(releases_by_deps: list[ReleaseInfo], execute=False):
    if len(releases_by_deps) == 0:
        print(" [*] Nothing to release")
        return

    for release in releases_by_deps:
        print(f"Crate: {release.crates[0].name if release.single_crate else "root"}")
        print(f" [*] Should release: {release.should_release}")
        print(f" [*] Version: {release.prev_version} -> {release.curr_version}")
        print(f" [*] Tag: {release.prev_tag} -> {release.curr_tag}")
        print(f" [*] Commits since: {release.commit_count}")
        print(f" [*] Feature: {release.has_feature_commit}")
        print(f" [*] Breaking: {release.breaking_with_reason()}")
        print(f" [*] Commits ({release.commit_count}):")
        for commit in release.commits:
            print(f"    - {commit.raw_subject} ({commit.sha[:8]})")

    # reverse dependency lookup
    depended_on_by: dict[str, set[CrateInfo]] = defaultdict(set)
    for release in releases_by_deps:
        for crate in release.crates:
            for dependency in crate.workspace_dependencies:
                depended_on_by[dependency].add(crate)

    filtered_releases = [
        release for release in releases_by_deps if release.should_release
    ]

    print(" [-] Bumping versions..." + ("" if execute else " (dry-run)"))
    for release in filtered_releases:
        name = release.crates[0].name if release.single_crate else "tulpje"
        print(f"     - {name}: {release.prev_version} -> {release.curr_version}")

        if execute:
            if release.single_crate:
                manifest_bump_version(release.crates[0], release.curr_version)
            else:
                workspace_bump_version(
                    "Cargo.toml", release.crates, release.curr_version
                )

            for crate in release.crates:
                for depends_on in depended_on_by[crate.name]:
                    workspace_update_dependency(depends_on, crate, release.curr_version)

    print(" [-] Writing changelogs..." + ("" if execute else " (dry-run)"))
    if execute:
        for release in filtered_releases:
            with open(release.changelog_path) as changelog_file:
                new_changelog = changelog_file.read().replace(
                    "##", f"{release.changelog}\n\n##", 1
                )

            with open(release.changelog_path, "w") as new_changelog_file:
                new_changelog_file.write(new_changelog)

    check_output_dry(
        " [-] Committing changes...",
        execute,
        [
            "git",
            "add",
            "CHANGELOG.md",
            "*/CHANGELOG.md",
            "Cargo.lock",
            "Cargo.toml",
            "*/Cargo.toml",
        ],
    )

    commit_message = "release: " + (
        ", ".join(
            f"{release.crates[0].name if release.single_crate else "tulpje"} v{release.curr_version}"
            for release in reversed(filtered_releases)
        )
    )
    check_output_dry(
        None,
        execute,
        ["git", "commit", "--cleanup=verbatim", "--message", commit_message],
    )

    print(" [-] Tagging release...")
    for release in filtered_releases:
        check_output_dry(
            None,
            execute,
            ["git", "tag", "--cleanup=verbatim", "--file=-", release.curr_tag],
            input=release.tag_changelog.encode("utf-8"),
        )

    check_output_dry(
        " [-] Pushing release...",
        execute,
        ["git", "push", "origin", "main"]
        + [release.curr_tag for release in filtered_releases],
    )

    print(" [-] Creating GitHub releases...")
    for release in filtered_releases:
        check_output_dry(
            f"    - {release.crates[0].name if release.single_crate else "tulpje"}",
            execute,
            [
                "gh",
                "release",
                "create",
                release.curr_tag,
                "--notes-file=-",
                "--title",
                release.curr_tag,
            ],
            input=release.changelog.encode("utf-8"),
        )


def check_output_dry(title: Optional[str], execute: bool, *args, **kwargs):
    if title is not None:
        print(title)

    print("     " + ("" if execute else "(dry-run) ") + "> " + " ".join(args[0]))

    if execute:
        return process_run(*args, **kwargs)
    else:
        return ""


def gather_crates() -> list[CrateInfo]:
    try:
        members = tomllib.load(open("Cargo.toml", "rb"))["workspace"]["members"]
    except IndexError:
        members = []

    return [CrateInfo.from_manifest(f"{member}/Cargo.toml") for member in members]


def main(args: argparse.Namespace) -> int:
    crates = gather_crates()
    independent_crates = [crate for crate in crates if crate.independent]
    grouped_crates = [crate for crate in crates if not crate.independent]

    releases = [
        gather_release([crate], independent_crates) for crate in independent_crates
    ] + [gather_release(grouped_crates, independent_crates)]
    releases_by_deps = sort_releases_by_deps(releases)
    releasable = process_dependencies(releases_by_deps)
    do_releases(releasable, args.execute)

    return 0


if __name__ == "__main__":
    sys.exit(main(argparser().parse_args(sys.argv[1:])))
