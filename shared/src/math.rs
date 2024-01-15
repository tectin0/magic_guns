use bevy::math::Vec2;

pub fn calc_center_points(vertices: &[[f32; 3]], indices: &[u32]) -> Vec<[f32; 3]> {
    let mut center_points = Vec::new();

    for chunk in indices.chunks(3) {
        let mut x = 0.0;
        let mut y = 0.0;

        for index in chunk {
            let vertex = vertices[*index as usize];

            x += vertex[0];
            y += vertex[1];
        }

        x /= 3.0;
        y /= 3.0;

        center_points.push([x, y, 0.0]);
    }

    center_points
}

pub trait IsPointInTriangle {
    fn is_point_in_triangle(&self, point: Vec2) -> bool;
}

impl IsPointInTriangle for [Vec2; 3] {
    fn is_point_in_triangle(&self, point: Vec2) -> bool {
        let v0 = self[2] - self[0];
        let v1 = self[1] - self[0];
        let v2 = point - self[0];

        let dot00 = v0.dot(v0);
        let dot01 = v0.dot(v1);
        let dot02 = v0.dot(v2);
        let dot11 = v1.dot(v1);
        let dot12 = v1.dot(v2);

        let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);

        let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
        let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

        (u >= 0.0) && (v >= 0.0) && (u + v < 1.0)
    }
}

pub fn triangulate(points: &[[f32; 3]]) -> Vec<u32> {
    let points = points
        .iter()
        .map(|point| delaunator::Point {
            x: point[0] as f64,
            y: point[1] as f64,
        })
        .collect::<Vec<delaunator::Point>>();

    let result = delaunator::triangulate(&points);

    result.triangles.iter().map(|index| *index as u32).collect()
}

#[cfg(test)]
mod tests {
    use crate::math::triangulate;

    #[test]
    fn test_triangulate() {
        let points = vec![
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ];

        let result = triangulate(&points);

        assert_eq!(result, vec![0, 2, 1, 0, 3, 2]);
    }
}
