use std::path::{Path, PathBuf};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn load_fixture(relative: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read fixture {}: {err}", path.display()))
}

fn fixture_name(path: &str) -> &str {
    Path::new(path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(path)
}

fn bench_json_parse_and_build(c: &mut Criterion) {
    let fixtures = [
        "fixtures/json/project.pbxproj",
        "fixtures/json/006-spm.pbxproj",
        "fixtures/json/007-xcode16.pbxproj",
    ];

    for fixture in fixtures {
        let input = load_fixture(fixture);
        let parsed = xcode::json::parse(&input).expect("fixture must parse");
        let name = fixture_name(fixture).to_string();

        c.bench_function(&format!("json/parse/{name}"), |b| {
            b.iter(|| {
                let value = xcode::json::parse(black_box(&input)).expect("fixture must parse");
                black_box(value)
            });
        });

        c.bench_function(&format!("json/build/{name}"), |b| {
            b.iter(|| {
                let out = xcode::json::build(black_box(&parsed));
                black_box(out)
            });
        });
    }
}

criterion_group!(benches, bench_json_parse_and_build);
criterion_main!(benches);
