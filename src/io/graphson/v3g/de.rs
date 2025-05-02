use crate::prelude::*;
use geo_types::{Coord, Geometry, LineString, Point, Polygon};
use serde_json::{Value, from_value};

// No custom types huh? TODO Speak with James re: Geojson
type Position = Vec<f64>;
type PointType = Position;
type LineStringType = Vec<Position>;
type PolygonType = Vec<Vec<Position>>;

pub fn geometry(val: &Value) -> GremlinResult<GValue> {
    if let Some(coordinates) = val.get("coordinates") {
        let value = if let Ok(coords) = from_value::<PointType>(coordinates.clone()) {
            let (x, y) = (coords[0], coords[1]);
            let coord = Coord { x, y };
            GValue::Geometry(Geometry::Point(Point(coord)))
        } else if let Ok(coords) = from_value::<LineStringType>(coordinates.clone()) {
            let gt_coords = coords
                .into_iter()
                .map(|c| Coord { x: c[0], y: c[1] })
                .collect::<Vec<Coord>>();
            GValue::Geometry(Geometry::LineString(LineString(gt_coords)))
        } else if let Ok(coords) = from_value::<PolygonType>(coordinates.clone()) {
            // TODO actually check bounds for Polygon type
            let mut lines = coords
                .into_iter()
                .map(|line| {
                    line.into_iter()
                        .map(|c| Coord { x: c[0], y: c[1] })
                        .collect()
                })
                .map(|coords| LineString::new(coords))
                .collect::<Vec<LineString>>();
            let exterior = lines.remove(0);
            GValue::Geometry(Geometry::Polygon(Polygon::new(exterior, lines)))
        } else {
            GValue::Null
        };

        Ok(value)
    } else {
        Err(GremlinError::Json("Expected Geometry".to_string()))
    }
}
