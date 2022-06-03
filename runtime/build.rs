// Copyright 2021-2022 Green Aureus GmbH

// Permission is hereby granted, free of charge, to any person obtaining a copy 
// of this software and associated documentation files (the "Software"), to read 
// the Software only. Permission is hereby NOT GRANTED to use, copy, modify, 
// merge, publish, distribute, sublicense, and/or sell copies of the Software.

use substrate_wasm_builder::WasmBuilder;

fn main() {
    WasmBuilder::new()
        .with_current_project()
        .export_heap_base()
        .import_memory()
        .build()
}
