#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn is_inside_polygon(&self, p: &Polygon) -> bool {
        let mut inside = false;

        let mut i = 0;
        let mut j = p.points.len() - 1;
        while i < p.points.len() {
            let a = &p.points[i];
            let b = &p.points[j];
            if (a.y > self.y) != (b.y > self.y)
                && self.x < (b.x - a.x) * (self.y - a.y) / (b.y - a.y) + a.x
            {
                inside = !inside;
            }
            j = i;
            i += 1;
        }

        inside
    }
}

struct Line {
    start: Point,
    end: Point,
}

struct Polygon {
    points: Vec<Point>,
}

#[derive(Debug, Clone, Copy)]
struct Box {
    top_left: Point,
    bottom_right: Point,
}

impl Line {
    const fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    fn do_segments_intersect(&self, other: &Line) -> bool {
        // Check if line1 is vertical and line2 is horizontal
        if self.start.x == self.end.x && other.start.y == other.end.y {
            // Vertical and Horizontal
            return (self.start.y.min(self.end.y) <= other.start.y
                && other.start.y <= self.start.y.max(self.end.y))
                && (other.start.x.min(other.end.x) <= self.start.x
                    && self.start.x <= other.start.x.max(other.end.x));
        }

        // Check if line2 is vertical and line1 is horizontal
        if other.start.x == other.end.x && self.start.y == self.end.y {
            // Vertical and Horizontal
            return (other.start.y.min(other.end.y) <= self.start.y
                && self.start.y <= other.start.y.max(other.end.y))
                && (self.start.x.min(self.end.x) <= other.start.x
                    && other.start.x <= self.start.x.max(self.end.x));
        }

        false // No intersection
    }
}

impl Box {
    fn new(p1: &Point, p2: &Point) -> Self {
        let tx = p1.x.min(p2.x);
        let ty = p1.y.min(p2.y);
        let bx = p1.x.max(p2.x);
        let by = p1.y.max(p2.y);

        Box {
            top_left: Point { x: tx, y: ty },
            bottom_right: Point { x: bx, y: by },
        }
    }

    fn points(&self) -> [Point; 4] {
        let mut top_right = self.top_left;
        top_right.x = self.bottom_right.x;
        let mut bottom_left = self.top_left;
        bottom_left.y = self.bottom_right.y;
        [top_right, bottom_left, self.top_left, self.bottom_right]
    }

    fn lines(&self) -> [Line; 4] {
        let [tr, bl, tl, br] = self.points();

        [
            Line::new(tl, tr),
            Line::new(tl, bl),
            Line::new(bl, br),
            Line::new(tr, br),
        ]
    }

    fn adjust(&self) -> Self {
        let mut c = *self;
        c.top_left.x += 1;
        c.top_left.y += 1;
        c.bottom_right.x -= 1;
        c.bottom_right.y -= 1;
        c
    }

    fn inside_polygon(&self, p: &Polygon) -> bool {
        for po in self.points() {
            if !po.is_inside_polygon(p) {
                return false;
            }
        }

        let mut i = 0;
        let mut j = p.points.len() - 1;
        while i < p.points.len() {
            let a = &p.points[i];
            let b = &p.points[j];
            let line = Line::new(*a, *b);

            for sline in self.lines() {
                if line.do_segments_intersect(&sline) {
                    return false;
                }
            }
            j = i;
            i += 1;
        }

        true
    }
}

fn to_coord(s: &str) -> Point {
    let p: Vec<_> = s.split(',').collect();
    Point {
        x: p[0].parse().unwrap(),
        y: p[1].parse().unwrap(),
    }
}

fn area(c1: &Point, c2: &Point) -> i64 {
    let x;
    let y;

    if c2.x > c1.x {
        x = c2.x - c1.x;
    } else {
        x = c1.x - c2.x;
    }

    if c2.y > c1.y {
        y = c2.y - c1.y;
    } else {
        y = c1.y - c2.y;
    }

    (x + 1) * (y + 1)
}

fn areas(coords: &Vec<Point>) -> i64 {
    let mut largest_area = 0;

    for i in 0..coords.len() - 1 {
        for j in (i + 1)..coords.len() {
            let p1 = &coords[i];
            let p2 = &coords[j];
            let a = area(p1, p2);
            // println!("{:?}, {:?} -> {}", p1, p2, a);
            largest_area = largest_area.max(a);
        }
    }

    largest_area
}

fn areas2(coords: &Vec<Point>) -> i64 {
    let mut boxes = vec![];

    for i in 0..coords.len() - 1 {
        for j in (i + 1)..coords.len() {
            let p1 = &coords[i];
            let p2 = &coords[j];
            let b = Box::new(p1, p2);
            boxes.push(b);
        }
    }

    boxes.sort_by(|a, b| {
        area(&b.top_left, &b.bottom_right).cmp(&area(&a.top_left, &a.bottom_right))
    });

    let poly = Polygon {
        points: coords.to_vec(),
    };

    let f = boxes
        .iter()
        .find(|b| b.adjust().inside_polygon(&poly))
        .unwrap();
    area(&f.top_left, &f.bottom_right)

    // largest_area
}

pub fn part_1(vec: &Vec<String>) -> i64 {
    let coords: Vec<_> = vec.iter().map(|pos| to_coord(&pos)).collect();
    areas(&coords)
}

pub fn part_2(vec: &Vec<String>) -> i64 {
    let coords: Vec<_> = vec.iter().map(|pos| to_coord(&pos)).collect();
    areas2(&coords)
}
