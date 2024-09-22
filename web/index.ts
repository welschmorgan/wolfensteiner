
import init, * as wasm from './node_modules/wolfensteiner/wolfensteiner.js';

(async () => {
    const {memory} = await init();

    const canvas: HTMLCanvasElement | null = document.querySelector('#game');
    const ctx = canvas?.getContext('2d');
    if (ctx) {
        canvas!.addEventListener('mousemove', (evt: MouseEvent) => {
            wasm.move_mouse(evt.movementX, evt.movementY);
        });
        ctx.fillStyle = '#000';
        ctx.fillRect(0, 0, canvas!.width, canvas!.height);
        wasm.init_game(canvas!.width, canvas!.height);
        const pixel_buf = wasm.get_buffer();
        let startTime: number = 0;
        function animate() {
            const dt = performance.now() - startTime;
            wasm.render_game(dt);
            const buf = new Uint8ClampedArray(memory.buffer.slice(pixel_buf.addr, pixel_buf.addr + pixel_buf.len));
            ctx?.putImageData(new ImageData(buf, canvas!.width, canvas!.height), 0, 0);
            startTime = performance.now();
            requestAnimationFrame(animate);
        }
        startTime = performance.now();
        requestAnimationFrame(animate);
    }
})()