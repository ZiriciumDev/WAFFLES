import initWasm, { WasmProgram, init_program as initChip8 } from '@firfi/rust-wasm-chip8';

const init = async () => {

    await initWasm();
    
    const romData = await loadRom();
    const canvas = document.getElementById("canvas");
    
    const cpu = initChip8(romData, canvas);
    cpu.run(); // will run asynchronously
    initKeyboardListeners(cpu); // wire up controls, listen to kb events etc

    // and at any point you call cpu.stop() which will clean up its resources  
}
