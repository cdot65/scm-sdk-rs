#!/usr/bin/env python3
"""Read-only prepublish guard for the reviewed management patch release."""
import argparse
import importlib.util
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]


def module(name, filename):
    spec = importlib.util.spec_from_file_location(name, ROOT / 'scripts' / filename)
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


def verify(review, phase=24, version="0.2.3"):
    if (phase, version) not in {(24, "0.2.3"), (28, "0.2.4")}:
        raise ValueError("Unreviewed release phase/version pair")
    publication = module('prepublish_publication', 'verify-publication.py')
    readiness = publication.require_release_readiness(version, review)
    candidate = module('prepublish_candidate', 'check-access-release.py')
    proof = candidate.verify(ROOT, phase=phase, version=version)
    if publication.require_release_readiness(version, review) != readiness:
        raise ValueError('Readiness changed during prepublication verification')
    return {'passed': True, 'version': version, 'releaseReadiness': readiness,
            'sourceCommit': proof['releaseCommit'], 'archiveSha256': proof['archiveSha256'],
            'publicationPerformed': False}


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--readiness-review', required=True, type=pathlib.Path)
    parser.add_argument("--phase", type=int, default=24)
    parser.add_argument("--version", default="0.2.3")
    args = parser.parse_args()
    try:
        print(json.dumps(verify(args.readiness_review, args.phase, args.version)))
    except (ValueError, OSError, KeyError) as error:
        sys.exit('Prepublication guard failed: ' + str(error))
