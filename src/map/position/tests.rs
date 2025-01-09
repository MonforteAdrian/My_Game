use super::*;

#[test]
fn position_addition() {
    assert_eq!(Position::ZERO + Position::ZERO, Position::ZERO);
    assert_eq!(Position::ZERO + Position::new(1, 1, 0), Position::new(1, 1, 0));
    assert_eq!(Position::new(1, 1, 0) + Position::new(1, 1, 0), Position::new(2, 2, 0));
    assert_eq!(Position::new(1, 1, 0) + Position::new(3, 4, 0), Position::new(4, 5, 0));
    assert_eq!(Position::new(1, 2, 3) + Position::new(4, 5, 6), Position::new(5, 7, 9));
    assert_eq!(Position::new(1, -2, 3) + Position::new(4, 5, 6), Position::new(5, 3, 9));
}

#[test]
fn position_sum() {
    // zero sum
    assert_eq!(Position::ZERO.line_to(Position::ZERO).sum::<Position>(), Position::ZERO);
    // correct sum
    assert_eq!(Position::ZERO.line_to(Position::X).sum::<Position>(), Position::X);
    assert_eq!(Position::ZERO.line_to(Position::new(5, 0, 0)).sum::<Position>(), Position::new((1..=5).sum(), 0, 0));
}

#[test]
fn position_product() {
    assert_eq!(
        Position::X.line_to(Position::new(5, 0, 0)).product::<Position>(),
        Position::new((1..=5).product(), 0, 0)
    );
}

#[test]
fn position_subtraction() {
    assert_eq!(Position::ZERO - Position::ZERO, Position::ZERO);
    assert_eq!(Position::new(1, 1, 0) - Position::ZERO, Position::new(1, 1, 0));
    assert_eq!(Position::new(1, 1, 1) - Position::new(1, 1, 0), Position::new(0, 0, 1));
    assert_eq!(Position::new(1, 1, 0) - Position::new(2, 2, 0), Position::new(-1, -1, 0));
    assert_eq!(Position::new(1, 1, 0) - Position::new(4, 5, 0), Position::new(-3, -4, 0));
}

#[test]
fn position_multiplication() {
    assert_eq!(Position::new(1, 1, 0) * Position::ZERO, Position::ZERO);
    assert_eq!(Position::new(1, 1, 0) * Position::new(1, 1, 0), Position::new(1, 1, 0));
    assert_eq!(Position::new(1, 1, 0) * Position::new(2, 2, 0), Position::new(2, 2, 0));
    assert_eq!(Position::new(1, 1, 0) * Position::new(5, 6, 0), Position::new(5, 6, 0));
    assert_eq!(Position::new(2, 3, 0) * Position::new(5, 10, 0), Position::new(10, 30, 0));
}

#[test]
fn position_division() {
    assert_eq!(Position::new(1, 1, 0) / Position::new(1, 1, 0), Position::new(1, 1, 0));
    assert_eq!(Position::new(2, 2, 0) / Position::new(2, 2, 0), Position::new(1, 1, 0));
    assert_eq!(Position::new(10, 30, 0) / Position::new(2, 6, 0), Position::new(5, 5, 0));
    assert_eq!(Position::new(11, 31, 10) / Position::new(2, 6, 5), Position::new(5, 5, 2));
}

#[test]
fn position_rem() {
    for x in 1..30 {
        for y in 1..30 {
            let p = Position::new(x, y, 0);
            for x2 in 1..30 {
                for y2 in 1..30 {
                    // Int
                    let rhs = Position::splat(x2);
                    let div = p / rhs;
                    let rem = p % rhs;
                    assert_eq!(div * rhs + rem, p);
                    // Position
                    let rhs = Position::new(x2, y2, 0);
                    let div = p / rhs;
                    let rem = p % rhs;
                    assert_eq!(div * rhs + rem, p);
                }
            }
        }
    }
}

#[test]
fn neighbors() {
    assert_eq!(
        Position::ZERO.all_neighbors(),
        [Position::new(1, 0, 0), Position::new(0, -1, 0), Position::new(-1, 0, 0), Position::new(0, 1, 0),]
    );
    assert_eq!(
        Position::new(-2, 5, 0).all_neighbors(),
        [Position::new(-1, 5, 0), Position::new(-2, 4, 0), Position::new(-3, 5, 0), Position::new(-2, 6, 0),]
    );
}

#[test]
fn diagonals() {
    assert_eq!(
        Position::ZERO.all_diagonals(),
        [Position::new(1, 1, 0), Position::new(1, -1, 0), Position::new(-1, -1, 0), Position::new(-1, 1, 0),]
    );
    assert_eq!(
        Position::new(-2, 5, 0).all_diagonals(),
        [Position::new(-1, 6, 0), Position::new(-1, 4, 0), Position::new(-3, 4, 0), Position::new(-3, 6, 0),]
    );
}

#[test]
fn line_to() {
    // Same start and end
    let start = Position::ZERO;
    assert_eq!(start.line_to(start).collect::<Vec<_>>(), vec![start]);

    // Known strait
    let end = Position::new(5, 0, 0);
    assert_eq!(
        start.line_to(end).collect::<Vec<_>>(),
        vec![
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(2, 0, 0),
            Position::new(3, 0, 0),
            Position::new(4, 0, 0),
            Position::new(5, 0, 0),
        ]
    );
}

#[test]
fn ring() {
    // Zero
    let center = Position::ZERO;
    assert_eq!(center.ring(0).collect::<Vec<_>>(), vec![center]);

    // Only neighbors
    let neighbors = center.all_neighbors();
    // Because the order might be different
    for neighbor in neighbors {
        assert!(center.ring(1).collect::<Vec<_>>().contains(&neighbor));
    }

    // Center is not Zero
    let target = Position::new(14, 7, 0);
    let expecteds = center.ring(10).map(|h| h + target).collect::<Vec<_>>();
    // Because the order might be different
    for expected in expecteds {
        assert!(target.ring(10).collect::<Vec<_>>().contains(&expected));
    }

    // Every ring between 0 and 1000 range is between 0.5 from the ideal
    for range in 0..1000 {
        let result = center.ring(range).collect::<HashSet<Position>>();
        for point in &result {
            // Calculate Euclidean distance
            let distance = (((point.x - center.x).pow(2) + (point.y - center.y).pow(2)) as f64).sqrt();
            assert!(
                (distance - range as f64).abs() < 0.5,
                "Point {:?} is not at the correct distance from center {:?}",
                point,
                center
            );
        }
    }
}
