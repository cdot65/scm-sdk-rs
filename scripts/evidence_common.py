"""Bounded, non-secret IO shared by auxiliary qualification runners."""
import hashlib
import os
import pathlib
import secrets
import signal
import stat
import subprocess
import tempfile
import time

MAX_FILE_BYTES = 64 * 1024 * 1024
MAX_TREE_BYTES = 256 * 1024 * 1024
MAX_FILES = 32768
EXCLUDED_DIRECTORIES = ("docs/reviews/", "docs/checks/")
EXCLUDED_FILES = {"docs/phase-status.json"}


def clean_environment():
    environment = os.environ.copy()
    for name in list(environment):
        if name.startswith(("PANW_", "AIRS_", "SCM_", "GIT_", "RUST_TEST_")):
            del environment[name]
    environment["CARGO_TERM_COLOR"] = "never"
    environment.pop("RUST_LOG", None)
    return environment


def safe_parts(path):
    """Require a normalized absolute path, without a symlink component."""
    path = pathlib.Path(path)
    if not path.is_absolute() or ".." in path.parts:
        raise ValueError("An absolute ordinary path is required")
    if any(part.is_symlink() for part in (path, *path.parents)):
        raise ValueError("Symlink paths are not accepted")
    return path


def open_regular(path, flags):
    """Use no-follow directory traversal where the platform supports dir_fd."""
    path = safe_parts(path)
    flags |= getattr(os, "O_NOFOLLOW", 0) | getattr(os, "O_NONBLOCK", 0)
    if os.open in os.supports_dir_fd and hasattr(os, "O_DIRECTORY"):
        directory = parent_descriptor(path)
        try:
            return os.open(path.name, flags, 0o600, dir_fd=directory)
        finally:
            os.close(directory)
    return os.open(path, flags, 0o600)


def parent_descriptor(path):
    """Open an already validated path's parent without following any component."""
    directory = os.open(path.anchor, os.O_RDONLY | os.O_DIRECTORY)
    try:
        for component in path.parts[1:-1]:
            next_directory = os.open(component, os.O_RDONLY | os.O_DIRECTORY
                                     | getattr(os, "O_NOFOLLOW", 0), dir_fd=directory)
            os.close(directory)
            directory = next_directory
        return directory
    except BaseException:
        os.close(directory)
        raise


def read_regular(path, limit=MAX_FILE_BYTES):
    descriptor = open_regular(path, os.O_RDONLY)
    try:
        before = os.fstat(descriptor)
        if not stat.S_ISREG(before.st_mode) or before.st_size > limit:
            raise ValueError("Input must be a bounded regular file")
        with os.fdopen(descriptor, "rb", closefd=False) as stream:
            content = stream.read(limit + 1)
        after = os.fstat(descriptor)
        if (len(content) > limit or before.st_size != len(content)
                or (before.st_dev, before.st_ino, before.st_mtime_ns, before.st_size)
                != (after.st_dev, after.st_ino, after.st_mtime_ns, after.st_size)):
            raise ValueError("Input exceeded its bound or changed during reading")
        return content
    finally:
        os.close(descriptor)


def write_private(path, content):
    """Create a new 0600 receipt without overwriting an input or following links."""
    descriptor = open_regular(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL)
    try:
        with os.fdopen(descriptor, "wb", closefd=False) as stream:
            stream.write(content)
            stream.flush()
            os.fsync(descriptor)
    finally:
        os.close(descriptor)


def atomic_replace_private(path, content):
    """Atomically replace an ordinary report inside its opened parent directory."""
    path = safe_parts(path)
    if not (os.open in os.supports_dir_fd and hasattr(os, "O_DIRECTORY")):
        raise ValueError("Report replacement requires directory-relative filesystem primitives")
    directory = parent_descriptor(path)
    temporary = ".scm-evidence-" + secrets.token_hex(16) + ".tmp"
    created = False
    try:
        def validate_destination():
            try:
                info = os.stat(path.name, dir_fd=directory, follow_symlinks=False)
            except FileNotFoundError:
                return
            if not stat.S_ISREG(info.st_mode):
                raise ValueError("Report destination must be an ordinary file")
        validate_destination()
        descriptor = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL
                              | getattr(os, "O_NOFOLLOW", 0), 0o600, dir_fd=directory)
        created = True
        with os.fdopen(descriptor, "wb") as stream:
            stream.write(content)
            stream.flush()
            os.fsync(stream.fileno())
        validate_destination()
        os.replace(temporary, path.name, src_dir_fd=directory, dst_dir_fd=directory)
        created = False
        os.fsync(directory)
    finally:
        if created:
            os.unlink(temporary, dir_fd=directory)
        os.close(directory)


def run_bounded(command, root, environment, timeout, output_limit=4 * 1024 * 1024):
    """Bound child time and captured output; kill its process group on POSIX."""
    start = time.monotonic()
    with tempfile.TemporaryFile() as stdout, tempfile.TemporaryFile() as stderr:
        process = subprocess.Popen(command, cwd=root, env=environment, stdin=subprocess.DEVNULL,
                                   stdout=stdout, stderr=stderr, start_new_session=os.name == "posix")
        reason = None
        while process.poll() is None:
            if time.monotonic() - start >= timeout:
                reason = "timeout"
            if os.fstat(stdout.fileno()).st_size + os.fstat(stderr.fileno()).st_size > output_limit:
                reason = "output_limit"
            if reason:
                if os.name == "posix":
                    try:
                        os.killpg(process.pid, signal.SIGKILL)
                    except ProcessLookupError:
                        pass
                else:
                    process.kill()
                break
            time.sleep(0.02)
        process.wait()
        stdout.seek(0)
        stderr.seek(0)
        out, err = stdout.read(output_limit + 1), stderr.read(output_limit + 1)
        if len(out) + len(err) > output_limit:
            reason = "output_limit"
        return {"exitCode": process.returncode, "failure": reason,
                "durationSeconds": round(time.monotonic() - start, 3),
                "stdout": out, "stderr": err}


def git_output(root, *arguments, timeout=30, output_limit=8 * 1024 * 1024):
    result = run_bounded(["git", *arguments], root, clean_environment(), timeout,
                         output_limit=output_limit)
    if result["exitCode"] or result["failure"]:
        raise ValueError("Git evidence acquisition failed")
    return result["stdout"]


def git_files(root, *, source_only=False):
    content = git_output(root, "ls-files", "--cached", "--others", "--exclude-standard", "-z")
    names = sorted(set(content.decode("utf-8").split("\0")) - {""})
    if source_only:
        names = [name for name in names
                 if name not in EXCLUDED_FILES and not name.startswith(EXCLUDED_DIRECTORIES)]
    if len(names) > MAX_FILES:
        raise ValueError("Repository file-count bound exceeded")
    files, total = {}, 0
    for name in names:
        relative = pathlib.PurePosixPath(name)
        if (relative.is_absolute() or ".." in relative.parts or "\\" in name
                or any(ord(character) < 32 or ord(character) == 127 for character in name)):
            raise ValueError("Repository contains an unsupported path")
        content = read_regular(root / name)
        total += len(content)
        if total > MAX_TREE_BYTES:
            raise ValueError("Repository byte-count bound exceeded")
        files[name] = content
    return files


def source_fingerprint(files):
    digest = hashlib.sha256()
    for name, content in sorted(files.items()):
        if name in EXCLUDED_FILES or name.startswith(EXCLUDED_DIRECTORIES):
            continue
        digest.update(name.encode() + b"\0" + hashlib.sha256(content).digest())
    return digest.hexdigest()


def runtime_fingerprint(root):
    paths = {root / name for name in ("Cargo.toml", "Cargo.lock", "LICENSE", "NOTICE",
                                      "crates/scm-sdk/Cargo.toml")}
    for directory in ("src", "examples"):
        base = root / "crates/scm-sdk" / directory
        for path in base.rglob("*"):
            if path.is_symlink():
                raise ValueError("Runtime inputs must be ordinary paths")
            if path.is_file():
                paths.add(path)
    if len(paths) > MAX_FILES:
        raise ValueError("Runtime file-count bound exceeded")
    digest, total = hashlib.sha256(), 0
    for path in sorted(paths):
        content = read_regular(path)
        total += len(content)
        if total > MAX_TREE_BYTES:
            raise ValueError("Runtime byte-count bound exceeded")
        digest.update(path.relative_to(root).as_posix().encode() + b"\0"
                      + hashlib.sha256(content).digest())
    return digest.hexdigest()
