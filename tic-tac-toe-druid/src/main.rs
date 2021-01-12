// Copyright 2019 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An example of an animating widget. It is just a widget that
//! requests an animation frame when it needs to, and draws the frame in the
//! `paint` method.
//! Once the animation is over it simply stops requesting animation frames.
//! Usually we would put the state in the `Data`, but for things like animation
//! we don't. This is because the animation state is not useful to know for the
//! rest of the app. If this is something the rest of your widgets should know
//! about, you could put it in the `data`.

use std::f64::consts::PI;

use druid::kurbo::{Circle, Line};
use druid::widget::prelude::*;
use druid::widget::Container;
use druid::{AppLauncher, Color, LocalizedString, Point, Vec2, WindowDesc};

struct AnimWidget {
    t: f64,
    delta: f64,
}

impl Widget<()> for AnimWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut (), _env: &Env) {
        match event {
            Event::MouseDown(_) => {
                self.delta = 0.2;
                ctx.request_anim_frame();
            }
            Event::AnimFrame(interval) => {
                ctx.request_paint();
                self.delta *= 1. - (*interval as f64) * 1e-9;
                self.t += self.delta;
                self.t %= 1.;
                if self.delta > 0.0001 {
                    ctx.request_anim_frame();
                }
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &(), _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &(), _data: &(), _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &(),
        _env: &Env,
    ) -> Size {
        bc.constrain((200.0, 200.0))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &(), _env: &Env) {
        let t = self.t;
        let center = Point::new(100.0, 100.0);
        ctx.paint_with_z_index(1, move |ctx| {
            let ambit = center + 90.0 * Vec2::from_angle((0.75 + t) * 2.0 * PI);
            ctx.stroke(Line::new(center, ambit), &Color::WHITE, 1.0);
        });

        ctx.fill(Circle::new(center, 100.0), &Color::BLACK);
    }
}

fn anim_widget_container() -> impl Widget<()> {
    AnimWidget { t: 0.0, delta: 0.2 }
}

pub fn main() {
    let window = WindowDesc::new(anim_widget_container).title(
        LocalizedString::new("anim-demo-window-title")
            .with_placeholder("You spin me right round..."),
    );
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(())
        .expect("launch failed");
}