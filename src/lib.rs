
/// Algorithm: [Winding Number](http://geomalgorithms.com/a03-_inclusion.html)
pub fn in_ring(
    pt: &Vec<f64>, 
    ring: &Vec<Vec<f64>>) -> bool {

  let mut wn = 0; // the winding number counter

  for edge in ring.windows(2) {
    let pt = pt.to_vec();
    let edge = edge.to_vec();
    let (y1, y2) = (edge[0][1], edge[1][1]);

    // check upward crossing
    if y1 <= pt[1] && y2 > pt[1] {
      // check if pt is left of a upward directed line
      if is_left(&pt, &edge) { wn += 1 };
    }
    // check downward crossing
    else if y1 > pt[1] && y2 <= pt[1] {
      // check if pt is right of a downward directed line
      if is_right(&pt, &edge) { wn -= 1 };
    }
  }

  wn != 0 // if 0, pt is outside the polygon
}

/// Returns true if the point is left of a directed line
fn is_left(
    pt: &Vec<f64>,
    line: &Vec<Vec<f64>>) -> bool {

  relative_pos(&pt, &line) > 0.0
}

/// Returns true if the point is right of a directed line
fn is_right(
    pt: &Vec<f64>,
    line: &Vec<Vec<f64>>) -> bool {

  relative_pos(&pt, &line) < 0.0
}

/// Determines if a point is Left|On|Right of a directed line.
///
/// ### Return
/// > 0 for `pt` left of the `line`
/// = 0 for `pt` on the `line`
/// < 0 for `pt` right of the `line`
fn relative_pos(
    pt: &Vec<f64>,
    line: &Vec<Vec<f64>>) -> f64 {

  let (x1, y1, x2, y2) = (line[0][0], line[0][1], line[1][0], line[1][1]);
  
  (x2 - x1) * (pt[1] - y1) - (y2 - y1) * (pt[0] -  x1)
}

#[cfg(test)]
mod tests {
  use super::{in_ring, is_left, is_right, relative_pos};

  #[test]
  fn test_in_ring() {
    let pt = vec![1.0, 1.0];
    let pt2 = vec![-1.0, 2.0];
    let pt3 = vec![1.0, 3.0];
    let pt4 = vec![0.1, 0.1];
    let pt5 = vec![0.0, 0.0];
    let pt6 = vec![3.0, 0.0];
    let pt7 = vec![1.0, 0.0];

    let ring1 = vec![vec![0.0, 0.0], vec![2.0, 0.0], vec![2.0, 2.0], vec![0.0, 2.0], vec![0.0, 0.0]];
    let ring2 = vec![vec![0.0, 0.0], vec![0.0, 2.0], vec![2.0, 2.0], vec![2.0, 0.0], vec![0.0, 0.0]];
    let ring3 = vec![vec![0.0, 0.0], vec![3.0, 3.0], vec![2.0, 0.0], vec![0.0, 0.0]];
    let ring4 = vec![vec![-1.0, -1.0], vec![3.0, 3.0], vec![2.0, 0.0], vec![5.0, -1.0], vec![-1.0, -1.0]];
    
    assert_eq!(in_ring(&pt, &ring1), true);
    assert_eq!(in_ring(&pt, &ring2), true);
    assert_eq!(in_ring(&pt, &ring3), true);
    assert_eq!(in_ring(&pt, &ring4), true);

    assert_eq!(in_ring(&pt2, &ring1), false);
    assert_eq!(in_ring(&pt2, &ring2), false);
    assert_eq!(in_ring(&pt2, &ring3), false);
    assert_eq!(in_ring(&pt2, &ring4), false);

    assert_eq!(in_ring(&pt3, &ring1), false);
    assert_eq!(in_ring(&pt3, &ring2), false);
    assert_eq!(in_ring(&pt3, &ring3), false);
    assert_eq!(in_ring(&pt3, &ring4), false);
    
    assert_eq!(in_ring(&pt4, &ring1), true);
    assert_eq!(in_ring(&pt4, &ring2), true);
    assert_eq!(in_ring(&pt4, &ring3), true);
    assert_eq!(in_ring(&pt4, &ring4), true);

    assert_eq!(in_ring(&pt5, &ring1), true);
    assert_eq!(in_ring(&pt5, &ring2), true);
    assert_eq!(in_ring(&pt5, &ring3), true);
    assert_eq!(in_ring(&pt5, &ring4), true);
    
    assert_eq!(in_ring(&pt6, &ring4), false);
    assert_eq!(in_ring(&pt7, &ring4), true);
  }

  #[test]
  fn test_is_left() {
    assert_eq!(is_left(&vec![0.9999999, 1.0], &vec![vec![0.0, 0.0], vec![2.0, 2.0]]), true);
    assert_eq!(is_left(&vec![1.0, 1.0], &vec![vec![0.0, 2.0], vec![0.0, 0.0]]), true);
    assert_eq!(is_left(&vec![-1.0, 1.0], &vec![vec![0.0, 2.0], vec![0.0, 0.0]]), false);
    assert_eq!(is_left(&vec![1.0, 1.0], &vec![vec![0.0, 0.0], vec![0.0, 2.0]]), false);
  }

  #[test]
  fn test_is_right() {
    assert_eq!(is_right(&vec![0.9999999, 1.0], &vec![vec![0.0, 0.0], vec![2.0, 2.0]]), false);
    assert_eq!(is_right(&vec![1.0, 1.0], &vec![vec![0.0, 2.0], vec![0.0, 0.0]]), false);
    assert_eq!(is_right(&vec![-1.0, 1.0], &vec![vec![0.0, 2.0], vec![0.0, 0.0]]), true);
    assert_eq!(is_right(&vec![1.0, 1.0], &vec![vec![0.0, 0.0], vec![0.0, 2.0]]), true);
  }

  #[test]
  fn test_relative_pos() {
    let line = vec![vec![0.0, 0.0], vec![2.0, 2.0]];
    let pt1 = vec![0.5, 1.0];
    let pt2 = vec![2.0, 1.0];
    let pt3 = vec![1.0, 1.0];

    assert_eq!(relative_pos(&pt1, &line) > 0.0, true);
    assert_eq!(relative_pos(&pt2, &line) < 0.0, true);
    assert_eq!(relative_pos(&pt3, &line) == 0.0, true);
  }
}
