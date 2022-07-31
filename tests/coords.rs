use Anjin::Coordinates2D;
use Anjin::coords::Coords2D;
#[test]
fn test_coords() {
    let a = Coordinates2D::new(1., 3.);
    let b = Coordinates2D::new(5., 6.);
    assert_eq!(a.distance_from(&b), 4.)
}