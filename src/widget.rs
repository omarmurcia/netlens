use iced::mouse;
use iced::widget::canvas::{self, Cache, Frame, Geometry, Path, Stroke};
use iced::{Color, Point, Rectangle, Renderer, Theme};
use std::collections::VecDeque;

fn draw_sparkline_into_frame<'a>(
    frame: &mut Frame,
    bounds: Rectangle,
    points: impl Iterator<Item = &'a f32> + Clone,
    len: usize,
    color: Color,
) {
    if len < 2 {
        return;
    }

    let max_val = points
        .clone()
        .copied()
        .fold(0.0f32, |a, b| a.max(b))
        .max(1.0);

    let step_x = bounds.width / (len - 1) as f32;

    let path = Path::new(|builder| {
        for (i, &val) in points.enumerate() {
            let x = i as f32 * step_x;
            let normalized = (val / max_val).clamp(0.0, 1.0);
            let y = bounds.height - (normalized * (bounds.height - 4.0)) - 2.0;
            if i == 0 {
                builder.move_to(Point::new(x, y));
            } else {
                builder.line_to(Point::new(x, y));
            }
        }
    });

    frame.stroke(
        &path,
        Stroke::default()
            .with_color(color)
            .with_width(1.8),
    );
}

/// Ultra-lightweight Sparkline chart widget to visualize real-time telemetry (traffic and latency)
#[derive(Debug, Clone)]
pub struct Sparkline {
    pub points: Vec<f32>,
    pub color: Color,
}

impl Sparkline {
    pub fn new(points: Vec<f32>, color: Color) -> Self {
        Self { points, color }
    }
}

impl<Message> canvas::Program<Message> for Sparkline {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        draw_sparkline_into_frame(&mut frame, bounds, self.points.iter(), self.points.len(), self.color);
        vec![frame.into_geometry()]
    }
}

/// Zero-allocation, GPU-cached Sparkline widget backed by a `canvas::Cache` and `&VecDeque<f32>`
pub struct CachedSparkline<'a> {
    pub points: &'a VecDeque<f32>,
    pub color: Color,
    pub cache: &'a Cache,
}

impl<'a> CachedSparkline<'a> {
    pub fn new(points: &'a VecDeque<f32>, color: Color, cache: &'a Cache) -> Self {
        Self {
            points,
            color,
            cache,
        }
    }
}

impl<'a, Message> canvas::Program<Message> for CachedSparkline<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            draw_sparkline_into_frame(frame, bounds, self.points.iter(), self.points.len(), self.color);
        });
        vec![geometry]
    }
}
