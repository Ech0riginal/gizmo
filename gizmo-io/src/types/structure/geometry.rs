use crate::*;
use geo_types::Geometry::*;
use std::hash::Hasher;

primitive_prelude!();
very_primitive!(Geometry, geo_types::Geometry<f64>);

// TODO impl for backends once available
impl Tag_<SQLg> for Geometry {
    const tag: &'static str = "g:Geometry";
}

impl Tag_<Janus> for Geometry {
    const tag: &'static str = "g:Geoshape";
}

impl std::hash::Hash for Geometry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.0 {
            Point(point) => {
                state.write(&point.x().to_be_bytes());
                state.write(&point.y().to_be_bytes());
            }
            Line(line) => {
                let bytes = line.slope().to_be_bytes();
                state.write(&bytes);
            }
            LineString(linestring) => {
                linestring.coords().for_each(|c| {
                    state.write(&c.x.to_be_bytes());
                    state.write(&c.y.to_be_bytes());
                });
            }
            Polygon(polygon) => {
                polygon.exterior().coords().for_each(|c| {
                    state.write(&c.x.to_be_bytes());
                    state.write(&c.y.to_be_bytes());
                });
                polygon.interiors().iter().for_each(|linestring| {
                    linestring.points().for_each(|c| {
                        state.write(&c.x().to_be_bytes());
                        state.write(&c.y().to_be_bytes());
                    });
                });
            }
            MultiPoint(multipoint) => {
                multipoint.iter().for_each(|c| {
                    state.write(&c.x().to_be_bytes());
                    state.write(&c.y().to_be_bytes());
                });
            }
            MultiLineString(multilinestring) => {
                multilinestring.into_iter().for_each(|linestring| {
                    linestring.points().for_each(|c| {
                        state.write(&c.x().to_be_bytes());
                        state.write(&c.y().to_be_bytes());
                    });
                });
            }
            MultiPolygon(multipolygon) => {
                multipolygon.into_iter().for_each(|polygon| {
                    polygon.exterior().coords().for_each(|c| {
                        state.write(&c.x.to_be_bytes());
                        state.write(&c.y.to_be_bytes());
                    });
                    polygon.interiors().iter().for_each(|linestring| {
                        linestring.points().for_each(|c| {
                            state.write(&c.x().to_be_bytes());
                            state.write(&c.y().to_be_bytes());
                        });
                    });
                });
            }
            GeometryCollection(collection) => {
                collection.len().hash(state);
            }
            Rect(rect) => {
                let c = rect.center();
                state.write(&c.x.to_be_bytes());
                state.write(&c.y.to_be_bytes());
            }
            Triangle(triangle) => {
                state.write(&triangle.0.x.to_be_bytes());
                state.write(&triangle.0.y.to_be_bytes());
                state.write(&triangle.1.x.to_be_bytes());
                state.write(&triangle.1.y.to_be_bytes());
                state.write(&triangle.2.x.to_be_bytes());
                state.write(&triangle.2.y.to_be_bytes());
            }
        }
    }
}
