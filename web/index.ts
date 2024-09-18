
import init, * as wasm from './node_modules/wolfensteiner/wolfensteiner.js';

(async () => {
    const {memory} = await init();

    const canvas: HTMLCanvasElement | null = document.querySelector('#game');
    const ctx = canvas?.getContext('2d');
    if (ctx) {
        ctx.fillStyle = '#000';
        ctx.fillRect(0, 0, canvas!.width, canvas!.height);
        wasm.init_game(canvas!.width, canvas!.height);
        const pixel_buf = wasm.get_buffer();
        console.log(`pixel_buf: ${pixel_buf.width}x${pixel_buf.height}, len = ${pixel_buf.len}`);
        function animate() {
            wasm.render_game();
            const buf = new Uint8ClampedArray(memory.buffer.slice(pixel_buf.addr, pixel_buf.addr + pixel_buf.len));
            ctx?.putImageData(new ImageData(buf, canvas!.width, canvas!.height), 0, 0);
            requestAnimationFrame(animate);
        }
        requestAnimationFrame(animate);
    }
})()