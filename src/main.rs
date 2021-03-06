extern crate rayon_logs as rayon;

use chess::Generator;
use chess::{Benchmark, Board, BoardGenerator};

fn generate(generator: &BoardGenerator) -> Board {
    println!("Generating...");
    let result = generator.generate();
    println!("Done.");
    result
}

pub fn main() {
    /*
    let board_size = std::env::args().nth(1).map(|s| s.parse().unwrap()).unwrap();
    let generator = BoardGenerator::new(board_size);

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.compare()
        .runs_number(20)
        .attach_algorithm_with_setup("seq", || generate(&generator), |g| g.get_rooks_captures())
        .attach_algorithm_with_setup(
            "par",
            || generate(&generator),
            |g| g.get_rooks_captures_par(),
        )
        .generate_logs("log.html")
        .unwrap();
     */

    let results = Benchmark::new()
        .threads(vec![1, 2, 3, 4])
        .sizes(vec![256, 512])
        .runs(2)
        .add_function(
            Box::new(|b: &Board| {
                b.get_rook_captures_par();
            }),
            "single_rook".to_string(),
        )
        .add_function(
            Box::new(|b: &Board| {
                b.get_rooks_captures_par();
            }),
            "multiple_rooks".to_string(),
        )
        .bench(|s: usize| BoardGenerator::new(s as u32).generate());
}
