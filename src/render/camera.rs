use bevy::{
    core_pipeline::ViewDepthTexture,
    ecs::prelude::*,
    render2::{
        camera::ActiveCameras,
        render_phase::RenderPhase,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsage,
        },
        renderer::RenderDevice,
        texture::TextureCache,
        view::ExtractedView,
    },
};

use super::node::TransparentUiPhase;

pub const CAMERA_UI: &str = "camera_ui";

pub fn extract_ui_camera(mut commands: Commands, active_cameras: Res<ActiveCameras>) {
    if let Some(camera_ui) = active_cameras.get(CAMERA_UI) {
        if let Some(entity) = camera_ui.entity {
            commands
                .get_or_spawn(entity)
                .insert(RenderPhase::<TransparentUiPhase>::default());
        }
    }
}

pub fn prepare_ui_views(
    mut commands: Commands,
    mut texture_cache: ResMut<TextureCache>,
    render_device: Res<RenderDevice>,
    views: Query<(Entity, &ExtractedView), With<RenderPhase<TransparentUiPhase>>>,
) {
    for (entity, view) in views.iter() {
        let cached_texture = texture_cache.get(
            &render_device,
            TextureDescriptor {
                label: None,
                size: Extent3d {
                    depth_or_array_layers: 1,
                    width: view.width as u32,
                    height: view.height as u32,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Depth32Float, /* PERF: vulkan docs recommend using 24
                                                      * bit depth for better performance */
                usage: TextureUsage::RENDER_ATTACHMENT,
            },
        );
        commands.entity(entity).insert(ViewDepthTexture {
            texture: cached_texture.texture,
            view: cached_texture.default_view,
        });
    }
}
