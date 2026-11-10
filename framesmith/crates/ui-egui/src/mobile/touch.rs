//! Touch gestures for phones and tablets.
//!
//! The panels were written for a mouse: a press is a click or the start of a drag, a right click
//! opens a context menu, the wheel scrolls and a pinch zooms. egui's touch support turns the first
//! finger into that mouse as soon as it lands, so every tap that scrolls a list also presses
//! whatever is under the finger, and there is no way to right-click. This translator sits in front
//! of egui ([`Touch::process`], from the app's raw input hook) and replaces the emulated mouse with
//! gestures a phone user expects:
//!
//! - **tap**: a click where the finger lifted (two taps make a double click);
//! - **drag**: once the finger has moved a few points, a press where it landed and a drag from
//!   there (moving a clip, trimming, scrubbing, dragging a slider);
//! - **long press**: a secondary click, which opens the panel's context menu;
//! - **two fingers**: never a click; moving them scrolls (a sideways move scrolls the Timeline in
//!   time) and spreading or pinching them zooms ([`egui::Event::Zoom`], as a trackpad pinch).
//!
//! A frame without touches (a mouse, a stylus reporting as a mouse, synthetic input) passes
//! through untouched.

use egui::{Event, Modifiers, MouseWheelUnit, PointerButton, Pos2, RawInput, TouchId, TouchPhase, Vec2};

/// How far (points) a finger may wander before a press becomes a drag.
pub const SLOP: f32 = 9.0;
/// How long (seconds) a still finger is held before it is a long press.
pub const LONG_PRESS: f64 = 0.45;

#[derive(Clone, Copy, Debug, PartialEq)]
enum State {
    /// No finger down.
    Idle,
    /// One finger down, still: a tap, a drag or a long press, not yet known.
    Pending { start: Pos2, at: f64, pos: Pos2 },
    /// One finger dragging: the press went to egui where the finger landed.
    Dragging { pos: Pos2 },
    /// The long press fired (a context click); the rest of this touch does nothing.
    LongPressed,
    /// Two fingers: scroll and zoom.
    TwoFinger { centroid: Pos2, spread: f32 },
    /// Fingers are still down after a gesture ended (one of two lifted): ignored until all lift.
    Done,
}

/// The gesture state, kept across frames.
#[derive(Clone, Debug)]
pub struct Touch {
    state: State,
    /// Fingers down: (id, position), in the order they landed.
    fingers: Vec<(TouchId, Pos2)>,
    /// A touch was seen since start: the app is on a touch screen (for touch-friendly sizes).
    pub seen: bool,
}

impl Default for Touch {
    fn default() -> Self {
        Self { state: State::Idle, fingers: Vec::new(), seen: false }
    }
}

fn press(pos: Pos2, button: PointerButton, pressed: bool) -> Event {
    Event::PointerButton { pos, button, pressed, modifiers: Modifiers::NONE }
}

impl Touch {
    /// Whether a gesture is undecided and needs the clock: the caller keeps repainting until the
    /// long press fires or the finger moves or lifts.
    pub fn waiting(&self) -> bool {
        matches!(self.state, State::Pending { .. })
    }

    /// Whether any finger is down.
    pub fn active(&self) -> bool {
        !self.fingers.is_empty()
    }

    /// Rewrite this frame's input: the emulated mouse events are replaced by the gestures the
    /// touches make. Returns true when another frame is needed soon (a long press may be due).
    pub fn process(&mut self, raw: &mut RawInput) -> bool {
        let touching = raw.events.iter().any(|e| matches!(e, Event::Touch { .. }));
        if !touching && self.state == State::Idle && self.fingers.is_empty() {
            return false;
        }
        self.seen |= touching;
        let now = raw.time.unwrap_or(0.0);
        let events = std::mem::take(&mut raw.events);
        let mut out = Vec::with_capacity(events.len() + 4);
        for e in events {
            match e {
                // egui-winit's mouse emulation of the first finger: replaced by the gestures
                Event::PointerMoved(_) | Event::PointerButton { .. } | Event::PointerGone if touching || self.active() => {}
                Event::Touch { id, phase, pos, .. } => {
                    out.push(e.clone());
                    self.touch(id, phase, pos, now, &mut out);
                }
                other => out.push(other),
            }
        }
        // the long press fires on the clock, not on an event
        if let State::Pending { start, at, .. } = self.state
            && now - at >= LONG_PRESS
        {
            out.push(Event::PointerMoved(start));
            out.push(press(start, PointerButton::Secondary, true));
            out.push(press(start, PointerButton::Secondary, false));
            self.state = State::LongPressed;
        }
        raw.events = out;
        self.waiting()
    }

    fn centroid_and_spread(&self) -> Option<(Pos2, f32)> {
        let (a, b) = (self.fingers.first()?.1, self.fingers.get(1)?.1);
        Some((a + (b - a) / 2.0, (b - a).length()))
    }

    fn touch(&mut self, id: TouchId, phase: TouchPhase, pos: Pos2, now: f64, out: &mut Vec<Event>) {
        match phase {
            TouchPhase::Start => {
                self.fingers.retain(|(f, _)| *f != id);
                self.fingers.push((id, pos));
                match (self.fingers.len(), self.state) {
                    (1, _) => {
                        self.state = State::Pending { start: pos, at: now, pos };
                        out.push(Event::PointerMoved(pos));
                    }
                    // a second finger before the first did anything: scroll / zoom
                    (2, State::Pending { .. }) => {
                        if let Some((centroid, spread)) = self.centroid_and_spread() {
                            self.state = State::TwoFinger { centroid, spread };
                            out.push(Event::PointerMoved(centroid));
                        }
                    }
                    // more fingers during a drag, a pinch or after a long press change nothing
                    _ => {}
                }
            }
            TouchPhase::Move => {
                let first = self.fingers.first().map(|(f, _)| *f);
                if let Some(f) = self.fingers.iter_mut().find(|(f, _)| *f == id) {
                    f.1 = pos;
                }
                match self.state {
                    State::Pending { start, at, .. } if first == Some(id) => {
                        if (pos - start).length() > SLOP {
                            // the drag starts where the finger landed, so nothing jumps
                            out.push(Event::PointerMoved(start));
                            out.push(press(start, PointerButton::Primary, true));
                            out.push(Event::PointerMoved(pos));
                            self.state = State::Dragging { pos };
                        } else {
                            self.state = State::Pending { start, at, pos };
                        }
                    }
                    State::Dragging { .. } if first == Some(id) => {
                        out.push(Event::PointerMoved(pos));
                        self.state = State::Dragging { pos };
                    }
                    State::TwoFinger { centroid, spread } => {
                        if let Some((c, s)) = self.centroid_and_spread() {
                            out.push(Event::PointerMoved(c));
                            let d: Vec2 = c - centroid;
                            if d != Vec2::ZERO {
                                out.push(Event::MouseWheel { unit: MouseWheelUnit::Point, delta: d, modifiers: Modifiers::NONE, phase: TouchPhase::Move });
                            }
                            if spread > 1.0 && s > 1.0 {
                                let z = s / spread;
                                if z.is_finite() && (z - 1.0).abs() > 0.002 {
                                    out.push(Event::Zoom(z));
                                }
                            }
                            self.state = State::TwoFinger { centroid: c, spread: s };
                        }
                    }
                    _ => {}
                }
            }
            TouchPhase::End | TouchPhase::Cancel => {
                let first = self.fingers.first().map(|(f, _)| *f);
                self.fingers.retain(|(f, _)| *f != id);
                match self.state {
                    State::Pending { start, .. } if phase == TouchPhase::End => {
                        // a tap: press and release where the finger landed, then no hover
                        out.push(Event::PointerMoved(start));
                        out.push(press(start, PointerButton::Primary, true));
                        out.push(press(start, PointerButton::Primary, false));
                        out.push(Event::PointerGone);
                        self.state = State::Idle;
                    }
                    State::Dragging { pos } if first == Some(id) => {
                        out.push(press(pos, PointerButton::Primary, false));
                        out.push(Event::PointerGone);
                        self.state = if self.fingers.is_empty() { State::Idle } else { State::Done };
                    }
                    _ => {
                        if self.fingers.is_empty() {
                            out.push(Event::PointerGone);
                            self.state = State::Idle;
                        } else if !matches!(self.state, State::Dragging { .. }) {
                            self.state = State::Done;
                        }
                    }
                }
                if self.fingers.is_empty() {
                    self.state = State::Idle;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::TouchDeviceId;

    fn t(id: u64, phase: TouchPhase, x: f32, y: f32) -> Event {
        Event::Touch { device_id: TouchDeviceId(0), id: TouchId(id), phase, pos: Pos2::new(x, y), force: None }
    }

    /// The events egui-winit sends for a touch: the touch itself, plus its mouse emulation.
    fn frame(touch: &mut Touch, time: f64, events: Vec<Event>) -> Vec<Event> {
        let mut raw = RawInput { time: Some(time), events, ..Default::default() };
        touch.process(&mut raw);
        raw.events.into_iter().filter(|e| !matches!(e, Event::Touch { .. })).collect()
    }

    fn buttons(events: &[Event]) -> Vec<(PointerButton, bool)> {
        events
            .iter()
            .filter_map(|e| match e {
                Event::PointerButton { button, pressed, .. } => Some((*button, *pressed)),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn a_tap_is_a_click_where_the_finger_landed() {
        let mut touch = Touch::default();
        let start = vec![t(1, TouchPhase::Start, 50.0, 50.0), Event::PointerMoved(Pos2::new(50.0, 50.0)), press(Pos2::new(50.0, 50.0), PointerButton::Primary, true)];
        let out = frame(&mut touch, 0.0, start);
        assert!(buttons(&out).is_empty(), "nothing is pressed while it could still be a scroll: {out:?}");
        let out = frame(&mut touch, 0.1, vec![t(1, TouchPhase::End, 51.0, 50.0), press(Pos2::new(51.0, 50.0), PointerButton::Primary, false), Event::PointerGone]);
        assert_eq!(buttons(&out), [(PointerButton::Primary, true), (PointerButton::Primary, false)]);
        assert!(matches!(out.last(), Some(Event::PointerGone)));
        assert!(!touch.active());
    }

    #[test]
    fn a_moving_finger_drags_from_where_it_landed() {
        let mut touch = Touch::default();
        frame(&mut touch, 0.0, vec![t(1, TouchPhase::Start, 10.0, 10.0)]);
        let out = frame(&mut touch, 0.05, vec![t(1, TouchPhase::Move, 30.0, 10.0)]);
        assert_eq!(buttons(&out), [(PointerButton::Primary, true)]);
        let first_press = out.iter().find_map(|e| match e {
            Event::PointerButton { pos, .. } => Some(*pos),
            _ => None,
        });
        assert_eq!(first_press, Some(Pos2::new(10.0, 10.0)));
        let out = frame(&mut touch, 0.1, vec![t(1, TouchPhase::Move, 60.0, 10.0)]);
        assert!(out.contains(&Event::PointerMoved(Pos2::new(60.0, 10.0))));
        let out = frame(&mut touch, 0.6, vec![t(1, TouchPhase::End, 60.0, 10.0)]);
        assert_eq!(buttons(&out), [(PointerButton::Primary, false)], "a drag never turns into a long press");
    }

    #[test]
    fn a_long_press_is_a_context_click_and_nothing_else() {
        let mut touch = Touch::default();
        frame(&mut touch, 0.0, vec![t(1, TouchPhase::Start, 10.0, 10.0)]);
        assert!(touch.waiting());
        let out = frame(&mut touch, 0.2, vec![]);
        assert!(buttons(&out).is_empty());
        let out = frame(&mut touch, 0.5, vec![]);
        assert_eq!(buttons(&out), [(PointerButton::Secondary, true), (PointerButton::Secondary, false)]);
        let out = frame(&mut touch, 0.6, vec![t(1, TouchPhase::Move, 40.0, 10.0)]);
        assert!(buttons(&out).is_empty(), "moving after a long press doesn't drag");
        let out = frame(&mut touch, 0.7, vec![t(1, TouchPhase::End, 40.0, 10.0)]);
        assert!(buttons(&out).is_empty(), "lifting after a long press doesn't click");
    }

    #[test]
    fn two_fingers_scroll_and_zoom_and_never_click() {
        let mut touch = Touch::default();
        frame(&mut touch, 0.0, vec![t(1, TouchPhase::Start, 100.0, 100.0)]);
        frame(&mut touch, 0.02, vec![t(2, TouchPhase::Start, 200.0, 100.0)]);
        // both move right by 10 and apart by 20 more
        let out = frame(&mut touch, 0.05, vec![t(1, TouchPhase::Move, 100.0, 100.0), t(2, TouchPhase::Move, 240.0, 100.0)]);
        let wheel: Vec2 = out
            .iter()
            .filter_map(|e| match e {
                Event::MouseWheel { delta, .. } => Some(*delta),
                _ => None,
            })
            .fold(Vec2::ZERO, |a, b| a + b);
        assert!((wheel.x - 20.0).abs() < 0.01, "{wheel:?}");
        let zoom: f32 = out
            .iter()
            .filter_map(|e| match e {
                Event::Zoom(z) => Some(*z),
                _ => None,
            })
            .product();
        assert!((zoom - 1.4).abs() < 0.01, "{zoom}");
        let out = frame(&mut touch, 0.1, vec![t(2, TouchPhase::End, 240.0, 100.0)]);
        assert!(buttons(&out).is_empty());
        let out = frame(&mut touch, 0.9, vec![t(1, TouchPhase::End, 100.0, 100.0)]);
        assert!(buttons(&out).is_empty(), "lifting the last finger of a pinch doesn't click: {out:?}");
        assert!(!touch.active());
    }

    #[test]
    fn mouse_input_passes_through() {
        let mut touch = Touch::default();
        let events = vec![Event::PointerMoved(Pos2::new(5.0, 5.0)), press(Pos2::new(5.0, 5.0), PointerButton::Primary, true)];
        let out = frame(&mut touch, 0.0, events.clone());
        assert_eq!(out, events);
    }
}
