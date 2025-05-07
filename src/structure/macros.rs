#[doc(hidden)]
#[macro_export]
macro_rules! vertex {
    ({ id => $id:expr,label => $label:expr, properties => {$($key:expr => [$({ id => $p_id:expr, value => $p_value:expr}),*]),*}}) => {{

        #[allow(unused_mut)]
        let mut properties  = ::std::collections::HashMap::<String,Vec<$crate::prelude::VertexProperty>>::new();
            $(
                let mut sub_props = vec![];
                $(
                    let p = $crate::prelude::VertexProperty::new($p_id,$key,$p_value);
                    sub_props.push(p);
                )*
                properties.insert($key.into(),sub_props);
            )*
        let v = $crate::prelude::Vertex::new($id.into(), $label,properties);
        v
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! edge {
    ({
       id => $id:expr,
       label => $label:expr,
       inV => {  id => $inVId:expr, label => $inVLabel:expr},
       outV => {  id => $outVId:expr, label => $outVLabel:expr} ,
       properties => {$($key:expr => $value:expr),*}}) => {{

           #[allow(unused_mut)]
        let mut properties  = ::std::collections::HashMap::<String,$crate::prelude::Property>::new();
            $(
                let p = Property::new($key.into(),$value.into());
                properties.insert($key.into(),p);
            )*
        $crate::prelude::Edge::new($id.into(), $label, $inVId.into(),$inVLabel,$outVId.into(),$outVLabel,properties)
    }};
}
