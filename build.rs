use protobuf_codegen::Codegen;

fn main() {
    Codegen::new()
        .pure()
        .includes(&["vendored/OpenMetrics"])
        .input("vendored/OpenMetrics/proto/openmetrics_data_model.proto")
        .out_dir("src/openmetrics")
        .run_from_script()
}
