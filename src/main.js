async function setupCanvas() {
    try {
        // await init(); // Initialize the WebAssembly module

        const canvas = document.getElementById('noise-canvas');
        const ctx = canvas.getContext('2d');

        function resizeCanvas() {
            canvas.width = window.innerWidth;
            canvas.height = window.innerHeight;
            console.log('Before generate_noise');
            window.wasmBindings.generate_noise(ctx, canvas.width, canvas.height, 50.0); // Adjust parameters as needed
            console.log('After generate_noise');
        }

        window.addEventListener('resize', resizeCanvas);
        resizeCanvas(); // Initial call to set up canvas
    } catch (error) {
        console.error('Error during WebAssembly module initialization:', error);
    }
}


//async function run() {
//    const scriptTag = document.querySelector('script[data-trunk-script]');
//    const wasmPath = scriptTag.getAttribute('data-trunk-wasm-path');
//
//    const { generate_noise } = await import(wasmPath);
//
//    const canvas = document.getElementById('noise-canvas');
//    const ctx = canvas.getContext('2d');
//
//    function resizeCanvas() {
//        canvas.width = window.innerWidth;
//        canvas.height = window.innerHeight;
//        console.log('Before generate_noise');
//        generate_noise(ctx, canvas.width, canvas.height, 50.0);
//        console.log('After generate_noise');
//    }
//    window.addEventListener('resize', resizeCanvas);
//    resizeCanvas();
//}
//
//run();
//
