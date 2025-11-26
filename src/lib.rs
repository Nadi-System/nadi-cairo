use nadi_core::nadi_plugin::nadi_plugin;
mod draw;
mod plots;

#[nadi_plugin]
mod cairo {
    use super::draw::*;
    use super::plots::*;
    use nadi_core::abi_stable::std_types::RSome;
    use nadi_core::anyhow::{self, Context};
    use nadi_core::graphics::color::AttrColor;
    use nadi_core::nadi_plugin::network_func;
    use nadi_core::prelude::*;
    use nadi_core::table::Table;
    use nadi_core::template::{Template, TemplateError};
    use std::path::PathBuf;
    use std::str::FromStr;

    /// Create a SVG file with the given network structure
    #[network_func(config = NetworkPlotConfig::default(), fit = false)]
    fn network(
        net: &mut Network,
        outfile: PathBuf,
        #[relaxed] config: NetworkPlotConfig,
        fit: bool,
        label: Option<Template>,
    ) -> anyhow::Result<()> {
        let n = net.nodes_count();
        if n == 0 {
            return Err(anyhow::Error::msg("Empty Network"));
        }
        let max_level = net.nodes().map(|n| n.lock().level()).max().unwrap_or(0);

        let mut surf =
            cairo::SvgSurface::new::<&std::path::Path>(config.width, config.height, None)?;
        let ctx = cairo::Context::new(&mut surf)?;
        ctx.set_line_width(1.0);
        ctx.set_font_size(config.fontsize);
        ctx.set_font_face(&config.fontface);

        let mut twidth = 0.0;
        let labels = if let Some(templ) = label {
            net.nodes_rev()
                .map(|n| templ.render(&n.lock()))
                .collect::<Result<Vec<String>, TemplateError>>()?
        } else {
            net.nodes_rev().map(|_| String::new()).collect()
        };
        calc_text_width(&labels, &ctx, &mut twidth)?;
        let mut delx = config.delta_x;
        let mut dely = config.delta_y;

        let mut width = delx * max_level as f64 + 2.0 * config.radius + config.offset + twidth;
        let mut height = dely * (n + 1) as f64 + 2.0 * config.radius;

        let mut surf = if fit {
            delx = (config.width - 2.0 * config.radius - twidth) / (max_level + 1) as f64;
            dely = (config.height - 2.0 * config.radius) / (n + 2) as f64;
            width = config.width;
            height = config.height;
            cairo::SvgSurface::new(config.width, config.height, Some(outfile))?
        } else {
            cairo::SvgSurface::new(width, height, Some(outfile))?
        };

        let ctx = cairo::Context::new(&mut surf)?;
        ctx.set_line_width(1.0);
        ctx.set_font_size(config.fontsize);
        ctx.set_font_face(&config.fontface);

        let offset = width - twidth;

        net.nodes_rev()
            .zip(labels)
            .try_for_each(|(n, l)| -> cairo::Result<()> {
                let n = n.lock();
                let y = height - (n.index() + 1) as f64 * dely;
                let x = n.level() as f64 * delx + delx / 2.0;

                if let RSome(o) = n.output() {
                    let o = o.lock();
                    let yo = height - (o.index() + 1) as f64 * dely;
                    let xo = o.level() as f64 * delx + delx / 2.0;
                    let dx = xo - x;
                    let dy = yo - y;
                    let l = (dx.powi(2) + dy.powi(2)).sqrt();
                    let (ux, uy) = (dx / l, dy / l);
                    let size = n.node_size();
                    let (sx, sy) = (x + ux * size * 1.4, y + uy * size * 1.4);
                    let (ex, ey) = (xo - ux * size * 1.4, yo - uy * size * 1.4);
                    draw_line(&n, &ctx, sx, sy, ex, ey)?;
                }
                draw_node(&n, &ctx, x, y)?;
                draw_text(&n, &ctx, offset, y, &l)
            })?;

        Ok(())
    }
    /// Create a SVG file with the given network structure
    #[network_func(config = NetworkPlotConfig::default(), fit = false)]
    fn table(
        net: &mut Network,
        table: Table,
        outfile: PathBuf,
        #[relaxed] config: NetworkPlotConfig,
        fit: bool,
    ) -> anyhow::Result<()> {
        export_svg_table(net, table, outfile, config, fit)
    }
}
