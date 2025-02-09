use super::*;
use crate::{fov, map::chunks::generate_mesh_of_chunks};
use std::f32::consts::PI;
use test::Bencher;

#[test]
fn position_addition() {
    assert_eq!(Position::ZERO + Position::ZERO, Position::ZERO);
    assert_eq!(Position::ZERO + position(1, 1, 0), position(1, 1, 0));
    assert_eq!(position(1, 1, 0) + position(1, 1, 0), position(2, 2, 0));
    assert_eq!(position(1, 1, 0) + position(3, 4, 0), position(4, 5, 0));
    assert_eq!(position(1, 2, 3) + position(4, 5, 6), position(5, 7, 9));
    assert_eq!(position(1, -2, 3) + position(4, 5, 6), position(5, 3, 9));
}

#[test]
fn position_sum() {
    // zero sum
    assert_eq!(Position::ZERO.line_to(Position::ZERO).sum::<Position>(), Position::ZERO);
    // correct sum
    assert_eq!(
        Position::ZERO.line_to(position(1, 0, 0)).sum::<Position>(),
        position(1, 0, 0)
    );
    assert_eq!(
        Position::ZERO.line_to(position(5, 0, 0)).sum::<Position>(),
        position((1..=5).sum(), 0, 0)
    );
}

#[test]
fn position_product() {
    assert_eq!(
        position(1, 0, 0).line_to(position(5, 0, 0)).product::<Position>(),
        position((1..=5).product(), 0, 0)
    );
}

#[test]
fn position_subtraction() {
    assert_eq!(Position::ZERO - Position::ZERO, Position::ZERO);
    assert_eq!(position(1, 1, 0) - Position::ZERO, position(1, 1, 0));
    assert_eq!(position(1, 1, 1) - position(1, 1, 0), position(0, 0, 1));
    assert_eq!(position(1, 1, 0) - position(2, 2, 0), position(-1, -1, 0));
    assert_eq!(position(1, 1, 0) - position(4, 5, 0), position(-3, -4, 0));
}

#[test]
fn position_multiplication() {
    assert_eq!(position(1, 1, 0) * Position::ZERO, Position::ZERO);
    assert_eq!(position(1, 1, 0) * position(1, 1, 0), position(1, 1, 0));
    assert_eq!(position(1, 1, 0) * position(2, 2, 0), position(2, 2, 0));
    assert_eq!(position(1, 1, 0) * position(5, 6, 0), position(5, 6, 0));
    assert_eq!(position(2, 3, 0) * position(5, 10, 0), position(10, 30, 0));
}

#[test]
fn position_division() {
    assert_eq!(position(1, 1, 0) / position(1, 1, 0), position(1, 1, 0));
    assert_eq!(position(2, 2, 0) / position(2, 2, 0), position(1, 1, 0));
    assert_eq!(position(10, 30, 0) / position(2, 6, 0), position(5, 5, 0));
    assert_eq!(position(11, 31, 10) / position(2, 6, 5), position(5, 5, 2));
}

#[test]
fn position_rem() {
    for x in 1..30 {
        for y in 1..30 {
            let p = position(x, y, 0);
            for x2 in 1..30 {
                for y2 in 1..30 {
                    // Int
                    let rhs = position(x2, x2, x2);
                    let div = p / rhs;
                    let rem = p % rhs;
                    assert_eq!(div * rhs + rem, p);
                    // Position
                    let rhs = position(x2, y2, 0);
                    let div = p / rhs;
                    let rem = p % rhs;
                    assert_eq!(div * rhs + rem, p);
                }
            }
        }
    }
}

//#[test]
//fn neighbors() {
//    assert_eq!(
//        Position::ZERO.neighbors(),
//        [
//            position(1, 0, 0),
//            position(1, 1, 0),
//            position(0, 1, 0),
//            position(-1, 1, 0),
//            position(-1, 0, 0),
//            position(-1, -1, 0),
//            position(0, -1, 0),
//            position(1, -1, 0)
//        ]
//    );
//    assert_eq!(
//        position(-2, 5, 0).neighbors(),
//        [
//            position(-1, 5, 0),
//            position(-1, 6, 0),
//            position(-2, 6, 0),
//            position(-3, 6, 0),
//            position(-3, 5, 0),
//            position(-3, 4, 0),
//            position(-2, 4, 0),
//            position(-1, 4, 0)
//        ]
//    );
//}

#[test]
fn line_to() {
    // Same start and end
    let start = Position::ZERO;
    assert_eq!(start.line_to(start).collect::<Vec<_>>(), vec![start]);

    // Known strait
    let end = position(5, 0, 0);
    assert_eq!(
        start.line_to(end).collect::<Vec<_>>(),
        vec![
            position(0, 0, 0),
            position(1, 0, 0),
            position(2, 0, 0),
            position(3, 0, 0),
            position(4, 0, 0),
            position(5, 0, 0),
        ]
    );
}

#[test]
fn ring() {
    // Zero
    let center = Position::ZERO;
    assert_eq!(center.ring(0).collect::<Vec<_>>(), vec![center]);

    // Center is not Zero
    let target = position(14, 7, 0);
    let expecteds = center.ring(10).map(|h| h + target).collect::<Vec<_>>();
    // Because the order might be different
    for expected in expecteds {
        assert!(target.ring(10).collect::<Vec<_>>().contains(&expected));
    }

    // Every ring between 0 and 1000 range is between 0.5 from the ideal
    for range in 0..200 {
        let result = center.ring(range).collect::<HashSet<Position>>();
        for point in &result {
            // Calculate Euclidean distance
            let distance = (((point.x - center.x).pow(2) + (point.y - center.y).pow(2)) as f32).sqrt();
            assert!(
                (distance - range as f32).abs() < 0.5,
                "Point {:?} is not at the correct distance from center {:?}",
                point,
                center
            );
        }
    }
}

//#[test]
//fn chunk() {
//    let chunks = generate_mesh_of_chunks(10, -10, 10, -10);
//    for chunk in chunks {
//        dbg!(chunk.x, chunk.y);
//        for pos in chunk.layers[0].tiles.iter() {
//            dbg!(pos.chunk());
//            assert_eq!(pos.chunk(), (chunk.x, chunk.y));
//        }
//    }
//}

#[bench]
fn bench_ring(b: &mut Bencher) {
    let center = Position::ZERO;
    b.iter(|| center.ring(5));
}

#[bench]
fn bench_cone(b: &mut Bencher) {
    let center = Position::ZERO;
    b.iter(|| center.cone(5, 0.0, PI));
}

#[bench]
fn bench_line_to(b: &mut Bencher) {
    let center = Position::ZERO;
    b.iter(|| center.line_to(position(150, 200, 0)));
    b.iter(|| center.line_to(position(5, 2, 0)));
}

// TODO this should be an itegration test
//#[bench]
//fn bench_fov(b: &mut Bencher) {
//    let center = Position::ZERO;
//    let blocked_coords = vec![position(5, 5, 0)];
//
//    b.iter(|| fov(center, 100,, |h| blocked_coords.contains(&h)));
//}
