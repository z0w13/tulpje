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
                return Version(self.major, self.minor + 1, self.patch)
            else:
                return Version(self.major, self.minor, self.patch + 1)
        else:
            if breaking:
                return Version(self.major + 1, self.minor, self.patch)
            elif feature:
                return Version(self.major, self.minor + 1, self.patch)
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
    latest = latest_version([t.removeprefix(f"v{prefix}") for t in tags])
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
    ".sqlx/*" "*.sql" "Cargo.lock",
    "compose.*.yml",
    "Dockerfile*",
    "!framework/**/*",
}


@dataclass
class ReleaseInfo:
    crates: list[CrateInfo]

    prev_version: Version
    curr_version: Version

    changelog: str

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
    def dir(self) -> str:
        return f"{self.crates[0].path}" if self.single_crate else ""

    @property
    def prev_tag(self) -> str:
        return f"{self.prefix}v{self.prev_version}"

    @property
    def curr_tag(self) -> str:
        return f"{self.prefix}v{self.curr_version}"

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
        )

    latest_tag = get_latest_tag(prefix)
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
    create_release = should_create_release(commits, file_whitelist, prefix)
    old_version = Version.parse(latest_tag.removeprefix(prefix).removeprefix("v"))
    new_version = old_version.bumped(
        has_feature_commit,
        has_breaking_change_commit,
    )
    new_tag = f"{prefix}-v{new_version}" if independent_crate else f"v{new_version}"

    print(f"Crate: {crates[0].name if independent_crate else "root"}")
    print(f" [*] Latest tag: {latest_tag}")
    print(f" [*] Commits since: {len(commits)}")
    print(f" [*] Feature: {has_feature_commit}")
    print(f" [*] Breaking change (commit): {has_breaking_change_commit}")
    print(f" [*] Breaking change (semver-checks): {has_breaking_change_semver_checks}")
    print(f" [*] Create release: {create_release}")
    print(f" [*] Version: {old_version} -> {new_version}")
    print(f" [*] Tag: {latest_tag} -> {new_tag}")
    print(f" [*] Commits ({len(commits)}):")
    for commit in commits:
        print(f"    - {commit.raw_subject} ({commit.sha[:8]})")

    print(" [-] Generating changelog...")
    new_changelog = create_changelog_update(
        prefix, new_version, independent_crate, independent_crates
    )
    print(" [*] Changelog Additions: ")
    print(new_changelog)

    return ReleaseInfo(
        crates=crates,
        prev_version=old_version,
        curr_version=new_version,
        changelog=new_changelog,
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
    manifest_path: str, crates: list[CrateInfo], version: Version
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


# TODO: Enforce independent crates not depending on workspace crates
def do_releases(releases: list[ReleaseInfo], crates: list[CrateInfo], execute=False):
    graph = TopologicalSorter(
        {crate.name: crate.workspace_dependencies for crate in crates}
    )
    dependency_order = [*graph.static_order()]

    releases_by_crate: dict[str, ReleaseInfo] = {}
    for release in releases:
        for crate in release.crates:
            releases_by_crate[crate.name] = release

    releases_sorted_by_deps = []
    for crate in dependency_order:
        release = releases_by_crate[crate]
        if not release in releases_sorted_by_deps:
            releases_sorted_by_deps.append(releases_by_crate[crate])

    depended_on_by: dict[str, set[CrateInfo]] = defaultdict(set)
    for crate in crates:
        for dependency in crate.workspace_dependencies:
            depended_on_by[dependency].add(crate)

    print(" [-] Bumping versions..." + ("" if execute else " (dry-run)"))
    for release in releases_sorted_by_deps:
        if release.changed:
            name = release.crates[0].name if release.single_crate else "tulpje"

            if release.single_crate:
                for crate in depended_on_by[release.crates[0].name]:
                    crate_release = releases_by_crate[crate.name]
                    if not crate_release.changed:
                        crate_release.curr_version = crate_release.curr_version.bumped(
                            False, False
                        )

    if execute:
        for release in releases_sorted_by_deps:
            name = release.crates[0].name if release.single_crate else "tulpje"
            print(f"     - {name}: {release.prev_version} -> {release.curr_version}")

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
        for release in releases:
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
            for release in reversed(releases)
        )
    )
    check_output_dry(
        None,
        execute,
        ["git", "commit", "--cleanup=verbatim", "--message", commit_message],
    )

    print(" [-] Tagging release...")
    for release in releases:
        check_output_dry(
            None,
            execute,
            ["git", "tag", "--cleanup=verbatim", "--file=-", release.curr_tag],
            input=release.tag_changelog.encode("utf-8"),
        )

    check_output_dry(
        " [-] Pushing release...",
        execute,
        ["git", "push", "origin", "main"] + [release.curr_tag for release in releases],
    )

    print(" [-] Creating GitHub releases...")
    for release in releases:
        check_output_dry(
            f"    - {release.crates[0].name if release.single_crate else "tulpje"}",
            execute,
            ["gh", "release", "create", release.curr_tag, "--notes-file=-"],
            input=release.changelog.encode("utf-8"),
        )


def check_output_dry(title: Optional[str], execute: bool, *args, **kwargs):
    if title is not None:
        print(title)

    print(("" if execute else "(dry-run) ") + "> " + " ".join(args[0]))

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
    do_releases(releases, crates, args.execute)

    return 0


if __name__ == "__main__":
    sys.exit(main(argparser().parse_args(sys.argv[1:])))
