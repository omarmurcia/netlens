use iced::mouse;
use iced::widget::canvas::{self, Frame, Geometry, Path, Stroke};
use iced::{Color, Point, Rectangle, Renderer, Theme};

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
        if self.points.len() < 2 {
            return vec![frame.into_geometry()];
        }

        let max_val = self
            .points
            .iter()
            .copied()
            .fold(0.0f32, |a, b| a.max(b))
            .max(1.0);

        let step_x = bounds.width / (self.points.len() - 1) as f32;

        let path = Path::new(|builder| {
            for (i, &val) in self.points.iter().enumerate() {
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
                .with_color(self.color)
                .with_width(1.8),
        );

        vec![frame.into_geometry()]
    }
}
