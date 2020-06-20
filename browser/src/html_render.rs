#![allow(dead_code)]

use glium;
use std::rc::Rc;

#[allow(unused_imports)]
use glium::{glutin, Surface};
use glium::index::PrimitiveType;

pub trait RenderCall {
	fn render(&self, target: &mut glium::Frame);
}

#[derive(Copy, Clone)]
struct Vertex {
	position: [f32; 2],
	tex_coord: [f32; 2]
}

pub struct RectRender {
	vertex_buffer: glium::VertexBuffer<Vertex>,
	index_buffer: glium::IndexBuffer<u16>,
	program: glium::Program
}
impl RectRender {
	pub fn new(display: &glium::Display) -> RectRender {
		RectRender {
			vertex_buffer: {
				implement_vertex!(Vertex, position, tex_coord);
			
				glium::VertexBuffer::new(display,
					&[
						Vertex { position: [0.0, 0.0], tex_coord: [0.0, 0.0] },
						Vertex { position: [0.0, 1.0], tex_coord: [0.0, 1.0] },
						Vertex { position: [1.0, 1.0], tex_coord: [1.0, 1.0] },
						Vertex { position: [1.0, 0.0], tex_coord: [1.0, 0.0] }
					]
				).unwrap()
			},
			index_buffer: glium::IndexBuffer::new(display, PrimitiveType::TrianglesList, &[0u16, 1, 2, 0, 2, 3]).unwrap(),
			program: program!(display,
				140 => {
					vertex: "
						#version 140
						uniform mat4 matrix;
						in vec2 position;
						void main() {
							gl_Position = matrix * vec4(position, 0.0, 1.0);
						}
					",
					fragment: "
						#version 140
						out vec4 f_color;
						uniform vec4 color;
						void main() {
							f_color = color;
						}
					"
				}
			).unwrap()
		}
	}
}
#[derive(Debug)]
pub struct RectDetail {
	x: f32, y: f32,
	w: f32, h: f32,
	color: (f32, f32, f32, f32)
}
impl RectDetail {
	pub fn new(x: f32, y: f32, w: f32, h: f32) -> RectDetail {
		RectDetail {
			x, y, w, h,
			color: (0.0, 0.0, 0.0, 0.0)
		}
	}

	pub fn color(&mut self, r: f32, g: f32, b: f32, a: f32) {
		self.color = (r, g, b, a);
	}

	pub fn opaque(&mut self, r: f32, g: f32, b: f32) {
		self.color = (r, g, b, 1.0);
	}

	fn uniforms(&self, target: &glium::Frame) -> glium::uniforms::UniformsStorage<[f32; 4], glium::uniforms::UniformsStorage<[[f32; 4]; 4], glium::uniforms::EmptyUniforms>> {
		let scale_x = 2.0 / (target.get_dimensions().0 as f32);
		let scale_y = 2.0 / (target.get_dimensions().1 as f32);

		uniform!{
            matrix: [
                [self.w * scale_x, 0.0, 0.0, 0.0],
                [0.0, self.h * -scale_y, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [(self.x * scale_x) - 1.0, 1.0 - (self.y * scale_y), 0.0, 1.0f32]
			],
			color: [ self.color.0, self.color.1, self.color.2, self.color.3 ]
		}
	}
}
impl RectRender {
	fn render(&self, shape: &RectDetail, target: &mut glium::Frame) {
		let uniforms = shape.uniforms(target);
        target.draw(&self.vertex_buffer, &self.index_buffer, &self.program, &uniforms, &Default::default()).unwrap();
	}
}
pub struct RectRenderCall {
	shape: Rc<RectDetail>,
	rect_render: Rc<RectRender>
}
impl RectRenderCall {
	pub fn new(shape: Rc<RectDetail>, rect_render: Rc<RectRender>) -> RectRenderCall {
		RectRenderCall {
			shape, rect_render
		}
	}
}
impl RenderCall for RectRenderCall {
	fn render(&self, target: &mut glium::Frame) {
		self.rect_render.render(self.shape.as_ref(), target);
	}
}

pub struct BlankRenderCall {}
impl BlankRenderCall {
	pub fn new() -> BlankRenderCall {
		BlankRenderCall {}
	}
}
impl RenderCall for BlankRenderCall {
	fn render(&self, _target: &mut glium::Frame) {}
}

pub struct Renderer {
	display: glium::Display,
	render_calls: Vec<Box<dyn RenderCall>>,
	queued: bool
}
impl Renderer {
	pub fn new(event_loop: &glium::glutin::event_loop::EventLoop<()>, wb: glium::glutin::window::WindowBuilder, cb: glium::glutin::ContextBuilder<glium::glutin::NotCurrent>) -> Renderer {
		Renderer {
			display: glium::Display::new(wb, cb, event_loop).unwrap(),
			render_calls: vec![],
			queued: false
		}
	}

	pub fn create_context(title: &str, el: &glium::glutin::event_loop::EventLoop<()>) -> Renderer {
		let wb = glutin::window::WindowBuilder::new().with_title(title);
		let cb = glutin::ContextBuilder::new();

		Renderer::new(el, wb, cb)
	}

	pub fn create_context_and_loop(title: &str) -> (Renderer, glutin::event_loop::EventLoop<()>) {
		let event_loop = glutin::event_loop::EventLoop::new();
		(Renderer::create_context(title, &event_loop), event_loop)
	}

	pub fn display(&self) -> &glium::Display {
		&self.display
	} 

	pub fn render(&mut self) {
		let mut target = self.display.draw();
		target.clear_color(1.0, 1.0, 1.0, 0.0);
		for rc in self.render_calls.iter() {
			rc.render(&mut target);
		}
		target.finish().unwrap();

		self.queued = false;
	}

	pub fn add(&mut self, rc: Box<dyn RenderCall>) {
		self.render_calls.push(rc);
	}

	pub fn event(&mut self, event: glium::glutin::event::Event<()>, control_flow: &mut glium::glutin::event_loop::ControlFlow) {
		*control_flow = match event {
			glutin::event::Event::WindowEvent { event, .. } => match event {
				glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
				glutin::event::WindowEvent::Resized(..) => {
					self.render();
					glutin::event_loop::ControlFlow::Poll
				},
				_ => glutin::event_loop::ControlFlow::Poll,
			},
			_ => glutin::event_loop::ControlFlow::Poll,
		};

		if self.queued { self.render(); }
	}

	pub fn queue(&mut self) {
		self.queued = true;
	}
}