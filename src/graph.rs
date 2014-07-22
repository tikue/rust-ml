use cluster::Cluster;
use point::Point;
use show::Show;

use std::iter::range_step;

static NUM_ROWS: uint = 50;
pub struct Graph {
    rows: [Row, ..NUM_ROWS],
}

static NUM_COLUMNS: uint = 50;
struct Row {
    symbols: [Symbol, ..NUM_COLUMNS],
    dash_step: uint,
    dash_row: bool,
}

struct PointSymbol {
    point: Point,
    symbol: char,
}

enum Symbol {
    Present(PointSymbol),
    Absent,
}

impl Symbol {
    fn is_present(&self) -> bool {
        match *self {
            Present(_) => true,
            Absent => false,
        }
    }

    fn get_point(&self) -> Point {
        match *self {
            Present(ps) => ps.point,
            Absent => fail!("Can't get point of Absent symbol"),
        }
    }
}

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
    fn empty(dash_step: uint) -> Row {
        Row {
            symbols: [Absent, ..NUM_COLUMNS],
            dash_step: dash_step,
            dash_row: false,
        }
    }

    fn points(&self) -> Vec<Point> {
        self.symbols.iter()
            .filter(|symbol| symbol.is_present())
            .map(|symbol| symbol.get_point())
            .collect()
    }

    fn dash_column(&self, column: uint) -> bool {
        self.dash_row || column % self.dash_step == 0
    }

    fn get_absent_symbol(&self, column: uint) -> char {
        if self.dash_column(column) { '.' } else { ' ' }
    }

    fn get_symbol(&self, symbol: &Symbol, column: uint) -> char {
        match *symbol {
            Absent => self.get_absent_symbol(column),
            Present(s) => s.symbol,
        }
    }

    fn set_symbol(&mut self, symbol: PointSymbol) {
        self.symbols[symbol.y().to_uint().unwrap()] = Present(symbol);
    }
}

impl Show for Row {
    fn show(&self) {
        for (column, symbol) in self.symbols.iter().enumerate() {
            print!("{}  ", self.get_symbol(symbol, column));
        }
        println!("");
    }
}

static DASH_STEP: uint = 10;
static SYMBOLS: &'static str = "o+#*";

impl Graph {
    /// Returns a new graph from the given clusters
    pub fn from_clusters(clusters: &[Cluster]) -> Graph {
        let mut graph = Graph::empty();
        graph.set_clusters(clusters);
        graph
    }

    pub fn from_points(points: &[Point]) -> Graph {
        let mut graph = Graph::empty();
        for point in points.iter().map(|&point|  PointSymbol::new(point, 'o')) {
            graph.set_symbol(point);
        }
        graph
    }

    /// Returns an empty graph
    pub fn empty() -> Graph {
        let mut graph = Graph {
            rows: [Row::empty(DASH_STEP), ..NUM_COLUMNS],
        };
        {
            let rows = &mut graph.rows;
            for i in range_step(0, NUM_COLUMNS, DASH_STEP) {
                rows[i].dash_row = true;
            }
        }
        graph
    }

    /// Returns a graph with a random number of points
    pub fn random(num_points: uint) -> Graph {
        let mut graph = Graph::empty();
        for _ in range(0, num_points) {
            graph.set_symbol(PointSymbol::random('o'));
        }
        graph
    }

    /// Returns a vector of all points on the graph
    pub fn points(&self) -> Vec<Point> {
        self.rows.iter()
            .flat_map(|row| row.points().move_iter())
            .collect()
    }

    fn set_symbol(&mut self, symbol: PointSymbol) {
        self.rows[symbol.x().to_uint().unwrap()].set_symbol(symbol);
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

impl Show for Graph {
    fn show(&self) {
        for row in self.rows.iter() {
            row.show();
        }
    }
}

