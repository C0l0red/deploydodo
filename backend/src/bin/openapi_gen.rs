use backend::openapi::openapi_json;

fn main() {
    std::fs::write("frontend/openapi.json", openapi_json()).unwrap();
}
