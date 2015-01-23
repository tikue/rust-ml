use points::cluster::Cluster;
use points::point::Point;

use std::fmt;
use std::num::ToPrimitive;

const NUM_ROWS: usize = 50;
const NUM_COLUMNS: usize = 50;
const DASH_STEP: usize = 10;
const SYMBOLS: &'static str = "o+#*";
const DASH_ROW: Row = Row {
    symbols: [None; NUM_COLUMNS],
    row_number: NUM_ROWS,
};
 
pub struct Plot {
    rows: [Row; NUM_ROWS],
}
impl Copy for Plot{}

struct Row {
    symbols: [Option<PointSymbol>; NUM_COLUMNS],
    row_number: usize,
}
impl Copy for Row{}

struct PointSymbol {
    point: Point,
    symbol: char,
}
impl Copy for PointSymbol{}

impl PointSymbol {
    fn new(point: Point, symbol: char) -> PointSymbol {
        PointSymbol {
            point: point,
            symbol: symbol,
        }
    }

    fn random(symbol: char) -> PointSymbol {
        PointSymbol::new(Point::random(NUM_ROWS as f64, NUM_COLUMNS as f64), symbol)
    }

    fn x(&self) -> f64 {
        self.point.x
    }

    fn y(&self) -> f64 {
        self.point.y
    }
}

impl Row {
    fn empty(row_number: usize) -> Row {
        Row {
            symbols: [None; NUM_COLUMNS],
            row_number: row_number,
        }
    }

    fn points(&self) -> Vec<Point> {
        self.symbols.iter()
            .flat_map(|symbol| symbol.map(|p| p.point).into_iter())
            .collect()
    }

    fn dash_row(&self) -> bool {
        self.row_number % DASH_STEP == 0
    }

    fn dash_column(&self, column: usize) -> bool {
        self.dash_row() || column % DASH_STEP == 0
    }

    fn get_absent_symbol(&self, column: usize) -> char {
        if self.dash_column(column) { '.' } else { ' ' }
    }

    fn get_symbol(&self, symbol: &Option<PointSymbol>, column: usize) -> char {
        match *symbol {
            None => self.get_absent_symbol(column),
            Some(s) => s.symbol,
        }
    }

    #[allow(unstable)]
    fn set_symbol(&mut self, symbol: PointSymbol) {
        self.symbols[symbol.y().to_u32().unwrap() as usize] = Some(symbol);
    }
}

#[allow(unstable)]
impl fmt::String for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "{:>2} ", self.row_number));
        for (column, symbol) in self.symbols.iter().enumerate() {
            try!(write!(f, "{}  ", self.get_symbol(symbol, column)));
        }
        writeln!(f, ".")
    }
}

impl Plot {
    /// Returns a new graph from the given clusters
    pub fn from_clusters(clusters: &[Cluster]) -> Plot {
        let mut graph = Plot::empty();
        graph.set_clusters(clusters);
        graph
    }

    pub fn from_points(points: &[Point]) -> Plot {
        let mut graph = Plot::empty();
        for point in points.iter().map(|&point|  PointSymbol::new(point, 'o')) {
            graph.set_symbol(point);
        }
        graph
    }

    /// Returns an empty graph
    pub fn empty() -> Plot {
        let mut graph = Plot {
            rows: [Row::empty(0); NUM_COLUMNS],
        };
        for (row_number, row) in graph.rows.iter_mut().enumerate() {
            row.row_number = row_number;
        }
        graph
    }

    /// Returns a graph with a random number of points
    pub fn random(num_points: u32) -> Plot {
        let mut graph = Plot::empty();
        for _ in (0..num_points) {
            graph.set_symbol(PointSymbol::random('o'));
        }
        graph
    }

    /// Returns a vector of all points on the graph
    pub fn points(&self) -> Vec<Point> {
        self.rows.iter()
            .flat_map(|row| row.points().into_iter())
            .collect()
    }

    #[allow(unstable)]
    fn set_symbol(&mut self, symbol: PointSymbol) {
        self.rows[symbol.x().to_u32().unwrap() as usize].set_symbol(symbol);
    }

    fn set_cluster(&mut self, cluster: &Cluster, symbol: char) {
        for point in cluster.iter().map(|&point|  PointSymbol::new(point, symbol)) {
            self.set_symbol(point);
        }
    }

    fn set_clusters(&mut self, clusters: &[Cluster]) {
        for (cluster, symbol) in clusters.iter().zip(SYMBOLS.chars()) {
            self.set_cluster(cluster, symbol);
        }
    }
}

#[allow(unstable)]
impl fmt::String for Plot {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for row in self.rows.iter() {
            try!(row.fmt(f));
        }
        DASH_ROW.fmt(f)
    }
}

