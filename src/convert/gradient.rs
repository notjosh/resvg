// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// external
use svgdom;

// self
use tree::prelude::*;
use short::{
    AId,
    EId,
};
use traits::{
    GetValue,
};


pub fn convert_linear(
    node: &svgdom::Node,
    rtree: &mut tree::RenderTree,
) {
    let ref attrs = node.attributes();
    let transform = attrs.get_transform(AId::GradientTransform).unwrap_or_default();

    let grad = rtree.append_to_defs(
        tree::NodeKind::LinearGradient(tree::LinearGradient {
            id: node.id().clone(),
            x1: attrs.get_number(AId::X1).unwrap_or(0.0),
            y1: attrs.get_number(AId::Y1).unwrap_or(0.0),
            x2: attrs.get_number(AId::X2).unwrap_or(1.0),
            y2: attrs.get_number(AId::Y2).unwrap_or(0.0),
            d: tree::BaseGradient {
                units: super::convert_element_units(attrs, AId::GradientUnits),
                transform,
                spread_method: convert_spread_method(&attrs),
            }
        })
    );

    convert_stops(node, grad, rtree);
}

pub fn convert_radial(
    node: &svgdom::Node,
    rtree: &mut tree::RenderTree,
) {
    let ref attrs = node.attributes();
    let transform = attrs.get_transform(AId::GradientTransform).unwrap_or_default();

    let grad = rtree.append_to_defs(
        tree::NodeKind::RadialGradient(tree::RadialGradient {
            id: node.id().clone(),
            cx: attrs.get_number(AId::Cx).unwrap_or(0.5),
            cy: attrs.get_number(AId::Cy).unwrap_or(0.5),
            r:  attrs.get_number(AId::R).unwrap_or(0.5),
            fx: attrs.get_number(AId::Fx).unwrap_or(0.5),
            fy: attrs.get_number(AId::Fy).unwrap_or(0.5),
            d: tree::BaseGradient {
                units: super::convert_element_units(attrs, AId::GradientUnits),
                transform,
                spread_method: convert_spread_method(&attrs),
            }
        })
    );

    convert_stops(node, grad, rtree);
}

fn convert_spread_method(
    attrs: &svgdom::Attributes
) -> tree::SpreadMethod {
    let av = attrs.get_predef(AId::SpreadMethod).unwrap_or(svgdom::ValueId::Pad);

    match av {
        svgdom::ValueId::Pad => tree::SpreadMethod::Pad,
        svgdom::ValueId::Reflect => tree::SpreadMethod::Reflect,
        svgdom::ValueId::Repeat => tree::SpreadMethod::Repeat,
        _ => tree::SpreadMethod::Pad,
    }
}

fn convert_stops(
    node: &svgdom::Node,
    parent: tree::NodeId,
    rtree: &mut tree::RenderTree,
) {
    for s in node.children() {
        if !s.is_tag_name(EId::Stop) {
            debug!("Invalid gradient child: '{:?}'.", s.tag_id().unwrap());
            continue;
        }

        let attrs = s.attributes();

        let offset = attrs.get_number(AId::Offset).unwrap_or(0.0);
        let color = attrs.get_color(AId::StopColor).unwrap_or(svgdom::Color::new(0, 0, 0));
        let opacity = attrs.get_number(AId::StopOpacity).unwrap_or(1.0);

        rtree.append_child(parent, tree::NodeKind::Stop(tree::Stop {
            offset,
            color,
            opacity,
        }));
    }
}
