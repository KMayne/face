use skia_safe::gpu::gl::FramebufferInfo;
use skia_safe::gpu::{BackendRenderTarget, SurfaceOrigin};
use skia_safe::{Color, ColorType, Surface};
use glutin::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent, ModifiersState};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, PossiblyCurrent};
use glutin::WindowedContext;
use glutin::GlProfile;
use std::convert::TryInto;
use gl::types::*;
use skia_safe::gradient_shader;
use skia_safe::Matrix;
use skia_safe::Paint;
use skia_safe::PaintJoin;
use skia_safe::PaintStyle;
use skia_safe::Path;
use skia_safe::Point;
use skia_safe::TileMode;

pub fn run_face_window() -> () {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("Face Demo");

    let window_context = ContextBuilder::new()
        .with_depth_buffer(0)
        .with_stencil_buffer(8)
        .with_pixel_format(24, 8)
        .with_double_buffer(Some(true))
        .with_gl_profile(GlProfile::Core)
        .build_windowed(wb, &el).unwrap();
    let window_context = unsafe { window_context.make_current().unwrap() };

    println!("Pixel format of the window's GL context: {:?}", window_context.get_pixel_format());

    gl::load_with(|s| window_context.get_proc_address(&s));

    let mut gr_context = skia_safe::gpu::Context::new_gl(None).unwrap();
    let fb_info = {
        let mut fboid: GLint = 0;
        unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

        FramebufferInfo {
            fboid: fboid.try_into().unwrap(),
            format: skia_safe::gpu::gl::Format::RGBA8.into(),
        }
    };

    window_context
        .window()
        .set_inner_size(glutin::dpi::Size::new(glutin::dpi::LogicalSize::new(
            800, 600,
        )));

    let mut frame = 0;
    let mut surface = create_surface(&window_context, &fb_info, &mut gr_context);

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => {}
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    surface = create_surface(&window_context, &fb_info, &mut gr_context);
                    window_context.resize(physical_size)
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        virtual_keycode,
                        modifiers,
                        ..
                    }, ..
                } => handle_keyboard_input(
                    &window_context,
                    control_flow,
                    virtual_keycode,
                    modifiers,
                ),
                _ => (),
            },
            Event::RedrawRequested(_) => {
                {
                    let canvas = surface.canvas();
                    canvas.clear(Color::GRAY);
                    render(canvas);
                }
                surface.canvas().flush();
                window_context.swap_buffers().unwrap();
                frame += 1;
            }
            _ => (),
        }
    });
}

fn handle_keyboard_input(
    window_context: &WindowedContext<PossiblyCurrent>,
    control_flow: &mut ControlFlow,
    virtual_keycode: Option<VirtualKeyCode>,
    modifiers: ModifiersState) {
    #[allow(deprecated)]
    if modifiers.logo() {
        if let Some(VirtualKeyCode::Q) = virtual_keycode {
            *control_flow = ControlFlow::Exit;
        }
    }
    if modifiers.ctrl() {
        if let Some(VirtualKeyCode::Q) = virtual_keycode {
            *control_flow = ControlFlow::Exit;
        }
    }
    window_context.window().request_redraw();
}

fn render(canvas: &mut skia_safe::canvas::Canvas) {
    let center = (50, 50);
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_stroke_width(4.0);
    paint.set_argb(255, 255, 255, 255);
    canvas.draw_circle(center, 50.0, &paint);
}


fn create_surface(
    window_context: &WindowedContext<PossiblyCurrent>,
    fb_info: &FramebufferInfo,
    gr_context: &mut skia_safe::gpu::Context,
) -> skia_safe::Surface {
    let pixel_format = window_context.get_pixel_format();
    let size = window_context.window().inner_size();
    let backend_render_target = BackendRenderTarget::new_gl(
        (
            size.width.try_into().unwrap(),
            size.height.try_into().unwrap(),
        ),
        pixel_format.multisampling.map(|s| s.try_into().unwrap()),
        pixel_format.stencil_bits.try_into().unwrap(),
        *fb_info,
    );
    let mut surface = Surface::from_backend_render_target(
        gr_context,
        &backend_render_target,
        SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        None,
        None,
    ).unwrap();
    let sf = window_context.window().scale_factor() as f32;
    surface.canvas().scale((sf, sf));
    surface
}
