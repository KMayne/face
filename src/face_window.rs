use std::convert::TryInto;

use gl::types::*;
use glutin::{ContextBuilder, PossiblyCurrent};
use glutin::event::{Event, KeyboardInput, ModifiersState, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::GlProfile;
use glutin::window::WindowBuilder;
use glutin::WindowedContext;
use skia_safe::{Color, ColorType, Surface};
use skia_safe::gpu::{BackendRenderTarget, SurfaceOrigin};
use skia_safe::gpu::gl::FramebufferInfo;

use crate::{renderer, layout};
use crate::layout::MarkupElement;

pub fn run_face_window(root_elem: MarkupElement) -> () {
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
                #[allow(deprecated)]
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
                    let image_info = canvas.image_info();
                    let rects = layout::generate_layout(&root_elem, image_info.width() as f32, image_info.height() as f32);
                    renderer::draw_ui(canvas, rects);
                }
                surface.canvas().flush();
                window_context.swap_buffers().unwrap();
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
