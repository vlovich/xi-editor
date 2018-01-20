extern crate xi_trace;

extern {
    fn example_main();
}

fn main() {
    xi_trace::enable_tracing();
    eprintln!("Running C entrypoint. enabled = {}", xi_trace::is_enabled());
    unsafe { example_main(); }
  
    let samples = xi_trace::samples_cloned_unsorted();
    eprintln!("C entrypoint finished. {} samples generated", samples.len());
    for sample in samples {
        eprintln!("{:?} sample {}", sample.sample_type, sample.name);
    }
}
