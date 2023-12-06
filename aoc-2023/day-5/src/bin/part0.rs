
#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    #[cfg(not(feature = "dhat-heap"))]
    tracing_subscriber::fmt::init();

    println!("answer: {}", day_5::part0::Puzzle::solve(vec![
        vec![34, 90, 89, 86],
        vec![204, 1713, 1210, 1780],
    ]));
}
