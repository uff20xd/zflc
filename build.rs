mod redoxr;
use redoxr::redoxr::*;

fn main() -> MainResult {
    let _redoxr = Redoxr::new(&[
        "--cfg", "run",
        "--cfg", "verbose",
    ]);

    let mut zirlib = RustCrate::new("z_ir", "z_ir")
        .flags(&[
            "-Copt-level=0",
        ])
        .set_src(".")
        .make_output()
        .stay();

    let mut main_crate = RustCrate::new("zflc", ".")
        .flags(&[
            "-Copt-level=0",
        ])
        .make_output()
        .make_bin()
        .depend_on(&mut zirlib)
        .stay();

    compile!(zirlib);
    compile!(main_crate);
    run!(main_crate, "test/test2.zf");
    Ok(())
}

