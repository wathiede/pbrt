#!/usr/bin/env python3
# Copyright 2018 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
import datetime
import os
import re
import sys
import tempfile

DOUBLE_SLASH_COMMENT_EXTS = ['.rs']
SLASH_STAR_CMMENT_EXTS = []
HASH_COMMENT_EXTS = ['.py']
SOURCE_EXTS = tuple(DOUBLE_SLASH_COMMENT_EXTS + SLASH_STAR_CMMENT_EXTS + HASH_COMMENT_EXTS)
LICENSE_HEADER = """Copyright YYYY Google LLC

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
""".replace('YYYY', str(datetime.date.today().year))

_HAS_COPYRIGHT_RE = re.compile(r'Copyright \d{4} Google LLC')
_DOUBLE_SLASH_COPYRIGHT = '\n'.join(['// {}'.format(l) if len(l) > 0 else '//'
    for l in LICENSE_HEADER.splitlines()])
_HASH_COPYRIGHT = '\n'.join(['# {}'.format(l) if len(l) > 0 else '#'
    for l in LICENSE_HEADER.splitlines()])

def atomic_write(path, data):
    mode = os.stat(path).st_mode
    with tempfile.NamedTemporaryFile(dir=os.path.dirname(path), mode='w+',
            delete=False) as fp:
        fp.write(data)
    os.replace(fp.name, path)
    os.chmod(path, mode)

def atomic_insert(path, license):
    source = open(path).read()
    if source.startswith('#!'):
        shebang, rest = source.split('\n', 1)
        atomic_write(path, '{}\n{}\n{}'.format(shebang, license, rest))
    else:
        atomic_write(path, '{}\n{}'.format(license, source))

def insert_double_slash_copyright(path):
    atomic_insert(path, _DOUBLE_SLASH_COPYRIGHT)


def insert_hash_copyright(path):
    atomic_insert(path, _HASH_COPYRIGHT)


def insert_copyright(path):
    _, ext = os.path.splitext(path)
    if ext in DOUBLE_SLASH_COMMENT_EXTS:
        insert_double_slash_copyright(path)
    elif ext in HASH_COMMENT_EXTS:
        insert_hash_copyright(path)
    else:
        raise SystemExit('Unknown comment style for {}'.format(path))


def check_source(presubmit, paths):
    missing_copyright = []
    for path in paths:
        _, ext = os.path.splitext(path)
        if ext in SOURCE_EXTS:
            src = open(path).read()
            if not _HAS_COPYRIGHT_RE.search(src):
                if presubmit:
                    missing_copyright.append(path)
                else:
                    print('Adding copyright to {}'.format(path))
                    insert_copyright(path)
    if presubmit and missing_copyright:
        print('The following files need a copyright header, run '
                'scripts/insert.py to '
                'fix:\n{}'.format('\n'.join(missing_copyright)))
        sys.exit(1)


def all_source():
    root = os.path.dirname(os.path.dirname(os.path.realpath(__file__)))
    for root, dirs, files in os.walk(root):
        if '.git' in dirs:
            dirs.remove('.git') # Don't recurse into .git
        for f in files:
            yield os.path.join(root, f)


if __name__ == '__main__':
    presubmit = False
    paths = all_source()
    if len(sys.argv) > 1:
        if sys.argv[1] == '--presubmit':
            presubmit = True
            if len(sys.argv) > 2:
                paths = sys.argv[2:]
        else:
            print('Unknown flag {}'.format(sys.argv[1]))
            sys.exit(1)

    check_source(presubmit, paths)

