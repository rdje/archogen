#!/usr/bin/env python3
"""Exercise a Unix compiler-provenance prototype; this does not change trust-inventory.

Run from any cwd with a new --out directory. All outputs stay under this repository's
same-volume target/. Native toolchain/OS dependencies are read only. Assertions are
intentional probe judgments; invoke with python3 -I -B, without optimization.
"""
from pathlib import Path
import argparse
import hashlib
import json
import os
import shutil
import subprocess
import tomllib

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--out', default='target/compiler-provenance-probe', help='new repository-relative output under target/')
options = parser.parse_args()
if not __debug__:
    parser.error('probe assertions require Python without -O')
root = Path(__file__).resolve().parent.parent
relative = Path(options.out)
if relative.is_absolute() or not relative.parts or relative.parts[0] != 'target' or '..' in relative.parts:
    parser.error('--out must be a new repository-relative directory under target/')
out = root / relative
ancestor = out.parent
while not ancestor.exists():
    ancestor = ancestor.parent
if not out.parent.resolve().is_relative_to(root) or ancestor.stat().st_dev != root.stat().st_dev:
    parser.error('--out must resolve on the repository volume')
out.parent.mkdir(parents=True, exist_ok=True)
out.mkdir()
for key in ['TMPDIR', 'TMP', 'TEMP']:
    (out/'tmp').mkdir(exist_ok=True)
    os.environ[key] = str(out/'tmp')
os.environ['CARGO_HOME'] = str(out/'cargo-home')
os.environ['XDG_CACHE_HOME'] = str(out/'cache')
pin = tomllib.loads((root/'rust-toolchain.toml').read_text())['toolchain']['channel']
os.environ['RUSTUP_TOOLCHAIN'] = pin
catalog = (root/'xtask/src/catalog_build.rs').read_text()
start = catalog.index('pub fn environment(')
fragment = catalog[start:catalog.index('\nfn command(', start)]
helper_source = 'use std::{fs, path::Path, io::Write};\n' + fragment + r'''
fn main() {
    let args: Vec<String> = std::env::args().collect();
    for (key, value) in environment(&args[1], Path::new(&args[2]), Path::new(&args[3])).unwrap() {
        std::io::stdout().write_all(key.as_bytes()).unwrap();
        std::io::stdout().write_all(&[0]).unwrap();
        std::io::stdout().write_all(value.as_bytes()).unwrap();
        std::io::stdout().write_all(&[0]).unwrap();
    }
}
'''
(out/'environment.rs').write_text(helper_source)
helpers = out/'environment'
for source, destination, compiler, flags in [
    (out/'environment.rs', helpers, 'rustc', ['--edition=2021']),
    (root/'scripts/fixtures/compiler_provenance_observer.rs', out/'observer', 'rustc', ['--edition=2021']),
    (root/'scripts/fixtures/compiler_provenance_receiver.c', out/'receiver', 'cc', ['-Wall','-Wextra','-Werror']),
]:
    subprocess.run([compiler,*flags,str(source),'-o',str(destination)],check=True,cwd=root)

def environment(target):
    fields = subprocess.check_output([helpers, pin, out/'cargo-home', target]).split(b'\0')
    assert fields.pop() == b''
    values = dict(zip(fields[::2], fields[1::2]))
    assert len(values) == 9
    values[b'RUSTC'] = b'rustc'
    return values

def unique_object(pairs):
    result = {}
    for key, value in pairs:
        assert key not in result, 'duplicate JSON field'
        result[key] = value
    return result


def decode_hex(value):
    assert isinstance(value, str) and len(value) % 2 == 0, 'hex framing'
    assert all(char in '0123456789abcdef' for char in value), 'hex alphabet'
    decoded = bytes.fromhex(value)
    assert b'\0' not in decoded, 'native argument contains NUL'
    return decoded


def read_records(directory, session, role):
    files = sorted(directory.iterdir())
    assert files, 'missing provenance'
    records = []
    seen = set()
    for file in files:
        assert file.is_file() and not file.is_symlink() and file.suffix == '.json', 'incomplete/unexpected record'
        record = json.loads(file.read_bytes(), object_pairs_hook=unique_object)
        assert set(record) == {'schema','session_hex','role_hex','cwd_hex','pid','exit','signal','argv_hex'}, 'record schema'
        assert record['schema'] == 'archogen-compiler-provenance-prototype/1', 'schema identity'
        assert decode_hex(record['session_hex']) == session, 'session identity'
        assert decode_hex(record['role_hex']) == role, 'role identity'
        assert type(record['pid']) is int and record['pid'] > 0 and file.name == str(record['pid'])+'.json', 'invocation identity'
        assert record['pid'] not in seen, 'duplicate identity'
        seen.add(record['pid'])
        assert type(record['exit']) is int and record['exit'] == 0 and record['signal'] is None, 'compiler status'
        assert isinstance(record['argv_hex'], list), 'argument vector'
        argv = [decode_hex(value) for value in record['argv_hex']]
        assert argv and argv[0] == b'rustc', 'compiler selection'
        assert decode_hex(record['cwd_hex']) == os.fsencode(workspace), 'compiler cwd'
        if b'--out-dir' in argv:
            assert b'--crate-name' in argv and any(value in argv for value in [b'--emit=dep-info,metadata,link',b'--emit=dep-info,link']), 'unknown compilation shape'
            record['crate'] = os.fsdecode(argv[argv.index(b'--crate-name')+1])
            record['kind'] = 'compilation'
        else:
            target_query = [b'-',b'--crate-name',b'___',b'--print=file-names',
                            b'--crate-type',b'bin',b'--crate-type',b'rlib',
                            b'--crate-type',b'dylib',b'--crate-type',b'cdylib',
                            b'--crate-type',b'staticlib',b'--crate-type',b'proc-macro',
                            b'--print=sysroot',b'--print=split-debuginfo',
                            b'--print=crate-name',b'--print=cfg',b'-Wwarnings']
            assert argv[1:] in [[b'-vV'],target_query], 'unknown query shape'
            record['kind'] = 'query'
        records.append(record)
    return records

transport = out/'transport'
transport.mkdir()
args = [b'', b'a b', b"a'b", b'"quoted"', b'line\nreturn\r', b'back\\slash', 'é'.encode(), b'nonutf8-\xff']
session = b'transport-session'
role = b'transport-role'
env = dict(os.environb)
env.update({b'ARGV_SINK':os.fsencode(out/'receiver.argv'), b'RUSTC_WRAPPER':os.fsencode(out/'observer'), b'ARCHOGEN_PROTO_TRACE_DIR':os.fsencode(transport), b'ARCHOGEN_PROTO_SESSION':session, b'ARCHOGEN_PROTO_ROLE':role})
run = subprocess.run([os.fsencode(out/'observer'), os.fsencode(out/'receiver'), *args], env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
assert run.returncode == 7 and run.stdout == b'native receiver stdout\n' and run.stderr == b'native receiver stderr\n'
native = (out/'receiver.argv').read_bytes().split(b'\0'); assert native.pop() == b''
record = json.loads(next(transport.glob('*.json')).read_bytes())
assert native == [os.fsencode(out/'receiver'), *args] == [bytes.fromhex(value) for value in record['argv_hex']]
assert record['exit'] == 7 and record['signal'] is None

workspace = out/'workspace'
workspace.mkdir()
(workspace/'Cargo.toml').write_text('[workspace]\nresolver="2"\nmembers=["common","app","api","generator"]\n')
for name in ['common','app','api','generator']:
    (workspace/name/'src').mkdir(parents=True)
    text = '[package]\nname="'+name+'"\nversion="0.1.0"\nedition="2021"\n'
    if name != 'common': text += '[dependencies]\ncommon={path="../common"}\n'
    (workspace/name/'Cargo.toml').write_text(text)
    if name in ['app','generator']:
        (workspace/name/'src/main.rs').write_text('fn main() { println!("{}", common::value()); }\n')
    else:
        (workspace/name/'src/lib.rs').write_text('pub fn value() -> u32 { '+('common::value()' if name == 'api' else '7')+' }\n')
(workspace/'api/tests').mkdir()
(workspace/'api/tests/adapter.rs').write_text('#[test] fn agrees() { assert_eq!(api::value(), 7); }\n')
lock = subprocess.run(['cargo','generate-lockfile','--offline'], cwd=workspace, env=environment(out/'lock-target'), capture_output=True)
(out/'lock.log').write_bytes(lock.stdout+lock.stderr); assert lock.returncode == 0

tick = chr(96)
payloads = {
    'ordinary':'',
    'noncompiler':'.warning "note\nRunning '+tick+'not-a-compiler'+tick+'"',
    'rustc':'.warning "note\nRunning '+tick+'rustc'+tick+'"',
    'command':'.warning "note\nRunning '+tick+'rustc --crate-name phantom phantom.rs --out-dir bogus'+tick+'"',
    'stdout-escaped-json':'.print "{\\"reason\\":\\"compiler-artifact\\",\\"prototype_payload\\":true}"',
}
roles = {
    'root-bin':(['build','--bin','app','-p','app'], {'common','app'}),
    'root-lib':(['build','--lib','-p','api'], {'common','api'}),
    'harness':(['test','--no-run','--test','adapter','-p','api'], {'common','api','adapter'}),
    'generator':(['build','--bin','generator','-p','generator'], {'common','generator'}),
}
results = []
for mode,payload in payloads.items():
    (workspace/'common/src/lib.rs').write_text(('core::arch::global_asm!(r#"'+payload.replace('{','{{').replace('}','}}')+'"#);\n' if payload else '')+'pub fn value() -> u32 { 7 }\n')
    for role,(selection,expected) in roles.items():
        case = mode+'-'+role
        target = out/('target-'+case)
        trace = target/'.archogen-provenance/traces'
        trace.mkdir(parents=True)
        observer = target/'.archogen-provenance/observer'
        os.link(out/'observer',observer)
        assert observer.read_bytes() == (out/'observer').read_bytes()
        session = ('session-'+case).encode()
        env = environment(target)
        env.update({b'RUSTC_WRAPPER':os.fsencode(observer),b'ARCHOGEN_PROTO_TRACE_DIR':os.fsencode(trace),b'ARCHOGEN_PROTO_SESSION':session,b'ARCHOGEN_PROTO_ROLE':role.encode()})
        command = ['cargo',*selection,'--release','--locked','--offline','--no-default-features','--message-format=json','-v']
        run = subprocess.run(command,cwd=workspace,env=env,capture_output=True)
        (out/(case+'.stdout')).write_bytes(run.stdout); (out/(case+'.stderr')).write_bytes(run.stderr)
        assert run.returncode == 0, (case,run.stderr.decode(errors='replace'))
        records = read_records(trace,session,role.encode())
        units = [r['crate'] for r in records if r['kind'] == 'compilation']
        assert len(units) == len(expected) and set(units) == expected, (case,units)
        artifacts=[]; payload_objects=[]
        for line in run.stdout.splitlines():
            try: value=json.loads(line)
            except ValueError: continue
            if value.get('reason') == 'compiler-artifact':
                if value.get('prototype_payload'): payload_objects.append(value)
                else: artifacts.append(value)
        assert len(artifacts) == len(expected)
        if mode == 'stdout-escaped-json':
            assert b'prototype_payload' in run.stdout and not payload_objects, case
        results.append(dict(case=case,exit=run.returncode,units=units,queries=sum(r['kind']=='query' for r in records),records=records,artifacts=artifacts,payload_objects=payload_objects))
        print(case, 'units='+','.join(units), 'queries='+str(results[-1]['queries']), 'payload_json='+str(len(payload_objects)),flush=True)

report=[]
for case in results:
    target=out/('target-'+case['case'])
    before={str(p.relative_to(target)):hashlib.sha256(p.read_bytes()).hexdigest() for p in target.rglob('*') if p.is_file()}
    checks=[]
    for record in case['records']:
        if record['kind']!='compilation': continue
        argv=[bytes.fromhex(value) for value in record['argv_hex']]
        query=[b'rustc',b'--print=file-names',b'-']
        for flag in [b'--crate-name',b'--crate-type',b'--target']:
            indexes=[i for i,value in enumerate(argv) if value==flag]
            assert len(indexes)<=1
            for i in indexes:query += [flag,argv[i+1]]
        if b'--test' in argv:query.append(b'--test')
        extras=[argv[i+1] for i,value in enumerate(argv[:-1]) if value==b'-C' and argv[i+1].startswith(b'extra-filename=')]
        assert len(extras)==1
        query += [b'-C',extras[0]]
        env=environment(target)
        run=subprocess.run(query,input=b'',cwd=out/'workspace',env=env,capture_output=True)
        assert run.returncode==0 and run.stderr==b'',run.stderr
        names=run.stdout.decode().splitlines();assert names and all(name and '/' not in name and '\\' not in name for name in names)
        directory=Path(os.fsdecode(argv[argv.index(b'--out-dir')+1]))
        observed=[]
        for name in names:
            path=directory/name;assert path.is_file()
            digest=hashlib.sha256(path.read_bytes()).hexdigest()
            reports=[a for a in case['artifacts'] if a['target']['name']==record['crate']]
            assert len(reports)==1,(record['crate'],reports)
            filenames=reports[0]['filenames']
            assert any(Path(file).is_file() and hashlib.sha256(Path(file).read_bytes()).hexdigest()==digest for file in filenames)
            observed.append(dict(name=name,sha256=digest,report_agrees=True))
        checks.append(dict(crate=record['crate'],query_hex=[a.hex() for a in query],exit=0,artifacts=observed))
    after={str(p.relative_to(target)):hashlib.sha256(p.read_bytes()).hexdigest() for p in target.rglob('*') if p.is_file()}
    assert before==after,'naming query changed build files'
    report.append(dict(case=case['case'],checks=checks,build_files_unchanged=True))
(out/'artifact-query-results.json').write_text(json.dumps(report,indent=2)+'\n')
print('PASS: artifact naming queries agree with actual Cargo artifacts; every build file unchanged')

# Each refusal edits a separate copy. Primary producer evidence remains byte exact.
controls = []
primary = results[0]
source = out/('target-'+primary['case'])/'.archogen-provenance/traces'
expected_session = ('session-'+primary['case']).encode()
expected_role = b'root-bin'
for fault in ['missing','partial','wrong-session','wrong-role','wrong-schema',
              'unknown-field','duplicate-field','bad-hex','wrong-pid','failed-compiler',
              'wrong-compiler','wrong-cwd','unknown-query','symlink']:
    directory = out/'refusals'/fault
    shutil.copytree(source, directory)
    file = next(directory.glob('*.json'))
    record = json.loads(file.read_bytes())
    if fault == 'missing':
        for item in directory.iterdir(): item.unlink()
    elif fault == 'partial':
        (directory/'.unfinished.part').write_bytes(b'partial')
    elif fault == 'duplicate-field':
        file.write_text(file.read_text().replace('{','{"schema":"duplicate",',1))
    elif fault == 'symlink':
        file.unlink()
        file.symlink_to(next(source.glob('*.json')))
    else:
        if fault == 'wrong-session': record['session_hex'] = b'old-session'.hex()
        elif fault == 'wrong-role': record['role_hex'] = b'harness'.hex()
        elif fault == 'wrong-schema': record['schema'] = 'unknown/9'
        elif fault == 'unknown-field': record['extra'] = True
        elif fault == 'bad-hex': record['argv_hex'][0] = 'not hex'
        elif fault == 'wrong-pid': record['pid'] += 1
        elif fault == 'failed-compiler': record['exit'] = 1
        elif fault == 'wrong-compiler': record['argv_hex'][0] = b'other-compiler'.hex()
        elif fault == 'wrong-cwd': record['cwd_hex'] = b'/different-tree'.hex()
        elif fault == 'unknown-query': record['argv_hex'] = [b'rustc'.hex(),b'--mystery-query'.hex()]
        file.write_text(json.dumps(record))
    try:
        read_records(directory, expected_session, expected_role)
    except (AssertionError, ValueError) as error:
        controls.append(dict(fault=fault, rejected=True, reason=str(error)))
    else:
        raise AssertionError('accepted false provenance: '+fault)
assert len(controls) == 14

# Distinct concurrent native processes publish without sharing or replacing files.
parallel = out/'parallel'
parallel.mkdir()
jobs = []
for index in range(8):
    env = dict(os.environb)
    env.update({b'ARGV_SINK':os.fsencode(parallel/(str(index)+'.argv')),
                b'ARCHOGEN_PROTO_TRACE_DIR':os.fsencode(parallel),
                b'ARCHOGEN_PROTO_SESSION':b'parallel',b'ARCHOGEN_PROTO_ROLE':b'parallel'})
    process = subprocess.Popen([out/'observer',out/'receiver',str(index)],env=env,
                               stdout=subprocess.PIPE,stderr=subprocess.PIPE)
    jobs.append((index,process))
for index,process in jobs:
    stdout,stderr = process.communicate(timeout=30)
    assert process.returncode == 7 and stdout == b'native receiver stdout\n' and stderr == b'native receiver stderr\n'
    record = json.loads((parallel/(str(process.pid)+'.json')).read_text())
    expected = [os.fsencode(out/'receiver'),str(index).encode()]
    native = (parallel/(str(index)+'.argv')).read_bytes().split(b'\0')
    assert native == [*expected,b'']
    assert [bytes.fromhex(value) for value in record['argv_hex']] == expected
    assert record['exit'] == 7
    assert record['session_hex'] == b'parallel'.hex() and record['role_hex'] == b'parallel'.hex()
assert len(list(parallel.glob('*.json'))) == 8 and not list(parallel.glob('*.part'))

# A producer setup failure must stop before the requested program can execute.
env = dict(os.environb)
env.update({b'ARGV_SINK':os.fsencode(out/'must-not-exist.argv'),
            b'ARCHOGEN_PROTO_TRACE_DIR':os.fsencode(out/'absent-directory'),
            b'ARCHOGEN_PROTO_SESSION':b'failure',b'ARCHOGEN_PROTO_ROLE':b'failure'})
failed = subprocess.run([out/'observer',out/'receiver'],env=env,capture_output=True)
assert failed.returncode == 2 and not (out/'must-not-exist.argv').exists()
assert failed.stdout == b'' and b'compiler provenance prototype:' in failed.stderr

signal_directory = out/'signal'
signal_directory.mkdir()
env[b'ARCHOGEN_PROTO_TRACE_DIR'] = os.fsencode(signal_directory)
env[b'ARGV_SINK'] = os.fsencode(out/'signal.argv')
env[b'ARGV_SIGNAL'] = b'1'
signaled = subprocess.run([out/'observer',out/'receiver'],env=env,capture_output=True)
assert signaled.returncode == 143
signal_record = json.loads(next(signal_directory.glob('*.json')).read_text())
assert signal_record['exit'] is None and signal_record['signal'] == 15

spawn_directory = out/'spawn-failure'
spawn_directory.mkdir()
env[b'ARCHOGEN_PROTO_TRACE_DIR'] = os.fsencode(spawn_directory)
spawn = subprocess.run([out/'observer',out/'absent-compiler'],env=env,capture_output=True)
assert spawn.returncode == 2 and not list(spawn_directory.glob('*.json'))
assert len(list(spawn_directory.glob('*.part'))) == 1
try:
    read_records(spawn_directory,b'failure',b'failure')
except AssertionError:
    pass
else:
    raise AssertionError('accepted incomplete compiler spawn')

sysroot = Path(subprocess.check_output(['rustc','--print','sysroot']).decode().strip())
docs = sysroot/'share/doc/rust/html/rustc/command-line-arguments/print-options.html'
assert 'The names of the files created by the' in docs.read_text()
cargo_docs = sysroot/'share/doc/rust/html/cargo/reference/environment-variables.html'
assert 'RUSTC_WRAPPER' in cargo_docs.read_text() and 'first argument' in cargo_docs.read_text()
identities = {}
for file in [Path(__file__),root/'scripts/fixtures/compiler_provenance_observer.rs',
             root/'scripts/fixtures/compiler_provenance_receiver.c',root/'xtask/src/catalog_build.rs',
             root/'xtask/src/trust.rs']:
    identities[str(file.relative_to(root))] = hashlib.sha256(file.read_bytes()).hexdigest()
result = dict(
    prototype_only=True, host=os.uname().sysname, toolchain=pin, sources=identities,
    environment_constructor_sha256=hashlib.sha256(fragment.encode()).hexdigest(),
    pinned_public_print_docs_sha256=hashlib.sha256(docs.read_bytes()).hexdigest(),
    pinned_public_wrapper_docs_sha256=hashlib.sha256(cargo_docs.read_bytes()).hexdigest(),
    rustc_identity=subprocess.check_output(['rustc','-vV']).decode(),
    cargo_identity=subprocess.check_output(['cargo','-V']).decode(),
    transport=dict(exit=7,arguments_hex=[arg.hex() for arg in args],native_argv_matches=True,
                   private_environment_removed=True,stdout_stderr_exact=True),
    cases=results,artifact_queries=report,refusals=controls,concurrent_writers=8,
    producer_setup_failure=dict(exit=2,requested_program_not_executed=True),
    signal_forwarding=dict(exit=143,signal=15),
    compiler_spawn_failure=dict(exit=2,partial_record_refused=True),
)
(out/'prototype-results.json').write_text(json.dumps(result,indent=2)+'\n')
print('PASS: native argv/exit/stdio/environment; Cargo roles/payloads; naming queries; refusals; concurrent writers')
