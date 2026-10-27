use wasm_bindgen::prelude::*;
use wasm_mutex
use wasm_bindgen_futures
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

static mut TIME: f64 = 0.0;

#[wasm_bindgen]
pub fn init_program(program: &[u8], canvas: JsValue) -> Result<WasmProgram, JsValue> {
   match canvas.dyn_into::<web_sys::CanvasRenderingContext2d>() {
       Ok(canvas) => {
           // in the code, screen is interchangeable; CPU constructor accepts also a ConsoleScreen!
           let mut cpu = CPU::new(Box::new(WasmCanvasScreen::new(canvas)));
           cpu.load_program(program.to_vec());
           Ok(WasmProgram { cpu: Arc::new(wasm_mutex::Mutex::new(cpu)) })
       }
       Err(_) => Err(JsValue::from_str("canvas argument not a HtmlCanvas")),
   }
}

pub async fn run(this: Arc<wasm_mutex::Mutex<CPU>>) {
   let arc = this.clone();
   loop {
       Delay::new(Duration::new(1 / SPEED, 0)).await;
       let mut guard = arc.lock().await;
       let guard: &mut CPU = guard.deref_mut(); // for compiler to understand splitting borrow later
       // is_done() will return true after we call cpu.stop()
       if guard.is_done() {
           break;
       }
       // we also call requestAnimationFrame https://developer.mozilla.org/en-US/docs/Web/API/window/requestAnimationFrame from inside the lib!
       guard.screen.request_animation_frame().await;
       // whereas CPU::cycle is chip8 specific method and doesn’t contain nothing we should focus on
       match CPU::cycle(&mut guard.state, &mut *guard.screen) {
           Ok(()) => (),
           Err(e) => {
               println!("Error during cycle, {}. STOPPING", e);
               guard.stop();
           }
       }
   }
}

#[wasm_bindgen]
pub struct WasmProgram {
   // we want wasm_bindgen to skip this field as it’s not compatible with wasm_bindgen, i.e. our chip8 has some incompatibility with wasm_bindgen structures (notably, it contains a reference to Canvas). You could have better luck with your simulation!
   cpu: Arc<Mutex<CPU>>,
}

#[wasm_bindgen]
impl WasmProgram {
   pub fn run(&self) {
       let clone = self.cpu.clone();
       spawn_local(async move {
           CPU::run(clone).await;
       })
   }
   pub fn stop(&mut self) {
       let clone = self.cpu.clone();
       spawn_local(async move {
           let mut guard = clone.lock().await;
           guard.stop();
       })
   }
    pub fn key_down(&mut self, k: usize) {
       let clone = self.cpu.clone();
       spawn_local(async move {
           let mut guard = clone.lock().await;
           guard.key_down(k);
       })
    }
    
    pub fn key_up(&mut self, k: usize) {
       let clone = self.cpu.clone();
       spawn_local(async move {
           let mut guard = clone.lock().await;
           guard.key_up(k);
       })
    }
}
