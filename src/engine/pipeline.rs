use super::Encode;


pub struct PointcloudPipeline {
    render_pipeline: wgpu::RenderPipeline,
}

impl PointcloudPipeline {
    pub fn new(
        device: &wgpu::Device, 
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
        format: wgpu::TextureFormat,
    ) -> Self {

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Pointcloud Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/intensity.wgsl").into()),
        });
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render PointcloudPipeline Layout"),
            bind_group_layouts: &[ &uniform_bind_group_layout ],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render PointcloudPipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main".into(),
                buffers: &[
                    wgpu::VertexBufferLayout { // Point
                        attributes: &[
                            wgpu::VertexAttribute {
                                shader_location: 0,
                                offset: 0,
                                format: wgpu::VertexFormat::Float32x3,
                            }
                        ],
                        array_stride: 12,
                        step_mode: wgpu::VertexStepMode::Instance,
                    },
                    wgpu::VertexBufferLayout { // Intensity
                        attributes: &[
                            wgpu::VertexAttribute {
                                shader_location: 1,
                                offset: 0,
                                format: wgpu::VertexFormat::Float32,
                            }
                        ],
                        array_stride: 4,
                        step_mode: wgpu::VertexStepMode::Instance,
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main".into(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
            cache: None,
        });

        Self {
            render_pipeline,
        }
    }
}

impl<'a> Encode<'a> for PointcloudPipeline {
    fn record_command(&'a self, recorder: &mut impl wgpu::util::RenderEncoder<'a>) {
        recorder.set_pipeline(&self.render_pipeline);
    }
}