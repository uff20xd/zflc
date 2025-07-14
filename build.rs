mod redoxr;
use redoxr::redoxr::*;

fn main() -> MainResult {
    let _redoxr = Redoxr::new(&[
        "--cfg", "run",
        "--cfg", "quiet",
    ]);

    let mut main_crate = RustCrate::new("zflc", ".")
        .flags(&[
            "-Copt-level=3",
        ])
        .make_output()
        .make_bin()
        .stay();
    compile!(main_crate);
    run!(main_crate);
    Ok(())
}

