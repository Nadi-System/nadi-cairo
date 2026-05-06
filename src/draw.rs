use cairo::Context;
use nadi_core::graphics::color::{AttrColor, Color};
use nadi_core::graphics::node::NodeShape;
use nadi_core::node::NodeInner;

#[derive(Debug, Clone)]
pub struct CairoColor {
    r: f64,
    g: f64,
    b: f64,
}

impl Default for CairoColor {
    fn default() -> Self {
        Self {
            r: 0.5,
            g: 0.5,
            b: 1.0,
        }
    }
}

impl CairoColor {
    pub fn set(&self, ctx: &Context) {
        ctx.set_source_rgb(self.r, self.g, self.b);
    }
}

impl From<Color> for CairoColor {
    fn from(val: Color) -> Self {
        Self {
            r: val.r as f64 / 255.0,
            g: val.g as f64 / 255.0,
            b: val.b as f64 / 255.0,
        }
    }
}

pub fn draw_node(node: &NodeInner, ctx: &Context, x: f64, y: f64) -> cairo::Result<()> {
    CairoColor::from(node.node_color()).set(ctx);
    let size = node.node_size();
    match node.node_shape() {
        NodeShape::Square => {
            ctx.move_to(x - size / 2.0, y - size / 2.0);
            ctx.rel_line_to(0.0, size);
            ctx.rel_line_to(size, 0.0);
            ctx.rel_line_to(0.0, -size);
            ctx.rel_line_to(-size, 0.0);
        }
        NodeShape::Rectangle(r) => {
            let r = r.abs();
            let (sizex, sizey) = if r > 1.0 {
                (size / r, size)
            } else {
                (size, size * r)
            };
            ctx.move_to(x - sizex / 2.0, y - sizey / 2.0);
            ctx.rel_line_to(0.0, sizey);
            ctx.rel_line_to(sizex, 0.0);
            ctx.rel_line_to(0.0, -sizey);
            ctx.rel_line_to(-sizex, 0.0);
        }
        NodeShape::Circle => {
            ctx.arc(x, y, size / 2.0, 0.0, 2.0 * 3.1416);
        }
        NodeShape::Ellipse(r) => {
            let r = r.abs();
            let (sizex, sizey) = if r > 1.0 {
                (size / r, size)
            } else {
                (size, size * r)
            };
            // todo: make ellipse
            ctx.arc(x, y, sizex / 2.0, 0.0, 2.0 * 3.1416);
        }
        NodeShape::Triangle => {
            let ht = 0.8660 * size;
            let dx = size / 2.0;
            ctx.move_to(x - dx, y + ht / 3.0);
            ctx.line_to(x, y - 2.0 * ht / 3.0);
            ctx.line_to(x + dx, y + ht / 3.0);
        }
        NodeShape::IsoTriangle(r) => {
            let ht = 0.8660 * size;
            let dx = size / 2.0;
            let r = r.abs();
            let (ht, dx) = if r > 1.0 { (ht / r, dx) } else { (ht, dx * r) };
            ctx.move_to(x - dx, y + ht / 3.0);
            ctx.line_to(x, y - 2.0 * ht / 3.0);
            ctx.line_to(x + dx, y + ht / 3.0);
        }
    }
    ctx.fill()?;
    ctx.stroke()
}

pub fn draw_line(
    node: &NodeInner,
    ctx: &Context,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
) -> cairo::Result<()> {
    CairoColor::from(node.line_color()).set(ctx);
    ctx.set_line_width(node.line_width());
    ctx.move_to(x1, y1);
    ctx.line_to(x2, y2);
    ctx.stroke()
    // todo: draw arrow
}

pub fn draw_text(node: &NodeInner, ctx: &Context, x: f64, y: f64, text: &str) -> cairo::Result<()> {
    CairoColor::from(node.text_color()).set(ctx);
    ctx.move_to(x, y);
    ctx.show_text(text)
}
