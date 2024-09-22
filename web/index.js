var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
import init, * as wasm from './node_modules/wolfensteiner/wolfensteiner.js';
(() => __awaiter(void 0, void 0, void 0, function* () {
    const { memory } = yield init();
    const canvas = document.querySelector('#game');
    const ctx = canvas === null || canvas === void 0 ? void 0 : canvas.getContext('2d');
    if (ctx) {
        canvas.addEventListener('mousemove', (evt) => {
            wasm.move_mouse(evt.movementX, evt.movementY);
        });
        ctx.fillStyle = '#000';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        wasm.init_game(canvas.width, canvas.height);
        const pixel_buf = wasm.get_buffer();
        let startTime = 0;
        function animate() {
            const dt = performance.now() - startTime;
            wasm.render_game(dt);
            const buf = new Uint8ClampedArray(memory.buffer.slice(pixel_buf.addr, pixel_buf.addr + pixel_buf.len));
            ctx === null || ctx === void 0 ? void 0 : ctx.putImageData(new ImageData(buf, canvas.width, canvas.height), 0, 0);
            startTime = performance.now();
            requestAnimationFrame(animate);
        }
        startTime = performance.now();
        requestAnimationFrame(animate);
    }
}))();
