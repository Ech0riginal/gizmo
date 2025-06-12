use crate::Geometry;
use crate::graphson::prelude::*;
use geo_types::{Coord, Geometry as GeoTypes, LineString, Point, Polygon};
use serde_json::{Value, from_value};

type PointType = Vec<f64>;
type LineStringType = Vec<PointType>;
type PolygonType = Vec<Vec<PointType>>;

impl Deserializer<Geometry> for V3 {
    fn deserialize(val: &Value) -> Result<Geometry, Error> {
        let coordinates = val.ensure("coordinates")?;

        // There must be a better way to do this
        if let Ok(coords) = from_value::<PointType>(coordinates.clone()) {
            let (x, y) = (coords[0], coords[1]);
            let coord = Coord { x, y };
            Ok(GeoTypes::Point(Point(coord)).into())
        } else if let Ok(coords) = from_value::<LineStringType>(coordinates.clone()) {
            let gt_coords = coords
                .into_iter()
                .map(|c| Coord { x: c[0], y: c[1] })
                .collect::<Vec<Coord>>();
            Ok(GeoTypes::LineString(LineString(gt_coords)).into())
        } else if let Ok(coords) = from_value::<PolygonType>(coordinates.clone()) {
            // TODO actually check bounds for Polygon type
            let mut lines = coords
                .into_iter()
                .map(|line| {
                    line.into_iter()
                        .map(|c| Coord { x: c[0], y: c[1] })
                        .collect()
                })
                .map(LineString::new)
                .collect::<Vec<LineString>>();
            let exterior = lines.remove(0);
            Ok(GeoTypes::Polygon(Polygon::new(exterior, lines)).into())
        } else {
            Err(Error::Unsupported {
                tag: "Non-trivial geometry".into(),
                location: location!(),
            })
        }
    }
}

impl Serializer<Geometry> for V3 {
    fn serialize(val: &Geometry) -> Result<Value, Error> {
        Ok(json!({
            "@type": Tag::Geometry,
            "@value": geojson::Value::from(&val.0)
        }))
    }
}

// TODO set up neo4j & janusgraph on the dev server
