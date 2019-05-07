#![cfg_attr(
not(any(feature = "dx12", feature = "metal", feature = "vulkan")),
allow(unused)
)]

mod window;

use window::WindowState;
use rendy::{
    command::{Families, QueueId, RenderPassEncoder},
    factory::{Config, Factory},
    graph::{
        present::PresentNode, render::*, Graph, GraphBuilder, GraphContext, NodeBuffer, NodeImage,
    },
    memory::Dynamic,
    mesh::PosColor,
    resource::{Buffer, BufferInfo, DescriptorSetLayout, Escape, Handle},
    shader::{ShaderKind, SourceLanguage, StaticShaderInfo},
};
use winit::Window;

#[cfg(feature = "vulkan")]
type Backend = rendy::vulkan::Backend;


#[cfg(any(feature = "dx12", feature = "metal", feature = "vulkan"))]
fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Warn)
        .filter_module("triangle", log::LevelFilter::Trace)
        .init();

    let mut window = WindowState::default();
    init(window.window);


}


fn init(window: Window) {
    let config: Config = Default::default();

    // Higher level device interface. Manges memory, resources and queue families.
    let (mut factory, mut families): (Factory<Backend>, _) = rendy::factory::init(config).expect("Failed to init factory");

    // Rendering target bound to window.
    let surface = factory.create_surface(window.into());

    // Build graph from nodes and resource.
    let mut graph_builder = GraphBuilder::<Backend, ()>::new();

    // Create new image owned by graph.
    let color = graph_builder.create_image(
        surface.kind(),
        1,
        factory.get_surface_format(&surface),
        Some(gfx_hal::command::ClearValue::Color(
            [1.0, 1.0, 0.0, 1.0].into(),
        ))
    );



}

#[cfg(not(any(feature = "dx12", feature = "metal", feature = "vulkan")))]
fn main() {
    panic!("Specify feature: { dx12, metal, vulkan }");
}
