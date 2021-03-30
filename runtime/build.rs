use substrate_wasm_builder::WasmBuilder;

fn main() {
    WasmBuilder::new()
        .with_current_project()
        // .with_wasm_builder_from_crates("3.0.0")
        .export_heap_base()
        .import_memory()
        .build()
}
