//! Integration tests for SigmaOS-arch-shader-ai-kernel.

use sigmaos_arch_shader_ai_kernel_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
