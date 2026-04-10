//! Rendering system using wgpu.

use wgpu::*;

/// Renderer state
pub struct Renderer {
    pub surface: Option<Surface<'static>>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
}

impl Renderer {
    pub async fn new(window: &winit::window::Window) -> anyhow::Result<Self> {
        let instance = Instance::new(&InstanceDescriptor::default());

        let surface = instance.create_surface(window.clone())?;

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .ok_or_else(|| anyhow::anyhow!("No adapter found"))?;

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    required_features: Features::empty(),
                    required_limits: Limits::downlevel_defaults(),
                    ..Default::default()
                },
                None,
            )
            .await?;

        let size = window.inner_size();
        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .ok_or_else(|| anyhow::anyhow!("Surface not supported"))?;

        surface.configure(&device, &config);

        Ok(Self {
            surface: Some(surface),
            device,
            queue,
            config,
        })
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        // Render frame
        Ok(())
    }
}
