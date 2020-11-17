use wasmer::{ImportObject, Instance, Module, Store};

fn main() {
    let store = Store::default();
    let module = Module::new(&store, "(module)").unwrap();

    let import_object = ImportObject::new();
    let instance1 = Instance::new(&module, &import_object).unwrap();
    let _instance2 = instance1.clone();
    let _instance3 = instance1.clone();
}
