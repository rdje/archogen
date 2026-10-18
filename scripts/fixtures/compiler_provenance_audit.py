#!/usr/bin/env python3
"""Audit a prototype receipt against raw records, literal units and actual artifacts."""
import argparse
import copy
import hashlib
import json
from pathlib import Path

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--input', required=True, help='repository-relative prototype-results.json')
options = parser.parse_args()
if not __debug__:
    parser.error('audit assertions require Python without -O')
root = Path(__file__).resolve().parents[2]
relative = Path(options.input)
if relative.is_absolute() or '..' in relative.parts:
    parser.error('--input must be repository-relative')
receipt = root / relative
if not receipt.resolve().is_relative_to(root):
    parser.error('--input must resolve inside the repository')
out = receipt.parent
if out.stat().st_dev != root.stat().st_dev:
    parser.error('--input must be on the repository volume')
roles = {
    'root-bin': ['common', 'app'],
    'root-lib': ['common', 'api'],
    'harness': ['common', 'api', 'adapter'],
    'generator': ['common', 'generator'],
}
modes = ['ordinary', 'noncompiler', 'rustc', 'command', 'stdout-escaped-json']
expected = {mode + '-' + role: units for mode in modes for role, units in roles.items()}


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def check(data):
    assert {case['case'] for case in data['cases']} == set(expected)
    assert len(data['cases']) == 20
    assert set(data['sources']) == {
        'scripts/trust_compiler_provenance_probe.py',
        'scripts/fixtures/compiler_provenance_observer.rs',
        'scripts/fixtures/compiler_provenance_receiver.c',
        'xtask/src/catalog_build.rs',
        'xtask/src/trust.rs',
    }
    for source, sha in data['sources'].items():
        assert digest(root / source) == sha
    for case in data['cases']:
        assert case['exit'] == 0 and sorted(case['units']) == sorted(expected[case['case']])
        assert case['queries'] == 3
        directory = out / ('target-' + case['case']) / '.archogen-provenance/traces'
        compilations = []
        queries = 0
        for file in directory.iterdir():
            record = json.loads(file.read_bytes())
            argv = [bytes.fromhex(value) for value in record['argv_hex']]
            assert record['exit'] == 0 and record['signal'] is None
            assert bytes.fromhex(record['session_hex']) == ('session-' + case['case']).encode()
            assert argv[0] == b'rustc'
            if any(value.startswith(b'--emit=') for value in argv):
                compilations.append(argv[argv.index(b'--crate-name') + 1].decode())
            else:
                queries += 1
        assert sorted(compilations) == sorted(expected[case['case']]) and queries == 3
        assert len(case['artifacts']) == len(compilations) and not case['payload_objects']
    assert len(data['artifact_queries']) == 20
    assert sum(len(case['checks']) for case in data['artifact_queries']) == 45
    for case in data['artifact_queries']:
        assert case['build_files_unchanged']
        assert sorted(item['crate'] for item in case['checks']) == sorted(expected[case['case']])
        for item in case['checks']:
            assert item['exit'] == 0 and item['artifacts']
            for artifact in item['artifacts']:
                file = out / ('target-' + case['case']) / 'release/deps' / artifact['name']
                assert artifact['report_agrees'] and digest(file) == artifact['sha256']
    assert len(data['refusals']) == len({item['fault'] for item in data['refusals']}) == 14
    assert all(item['rejected'] for item in data['refusals'])
    assert data['concurrent_writers'] == 8 and data['transport']['native_argv_matches']
    assert data['signal_forwarding'] == {'exit': 143, 'signal': 15}
    assert data['producer_setup_failure'] == {'exit': 2, 'requested_program_not_executed': True}
    assert data['compiler_spawn_failure'] == {'exit': 2, 'partial_record_refused': True}


data = json.loads(receipt.read_bytes())
check(data)
rejected = []
for fault in ['missing-case', 'false-unit', 'false-build-exit', 'false-query-count',
              'false-artifact-hash', 'false-source', 'missing-source', 'accepted-refusal', 'false-restoration']:
    changed = copy.deepcopy(data)
    if fault == 'missing-case':
        changed['cases'].pop()
    elif fault == 'false-unit':
        changed['cases'][0]['units'] = ['phantom']
    elif fault == 'false-build-exit':
        changed['cases'][0]['exit'] = 1
    elif fault == 'false-query-count':
        changed['cases'][0]['queries'] = 2
    elif fault == 'false-artifact-hash':
        changed['artifact_queries'][0]['checks'][0]['artifacts'][0]['sha256'] = '0' * 64
    elif fault == 'false-source':
        changed['sources']['scripts/trust_compiler_provenance_probe.py'] = '0' * 64
    elif fault == 'missing-source':
        changed['sources'] = {}
    elif fault == 'accepted-refusal':
        changed['refusals'][0]['rejected'] = False
    else:
        changed['artifact_queries'][0]['build_files_unchanged'] = False
    try:
        check(changed)
    except AssertionError:
        rejected.append(fault)
    else:
        raise AssertionError('false receipt accepted: ' + fault)
result = dict(status='passed', native_matrix=20, actual_compilations=45,
              false_receipts_rejected=rejected, receipt_sha256=digest(receipt),
              audit_producer_sha256=digest(Path(__file__)))
(out / 'summary-audit.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps(result))
