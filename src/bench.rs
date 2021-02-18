use crate::{Board, PieceKind, Position};
use rayon::ThreadPoolBuilder;

use std::time::Instant;

pub trait Generator {
    type Output;
    fn generate(&self) -> Self::Output;
}

pub struct BoardGenerator {
    board_size: u32,
    pawn_count: u32,
    bishop_count: u32,
    rook_count: u32,
}

impl BoardGenerator {
    pub fn new(board_size: u32) -> Self {
        BoardGenerator {
            board_size,
            pawn_count: (board_size * board_size) / 8,
            bishop_count: (board_size * board_size) / 8,
            rook_count: (board_size * board_size) / 8,
        }
    }

    pub fn pawn_count(mut self, pawn_count: u32) -> Self {
        self.pawn_count = pawn_count;
        self
    }

    pub fn bishop_count(mut self, bishop_count: u32) -> Self {
        self.bishop_count = bishop_count;
        self
    }

    pub fn rook_count(mut self, rook_count: u32) -> Self {
        self.rook_count = rook_count;
        self
    }
}

impl Generator for BoardGenerator {
    type Output = Board;

    fn generate(&self) -> Board {
        let mut board = Board::new(self.board_size);
        let mut random = rand::thread_rng();

        let positions = Position::generate_unique_positions(
            &mut random,
            self.bishop_count + self.pawn_count + self.rook_count,
            self.board_size,
        );

        let pieces = &[
            (PieceKind::Bishop, &positions[0..self.bishop_count as usize]),
            (
                PieceKind::Pawn,
                &positions
                    [self.bishop_count as usize..(self.bishop_count + self.pawn_count) as usize],
            ),
            (
                PieceKind::Rook,
                &positions[(self.bishop_count + self.pawn_count) as usize..],
            ),
        ];

        board.set_pieces(
            pieces
                .into_iter()
                .flat_map(|(k, ps)| ps.iter().map(|p| (*k, *p)).collect::<Vec<_>>())
                .collect(),
        );
        board
    }
}

pub struct Benchmark<U: Sync + Send> {
    sizes: Option<Vec<usize>>,
    threads: Option<Vec<usize>>,
    runs: usize,
    functions: Vec<(String, Box<dyn FnMut(&U) -> () + Sync + Send>)>,
}

impl<U: Sync + Send> Benchmark<U> {
    pub fn new() -> Self {
        Benchmark {
            sizes: None,
            threads: None,
            runs: 20,
            functions: Vec::new(),
        }
    }

    pub fn sizes(mut self, sizes: Vec<usize>) -> Self {
        self.sizes = Some(sizes);
        self
    }

    pub fn threads(mut self, threads: Vec<usize>) -> Self {
        self.threads = Some(threads);
        self
    }

    pub fn runs(mut self, runs: usize) -> Self {
        self.runs = runs;
        self
    }

    pub fn add_function(
        mut self,
        function: Box<dyn FnMut(&U) -> () + Sync + Send>,
        name: String,
    ) -> Self {
        self.functions.push((name, function));
        self
    }

    pub fn bench<G>(self, mut gen: G) -> BenchmarkResult
    where
        G: FnMut(usize) -> U,
    {
        let threads = self.threads.unwrap();
        let sizes = self.sizes.unwrap();
        let mut functions = self.functions;
        let runs = self.runs;

        let names = functions.iter().map(|(n, _)| n.clone()).collect();

        let data = threads
            .iter()
            .map(|&t| {
                let threads = ThreadPoolBuilder::new().num_threads(t).build().unwrap();
                println!("With {} threads...", t);

                sizes
                    .iter()
                    .map(|s| {
                        let u = gen(*s);

                        println!("  With size {}...", *s);

                        (0..runs)
                            .map(|r| {
                                functions
                                    .iter_mut()
                                    .map(|(n, f)| {
                                        println!("    Run {}: {}", r, n);
                                        threads.install(|| {
                                            let start = Instant::now();
                                            f(&u);
                                            start.elapsed().as_nanos() as u64
                                        })
                                    })
                                    .collect::<Vec<_>>()
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        BenchmarkResult::new(data, names, threads.clone(), sizes.clone())
    }
}

pub struct BenchmarkResult {
    data: Vec<Vec<Vec<Vec<u64>>>>,
    functions: Vec<String>,
    threads: Vec<usize>,
    sizes: Vec<usize>,
}

impl BenchmarkResult {
    pub fn new(
        data: Vec<Vec<Vec<Vec<u64>>>>,
        functions: Vec<String>,
        threads: Vec<usize>,
        sizes: Vec<usize>,
    ) -> Self {
        BenchmarkResult {
            data,
            functions,
            threads,
            sizes,
        }
    }
}
